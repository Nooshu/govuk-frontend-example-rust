//! Axum routes for the example service.

mod catalogue;
mod journey;
mod static_pages;

use crate::baseline::CacheKind;
use crate::config::Config;
use crate::httpx::{self, Assets};
use crate::session::SessionStore;
use axum::body::Body;
use axum::extract::State;
use axum::http::{header, HeaderMap, HeaderValue, Response, StatusCode};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Form, Router};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub assets: Assets,
    pub sessions: SessionStore,
}

pub fn router(state: AppState) -> Router {
    let mut app = Router::new()
        .route("/health", get(health))
        .route("/robots.txt", get(robots))
        .route("/assets/*path", get(httpx::serve_assets))
        .route("/", get(journey::start_en))
        .route("/cy", get(journey::start_cy))
        .route("/new-application", get(journey::new_application))
        .route("/task-list", get(journey::task_list))
        .route("/name", get(journey::name_get).post(journey::name_post))
        .route("/email", get(journey::email_get).post(journey::email_post))
        .route(
            "/create-a-password",
            get(journey::password_get).post(journey::password_post),
        )
        .route(
            "/check-answers",
            get(journey::check_answers_get).post(journey::check_answers_post),
        )
        .route("/confirmation", get(journey::confirmation))
        .route("/cookie-choices", post(cookie_choices))
        .route(
            "/cookies",
            get(static_pages::cookies).post(static_pages::cookies_post),
        )
        .route("/about", get(static_pages::about))
        .route("/help", get(static_pages::help))
        .route("/fees", get(static_pages::fees))
        .route("/guidance", get(static_pages::guidance))
        .route("/accessibility", get(static_pages::accessibility))
        .route("/updates", get(static_pages::updates));

    for path in [
        "/date-of-birth",
        "/contact-preference",
        "/where-you-will-fish",
        "/licence-length",
        "/start-month",
        "/address",
        "/evidence",
        "/additional-details",
    ] {
        app = app.route(path, get(journey::generic_get).post(journey::generic_post));
    }

    if state.config.demos_enabled {
        app = app
            .route("/components", get(catalogue::index))
            .route("/components/:name", get(catalogue::component))
            .route("/components/:name/fixture", get(catalogue::raw_fixture))
            .route("/examples", get(static_pages::examples_index))
            .route(
                "/examples/exit-this-page",
                get(static_pages::exit_this_page),
            )
            .route(
                "/examples/service-unavailable",
                get(static_pages::unavailable),
            )
            .route(
                "/examples/problem-with-the-service",
                get(static_pages::problem),
            );
    }

    app.fallback(not_found).with_state(state)
}

async fn health() -> impl IntoResponse {
    (
        [(
            header::HeaderName::from_static("x-robots-tag"),
            "noindex, nofollow, noarchive, nosnippet, noimageindex",
        )],
        "ok",
    )
}

async fn robots() -> impl IntoResponse {
    (
        [
            (header::CONTENT_TYPE, "text/plain; charset=utf-8"),
            (
                header::HeaderName::from_static("x-robots-tag"),
                "noindex, nofollow, noarchive, nosnippet, noimageindex",
            ),
        ],
        concat!(
            "# This is a demonstration service — do not index any path.\n",
            "User-agent: *\n",
            "Disallow: /\n",
            "\n",
            "User-agent: Googlebot\n",
            "Disallow: /\n",
            "\n",
            "User-agent: Googlebot-Image\n",
            "Disallow: /\n",
            "\n",
            "User-agent: Bingbot\n",
            "Disallow: /\n",
        ),
    )
}

async fn not_found(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let content = r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        <h1 class="govuk-heading-l">Page not found</h1>
        <p class="govuk-body">If you typed the web address, check it is correct.</p>
        <p class="govuk-body"><a class="govuk-link" href="/">Go to the start page</a></p>
        </div></div>"#
        .to_string();
    let html = crate::pages::render_page(crate::pages::Page {
        title: "Page not found",
        content,
        back_href: None,
        breadcrumbs: false,
        sensitive: false,
        welsh: false,
        show_feedback: false,
        assets: &state.assets,
        session: &data,
        return_path: "/",
    });
    let mut res = html_response(&state, html, &sid, false);
    *res.status_mut() = StatusCode::NOT_FOUND;
    res
}

#[derive(Deserialize)]
struct CookieForm {
    csrf: String,
    #[serde(rename = "returnPath")]
    return_path: String,
    cookies: String,
}

async fn cookie_choices(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<CookieForm>,
) -> Response<Body> {
    let sid = session_id_from(&headers);
    let (sid, mut data) = state.sessions.get_or_create(sid.as_deref());
    if form.csrf == data.csrf {
        match form.cookies.as_str() {
            "accept" | "reject" => {
                data.cookie_choice = Some(form.cookies);
            }
            "hide" => {}
            _ => {}
        }
        state.sessions.save(&sid, data);
    }
    let dest = safe_return(&form.return_path);
    redirect_with_session(dest, &sid, state.assets.secure_transport)
}

fn safe_return(value: &str) -> &str {
    if !value.starts_with('/')
        || value.starts_with("//")
        || value.contains("://")
        || value.contains('\\')
        || value.contains('\r')
        || value.contains('\n')
    {
        "/"
    } else {
        value
    }
}

pub fn session_id_from(headers: &HeaderMap) -> Option<String> {
    headers
        .get(header::COOKIE)
        .and_then(|v| v.to_str().ok())
        .and_then(|c| {
            c.split(';').find_map(|part| {
                let part = part.trim();
                part.strip_prefix("rod_session=")
                    .or_else(|| part.strip_prefix("__Host-session="))
                    .or_else(|| part.strip_prefix("session="))
                    .map(|v| v.trim().to_string())
            })
        })
}

pub fn html_response(state: &AppState, body: String, sid: &str, sensitive: bool) -> Response<Body> {
    let kind = if sensitive {
        CacheKind::SensitiveDocument
    } else {
        CacheKind::Document
    };
    let mut res = Response::new(Body::from(body));
    *res.status_mut() = StatusCode::OK;
    let headers = res.headers_mut();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/html; charset=utf-8"),
    );
    let cookie_name = if state.assets.secure_transport {
        "__Host-session"
    } else {
        "rod_session"
    };
    let cookie = format!(
        "{cookie_name}={sid}; Path=/; HttpOnly; SameSite=Lax; Max-Age=14400{}",
        if state.assets.secure_transport {
            "; Secure"
        } else {
            ""
        }
    );
    headers.append(header::SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());
    httpx::apply_policy_headers(
        headers,
        &state.assets.policy,
        kind,
        state.assets.secure_transport,
    );
    res
}

pub fn redirect_with_session(path: &str, sid: &str, secure: bool) -> Response<Body> {
    let mut res = Response::new(Body::empty());
    *res.status_mut() = StatusCode::SEE_OTHER;
    let headers = res.headers_mut();
    headers.insert(header::LOCATION, HeaderValue::from_str(path).unwrap());
    headers.insert(
        header::HeaderName::from_static("x-robots-tag"),
        HeaderValue::from_static("noindex, nofollow, noarchive, nosnippet, noimageindex"),
    );
    if !sid.is_empty() {
        let cookie_name = if secure {
            "__Host-session"
        } else {
            "rod_session"
        };
        let cookie = format!(
            "{cookie_name}={sid}; Path=/; HttpOnly; SameSite=Lax; Max-Age=14400{}",
            if secure { "; Secure" } else { "" }
        );
        headers.append(header::SET_COOKIE, HeaderValue::from_str(&cookie).unwrap());
    }
    res
}
