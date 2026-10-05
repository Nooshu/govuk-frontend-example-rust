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
) -> (u16, String, Option<String>, Option<String>) {
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
    let location = res
        .headers()
        .get(header::LOCATION)
        .and_then(|v| v.to_str().ok())
        .map(str::to_string);
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
        location,
    )
}

#[tokio::test]
async fn journey_name_validation_and_success() {
    let state = test_state();
    let (status, _body, sid, _) = request(state.clone(), "GET", "/name", None, None).await;
    assert_eq!(status, 200);
    let sid = sid.expect("session cookie");

    let (status, body, _, _) = request(
        state.clone(),
        "POST",
        "/name",
        Some(&sid),
        Some("full-name="),
    )
    .await;
    assert_eq!(status, 200);
    assert!(body.contains("There is a problem") || body.contains("Enter your full name"));

    let (status, _, _, location) = request(
        state,
        "POST",
        "/name",
        Some(&sid),
        Some("full-name=Sam+Taylor"),
    )
    .await;
    assert_eq!(status, 303);
    assert_eq!(location.as_deref(), Some("/date-of-birth"));
}

#[tokio::test]
async fn confirmation_requires_submit() {
    let (status, _, _, location) = request(test_state(), "GET", "/confirmation", None, None).await;
    assert_eq!(status, 303);
    assert_eq!(location.as_deref(), Some("/licence-length"));
}

#[tokio::test]
async fn full_licence_journey() {
    let state = test_state();
    let (status, body, sid, _) = request(state.clone(), "GET", "/", None, None).await;
    assert_eq!(status, 200);
    assert!(body.contains("href=\"/licence-length\""));
    let sid = sid.expect("session");

    let (status, body, _, _) =
        request(state.clone(), "GET", "/licence-length", Some(&sid), None).await;
    assert_eq!(status, 200);
    assert!(body.contains("How long do you need the licence for?"));
    assert!(body.contains("1 day"));
    assert!(!body.contains("£"));

    let (status, _, _, location) = request(
        state.clone(),
        "POST",
        "/licence-length",
        Some(&sid),
        Some("licence-length=12-months"),
    )
    .await;
    assert_eq!(status, 303);
    assert_eq!(location.as_deref(), Some("/name"));

    let (status, _, _, location) = request(
        state.clone(),
        "POST",
        "/name",
        Some(&sid),
        Some("full-name=Ada+Lovelace"),
    )
    .await;
    assert_eq!(status, 303);
    assert_eq!(location.as_deref(), Some("/date-of-birth"));

    let (status, body, _, _) =
        request(state.clone(), "GET", "/date-of-birth", Some(&sid), None).await;
    assert_eq!(status, 200);
    assert!(body.contains("For example, 31 3 1980"));

    let (status, _, _, location) = request(
        state.clone(),
        "POST",
        "/date-of-birth",
        Some(&sid),
        Some("date-of-birth-day=10&date-of-birth-month=12&date-of-birth-year=1815"),
    )
    .await;
    assert_eq!(status, 303);
    assert_eq!(location.as_deref(), Some("/where-you-will-fish"));

    let (status, body, _, _) = request(
        state.clone(),
        "GET",
        "/where-you-will-fish",
        Some(&sid),
        None,
    )
    .await;
    assert_eq!(status, 200);
    assert!(body.contains("England"));
    assert!(body.contains("This example is fictional"));

    let (status, _, _, location) = request(
        state.clone(),
        "POST",
        "/where-you-will-fish",
        Some(&sid),
        Some("country=England"),
    )
    .await;
    assert_eq!(status, 303);
    assert_eq!(location.as_deref(), Some("/email"));

    let (status, body, _, _) = request(state.clone(), "GET", "/email", Some(&sid), None).await;
    assert_eq!(status, 200);
    assert!(body.contains("browser session only"));

    let (status, _, _, location) = request(
        state.clone(),
        "POST",
        "/email",
        Some(&sid),
        Some("email=ada%40example.com"),
    )
    .await;
    assert_eq!(status, 303);
    assert_eq!(location.as_deref(), Some("/check-answers"));

    let (status, body, _, _) =
        request(state.clone(), "GET", "/check-answers", Some(&sid), None).await;
    assert_eq!(status, 200);
    assert!(body.contains("12 months"));
    assert!(body.contains("Ada Lovelace"));
    assert!(body.contains("10 12 1815"));
    assert!(body.contains("Accept and continue"));

    let (status, _, _, location) = request(
        state.clone(),
        "POST",
        "/check-answers",
        Some(&sid),
        Some(""),
    )
    .await;
    assert_eq!(status, 303);
    assert_eq!(location.as_deref(), Some("/confirmation"));

    let (status, body, _, _) =
        request(state.clone(), "GET", "/confirmation", Some(&sid), None).await;
    assert_eq!(status, 200);
    assert!(body.contains("Your example reference number"));
    assert!(body.contains("Nobody will send you a fishing rod licence"));
    assert!(body.contains("href=\"/components\""));
    assert!(body.contains("Back to the component list"));
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
        "/licence-length",
    ] {
        let (status, _, _, _) = request(state.clone(), "GET", path, None, None).await;
        assert_eq!(status, 200, "{path}");
    }
}

#[tokio::test]
async fn raw_fixture_fragment() {
    let (status, body, _, _) = request(
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
    let (status, body, sid, _) = request(state.clone(), "GET", "/", None, None).await;
    assert_eq!(status, 200);
    let sid = sid.expect("session");
    let csrf = body
        .split("name=\"csrf\" value=\"")
        .nth(1)
        .and_then(|s| s.split('"').next())
        .expect("csrf");
    let form = format!("csrf={csrf}&returnPath=%2F&cookies=accept");
    let (status, _, _, _) =
        request(state, "POST", "/cookie-choices", Some(&sid), Some(&form)).await;
    assert_eq!(status, 303);
}

#[tokio::test]
async fn new_application_resets() {
    let (status, _, _, _) = request(test_state(), "GET", "/new-application", None, None).await;
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
