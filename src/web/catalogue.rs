//! Component catalogue and fixture preview pages.

use crate::govuk::{fixture_components, load_fixtures, must_render, params, render, v_str};
use crate::pages::{html_escape, render_page, Page};
use crate::web::{html_response, session_id_from, AppState};
use axum::body::Body;
use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, Response};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ComponentQuery {
    fixture: Option<String>,
}

const DESCRIPTIONS: &[(&str, &str)] = &[
    (
        "accordion",
        "Show and hide sections of related content on a page.",
    ),
    (
        "back-link",
        "Help users go back to the previous page in a multi-page transaction.",
    ),
    (
        "breadcrumbs",
        "Help users understand where they are and move between pages.",
    ),
    ("button", "Help users carry out an action."),
    (
        "character-count",
        "Help users enter text within a character limit.",
    ),
    ("checkboxes", "Let users select one or more options."),
    ("cookie-banner", "Allow users to accept or reject cookies."),
    ("date-input", "Let users enter a date."),
    (
        "details",
        "Make a page easier to scan by letting users reveal more information.",
    ),
    (
        "error-message",
        "Show an error message next to a form field.",
    ),
    (
        "error-summary",
        "Summarise form errors at the top of a page.",
    ),
    ("exit-this-page", "Help users leave a page quickly."),
    ("feedback", "Ask for feedback about the service."),
    ("fieldset", "Group related form fields."),
    ("file-upload", "Let users upload a file."),
    ("footer", "The GOV.UK footer."),
    ("generic-header", "A generic application header."),
    ("header", "The GOV.UK header."),
    ("hint", "Extra help for a form field."),
    ("input", "Lets users enter a single line of text."),
    (
        "inset-text",
        "Draws attention to important content on the page.",
    ),
    ("label", "Labels a form field."),
    (
        "language-navigation",
        "Lets users switch between languages.",
    ),
    (
        "notification-banner",
        "Tells users about something that affects the whole service.",
    ),
    ("pagination", "Splits a long list across pages."),
    ("panel", "Confirms a transaction is complete."),
    ("password-input", "Lets users enter a password."),
    (
        "phase-banner",
        "Shows users that the service is still being tried out.",
    ),
    ("radios", "Lets users select one option from a list."),
    ("select", "Lets users choose one option from a long list."),
    (
        "service-navigation",
        "Shows the service name under the GOV.UK masthead.",
    ),
    ("skip-link", "Lets keyboard users skip to the main content."),
    (
        "summary-list",
        "Summarises answers so users can check them.",
    ),
    ("table", "Shows information in rows and columns."),
    ("tabs", "Lets users switch between related views."),
    ("tag", "Shows a short status."),
    (
        "task-list",
        "Shows the tasks in an application and whether they are done.",
    ),
    ("textarea", "Lets users enter more than one line of text."),
    (
        "warning-text",
        "Tells users about something important before they continue.",
    ),
];

pub async fn index(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let names = fixture_components(&state.config.components_root).unwrap_or_default();
    let mut list = String::from(r#"<ul class="govuk-list">"#);
    for name in &names {
        let desc = DESCRIPTIONS
            .iter()
            .find(|(n, _)| *n == name.as_str())
            .map(|(_, d)| *d)
            .unwrap_or("");
        let title = title_case(name);
        list.push_str(&format!(
            r#"<li><h2 class="govuk-heading-s govuk-!-margin-bottom-1"><a class="govuk-link" href="/components/{name}">{title}</a></h2><p class="govuk-body">{desc}</p></li>"#,
            desc = html_escape(desc),
            title = html_escape(&title),
        ));
    }
    list.push_str("</ul>");
    let content = format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        <h1 class="govuk-heading-xl">Component catalogue</h1>
        <p class="govuk-body">Preview of each GOV.UK Frontend component rendered by this service’s Rust library (Frontend {}).</p>
        {list}
        </div></div>"#,
        html_escape(&state.config.frontend_version)
    );
    let html = render_page(Page {
        title: "Component catalogue",
        content,
        back_href: None,
        breadcrumbs: true,
        sensitive: false,
        welsh: false,
        show_feedback: false,
        assets: &state.assets,
        session: &data,
        return_path: "/components",
    });
    html_response(&state, html, &sid, false)
}

pub async fn component(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Query(q): Query<ComponentQuery>,
    headers: HeaderMap,
) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let Ok(set) = load_fixtures(&state.config.components_root, &name) else {
        return html_response(&state, "Component not found".into(), &sid, false);
    };
    let fixture = match select_fixture(&set.fixtures, q.fixture.as_deref()) {
        Some(f) => f,
        None => return html_response(&state, "Fixture not found".into(), &sid, false),
    };
    let fixture_name = fixture.name.clone();

    let rendered = render(&name, &fixture.options).unwrap_or_default();
    let matches = rendered == fixture.html;

    // Only show the green “HTML matches” panel when equality is actually true.
    let parity = if matches {
        must_render(
            "notification-banner",
            &params(&[
                ("type", v_str("success")),
                ("titleText", v_str("HTML matches the fixture")),
                (
                    "text",
                    v_str("The Rust output is the same as the official fixture HTML."),
                ),
            ]),
        )
    } else {
        must_render(
            "notification-banner",
            &params(&[
                ("titleText", v_str("HTML does not match the fixture")),
                (
                    "text",
                    v_str("The Rust output is different from the official fixture HTML."),
                ),
            ]),
        )
    };

    let mut versions = String::from(
        r#"<h2 class="govuk-heading-m">Versions (Fixtures)</h2><ul class="govuk-list">"#,
    );
    for f in &set.fixtures {
        let current = if f.name == fixture_name {
            r#" <strong class="govuk-tag govuk-tag--blue">Current</strong>"#
        } else {
            ""
        };
        versions.push_str(&format!(
            r#"<li><a class="govuk-link" href="/components/{name}?fixture={}">{}</a>{current}</li>"#,
            urlencoding(&f.name),
            html_escape(&f.name),
        ));
    }
    versions.push_str("</ul>");

    let title = title_case(&name);
    let ds = design_system_url(&name);
    let content = format!(
        r#"{parity}
        <h1 class="govuk-heading-xl">{title}</h1>
        <p class="govuk-body">Preview of the Rust renderer for this component. Current version: <strong>{fixture}</strong>. Choose a version below to compare the Rust HTML with the official fixture from GOV.UK Frontend {}.</p>
        <p class="govuk-body"><a class="govuk-link" href="{ds}">{title} on the GOV.UK Design System</a></p>
        <h2 class="govuk-heading-m">Component preview</h2>
        <div class="app-component-preview"><div class="app-component-preview__frame">{preview}</div></div>
        {versions}"#,
        html_escape(&state.config.frontend_version),
        title = html_escape(&title),
        fixture = html_escape(&fixture_name),
        preview = rendered,
    );

    let html = render_page(Page {
        title: &title,
        content,
        back_href: Some("/components"),
        breadcrumbs: false,
        sensitive: false,
        welsh: false,
        show_feedback: false,
        assets: &state.assets,
        session: &data,
        return_path: &format!("/components/{name}"),
    });
    html_response(&state, html, &sid, false)
}

pub async fn raw_fixture(
    State(state): State<AppState>,
    Path(name): Path<String>,
    Query(q): Query<ComponentQuery>,
) -> Response<Body> {
    let Ok(set) = load_fixtures(&state.config.components_root, &name) else {
        return Response::builder()
            .status(404)
            .header(
                "x-robots-tag",
                "noindex, nofollow, noarchive, nosnippet, noimageindex",
            )
            .body(Body::from("not found"))
            .unwrap();
    };
    let Some(fixture) = select_fixture(&set.fixtures, q.fixture.as_deref()) else {
        return Response::builder()
            .status(404)
            .header(
                "x-robots-tag",
                "noindex, nofollow, noarchive, nosnippet, noimageindex",
            )
            .body(Body::from("not found"))
            .unwrap();
    };
    // Serve the Rust-rendered HTML fragment (same path as parity tests).
    let html = render(&name, &fixture.options).unwrap_or_default();
    let mut res = Response::new(Body::from(html));
    *res.status_mut() = axum::http::StatusCode::OK;
    let headers = res.headers_mut();
    headers.insert(
        axum::http::header::CONTENT_TYPE,
        axum::http::HeaderValue::from_static("text/html; charset=utf-8"),
    );
    crate::httpx::apply_policy_headers(
        headers,
        &state.assets.policy,
        crate::baseline::CacheKind::Document,
        state.assets.secure_transport,
    );
    res
}

fn select_fixture<'a>(
    fixtures: &'a [crate::govuk::Fixture],
    requested: Option<&str>,
) -> Option<&'a crate::govuk::Fixture> {
    if let Some(name) = requested {
        return fixtures.iter().find(|f| f.name == name);
    }
    fixtures
        .iter()
        .find(|f| !f.hidden)
        .or_else(|| fixtures.first())
}

fn title_case(kebab: &str) -> String {
    kebab
        .split('-')
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn design_system_url(name: &str) -> String {
    format!("https://design-system.service.gov.uk/components/{name}/")
}

fn urlencoding(s: &str) -> String {
    form_urlencoded::byte_serialize(s.as_bytes()).collect()
}
