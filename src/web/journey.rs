//! Rod fishing licence journey handlers.

use crate::govuk::{must_render, params, v_bool, v_obj, v_str, Value};
use crate::pages::{html_escape, render_page, Page};
use crate::service::{self, FieldError, StepId, COUNTRIES, LICENCE_LENGTHS};
use crate::session::{reference_for, SessionData};
use crate::web::{html_response, redirect_with_session, session_id_from, AppState};
use axum::body::Body;
use axum::extract::{Query, State};
use axum::http::{HeaderMap, Response};
use axum::Form;
use serde::Deserialize;

#[derive(Deserialize, Default)]
pub struct ReturnQuery {
    #[serde(rename = "return")]
    return_to: Option<String>,
}

fn return_to_check(return_to: Option<&str>) -> bool {
    return_to == Some("check-answers")
}

fn back_for(step: StepId, to_check: bool) -> &'static str {
    if to_check {
        "/check-answers"
    } else {
        service::previous_step(step).map(|s| s.path).unwrap_or("/")
    }
}

fn next_for(step: StepId, to_check: bool) -> &'static str {
    if to_check {
        "/check-answers"
    } else {
        service::next_step(step)
            .map(|s| s.path)
            .unwrap_or("/check-answers")
    }
}

fn return_hidden(to_check: bool) -> &'static str {
    if to_check {
        r#"<input type="hidden" name="returnTo" value="check-answers">"#
    } else {
        ""
    }
}

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
            ("href", v_str("/licence-length")),
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
                "html",
                v_str(
                    r#"<ul class="govuk-list govuk-list--bullet"><li>How long you need the licence</li><li>Your name</li><li>Your date of birth</li><li>The country where you will fish</li><li>Your email address</li></ul>"#,
                ),
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

#[derive(Deserialize)]
pub struct LicenceLengthForm {
    #[serde(rename = "licence-length", default)]
    licence_length: String,
    #[serde(rename = "returnTo", default)]
    return_to: Option<String>,
}

pub async fn licence_length_get(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<ReturnQuery>,
) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let to_check = return_to_check(q.return_to.as_deref());
    let content = licence_length_form(&data.application.licence_length, &[], to_check);
    page_question(
        &state,
        &sid,
        &data,
        "How long do you need the licence for?",
        if to_check {
            "/licence-length?return=check-answers"
        } else {
            "/licence-length"
        },
        back_for(StepId::LicenceLength, to_check),
        content,
    )
}

pub async fn licence_length_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<LicenceLengthForm>,
) -> Response<Body> {
    let (sid, mut data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let to_check = return_to_check(form.return_to.as_deref());
    let errors = service::validate_licence_length(&form.licence_length);
    if errors.is_empty() {
        data.application.licence_length = form.licence_length;
        data.application.mark_completed(StepId::LicenceLength);
        state.sessions.save(&sid, data);
        redirect_with_session(
            next_for(StepId::LicenceLength, to_check),
            &sid,
            state.assets.secure_transport,
        )
    } else {
        let content = licence_length_form(&form.licence_length, &errors, to_check);
        page_question(
            &state,
            &sid,
            &data,
            "How long do you need the licence for?",
            if to_check {
                "/licence-length?return=check-answers"
            } else {
                "/licence-length"
            },
            back_for(StepId::LicenceLength, to_check),
            content,
        )
    }
}

fn licence_length_form(selected: &str, errors: &[FieldError], to_check: bool) -> String {
    let summary = error_summary(errors);
    let message = field_error(errors, "licence-length");
    let items: Vec<Value> = LICENCE_LENGTHS
        .iter()
        .enumerate()
        .map(|(index, option)| {
            let mut fields = vec![
                ("value", v_str(option.value)),
                ("text", v_str(option.text)),
                ("checked", Value::Bool(selected == option.value)),
            ];
            if index == 0 {
                fields.push(("id", v_str("licence-length")));
            }
            v_obj(params(&fields))
        })
        .collect();
    let mut radio_params = vec![
        ("idPrefix", v_str("licence-length")),
        ("name", v_str("licence-length")),
        (
            "fieldset",
            v_obj(params(&[(
                "legend",
                v_obj(params(&[
                    ("text", v_str("How long do you need the licence for?")),
                    ("isPageHeading", Value::Bool(true)),
                    ("classes", v_str("govuk-fieldset__legend--l")),
                ])),
            )])),
        ),
        ("items", Value::Array(items)),
    ];
    if let Some(text) = message {
        radio_params.push(("errorMessage", v_obj(params(&[("text", v_str(text))]))));
    }
    let radios = must_render("radios", &params(&radio_params));
    let button = must_render("button", &params(&[("text", v_str("Continue"))]));
    let hidden = return_hidden(to_check);
    format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        {summary}<form method="post" novalidate>{hidden}{radios}{button}</form></div></div>"#
    )
}

#[derive(Deserialize)]
pub struct NameForm {
    #[serde(rename = "full-name", default)]
    full_name: String,
    #[serde(rename = "returnTo", default)]
    return_to: Option<String>,
}

pub async fn name_get(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<ReturnQuery>,
) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let to_check = return_to_check(q.return_to.as_deref());
    let content = name_form(&data.application.full_name, &[], to_check);
    page_question(
        &state,
        &sid,
        &data,
        "What is your full name?",
        if to_check {
            "/name?return=check-answers"
        } else {
            "/name"
        },
        back_for(StepId::Name, to_check),
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
    let to_check = return_to_check(form.return_to.as_deref());
    let errors = service::validate_name(&form.full_name);
    if errors.is_empty() {
        data.application.full_name = service::clean(&form.full_name);
        data.application.mark_completed(StepId::Name);
        state.sessions.save(&sid, data);
        redirect_with_session(
            next_for(StepId::Name, to_check),
            &sid,
            state.assets.secure_transport,
        )
    } else {
        let content = name_form(&form.full_name, &errors, to_check);
        page_question(
            &state,
            &sid,
            &data,
            "What is your full name?",
            if to_check {
                "/name?return=check-answers"
            } else {
                "/name"
            },
            back_for(StepId::Name, to_check),
            content,
        )
    }
}

fn name_form(full_name: &str, errors: &[FieldError], to_check: bool) -> String {
    let summary = error_summary(errors);
    let input = must_render(
        "input",
        &params(&[
            ("id", v_str("full-name")),
            ("name", v_str("full-name")),
            ("autocomplete", v_str("name")),
            (
                "label",
                v_obj(params(&[
                    ("text", v_str("What is your full name?")),
                    ("classes", v_str("govuk-label--l")),
                    ("isPageHeading", Value::Bool(true)),
                ])),
            ),
            ("value", v_str(full_name)),
            (
                "errorMessage",
                error_message_value(field_error(errors, "full-name")),
            ),
        ]),
    );
    let button = must_render("button", &params(&[("text", v_str("Continue"))]));
    let hidden = return_hidden(to_check);
    format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        {summary}<form method="post" novalidate>{hidden}{input}{button}</form></div></div>"#
    )
}

#[derive(Deserialize)]
pub struct DateOfBirthForm {
    #[serde(rename = "date-of-birth-day", default)]
    day: String,
    #[serde(rename = "date-of-birth-month", default)]
    month: String,
    #[serde(rename = "date-of-birth-year", default)]
    year: String,
    #[serde(rename = "returnTo", default)]
    return_to: Option<String>,
}

pub async fn date_of_birth_get(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<ReturnQuery>,
) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let to_check = return_to_check(q.return_to.as_deref());
    let app = &data.application;
    let content = date_of_birth_form(&app.day, &app.month, &app.year, &[], to_check);
    page_question(
        &state,
        &sid,
        &data,
        "What is your date of birth?",
        if to_check {
            "/date-of-birth?return=check-answers"
        } else {
            "/date-of-birth"
        },
        back_for(StepId::DateOfBirth, to_check),
        content,
    )
}

pub async fn date_of_birth_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<DateOfBirthForm>,
) -> Response<Body> {
    let (sid, mut data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let to_check = return_to_check(form.return_to.as_deref());
    let errors = service::validate_date_of_birth(&form.day, &form.month, &form.year);
    if errors.is_empty() {
        data.application.day = service::clean(&form.day);
        data.application.month = service::clean(&form.month);
        data.application.year = service::clean(&form.year);
        data.application.mark_completed(StepId::DateOfBirth);
        state.sessions.save(&sid, data);
        redirect_with_session(
            next_for(StepId::DateOfBirth, to_check),
            &sid,
            state.assets.secure_transport,
        )
    } else {
        let content = date_of_birth_form(&form.day, &form.month, &form.year, &errors, to_check);
        page_question(
            &state,
            &sid,
            &data,
            "What is your date of birth?",
            if to_check {
                "/date-of-birth?return=check-answers"
            } else {
                "/date-of-birth"
            },
            back_for(StepId::DateOfBirth, to_check),
            content,
        )
    }
}

fn date_of_birth_form(
    day: &str,
    month: &str,
    year: &str,
    errors: &[FieldError],
    to_check: bool,
) -> String {
    let summary = error_summary(errors);
    let message = field_error(errors, "date-of-birth");
    let mut date_params = vec![
        ("id", v_str("date-of-birth")),
        ("namePrefix", v_str("date-of-birth")),
        (
            "fieldset",
            v_obj(params(&[(
                "legend",
                v_obj(params(&[
                    ("text", v_str("What is your date of birth?")),
                    ("isPageHeading", Value::Bool(true)),
                    ("classes", v_str("govuk-fieldset__legend--l")),
                ])),
            )])),
        ),
        (
            "hint",
            v_obj(params(&[("text", v_str("For example, 31 3 1980"))])),
        ),
        (
            "items",
            Value::Array(vec![
                v_obj(params(&[("name", v_str("day")), ("value", v_str(day))])),
                v_obj(params(&[("name", v_str("month")), ("value", v_str(month))])),
                v_obj(params(&[("name", v_str("year")), ("value", v_str(year))])),
            ]),
        ),
    ];
    if let Some(text) = message {
        date_params.push(("errorMessage", v_obj(params(&[("text", v_str(text))]))));
    }
    let date_input = must_render("date-input", &params(&date_params));
    let button = must_render("button", &params(&[("text", v_str("Continue"))]));
    let hidden = return_hidden(to_check);
    format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        {summary}<form method="post" novalidate>{hidden}{date_input}{button}</form></div></div>"#
    )
}

#[derive(Deserialize)]
pub struct CountryForm {
    #[serde(default)]
    country: String,
    #[serde(rename = "returnTo", default)]
    return_to: Option<String>,
}

pub async fn country_get(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<ReturnQuery>,
) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let to_check = return_to_check(q.return_to.as_deref());
    let content = country_form(&data.application.country, &[], to_check);
    page_question(
        &state,
        &sid,
        &data,
        "Where will you fish?",
        if to_check {
            "/where-you-will-fish?return=check-answers"
        } else {
            "/where-you-will-fish"
        },
        back_for(StepId::WhereYouWillFish, to_check),
        content,
    )
}

pub async fn country_post(
    State(state): State<AppState>,
    headers: HeaderMap,
    Form(form): Form<CountryForm>,
) -> Response<Body> {
    let (sid, mut data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let to_check = return_to_check(form.return_to.as_deref());
    let errors = service::validate_country(&form.country);
    if errors.is_empty() {
        data.application.country = form.country;
        data.application.mark_completed(StepId::WhereYouWillFish);
        state.sessions.save(&sid, data);
        redirect_with_session(
            next_for(StepId::WhereYouWillFish, to_check),
            &sid,
            state.assets.secure_transport,
        )
    } else {
        let content = country_form(&form.country, &errors, to_check);
        page_question(
            &state,
            &sid,
            &data,
            "Where will you fish?",
            if to_check {
                "/where-you-will-fish?return=check-answers"
            } else {
                "/where-you-will-fish"
            },
            back_for(StepId::WhereYouWillFish, to_check),
            content,
        )
    }
}

fn country_form(selected: &str, errors: &[FieldError], to_check: bool) -> String {
    let summary = error_summary(errors);
    let message = field_error(errors, "country");
    let items: Vec<Value> = COUNTRIES
        .iter()
        .enumerate()
        .map(|(index, option)| {
            let mut fields = vec![
                ("value", v_str(option.value)),
                ("text", v_str(option.text)),
                ("checked", Value::Bool(selected == option.value)),
            ];
            if index == 0 {
                fields.push(("id", v_str("country")));
            }
            v_obj(params(&fields))
        })
        .collect();
    let mut radio_params = vec![
        ("idPrefix", v_str("country")),
        ("name", v_str("country")),
        (
            "fieldset",
            v_obj(params(&[(
                "legend",
                v_obj(params(&[
                    ("text", v_str("Where will you fish?")),
                    ("isPageHeading", Value::Bool(true)),
                    ("classes", v_str("govuk-fieldset__legend--l")),
                ])),
            )])),
        ),
        (
            "hint",
            v_obj(params(&[(
                "text",
                v_str("This example is fictional. It does not check a real fishing area."),
            )])),
        ),
        ("items", Value::Array(items)),
    ];
    if let Some(text) = message {
        radio_params.push(("errorMessage", v_obj(params(&[("text", v_str(text))]))));
    }
    let radios = must_render("radios", &params(&radio_params));
    let button = must_render("button", &params(&[("text", v_str("Continue"))]));
    let hidden = return_hidden(to_check);
    format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        {summary}<form method="post" novalidate>{hidden}{radios}{button}</form></div></div>"#
    )
}

#[derive(Deserialize)]
pub struct EmailForm {
    #[serde(default)]
    email: String,
    #[serde(rename = "returnTo", default)]
    return_to: Option<String>,
}

pub async fn email_get(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(q): Query<ReturnQuery>,
) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    let to_check = return_to_check(q.return_to.as_deref());
    let content = email_form(&data.application.email, &[], to_check);
    page_question(
        &state,
        &sid,
        &data,
        "What is your email address?",
        if to_check {
            "/email?return=check-answers"
        } else {
            "/email"
        },
        back_for(StepId::Email, to_check),
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
    let to_check = return_to_check(form.return_to.as_deref());
    let errors = service::validate_email(&form.email);
    if errors.is_empty() {
        data.application.email = service::clean(&form.email);
        data.application.mark_completed(StepId::Email);
        state.sessions.save(&sid, data);
        redirect_with_session(
            next_for(StepId::Email, to_check),
            &sid,
            state.assets.secure_transport,
        )
    } else {
        let content = email_form(&form.email, &errors, to_check);
        page_question(
            &state,
            &sid,
            &data,
            "What is your email address?",
            if to_check {
                "/email?return=check-answers"
            } else {
                "/email"
            },
            back_for(StepId::Email, to_check),
            content,
        )
    }
}

fn email_form(email: &str, errors: &[FieldError], to_check: bool) -> String {
    let summary = error_summary(errors);
    let input = must_render(
        "input",
        &params(&[
            ("id", v_str("email")),
            ("name", v_str("email")),
            ("type", v_str("email")),
            ("autocomplete", v_str("email")),
            ("spellcheck", v_bool(false)),
            (
                "hint",
                v_obj(params(&[(
                    "text",
                    v_str("This example stores the address in your browser session only."),
                )])),
            ),
            (
                "label",
                v_obj(params(&[
                    ("text", v_str("What is your email address?")),
                    ("classes", v_str("govuk-label--l")),
                    ("isPageHeading", Value::Bool(true)),
                ])),
            ),
            ("value", v_str(email)),
            (
                "errorMessage",
                error_message_value(field_error(errors, "email")),
            ),
        ]),
    );
    let button = must_render("button", &params(&[("text", v_str("Continue"))]));
    let hidden = return_hidden(to_check);
    format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        {summary}<form method="post" novalidate>{hidden}{input}{button}</form></div></div>"#
    )
}

pub async fn check_answers_get(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    if data.application.submitted {
        return redirect_with_session("/confirmation", &sid, state.assets.secure_transport);
    }
    if !data.application.required_complete() {
        let dest = service::first_incomplete(&data.application)
            .map(|s| s.path)
            .unwrap_or("/licence-length");
        return redirect_with_session(dest, &sid, state.assets.secure_transport);
    }
    let app = &data.application;
    let rows = Value::Array(vec![
        summary_row(
            "Licence length",
            &service::label_for(LICENCE_LENGTHS, &app.licence_length),
            "/licence-length",
            "licence length",
        ),
        summary_row("Name", &app.full_name, "/name", "name"),
        summary_row(
            "Date of birth",
            &service::format_dob(&app.day, &app.month, &app.year),
            "/date-of-birth",
            "date of birth",
        ),
        summary_row(
            "Where you will fish",
            &app.country,
            "/where-you-will-fish",
            "where you will fish",
        ),
        summary_row("Email address", &app.email, "/email", "email address"),
    ]);
    let list = must_render("summary-list", &params(&[("rows", rows)]));
    let button = must_render("button", &params(&[("text", v_str("Accept and continue"))]));
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
        back_href: Some("/email"),
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
    if data.application.submitted {
        return redirect_with_session("/confirmation", &sid, state.assets.secure_transport);
    }
    if !data.application.required_complete() {
        let dest = service::first_incomplete(&data.application)
            .map(|s| s.path)
            .unwrap_or("/licence-length");
        return redirect_with_session(dest, &sid, state.assets.secure_transport);
    }
    data.application.submitted = true;
    data.application.reference = reference_for(&sid);
    state.sessions.save(&sid, data);
    redirect_with_session("/confirmation", &sid, state.assets.secure_transport)
}

pub async fn confirmation(State(state): State<AppState>, headers: HeaderMap) -> Response<Body> {
    let (sid, data) = state
        .sessions
        .get_or_create(session_id_from(&headers).as_deref());
    if !data.application.submitted {
        let dest = service::first_incomplete(&data.application)
            .map(|s| s.path)
            .unwrap_or("/licence-length");
        return redirect_with_session(dest, &sid, state.assets.secure_transport);
    }
    let panel = must_render(
        "panel",
        &params(&[
            ("titleText", v_str("Application complete")),
            (
                "html",
                v_str(format!(
                    "Your example reference number<br><strong>{}</strong>",
                    html_escape(&data.application.reference)
                )),
            ),
        ]),
    );
    let content = format!(
        r#"<div class="govuk-grid-row"><div class="govuk-grid-column-two-thirds">
        {panel}
        <p class="govuk-body">This is a fictional example. Nobody will send you a fishing rod licence.</p>
        <p class="govuk-body"><a class="govuk-link" href="/components">Back to the component list</a></p>
        </div></div>"#
    );
    let html = render_page(Page {
        title: "Application complete",
        content,
        back_href: None,
        breadcrumbs: false,
        sensitive: true,
        welsh: false,
        show_feedback: false,
        assets: &state.assets,
        session: &data,
        return_path: "/confirmation",
    });
    html_response(&state, html, &sid, true)
}

fn summary_row(key: &str, value: &str, href: &str, hidden: &str) -> Value {
    let display = if value.trim().is_empty() {
        "Not provided"
    } else {
        value
    };
    v_obj(params(&[
        ("key", v_obj(params(&[("text", v_str(key))]))),
        ("value", v_obj(params(&[("text", v_str(display))]))),
        (
            "actions",
            v_obj(params(&[(
                "items",
                Value::Array(vec![v_obj(params(&[
                    ("href", v_str(format!("{href}?return=check-answers"))),
                    ("text", v_str("Change")),
                    ("visuallyHiddenText", v_str(hidden)),
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
