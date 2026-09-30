//! Extra HTTP journey and static-page coverage.

use axum::http::header;
use govuk_frontend_example_rust::baseline::Policy;
use govuk_frontend_example_rust::config::Config;
use govuk_frontend_example_rust::httpx::Assets;
use govuk_frontend_example_rust::session::SessionStore;
use govuk_frontend_example_rust::web::{router, AppState};
use http_body_util::BodyExt;
use std::sync::Arc;
use tower::ServiceExt;

fn test_state() -> AppState {
    let root = std::env::current_dir().expect("cwd");
    let config = Arc::new(Config::from_root(root).expect("config"));
    let policy = Arc::new(Policy::load(&config.policy_file).expect("policy"));
    let assets =
        Assets::load(&config.stylesheet, &config.frontend_assets, policy, false).expect("assets");
    AppState {
        config,
        assets,
        sessions: SessionStore::new(),
    }
}

async fn request(
    state: AppState,
    method: &str,
    path: &str,
    cookie: Option<&str>,
    body: Option<&str>,
) -> (u16, String, Option<String>) {
    let app = router(state);
    let mut builder = axum::http::Request::builder().method(method).uri(path);
    if let Some(c) = cookie {
        builder = builder.header(header::COOKIE, format!("rod_session={c}"));
    }
    let req = if let Some(b) = body {
        builder
            .header(header::CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(axum::body::Body::from(b.to_string()))
            .unwrap()
    } else {
        builder.body(axum::body::Body::empty()).unwrap()
    };
    let res = app.oneshot(req).await.unwrap();
    let status = res.status().as_u16();
    let set_cookie = res
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .find_map(|v| {
            v.split(';')
                .next()
                .and_then(|part| {
                    part.strip_prefix("rod_session=")
                        .or_else(|| part.strip_prefix("session="))
                        .or_else(|| part.strip_prefix("__Host-session="))
                })
                .map(str::to_string)
        });
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        String::from_utf8_lossy(&bytes).into_owned(),
        set_cookie,
    )
}

#[tokio::test]
async fn journey_name_validation_and_success() {
    let state = test_state();
    let (status, _body, sid) = request(state.clone(), "GET", "/name", None, None).await;
    assert_eq!(status, 200);
    let sid = sid.expect("session cookie");

    let (status, body, _) = request(
        state.clone(),
        "POST",
        "/name",
        Some(&sid),
        Some("first-name=&last-name="),
    )
    .await;
    assert_eq!(status, 200);
    assert!(body.contains("There is a problem") || body.contains("Enter your first name"));

    let (status, _, _) = request(
        state,
        "POST",
        "/name",
        Some(&sid),
        Some("first-name=Sam&last-name=Taylor"),
    )
    .await;
    assert_eq!(status, 303);
}

#[tokio::test]
async fn confirmation_requires_submit() {
    let (status, _, _) = request(test_state(), "GET", "/confirmation", None, None).await;
    assert_eq!(status, 303);
}

#[tokio::test]
async fn static_and_example_pages() {
    let state = test_state();
    for path in [
        "/help",
        "/guidance",
        "/accessibility",
        "/updates",
        "/cookies",
        "/examples",
        "/examples/exit-this-page",
        "/examples/service-unavailable",
        "/examples/problem-with-the-service",
        "/cy",
        "/date-of-birth",
        "/fees",
    ] {
        let (status, _, _) = request(state.clone(), "GET", path, None, None).await;
        assert_eq!(status, 200, "{path}");
    }
}

#[tokio::test]
async fn generic_step_continues() {
    let state = test_state();
    let (_, _, sid) = request(state.clone(), "GET", "/date-of-birth", None, None).await;
    let sid = sid.expect("session");
    let (status, _, _) = request(
        state,
        "POST",
        "/date-of-birth",
        Some(&sid),
        Some("continue=1"),
    )
    .await;
    assert_eq!(status, 303);
}

#[tokio::test]
async fn raw_fixture_fragment() {
    let (status, body, _) = request(
        test_state(),
        "GET",
        "/components/button/fixture?fixture=default",
        None,
        None,
    )
    .await;
    assert_eq!(status, 200);
    assert!(body.contains("govuk-button"));
}

#[tokio::test]
async fn cookie_choices_post() {
    let state = test_state();
    let (status, body, sid) = request(state.clone(), "GET", "/", None, None).await;
    assert_eq!(status, 200);
    let sid = sid.expect("session");
    let csrf = body
        .split("name=\"csrf\" value=\"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .expect("csrf");
    let form = format!("csrf={csrf}&returnPath=%2F&cookies=accept");
    let (status, _, _) = request(state, "POST", "/cookie-choices", Some(&sid), Some(&form)).await;
    assert_eq!(status, 303);
}

#[tokio::test]
async fn new_application_resets() {
    let (status, _, _) = request(test_state(), "GET", "/new-application", None, None).await;
    assert_eq!(status, 303);
}

#[tokio::test]
async fn policy_headers_on_html() {
    let app = router(test_state());
    let res = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/about")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert!(res.headers().get("content-security-policy").is_some());
    assert!(res.headers().get("x-content-type-options").is_some());
}
