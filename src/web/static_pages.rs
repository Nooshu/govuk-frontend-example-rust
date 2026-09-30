//! Static supporting pages and cookie settings.

use crate::govuk::{must_render, params, v_bool, v_obj, v_safe, v_str, Value};
use crate::pages::{html_escape, render_page, Page};
use crate::service::{validate_cookie_choice, LICENCE_LENGTHS};
use crate::web::{html_response, redirect_with_session, session_id_from, AppState};
use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::{HeaderMap, Response};
use axum::Form;
use serde::Deserialize;

fn simple_page(
    state: &AppState,
    headers: &HeaderMap,
    title: &str,
    content: String,
    return_path: &str,
    feedback: bool,
) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(headers).as_deref());
    let html = render_page(Page {
        title,
        content,
        back_href: None,
        breadcrumbs: true,
        sensitive: false,
        welsh: false,
        show_feedback: feedback,
        assets: &state.assets,
        session: &data,
        return_path,
    });
    html_response(state, html, &sid, false)
}

pub async fn about(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let content = format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        <h1 class="govuk-heading-xl">About this example</h1>
        <p class="govuk-body">This is a Rust example of a GOV.UK Frontend service. It uses Axum and Askama-style page composition with native Rust component renderers.</p>
        <p class="govuk-body">Pinned GOV.UK Frontend version: <strong>{}</strong>.</p>
        <p class="govuk-body">It does not take payment, send email, or issue a licence.</p>
        </div></div>"#,
        html_escape(&state.config.frontend_version)
    );
    simple_page(
        &state,
        &headers,
        "About this example",
        content,
        "/about",
        true,
    )
}

pub async fn help(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let accordion = must_render(
        "accordion",
        &params(&[
            ("id", v_str("help")),
            (
                "items",
                Value::Array(vec![
                    v_obj(params(&[
                        ("heading", v_obj(params(&[("text", v_str("Who can apply"))]))),
                        (
                            "content",
                            v_obj(params(&[(
                                "text",
                                v_str(
                                    "You can apply if you are 13 or over and you will fish with a rod in England or Wales.",
                                ),
                            )])),
                        ),
                    ])),
                    v_obj(params(&[
                        (
                            "heading",
                            v_obj(params(&[("text", v_str("What a licence covers"))])),
                        ),
                        (
                            "content",
                            v_obj(params(&[(
                                "html",
                                v_safe(
                                    r#"<ul class="govuk-list govuk-list--bullet"><li>Rod and line fishing</li><li>Up to 2 rods where the licence allows it</li><li>The dates printed on your licence</li></ul>"#,
                                ),
                            )])),
                        ),
                    ])),
                    v_obj(params(&[
                        (
                            "heading",
                            v_obj(params(&[("text", v_str("If you need help to apply"))])),
                        ),
                        (
                            "content",
                            v_obj(params(&[(
                                "text",
                                v_str(
                                    "You can ask someone to apply for you. This example service does not offer a phone application line.",
                                ),
                            )])),
                        ),
                    ])),
                ]),
            ),
        ]),
    );
    let content = format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        <h1 class="govuk-heading-xl">Help</h1>
        {accordion}
        </div></div>"#
    );
    simple_page(&state, &headers, "Help", content, "/help", true)
}

pub async fn fees(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let rows: Vec<Value> = LICENCE_LENGTHS
        .iter()
        .map(|o| {
            Value::Array(vec![
                v_obj(params(&[("text", v_str(o.text))])),
                v_obj(params(&[
                    ("text", v_str(o.fee)),
                    ("format", v_str("numeric")),
                ])),
            ])
        })
        .collect();
    let table = must_render(
        "table",
        &params(&[
            ("caption", v_str("Rod licence fees")),
            ("captionClasses", v_str("govuk-table__caption--m")),
            ("firstCellIsHeader", v_bool(true)),
            (
                "head",
                Value::Array(vec![
                    v_obj(params(&[("text", v_str("Licence"))])),
                    v_obj(params(&[
                        ("text", v_str("Fee")),
                        ("format", v_str("numeric")),
                    ])),
                ]),
            ),
            ("rows", Value::Array(rows)),
        ]),
    );
    let content = format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        <h1 class="govuk-heading-xl">Licence fees</h1>
        <p class="govuk-body">Example fees for the 2026 to 2027 season. This example does not take payment.</p>
        {table}
        </div></div>"#
    );
    simple_page(&state, &headers, "Licence fees", content, "/fees", false)
}

pub async fn guidance(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let tabs = must_render(
        "tabs",
        &params(&[
            ("id", v_str("guidance")),
            (
                "items",
                Value::Array(vec![
                    v_obj(params(&[
                        ("label", v_str("Before you apply")),
                        ("id", v_str("before-you-apply")),
                        (
                            "panel",
                            v_obj(params(&[(
                                "html",
                                v_safe(
                                    r#"<h2 class="govuk-heading-l">Before you apply</h2><p class="govuk-body">You need your name, date of birth, email address, and home address.</p>"#,
                                ),
                            )])),
                        ),
                    ])),
                    v_obj(params(&[
                        ("label", v_str("Fees")),
                        ("id", v_str("fees")),
                        (
                            "panel",
                            v_obj(params(&[(
                                "html",
                                v_safe(
                                    r#"<h2 class="govuk-heading-l">Fees</h2><p class="govuk-body">Fees depend on the length of the licence. <a class="govuk-link" href="/fees">See licence fees</a>.</p>"#,
                                ),
                            )])),
                        ),
                    ])),
                    v_obj(params(&[
                        ("label", v_str("After you apply")),
                        ("id", v_str("after-you-apply")),
                        (
                            "panel",
                            v_obj(params(&[(
                                "html",
                                v_safe(
                                    r#"<h2 class="govuk-heading-l">After you apply</h2><p class="govuk-body">This example shows a confirmation page with a reference number. It does not send email and it does not take payment.</p>"#,
                                ),
                            )])),
                        ),
                    ])),
                ]),
            ),
        ]),
    );
    let content = format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-full">
        <h1 class="govuk-heading-xl">Guidance</h1>
        {tabs}
        </div></div>"#
    );
    simple_page(&state, &headers, "Guidance", content, "/guidance", false)
}

pub async fn accessibility(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let content = r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        <h1 class="govuk-heading-xl">Accessibility statement</h1>
        <p class="govuk-body">This example aims to meet WCAG 2.2 AA. It is a demonstration, not a live government service.</p>
        </div></div>"#
        .to_string();
    simple_page(
        &state,
        &headers,
        "Accessibility statement",
        content,
        "/accessibility",
        true,
    )
}

#[derive(Deserialize)]
pub struct UpdatesQuery {
    page: Option<String>,
}

pub async fn updates(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<UpdatesQuery>,
) -> Response<Body> {
    let page = match q.page.as_deref() {
        None | Some("1") => 1,
        Some("2") => 2,
        _ => {
            return redirect_with_session("/updates", "", state.assets.secure_transport);
        }
    };
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let body = if page == 2 {
        "There are no further fee changes planned in this example."
    } else {
        "Example fees for the 2026 to 2027 season are on the fees page."
    };
    let mut pagination_params = vec![(
        "items",
        Value::Array(vec![
            v_obj(params(&[
                ("number", Value::Number("1".into())),
                ("href", v_str("/updates")),
                ("current", v_bool(page == 1)),
            ])),
            v_obj(params(&[
                ("number", Value::Number("2".into())),
                ("href", v_str("/updates?page=2")),
                ("current", v_bool(page == 2)),
            ])),
        ]),
    )];
    if page > 1 {
        pagination_params.push(("previous", v_obj(params(&[("href", v_str("/updates"))]))));
    }
    if page < 2 {
        pagination_params.push(("next", v_obj(params(&[("href", v_str("/updates?page=2"))]))));
    }
    let pagination = must_render("pagination", &params(&pagination_params));
    let content = format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        <h1 class="govuk-heading-xl">Service updates</h1>
        <p class="govuk-body">{}</p>
        {pagination}
        </div></div>"#,
        html_escape(body)
    );
    let html = render_page(Page {
        title: "Service updates",
        content,
        back_href: None,
        breadcrumbs: true,
        sensitive: false,
        welsh: false,
        show_feedback: false,
        assets: &state.assets,
        session: &data,
        return_path: "/updates",
    });
    html_response(&state, html, &sid, false)
}

pub async fn cookies(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let (sid, mut data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let errors = std::mem::take(&mut data.flash_errors);
    let notice = data.flash_notice.take();
    state.sessions.save(&sid, data.clone());

    let selected = match data.cookie_choice.as_deref() {
        Some("accept") => "yes",
        Some("reject") => "no",
        _ => "",
    };
    let mut radios = vec![
        ("idPrefix", v_str("analytics")),
        ("name", v_str("analytics")),
        (
            "fieldset",
            v_obj(params(&[(
                "legend",
                v_obj(params(&[
                    ("text", v_str("Do you want to accept analytics cookies?")),
                    ("isPageHeading", v_bool(true)),
                    ("classes", v_str("govuk-fieldset__legend--l")),
                ])),
            )])),
        ),
        (
            "hint",
            v_obj(params(&[(
                "text",
                v_str("This example stores your choice. It does not set analytics cookies."),
            )])),
        ),
        (
            "items",
            Value::Array(vec![
                v_obj(params(&[
                    ("value", v_str("yes")),
                    ("text", v_str("Yes")),
                    ("id", v_str("analytics")),
                    ("checked", v_bool(selected == "yes")),
                ])),
                v_obj(params(&[
                    ("value", v_str("no")),
                    ("text", v_str("No")),
                    ("checked", v_bool(selected == "no")),
                ])),
            ]),
        ),
    ];
    if let Some((_, msg)) = errors.iter().find(|(f, _)| f == "analytics") {
        radios.push((
            "errorMessage",
            v_obj(params(&[("text", v_str(msg.clone()))])),
        ));
    }
    let radios_html = must_render("radios", &params(&radios));
    let button = must_render(
        "button",
        &params(&[("text", v_str("Save cookie settings"))]),
    );
    let notice_html = if let Some(text) = notice {
        must_render(
            "notification-banner",
            &params(&[
                ("type", v_str("success")),
                ("titleText", v_str("Success")),
                ("text", v_str(text)),
            ]),
        )
    } else {
        String::new()
    };
    let content = format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        {notice_html}
        <form method="post" novalidate>
          <input type="hidden" name="csrf" value="{}">
          {radios_html}
          {button}
        </form>
        </div></div>"#,
        html_escape(&data.csrf)
    );
    let html = render_page(Page {
        title: "Cookies",
        content,
        back_href: None,
        breadcrumbs: true,
        sensitive: true,
        welsh: false,
        show_feedback: false,
        assets: &state.assets,
        session: &data,
        return_path: "/cookies",
    });
    html_response(&state, html, &sid, true)
}

#[derive(Deserialize)]
pub struct CookiesForm {
    csrf: String,
    analytics: Option<String>,
}

pub async fn cookies_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<CookiesForm>,
) -> Response<Body> {
    let (sid, mut data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    if form.csrf != data.csrf {
        return redirect_with_session("/", &sid, state.assets.secure_transport);
    }
    let value = form.analytics.unwrap_or_default();
    let errors = validate_cookie_choice(&value);
    if !errors.is_empty() {
        data.flash_errors = errors.into_iter().map(|e| (e.field, e.text)).collect();
        data.flash_notice = None;
        state.sessions.save(&sid, data);
        return redirect_with_session("/cookies", &sid, state.assets.secure_transport);
    }
    data.cookie_choice = Some(if value == "yes" {
        "accept".into()
    } else {
        "reject".into()
    });
    data.flash_errors.clear();
    data.flash_notice = Some("Your cookie settings were saved".into());
    state.sessions.save(&sid, data);
    redirect_with_session("/cookies", &sid, state.assets.secure_transport)
}

pub async fn examples_index(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let content = r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        <h1 class="govuk-heading-xl">Example pages</h1>
        <ul class="govuk-list">
          <li><a class="govuk-link" href="/examples/exit-this-page">Exit this page</a></li>
          <li><a class="govuk-link" href="/examples/service-unavailable">Service unavailable</a></li>
          <li><a class="govuk-link" href="/examples/problem-with-the-service">Problem with the service</a></li>
        </ul>
        </div></div>"#
        .to_string();
    simple_page(
        &state,
        &headers,
        "Example pages",
        content,
        "/examples",
        false,
    )
}

pub async fn exit_this_page(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let exit = must_render(
        "exit-this-page",
        &params(&[("redirectUrl", v_str("https://www.bbc.co.uk/weather"))]),
    );
    let content = format!(
        r#"{exit}
        <div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        <h1 class="govuk-heading-xl">Exit this page</h1>
        <p class="govuk-body">The exit this page button leaves this example and opens the BBC weather forecast.</p>
        </div></div>"#
    );
    let html = render_page(Page {
        title: "Exit this page",
        content,
        back_href: Some("/examples"),
        breadcrumbs: false,
        sensitive: false,
        welsh: false,
        show_feedback: false,
        assets: &state.assets,
        session: &data,
        return_path: "/examples/exit-this-page",
    });
    // Inject exit component at body start via content only — shell does not have exit slot.
    // Prepend by replacing after body open is hard; include at top of main is acceptable for demo.
    html_response(&state, html, &sid, false)
}

pub async fn unavailable(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let content = r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        <h1 class="govuk-heading-xl">Sorry, the service is unavailable</h1>
        <p class="govuk-body">You will be able to use the service later.</p>
        </div></div>"#
        .to_string();
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let html = render_page(Page {
        title: "Sorry, the service is unavailable",
        content,
        back_href: Some("/examples"),
        breadcrumbs: false,
        sensitive: false,
        welsh: false,
        show_feedback: false,
        assets: &state.assets,
        session: &data,
        return_path: "/examples/service-unavailable",
    });
    html_response(&state, html, &sid, false)
}

pub async fn problem(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let content = r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        <h1 class="govuk-heading-xl">Sorry, there is a problem with the service</h1>
        <p class="govuk-body">Try again later.</p>
        </div></div>"#
        .to_string();
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let html = render_page(Page {
        title: "Sorry, there is a problem with the service",
        content,
        back_href: Some("/examples"),
        breadcrumbs: false,
        sensitive: false,
        welsh: false,
        show_feedback: false,
        assets: &state.assets,
        session: &data,
        return_path: "/examples/problem-with-the-service",
    });
    html_response(&state, html, &sid, false)
}
