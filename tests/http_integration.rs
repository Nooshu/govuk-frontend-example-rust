//! HTTP integration tests for the example service.

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

async fn get(path: &str) -> (u16, String) {
    let app = router(test_state());
    let res = app
        .oneshot(
            axum::http::Request::builder()
                .uri(path)
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = res.status().as_u16();
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    (status, String::from_utf8_lossy(&bytes).into_owned())
}

#[tokio::test]
async fn health_ok() {
    let (status, body) = get("/health").await;
    assert_eq!(status, 200);
    assert_eq!(body, "ok");
}

#[tokio::test]
async fn robots_disallow() {
    let (status, body) = get("/robots.txt").await;
    assert_eq!(status, 200);
    assert!(body.contains("Disallow: /"));
    assert!(body.contains("User-agent: *"));
    assert!(body.contains("Googlebot"));
}

#[tokio::test]
async fn html_pages_are_noindex() {
    let app = router(test_state());
    let res = app
        .oneshot(
            axum::http::Request::builder()
                .uri("/")
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let robots = res
        .headers()
        .get("x-robots-tag")
        .expect("X-Robots-Tag")
        .to_str()
        .unwrap();
    assert!(robots.contains("noindex"));
    assert!(robots.contains("nofollow"));
    assert!(robots.contains("noarchive"));
    let bytes = res.into_body().collect().await.unwrap().to_bytes();
    let body = String::from_utf8_lossy(&bytes);
    assert!(body.contains(
        r#"name="robots" content="noindex, nofollow, noarchive, nosnippet, noimageindex""#
    ));
}

#[tokio::test]
async fn start_page_renders() {
    let (status, body) = get("/").await;
    assert_eq!(status, 200);
    assert!(body.contains("Apply for a rod fishing licence"));
    assert!(body.contains("Start now"));
    assert!(body.contains("govuk-template"));
}

#[tokio::test]
async fn welsh_start_page() {
    let (status, body) = get("/cy").await;
    assert_eq!(status, 200);
    assert!(body.contains("Dechrau nawr"));
}

#[tokio::test]
async fn catalogue_lists_components() {
    let (status, body) = get("/components").await;
    assert_eq!(status, 200);
    assert!(body.contains("Component catalogue"));
    assert!(body.contains("/components/button"));
}

#[tokio::test]
async fn button_preview_shows_parity_when_matching() {
    let (status, body) = get("/components/button").await;
    assert_eq!(status, 200);
    assert!(
        body.contains("HTML matches the fixture"),
        "expected green parity banner when Rust ≡ fixture"
    );
}

#[tokio::test]
async fn task_list_and_name_question() {
    let (status, body) = get("/task-list").await;
    assert_eq!(status, 200);
    assert!(body.contains("What is your name?"));

    let (status, body) = get("/name").await;
    assert_eq!(status, 200);
    assert!(body.contains("novalidate"));
    assert!(body.contains("Save and continue"));
}

#[tokio::test]
async fn about_and_fees() {
    let (status, _) = get("/about").await;
    assert_eq!(status, 200);
    let (status, body) = get("/fees").await;
    assert_eq!(status, 200);
    assert!(body.contains("Licence fees"));
}

#[tokio::test]
async fn stylesheet_fingerprinted() {
    let state = test_state();
    let href = state.assets.stylesheet_href.clone();
    assert!(href.starts_with("/assets/application."));
    assert!(href.ends_with(".css"));
    let app = router(state);
    let res = app
        .oneshot(
            axum::http::Request::builder()
                .uri(&href)
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    assert!(res
        .headers()
        .get("content-type")
        .unwrap()
        .to_str()
        .unwrap()
        .contains("text/css"));
}
