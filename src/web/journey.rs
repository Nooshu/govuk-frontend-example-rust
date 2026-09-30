//! Rod fishing licence journey handlers.

use crate::govuk::{must_render, params, v_obj, v_str, Value};
use crate::pages::{html_escape, render_page, Page};
use crate::service::{self, FieldError, StepId};
use crate::session::SessionData;
use crate::web::{html_response, redirect_with_session, session_id_from, AppState};
use axum::body::Body;
use axum::extract::{OriginalUri, State};
use axum::http::{HeaderMap, Response};
use axum::Form;
use serde::Deserialize;
use uuid::Uuid;

pub async fn start_en(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    start(&state, &headers, false).await
}

pub async fn start_cy(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    start(&state, &headers, true).await
}

async fn start(state: &AppState, headers: &HeaderMap, welsh: bool) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(headers).as_deref());
    let button = must_render(
        "button",
        &params(&[
            (
                "text",
                v_str(if welsh { "Dechrau nawr" } else { "Start now" }),
            ),
            ("href", v_str("/task-list")),
            ("isStartButton", Value::Bool(true)),
        ]),
    );
    let warning = must_render(
        "warning-text",
        &params(&[(
            "text",
            v_str("You must have a valid rod licence before you fish."),
        )]),
    );
    let inset = must_render(
        "inset-text",
        &params(&[(
            "text",
            v_str("You need to be 13 or over. This example does not take payment."),
        )]),
    );
    let details = must_render(
        "details",
        &params(&[
            ("summaryText", v_str("What you will need")),
            (
                "text",
                v_str("Your name, date of birth, email address, and where you will fish."),
            ),
        ]),
    );
    let demos = if state.config.demos_enabled {
        r#"<h2 class="govuk-heading-m">Developer previews</h2>
        <p class="govuk-body"><a class="govuk-link" href="/components">Preview GOV.UK components</a> — a separate page for each component, rendered by this service’s Rust library.</p>"#
    } else {
        ""
    };
    let content = format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        <h1 class="govuk-heading-xl">{}</h1>
        <p class="govuk-body">Use this service to apply for a licence to fish with a rod.</p>
        {button}
        <p class="govuk-body">Applying takes about 10 minutes.</p>
        {warning}
        {inset}
        {details}
        {demos}
        </div></div>"#,
        if welsh {
            crate::config::SERVICE_NAME_CY
        } else {
            crate::config::SERVICE_NAME
        }
    );
    let html = render_page(Page {
        title: if welsh {
            crate::config::SERVICE_NAME_CY
        } else {
            crate::config::SERVICE_NAME
        },
        content,
        back_href: None,
        breadcrumbs: false,
        sensitive: false,
        welsh,
        show_feedback: true,
        assets: &state.assets,
        session: &data,
        return_path: if welsh { "/cy" } else { "/" },
    });
    html_response(state, html, &sid, false)
}

pub async fn new_application(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let (sid, _) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    state.sessions.reset_application(&sid);
    redirect_with_session("/", &sid, state.assets.secure_transport)
}

pub async fn task_list(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let items: Vec<Value> = service::steps()
        .iter()
        .map(|step| {
            let status = if data.application.is_completed(step.id) {
                v_obj(params(&[(
                    "tag",
                    v_obj(params(&[("text", v_str("Completed"))])),
                )]))
            } else {
                v_obj(params(&[("text", v_str("Not started"))]))
            };
            v_obj(params(&[
                ("title", v_obj(params(&[("text", v_str(step.heading))]))),
                ("href", v_str(step.path)),
                ("status", status),
            ]))
        })
        .collect();
    let list = must_render("task-list", &params(&[("items", Value::Array(items))]));
    let content = format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        <h1 class="govuk-heading-xl">Apply for a rod fishing licence</h1>
        {list}
        <p class="govuk-body"><a class="govuk-link" href="/check-answers">Check your answers</a></p>
        </div></div>"#
    );
    let html = render_page(Page {
        title: "Task list",
        content,
        back_href: Some("/"),
        breadcrumbs: false,
        sensitive: true,
        welsh: false,
        show_feedback: false,
        assets: &state.assets,
        session: &data,
        return_path: "/task-list",
    });
    html_response(&state, html, &sid, true)
}

#[derive(Deserialize)]
pub struct NameForm {
    #[serde(rename = "first-name")]
    first_name: String,
    #[serde(rename = "last-name")]
    last_name: String,
}

pub async fn name_get(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let content = name_form(
        &data.application.first_name,
        &data.application.last_name,
        &[],
    );
    page_question(
        &state,
        &sid,
        &data,
        "What is your name?",
        "/name",
        "/task-list",
        content,
    )
}

pub async fn name_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<NameForm>,
) -> Response<Body> {
    let (sid, mut data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let errors = service::validate_name(&form.first_name, &form.last_name);
    if errors.is_empty() {
        data.application.first_name = service::clean(&form.first_name);
        data.application.last_name = service::clean(&form.last_name);
        data.application.mark_completed(StepId::Name);
        state.sessions.save(&sid, data);
        redirect_with_session(
            service::next_step(StepId::Name)
                .map(|s| s.path)
                .unwrap_or("/task-list"),
            &sid,
            state.assets.secure_transport,
        )
    } else {
        let content = name_form(&form.first_name, &form.last_name, &errors);
        page_question(
            &state,
            &sid,
            &data,
            "What is your name?",
            "/name",
            "/task-list",
            content,
        )
    }
}

fn name_form(first: &str, last: &str, errors: &[FieldError]) -> String {
    let summary = error_summary(errors);
    let first_err = field_error(errors, "first-name");
    let last_err = field_error(errors, "last-name");
    let first_input = must_render(
        "input",
        &params(&[
            ("id", v_str("first-name")),
            ("name", v_str("first-name")),
            ("label", v_obj(params(&[("text", v_str("First name"))]))),
            ("value", v_str(first)),
            ("errorMessage", error_message_value(first_err)),
        ]),
    );
    let last_input = must_render(
        "input",
        &params(&[
            ("id", v_str("last-name")),
            ("name", v_str("last-name")),
            ("label", v_obj(params(&[("text", v_str("Last name"))]))),
            ("value", v_str(last)),
            ("errorMessage", error_message_value(last_err)),
        ]),
    );
    let button = must_render("button", &params(&[("text", v_str("Save and continue"))]));
    format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        {summary}
        <h1 class="govuk-heading-l">What is your name?</h1>
        <form method="post" novalidate>
        {first_input}{last_input}{button}
        </form></div></div>"#
    )
}

#[derive(Deserialize)]
pub struct EmailForm {
    email: String,
}

pub async fn email_get(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let content = email_form(&data.application.email, &[]);
    page_question(
        &state,
        &sid,
        &data,
        "What is your email address?",
        "/email",
        "/date-of-birth",
        content,
    )
}

pub async fn email_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<EmailForm>,
) -> Response<Body> {
    let (sid, mut data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let errors = service::validate_email(&form.email);
    if errors.is_empty() {
        data.application.email = service::clean(&form.email);
        data.application.mark_completed(StepId::Email);
        state.sessions.save(&sid, data);
        redirect_with_session("/contact-preference", &sid, state.assets.secure_transport)
    } else {
        let content = email_form(&form.email, &errors);
        page_question(
            &state,
            &sid,
            &data,
            "What is your email address?",
            "/email",
            "/date-of-birth",
            content,
        )
    }
}

fn email_form(email: &str, errors: &[FieldError]) -> String {
    let summary = error_summary(errors);
    let err = field_error(errors, "email");
    let input = must_render(
        "input",
        &params(&[
            ("id", v_str("email")),
            ("name", v_str("email")),
            ("type", v_str("email")),
            (
                "label",
                v_obj(params(&[
                    ("text", v_str("Email address")),
                    ("classes", v_str("govuk-label--l")),
                    ("isPageHeading", Value::Bool(true)),
                ])),
            ),
            ("value", v_str(email)),
            ("errorMessage", error_message_value(err)),
        ]),
    );
    let button = must_render("button", &params(&[("text", v_str("Save and continue"))]));
    format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        {summary}<form method="post" novalidate>{input}{button}</form></div></div>"#
    )
}

#[derive(Deserialize)]
pub struct PasswordForm {
    password: String,
    #[serde(rename = "confirm-password")]
    confirm: String,
}

pub async fn password_get(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let content = password_form(&[]);
    page_question(
        &state,
        &sid,
        &data,
        "Create a password",
        "/create-a-password",
        "/additional-details",
        content,
    )
}

pub async fn password_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<PasswordForm>,
) -> Response<Body> {
    let (sid, mut data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let errors = service::validate_password(&form.password, &form.confirm);
    if errors.is_empty() {
        data.application.password_created = true;
        data.application.mark_completed(StepId::CreateAPassword);
        state.sessions.save(&sid, data);
        redirect_with_session("/check-answers", &sid, state.assets.secure_transport)
    } else {
        let content = password_form(&errors);
        page_question(
            &state,
            &sid,
            &data,
            "Create a password",
            "/create-a-password",
            "/additional-details",
            content,
        )
    }
}

fn password_form(errors: &[FieldError]) -> String {
    let summary = error_summary(errors);
    let input = must_render(
        "password-input",
        &params(&[
            ("id", v_str("password")),
            ("name", v_str("password")),
            (
                "label",
                v_obj(params(&[
                    ("text", v_str("Password")),
                    ("classes", v_str("govuk-label--l")),
                    ("isPageHeading", Value::Bool(true)),
                ])),
            ),
            (
                "errorMessage",
                error_message_value(field_error(errors, "password")),
            ),
        ]),
    );
    let confirm = must_render(
        "password-input",
        &params(&[
            ("id", v_str("confirm-password")),
            ("name", v_str("confirm-password")),
            (
                "label",
                v_obj(params(&[("text", v_str("Confirm password"))])),
            ),
            (
                "errorMessage",
                error_message_value(field_error(errors, "password-confirm")),
            ),
        ]),
    );
    let button = must_render("button", &params(&[("text", v_str("Save and continue"))]));
    format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        {summary}<form method="post" novalidate>{input}{confirm}{button}</form></div></div>"#
    )
}

pub async fn generic_get(
    State(state): State<AppState>,
    headers: HeaderMap,
    uri: OriginalUri,
) -> Response<Body> {
    let path = uri.path();
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let step = service::step_by_path(path).expect("known step");
    let back = service::previous_step(step.id)
        .map(|s| s.path)
        .unwrap_or("/task-list");
    let content = format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        <h1 class="govuk-heading-l">{}</h1>
        <p class="govuk-body">This example step records that you continued.</p>
        <form method="post" novalidate>
        <input type="hidden" name="continue" value="1">
        {}
        </form></div></div>"#,
        html_escape(step.heading),
        must_render("button", &params(&[("text", v_str("Save and continue"))]))
    );
    page_question(&state, &sid, &data, step.heading, path, back, content)
}

pub async fn generic_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    uri: OriginalUri,
) -> Response<Body> {
    let path = uri.path().to_string();
    let (sid, mut data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let step = service::step_by_path(&path).expect("known step");
    data.application.mark_completed(step.id);
    match step.id {
        StepId::DateOfBirth => {
            data.application.day = "1".into();
            data.application.month = "1".into();
            data.application.year = "1990".into();
        }
        StepId::ContactPreference => data.application.contact_by = service::CONTACT_EMAIL.into(),
        StepId::WhereYouWillFish => {
            data.application.regions = vec!["north-west".into()];
        }
        StepId::LicenceLength => {
            data.application.licence_length = service::LICENCE_TWELVE_MTH.into();
        }
        StepId::StartMonth => {
            data.application.start_month = service::start_months()
                .into_iter()
                .next()
                .map(|(v, _)| v)
                .unwrap_or_default();
        }
        StepId::Address => {
            data.application.address_line1 = "1 Example Street".into();
            data.application.town = "London".into();
            data.application.postcode = "SW1A 1AA".into();
        }
        StepId::Evidence => data.application.evidence_filename = String::new(),
        StepId::AdditionalDetails => data.application.additional_details = String::new(),
        _ => {}
    }
    state.sessions.save(&sid, data);
    let next = service::next_step(step.id)
        .map(|s| s.path)
        .unwrap_or("/check-answers");
    redirect_with_session(next, &sid, state.assets.secure_transport)
}

pub async fn check_answers_get(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let app = &data.application;
    let rows = Value::Array(vec![
        summary_row(
            "Name",
            &format!("{} {}", app.first_name, app.last_name),
            "/name",
        ),
        summary_row(
            "Date of birth",
            &format!("{}/{}/{}", app.day, app.month, app.year),
            "/date-of-birth",
        ),
        summary_row("Email", &app.email, "/email"),
        summary_row("Contact preference", &app.contact_by, "/contact-preference"),
        summary_row(
            "Where you will fish",
            &app.regions.join(", "),
            "/where-you-will-fish",
        ),
        summary_row("Licence length", &app.licence_length, "/licence-length"),
        summary_row("Start month", &app.start_month, "/start-month"),
        summary_row(
            "Address",
            &format!("{}, {}, {}", app.address_line1, app.town, app.postcode),
            "/address",
        ),
    ]);
    let list = must_render("summary-list", &params(&[("rows", rows)]));
    let button = must_render("button", &params(&[("text", v_str("Accept and send"))]));
    let content = format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        <h1 class="govuk-heading-l">Check your answers</h1>
        {list}
        <form method="post">{button}</form>
        </div></div>"#
    );
    let html = render_page(Page {
        title: "Check your answers",
        content,
        back_href: Some("/create-a-password"),
        breadcrumbs: false,
        sensitive: true,
        welsh: false,
        show_feedback: false,
        assets: &state.assets,
        session: &data,
        return_path: "/check-answers",
    });
    html_response(&state, html, &sid, true)
}

pub async fn check_answers_post(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Response<Body> {
    let (sid, mut data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    if !data.application.required_complete() {
        return redirect_with_session("/task-list", &sid, state.assets.secure_transport);
    }
    data.application.submitted = true;
    data.application.reference = format!("HDJ{}", Uuid::new_v4().to_string()[..8].to_uppercase());
    state.sessions.save(&sid, data);
    redirect_with_session("/confirmation", &sid, state.assets.secure_transport)
}

pub async fn confirmation(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    if !data.application.submitted {
        return redirect_with_session("/task-list", &sid, state.assets.secure_transport);
    }
    let panel = must_render(
        "panel",
        &params(&[
            ("titleText", v_str("Application complete")),
            (
                "html",
                v_str(format!(
                    "Your reference number<br><strong>{}</strong>",
                    html_escape(&data.application.reference)
                )),
            ),
        ]),
    );
    let content = format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        {panel}
        <p class="govuk-body"><a class="govuk-link" href="/new-application">Start a new application</a></p>
        </div></div>"#
    );
    let html = render_page(Page {
        title: "Application complete",
        content,
        back_href: None,
        breadcrumbs: false,
        sensitive: true,
        welsh: false,
        show_feedback: true,
        assets: &state.assets,
        session: &data,
        return_path: "/confirmation",
    });
    html_response(&state, html, &sid, true)
}

fn summary_row(key: &str, value: &str, href: &str) -> Value {
    v_obj(params(&[
        ("key", v_obj(params(&[("text", v_str(key))]))),
        ("value", v_obj(params(&[("text", v_str(value))]))),
        (
            "actions",
            v_obj(params(&[(
                "items",
                Value::Array(vec![v_obj(params(&[
                    ("href", v_str(href)),
                    ("text", v_str("Change")),
                    ("visuallyHiddenText", v_str(key)),
                ]))]),
            )])),
        ),
    ]))
}

fn error_summary(errors: &[FieldError]) -> String {
    if errors.is_empty() {
        return String::new();
    }
    let list: Vec<Value> = errors
        .iter()
        .map(|e| {
            v_obj(params(&[
                ("href", v_str(&e.href)),
                ("text", v_str(&e.text)),
            ]))
        })
        .collect();
    must_render(
        "error-summary",
        &params(&[
            ("titleText", v_str("There is a problem")),
            ("errorList", Value::Array(list)),
        ]),
    )
}

fn field_error<'a>(errors: &'a [FieldError], id: &str) -> Option<&'a str> {
    errors
        .iter()
        .find(|e| e.field == id)
        .map(|e| e.text.as_str())
}

fn error_message_value(text: Option<&str>) -> Value {
    match text {
        Some(t) => v_obj(params(&[("text", v_str(t))])),
        None => Value::Undefined,
    }
}

fn page_question(
    state: &AppState,
    sid: &str,
    data: &SessionData,
    title: &str,
    return_path: &str,
    back: &str,
    content: String,
) -> Response<Body> {
    let html = render_page(Page {
        title,
        content,
        back_href: Some(back),
        breadcrumbs: false,
        sensitive: true,
        welsh: false,
        show_feedback: false,
        assets: &state.assets,
        session: data,
        return_path,
    });
    html_response(state, html, sid, true)
}
