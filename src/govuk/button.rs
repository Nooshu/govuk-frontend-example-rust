//! Button and exit-this-page components.

use crate::govuk::attributes::{attribute_if, attributes, classes_if, flag_if};
use crate::govuk::escape::{def_truthy, escape, indent, is_undefined, out, str_val, trim, truthy};
use crate::govuk::params::{Params, Safe, Value};

const START_ICON: &str = concat!(
    "\n",
    r#"  <svg class="govuk-button__start-icon" xmlns="http://www.w3.org/2000/svg" width="17.5" height="19" viewBox="0 0 33 40" aria-hidden="true" focusable="false">"#,
    "\n",
    r#"    <path fill="currentColor" d="M0 0h13l20 20-20 20H0l20-20z"/>"#,
    "\n",
    "  </svg>"
);

const EXIT_THIS_PAGE_DEFAULT_HTML: &str =
    "  <span class=\"govuk-visually-hidden\">Emergency</span> Exit this page\n";

pub fn render_button(p: &Params) -> String {
    let mut class_names = String::from("govuk-button");
    let classes = p.get("classes");
    if truthy(classes) {
        class_names.push(' ');
        class_names.push_str(&str_val(classes));
    }
    let start_button = truthy(p.get("isStartButton"));
    if start_button {
        class_names.push_str(" govuk-button--start");
    }

    let common_attributes = format!(
        r#" class="{}" data-module="govuk-button"{}{}"#,
        escape(&class_names),
        attributes(p.get("attributes")),
        attribute_if("id", p.get("id"))
    );

    let html = p.get("html");
    let mut text = out(p.get("text"));
    if truthy(html) && start_button {
        text = format!("<span>{}</span>", trim(&str_val(html)));
    } else if truthy(html) {
        text = trim(&str_val(html));
    }

    let mut out_s = String::new();
    let href = p.get("href");
    if truthy(href) {
        out_s.push_str(&format!(
            r#"<a href="{}" role="button" draggable="false"{}>"#,
            out(href),
            common_attributes
        ));
        out_s.push_str("\n  ");
        out_s.push_str(&indent(&text, 2, false));
    } else {
        let submit = Value::String("submit".into());
        out_s.push_str(&format!(
            r#"<button type="{}""#,
            out(def_truthy(p.get("type"), &submit))
        ));
        out_s.push_str(&attribute_if("value", p.get("value")));
        out_s.push_str(&attribute_if("name", p.get("name")));
        out_s.push_str(&flag_if(
            r#" disabled aria-disabled="true""#,
            p.get("disabled"),
        ));
        let prevent = p.get("preventDoubleClick");
        if !is_undefined(prevent) {
            out_s.push_str(&format!(r#" data-prevent-double-click="{}""#, out(prevent)));
        }
        out_s.push_str(&common_attributes);
        out_s.push_str(">\n  ");
        out_s.push_str(&indent(&text, 2, false));
    }
    if start_button {
        out_s.push_str(START_ICON);
    }
    if truthy(href) {
        out_s.push_str("\n</a>");
    } else {
        out_s.push_str("\n</button>");
    }
    out_s
}

pub fn render_exit_this_page(p: &Params) -> String {
    let html = p.get("html");
    let html_val = if !truthy(html) && !truthy(p.get("text")) {
        Value::Safe(Safe(EXIT_THIS_PAGE_DEFAULT_HTML.into()))
    } else {
        html.clone()
    };

    let bbc = Value::String("https://www.bbc.co.uk/weather".into());
    let warning_classes = Value::String(
        "govuk-button--warning govuk-exit-this-page__button govuk-js-exit-this-page-button".into(),
    );
    let mut attrs = Params::new();
    attrs.set("rel", Value::String("nofollow noreferrer".into()));

    let button = render_button(&Params::from_pairs(&[
        ("html", html_val),
        ("text", p.get("text").clone()),
        ("classes", warning_classes),
        ("href", def_truthy(p.get("redirectUrl"), &bbc).clone()),
        ("attributes", Value::Object(attrs)),
    ]));

    format!(
        r#"<div{} class="govuk-exit-this-page{}" data-module="govuk-exit-this-page"{}{}{}{}{}>"#,
        attribute_if("id", p.get("id")),
        classes_if(p.get("classes")),
        attributes(p.get("attributes")),
        attribute_if("data-i18n.activated", p.get("activatedText")),
        attribute_if("data-i18n.timed-out", p.get("timedOutText")),
        attribute_if(
            "data-i18n.press-two-more-times",
            p.get("pressTwoMoreTimesText")
        ),
        attribute_if(
            "data-i18n.press-one-more-time",
            p.get("pressOneMoreTimeText")
        ),
    ) + "\n  "
        + &indent(&trim(&button), 2, false)
        + "\n</div>"
}
