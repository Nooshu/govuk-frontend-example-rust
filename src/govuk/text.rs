//! Ports for text components (inset-text, warning-text, tag, …).

use crate::govuk::attributes::{
    attribute_if, attributes, classes_if, content_indent, content_indent_params, content_params,
    flag_if,
};
use crate::govuk::button::render_button;
use crate::govuk::escape::{
    concat_if, contains, escape, get, heading, indent, out, str_val, trim, truthy,
};
use crate::govuk::params::{v_str, Params, Value};

pub fn render_back_link(p: &Params) -> String {
    let back = v_str("Back");
    let mut text = out(crate::govuk::escape::def_truthy(p.get("text"), &back));
    let html = p.get("html");
    if truthy(html) {
        text = str_val(html);
    }
    let hash = v_str("#");
    format!(
        r#"<a href="{}" class="govuk-back-link{}"{}>{}</a>"#,
        out(crate::govuk::escape::def_truthy(p.get("href"), &hash)),
        classes_if(p.get("classes")),
        attributes(p.get("attributes")),
        text
    )
}

pub fn render_skip_link(p: &Params) -> String {
    let content_fallback = v_str("#content");
    format!(
        r#"<a href="{}" class="govuk-skip-link{}"{} data-module="govuk-skip-link">{}</a>"#,
        out(crate::govuk::escape::def_truthy(
            p.get("href"),
            &content_fallback
        )),
        classes_if(p.get("classes")),
        attributes(p.get("attributes")),
        content_params(p, "html", "text")
    )
}

pub fn render_hint(p: &Params) -> String {
    format!(
        r#"<div{} class="govuk-hint{}"{}>"#,
        attribute_if("id", p.get("id")),
        classes_if(p.get("classes")),
        attributes(p.get("attributes"))
    ) + "\n  "
        + &content_indent_params(p, "html", "text", 2)
        + "\n</div>"
}

pub fn render_inset_text(p: &Params) -> String {
    format!(
        r#"<div{} class="govuk-inset-text{}"{}>"#,
        attribute_if("id", p.get("id")),
        classes_if(p.get("classes")),
        attributes(p.get("attributes"))
    ) + "\n  "
        + &content_indent_params(p, "html", "text", 2)
        + "\n</div>"
}

pub fn render_tag(p: &Params) -> String {
    format!(
        r#"<strong class="govuk-tag{}"{}>"#,
        classes_if(p.get("classes")),
        attributes(p.get("attributes"))
    ) + "\n  "
        + &content_indent_params(p, "html", "text", 2)
        + "\n</strong>"
}

pub fn render_warning_text(p: &Params) -> String {
    let warning = v_str("Warning");
    format!(
        r#"<div class="govuk-warning-text{}"{}>"#,
        classes_if(p.get("classes")),
        attributes(p.get("attributes"))
    ) + "\n"
        + "  <span class=\"govuk-warning-text__icon\" aria-hidden=\"true\">!</span>\n"
        + "  <strong class=\"govuk-warning-text__text\">\n"
        + &format!(
            r#"    <span class="govuk-visually-hidden">{}</span>"#,
            out(crate::govuk::escape::def_truthy(
                p.get("iconFallbackText"),
                &warning
            ))
        )
        + "\n    "
        + &content_params(p, "html", "text")
        + "\n  </strong>\n</div>"
}

pub fn render_error_message(p: &Params) -> String {
    let error_fb = v_str("Error");
    let visually_hidden = crate::govuk::escape::def(p.get("visuallyHiddenText"), &error_fb);
    let message = content_indent_params(p, "html", "text", 2);

    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<p{} class="govuk-error-message{}"{}>"#,
        attribute_if("id", p.get("id")),
        classes_if(p.get("classes")),
        attributes(p.get("attributes"))
    ));
    out_s.push('\n');
    if truthy(visually_hidden) {
        out_s.push_str(&format!(
            r#"  <span class="govuk-visually-hidden">{}:</span> {}"#,
            out(visually_hidden),
            message
        ));
        out_s.push('\n');
    } else {
        out_s.push_str("  ");
        out_s.push_str(&message);
        out_s.push('\n');
    }
    out_s.push_str("</p>");
    out_s
}

pub fn render_details(p: &Params) -> String {
    format!(
        r#"<details{} class="govuk-details{}"{}{}>"#,
        attribute_if("id", p.get("id")),
        classes_if(p.get("classes")),
        attributes(p.get("attributes")),
        flag_if(" open", p.get("open"))
    ) + "\n"
        + "  <summary class=\"govuk-details__summary\">\n"
        + "    <span class=\"govuk-details__summary-text\">\n"
        + "      "
        + &content_indent_params(p, "summaryHtml", "summaryText", 6)
        + "\n"
        + "    </span>\n  </summary>\n"
        + "  <div class=\"govuk-details__text\">\n"
        + "    "
        + &content_params(p, "html", "text")
        + "\n"
        + "  </div>\n</details>"
}

pub fn render_label(p: &Params) -> String {
    if !truthy(p.get("html")) && !truthy(p.get("text")) {
        return String::new();
    }
    let label = format!(
        r#"<label class="govuk-label{}"{}{}>"#,
        classes_if(p.get("classes")),
        attributes(p.get("attributes")),
        attribute_if("for", p.get("for"))
    ) + "\n  "
        + &content_indent_params(p, "html", "text", 2)
        + "\n</label>\n";

    if truthy(p.get("isPageHeading")) {
        format!(
            "<h1 class=\"govuk-label-wrapper\">\n  {}\n</h1>\n",
            indent(&trim(&label), 2, false)
        )
    } else {
        format!("{}\n", trim(&label))
    }
}

pub fn render_panel(p: &Params) -> String {
    let classes = p.get("classes");
    let interruption = truthy(classes) && contains(&v_str("govuk-panel--interruption"), classes);
    let level = heading(p.get("headingLevel"), "1");

    let mut out_s = String::from(r#"<div class="govuk-panel"#);
    if !interruption {
        out_s.push_str(" govuk-panel--confirmation");
    }
    out_s.push_str(&classes_if(classes));
    out_s.push('"');
    out_s.push_str(&attributes(p.get("attributes")));
    out_s.push_str(">\n");
    out_s.push_str(&format!(r#"  <h{level} class="govuk-panel__title">"#));
    out_s.push_str("\n    ");
    out_s.push_str(&content_params(p, "titleHtml", "titleText"));
    out_s.push_str(&format!("\n  </h{level}>\n"));

    if truthy(p.get("html")) || truthy(p.get("text")) {
        out_s.push_str("  <div class=\"govuk-panel__body\">\n    ");
        out_s.push_str(&content_indent_params(p, "html", "text", 4));
        out_s.push_str("\n  </div>\n");
    }

    let actions = p.get("actions");
    if interruption && truthy(actions) {
        out_s.push_str(&format!(
            r#"  <div class="govuk-panel__actions{}"{}>"#,
            classes_if(get(actions, &["classes"])),
            attributes(get(actions, &["attributes"]))
        ));
        let entries = items_owned(get(actions, &["items"]));
        if !entries.is_empty() {
            out_s.push_str("<div class=\"govuk-button-group\">\n");
            for action in &entries {
                out_s.push_str("      ");
                out_s.push_str(&indent(&trim(&panel_action(action)), 6, false));
                out_s.push('\n');
            }
            out_s.push_str("    </div>");
        }
        out_s.push_str("</div>\n");
    }

    out_s.push_str("</div>");
    out_s
}

fn items_owned(value: &Value) -> Vec<Value> {
    crate::govuk::escape::items(value).to_vec()
}

fn panel_action(action: &Value) -> String {
    let href = get(action, &["href"]);
    let type_v = get(action, &["type"]);
    if !truthy(href) || str_val(type_v) == "button" {
        let button_type = v_str("button");
        let classes = Value::String(format!(
            "govuk-button--inverse{}",
            concat_if(" ", get(action, &["classes"]))
        ));
        return render_button(&Params::from_pairs(&[
            ("text", get(action, &["text"]).clone()),
            (
                "type",
                crate::govuk::escape::def_truthy(type_v, &button_type).clone(),
            ),
            ("classes", classes),
            ("href", href.clone()),
            ("attributes", get(action, &["attributes"]).clone()),
        ]));
    }
    format!(
        r#"<a class="govuk-link govuk-link--inverse{}" href="{}"{}>{}</a>"#,
        classes_if(get(action, &["classes"])),
        out(href),
        attributes(get(action, &["attributes"])),
        out(get(action, &["text"]))
    )
}

pub fn render_phase_banner(p: &Params) -> String {
    let tag = p.get("tag");
    let tag_classes = Value::String(format!(
        "govuk-phase-banner__content__tag{}",
        concat_if(" ", get(tag, &["classes"]))
    ));
    let tag_html = render_tag(&Params::from_pairs(&[
        ("text", get(tag, &["text"]).clone()),
        ("html", get(tag, &["html"]).clone()),
        ("classes", tag_classes),
    ]));
    format!(
        r#"<div class="govuk-phase-banner govuk-width-container{}"{}>"#,
        classes_if(p.get("classes")),
        attributes(p.get("attributes"))
    ) + "\n"
        + "  <p class=\"govuk-phase-banner__content\">\n"
        + "    "
        + &indent(&trim(&tag_html), 4, false)
        + "\n"
        + "    <span class=\"govuk-phase-banner__text\">\n"
        + "      "
        + &content_indent_params(p, "html", "text", 6)
        + "\n"
        + "    </span>\n  </p>\n</div>"
}

pub fn render_feedback(p: &Params) -> String {
    let level = heading(p.get("headingLevel"), "2");
    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<div class="govuk-feedback govuk-width-container{}"{}>"#,
        classes_if(p.get("classes")),
        attributes(p.get("attributes"))
    ));
    out_s.push('\n');
    out_s.push_str("  <div class=\"govuk-grid-row\">\n");
    out_s.push_str("    <div class=\"govuk-grid-column-two-thirds\">\n");
    out_s.push_str(&format!(
        r#"      <h{level} class="govuk-feedback__title">"#
    ));
    out_s.push_str("\n        ");
    out_s.push_str(&content_params(p, "titleHtml", "titleText"));
    out_s.push_str(&format!("\n      </h{level}>\n"));

    let html = p.get("html");
    let text = p.get("text");
    if truthy(html) || truthy(text) {
        out_s.push_str("        <div class=\"govuk-feedback__body\">\n");
        if truthy(html) {
            out_s.push_str("            ");
            out_s.push_str(&indent(&trim(&str_val(html)), 4, false));
            out_s.push('\n');
        } else if truthy(text) {
            out_s.push_str("            <p class=\"govuk-body\">\n");
            out_s.push_str("              ");
            out_s.push_str(&escape(&indent(&trim(&str_val(text)), 6, false)));
            out_s.push_str("\n            </p>\n");
        }
        out_s.push_str("        </div>\n");
    }

    out_s.push_str("    </div>\n  </div>\n</div>");
    out_s
}

pub fn render_fieldset(p: &Params) -> String {
    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<fieldset class="govuk-fieldset{}"{}{}{}>"#,
        classes_if(p.get("classes")),
        attribute_if("role", p.get("role")),
        attribute_if("aria-describedby", p.get("describedBy")),
        attributes(p.get("attributes"))
    ));
    out_s.push('\n');

    let legend = p.get("legend");
    if truthy(get(legend, &["html"])) || truthy(get(legend, &["text"])) {
        out_s.push_str(&format!(
            r#"  <legend class="govuk-fieldset__legend{}">"#,
            classes_if(get(legend, &["classes"]))
        ));
        out_s.push('\n');
        if truthy(get(legend, &["isPageHeading"])) {
            out_s.push_str("    <h1 class=\"govuk-fieldset__heading\">\n");
            out_s.push_str("      ");
            out_s.push_str(&content_indent(legend, "html", "text", 6));
            out_s.push_str("\n    </h1>\n");
        } else {
            out_s.push_str("    ");
            out_s.push_str(&content_indent(legend, "html", "text", 4));
            out_s.push('\n');
        }
        out_s.push_str("  </legend>\n");
    }

    let html = p.get("html");
    if truthy(html) {
        out_s.push_str("  ");
        out_s.push_str(&str_val(html));
        out_s.push('\n');
    }

    out_s.push_str("</fieldset>");
    out_s
}
