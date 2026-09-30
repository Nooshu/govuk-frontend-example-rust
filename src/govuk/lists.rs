//! Ports for list-like components (table, task-list, summary-list, …).

use crate::govuk::attributes::{
    attribute_if, attributes, classes_if, content, content_indent, content_indent_params, flag_if,
    i18n_attributes,
};
use crate::govuk::escape::{
    def, def_truthy, escape, get, heading, indent, is_undefined, items, length, out, str_val, trim,
    truthy,
};
use crate::govuk::params::{v_str, Params, Value};
use crate::govuk::text::render_tag;

pub fn render_accordion(p: &Params) -> String {
    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<div class="govuk-accordion{}" data-module="govuk-accordion" id="{}""#,
        classes_if(p.get("classes")),
        out(p.get("id"))
    ));
    out_s.push_str(&i18n_attributes(
        "hide-all-sections",
        p.get("hideAllSectionsText"),
        &Value::Undefined,
    ));
    out_s.push_str(&i18n_attributes(
        "hide-section",
        p.get("hideSectionText"),
        &Value::Undefined,
    ));
    out_s.push_str(&i18n_attributes(
        "hide-section-aria-label",
        p.get("hideSectionAriaLabelText"),
        &Value::Undefined,
    ));
    out_s.push_str(&i18n_attributes(
        "show-all-sections",
        p.get("showAllSectionsText"),
        &Value::Undefined,
    ));
    out_s.push_str(&i18n_attributes(
        "show-section",
        p.get("showSectionText"),
        &Value::Undefined,
    ));
    out_s.push_str(&i18n_attributes(
        "show-section-aria-label",
        p.get("showSectionAriaLabelText"),
        &Value::Undefined,
    ));
    let remember = p.get("rememberExpanded");
    if !is_undefined(remember) {
        out_s.push_str(&format!(
            r#" data-remember-expanded="{}""#,
            escape(&str_val(remember))
        ));
    }
    out_s.push_str(&attributes(p.get("attributes")));
    out_s.push_str(">\n");
    for (index, item) in items(p.get("items")).iter().enumerate() {
        if !truthy(item) {
            continue;
        }
        out_s.push_str(&accordion_item(p, item, index + 1));
    }
    out_s.push_str("</div>");
    out_s
}

fn accordion_item(p: &Params, item: &Value, index: usize) -> String {
    let level = heading(p.get("headingLevel"), "2");
    let id = out(p.get("id"));
    let position = index.to_string();
    let item_heading = get(item, &["heading"]);
    let summary = get(item, &["summary"]);
    let item_content = get(item, &["content"]);
    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"  <div class="govuk-accordion__section{}">"#,
        flag_if(
            " govuk-accordion__section--expanded",
            get(item, &["expanded"])
        )
    ));
    out_s.push('\n');
    out_s.push_str("    <div class=\"govuk-accordion__section-header\">\n");
    out_s.push_str(&format!(
        "      <h{level} class=\"govuk-accordion__section-heading\">\n"
    ));
    out_s.push_str(&format!(
        r#"        <span class="govuk-accordion__section-button" id="{id}-heading-{position}">"#
    ));
    out_s.push('\n');
    out_s.push_str("          ");
    out_s.push_str(&content_indent(item_heading, "html", "text", 8));
    out_s.push_str(&format!("\n        </span>\n      </h{level}>\n"));
    if truthy(get(summary, &["html"])) || truthy(get(summary, &["text"])) {
        out_s.push_str(&format!(
            r#"      <div class="govuk-accordion__section-summary govuk-body" id="{id}-summary-{position}">"#
        ));
        out_s.push('\n');
        out_s.push_str("        ");
        out_s.push_str(&content_indent(summary, "html", "text", 8));
        out_s.push_str("\n      </div>\n");
    }
    out_s.push_str("    </div>\n");
    out_s.push_str(&format!(
        r#"    <div id="{id}-content-{position}" class="govuk-accordion__section-content">"#
    ));
    out_s.push('\n');
    let html = get(item_content, &["html"]);
    let text = get(item_content, &["text"]);
    if truthy(html) {
        out_s.push_str("      ");
        out_s.push_str(&indent(&trim(&str_val(html)), 6, false));
        out_s.push('\n');
    } else if truthy(text) {
        out_s.push_str("      <p class=\"govuk-body\">\n");
        out_s.push_str("        ");
        out_s.push_str(&escape(&indent(&trim(&str_val(text)), 8, false)));
        out_s.push_str("\n      </p>\n");
    }
    out_s.push_str("    </div>\n  </div>\n");
    out_s
}

pub fn render_error_summary(p: &Params) -> String {
    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<div class="govuk-error-summary{}""#,
        classes_if(p.get("classes"))
    ));
    let auto_focus = p.get("disableAutoFocus");
    if !is_undefined(auto_focus) {
        out_s.push_str(&format!(
            r#" data-disable-auto-focus="{}""#,
            out(auto_focus)
        ));
    }
    out_s.push_str(&attributes(p.get("attributes")));
    out_s.push_str(r#" data-module="govuk-error-summary">"#);
    out_s.push_str("\n  <div role=\"alert\">\n");
    out_s.push_str("    <h2 class=\"govuk-error-summary__title\">\n");
    out_s.push_str("      ");
    out_s.push_str(&content_indent_params(p, "titleHtml", "titleText", 6));
    out_s.push_str("\n    </h2>\n");
    out_s.push_str("    <div class=\"govuk-error-summary__body\">\n");
    if truthy(p.get("descriptionHtml")) || truthy(p.get("descriptionText")) {
        out_s.push_str("      <p>\n        ");
        out_s.push_str(&content_indent_params(
            p,
            "descriptionHtml",
            "descriptionText",
            8,
        ));
        out_s.push_str("\n      </p>\n");
    }
    let error_list = items(p.get("errorList"));
    if !error_list.is_empty() {
        out_s.push_str("        <ul class=\"govuk-list govuk-error-summary__list\">\n");
        for item in error_list {
            out_s.push_str("          <li>\n");
            let href = get(item, &["href"]);
            if truthy(href) {
                out_s.push_str(&format!(
                    r#"            <a href="{}"{}>{}</a>"#,
                    out(href),
                    attributes(get(item, &["attributes"])),
                    content_indent(item, "html", "text", 12)
                ));
                out_s.push('\n');
            } else {
                out_s.push_str("            ");
                out_s.push_str(&content_indent(item, "html", "text", 10));
                out_s.push('\n');
            }
            out_s.push_str("          </li>\n");
        }
        out_s.push_str("        </ul>\n");
    }
    out_s.push_str("    </div>\n  </div>\n</div>");
    out_s
}

pub fn render_notification_banner(p: &Params) -> String {
    let success = str_val(p.get("type")) == "success";
    let type_class = if success {
        format!(
            " govuk-notification-banner--{}",
            escape(&str_val(p.get("type")))
        )
    } else {
        String::new()
    };
    let role = if truthy(p.get("role")) {
        str_val(p.get("role"))
    } else if success {
        "alert".into()
    } else {
        "region".into()
    };
    let title = if truthy(p.get("titleHtml")) {
        str_val(p.get("titleHtml"))
    } else if truthy(p.get("titleText")) {
        out(p.get("titleText"))
    } else if success {
        "Success".into()
    } else {
        "Important".into()
    };
    let title_id_fb = v_str("govuk-notification-banner-title");
    let level_fb = v_str("2");
    let title_id = out(def_truthy(p.get("titleId"), &title_id_fb));
    let level = out(def_truthy(p.get("titleHeadingLevel"), &level_fb));
    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<div class="govuk-notification-banner{}{}" role="{}" aria-labelledby="{}" data-module="govuk-notification-banner""#,
        type_class,
        classes_if(p.get("classes")),
        escape(&role),
        title_id
    ));
    let auto_focus = p.get("disableAutoFocus");
    if !is_undefined(auto_focus) {
        out_s.push_str(&format!(
            r#" data-disable-auto-focus="{}""#,
            out(auto_focus)
        ));
    }
    out_s.push_str(&attributes(p.get("attributes")));
    out_s.push_str(">\n");
    out_s.push_str("  <div class=\"govuk-notification-banner__header\">\n");
    out_s.push_str(&format!(
        r#"    <h{level} class="govuk-notification-banner__title" id="{title_id}">"#
    ));
    out_s.push('\n');
    out_s.push_str(&format!("      {title}\n    </h{level}>\n  </div>\n"));
    out_s.push_str("  <div class=\"govuk-notification-banner__content\">\n");
    let html = p.get("html");
    let text = p.get("text");
    if truthy(html) {
        out_s.push_str("    ");
        out_s.push_str(&indent(&trim(&str_val(html)), 4, false));
        out_s.push('\n');
    } else if truthy(text) {
        out_s.push_str("    <p class=\"govuk-notification-banner__heading\">\n");
        out_s.push_str("      ");
        out_s.push_str(&escape(&indent(&trim(&str_val(text)), 6, false)));
        out_s.push_str("\n    </p>\n");
    }
    out_s.push_str("  </div>\n</div>");
    out_s
}

pub fn render_summary_list(p: &Params) -> String {
    let card = p.get("card");
    let card_title = get(card, &["title"]);

    let mut any_row_has_actions = false;
    for row in items(p.get("rows")) {
        if length(get(row, &["actions", "items"])) > 0 {
            any_row_has_actions = true;
        }
    }

    let mut list = String::new();
    list.push_str(&format!(
        r#"<dl class="govuk-summary-list{}"{}>"#,
        classes_if(p.get("classes")),
        attributes(p.get("attributes"))
    ));
    list.push('\n');
    for row in items(p.get("rows")) {
        if !truthy(row) {
            continue;
        }
        let key = get(row, &["key"]);
        let value = get(row, &["value"]);
        let actions = get(row, &["actions"]);
        list.push_str(&format!(
            r#"  <div class="govuk-summary-list__row{}{}">"#,
            flag_if(
                " govuk-summary-list__row--no-actions",
                &Value::Bool(any_row_has_actions && !truthy(get(actions, &["items"])))
            ),
            classes_if(get(row, &["classes"]))
        ));
        list.push('\n');
        list.push_str(&format!(
            r#"    <dt class="govuk-summary-list__key{}">"#,
            classes_if(get(key, &["classes"]))
        ));
        list.push('\n');
        list.push_str("      ");
        list.push_str(&content_indent(key, "html", "text", 6));
        list.push_str("\n    </dt>\n");
        list.push_str(&format!(
            r#"    <dd class="govuk-summary-list__value{}">"#,
            classes_if(get(value, &["classes"]))
        ));
        list.push('\n');
        list.push_str("      ");
        list.push_str(&content_indent(value, "html", "text", 6));
        list.push_str("\n    </dd>\n");

        let entries = items(get(actions, &["items"]));
        if !entries.is_empty() {
            list.push_str(&format!(
                r#"    <dd class="govuk-summary-list__actions{}">"#,
                classes_if(get(actions, &["classes"]))
            ));
            list.push('\n');
            if entries.len() == 1 {
                list.push_str(&indent(
                    &trim(&summary_action_link(&entries[0], card_title)),
                    6,
                    true,
                ));
                list.push('\n');
            } else {
                list.push_str("      <ul class=\"govuk-summary-list__actions-list\">\n");
                for action in entries {
                    list.push_str("        <li class=\"govuk-summary-list__actions-list-item\">\n");
                    list.push_str("          ");
                    list.push_str(&indent(
                        &trim(&summary_action_link(action, card_title)),
                        8,
                        false,
                    ));
                    list.push('\n');
                    list.push_str("        </li>\n");
                }
                list.push_str("      </ul>\n");
            }
            list.push_str("    </dd>\n");
        }
        list.push_str("  </div>\n");
    }
    list.push_str("</dl>");

    if truthy(card) {
        summary_card(card, &indent(&trim(&list), 4, false))
    } else {
        trim(&list)
    }
}

fn summary_action_link(action: &Value, card_title: &Value) -> String {
    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"  <a class="govuk-link{}" href="{}"{}>"#,
        classes_if(get(action, &["classes"])),
        out(get(action, &["href"])),
        attributes(get(action, &["attributes"]))
    ));
    let html = get(action, &["html"]);
    if truthy(html) {
        out_s.push_str(&indent(&str_val(html), 4, false));
    } else {
        out_s.push_str(&out(get(action, &["text"])));
    }
    let visually_hidden = get(action, &["visuallyHiddenText"]);
    if truthy(visually_hidden) || truthy(card_title) {
        out_s.push_str(r#"<span class="govuk-visually-hidden">"#);
        if truthy(visually_hidden) {
            out_s.push(' ');
            out_s.push_str(&out(visually_hidden));
        }
        if truthy(card_title) {
            let mut title = out(get(card_title, &["text"]));
            let title_html = get(card_title, &["html"]);
            if truthy(title_html) {
                title = indent(&str_val(title_html), 6, false);
            }
            out_s.push_str(" (");
            out_s.push_str(&title);
            out_s.push(')');
        }
        out_s.push_str("</span>");
    }
    out_s.push_str("</a>\n");
    out_s
}

fn summary_card(card: &Value, body: &str) -> String {
    let title = get(card, &["title"]);
    let level = heading(get(title, &["headingLevel"]), "2");
    let actions = get(card, &["actions"]);

    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<div class="govuk-summary-card{}"{}>"#,
        classes_if(get(card, &["classes"])),
        attributes(get(card, &["attributes"]))
    ));
    out_s.push('\n');
    out_s.push_str("  <div class=\"govuk-summary-card__title-wrapper\">\n");
    if truthy(title) {
        out_s.push_str(&format!(
            r#"    <h{level} class="govuk-summary-card__title{}">"#,
            classes_if(get(title, &["classes"]))
        ));
        out_s.push('\n');
        out_s.push_str("      ");
        out_s.push_str(&content_indent(title, "html", "text", 6));
        out_s.push_str(&format!("\n    </h{level}>\n"));
    }
    let entries = items(get(actions, &["items"]));
    if !entries.is_empty() {
        if entries.len() == 1 {
            out_s.push_str(&format!(
                r#"    <div class="govuk-summary-card__actions{}">"#,
                classes_if(get(actions, &["classes"]))
            ));
            out_s.push('\n');
            out_s.push_str("      ");
            out_s.push_str(&indent(
                &trim(&summary_action_link(&entries[0], title)),
                4,
                false,
            ));
            out_s.push('\n');
            out_s.push_str("    </div>\n");
        } else {
            out_s.push_str(&format!(
                r#"    <ul class="govuk-summary-card__actions{}">"#,
                classes_if(get(actions, &["classes"]))
            ));
            out_s.push('\n');
            for action in entries {
                out_s.push_str("      <li class=\"govuk-summary-card__action\">\n");
                out_s.push_str("        ");
                out_s.push_str(&indent(
                    &trim(&summary_action_link(action, title)),
                    8,
                    false,
                ));
                out_s.push('\n');
                out_s.push_str("      </li>\n");
            }
            out_s.push_str("    </ul>\n");
        }
    }
    out_s.push_str("  </div>\n\n");
    out_s.push_str("  <div class=\"govuk-summary-card__content\">\n    ");
    out_s.push_str(body);
    out_s.push_str("\n  </div>\n</div>\n");
    out_s
}

pub fn render_table(p: &Params) -> String {
    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<table class="govuk-table{}"{}>"#,
        classes_if(p.get("classes")),
        attributes(p.get("attributes"))
    ));
    out_s.push('\n');

    let caption = p.get("caption");
    if truthy(caption) {
        out_s.push_str(&format!(
            r#"  <caption class="govuk-table__caption{}">{}</caption>"#,
            classes_if(p.get("captionClasses")),
            out(caption)
        ));
        out_s.push('\n');
    }

    let head = items(p.get("head"));
    if truthy(p.get("head")) {
        out_s.push_str("  <thead class=\"govuk-table__head\">\n");
        out_s.push_str("    <tr class=\"govuk-table__row\">\n");
        for item in head {
            out_s.push_str(&format!(
                r#"      <th scope="col" class="govuk-table__header{}{}"{}{}{}>{}</th>"#,
                format_class("govuk-table__header--", get(item, &["format"])),
                classes_if(get(item, &["classes"])),
                attribute_if("colspan", get(item, &["colspan"])),
                attribute_if("rowspan", get(item, &["rowspan"])),
                attributes(get(item, &["attributes"])),
                content(item, "html", "text")
            ));
            out_s.push('\n');
        }
        out_s.push_str("    </tr>\n  </thead>\n");
    }

    out_s.push_str("  <tbody class=\"govuk-table__body\">\n");
    for row in items(p.get("rows")) {
        if !truthy(row) {
            continue;
        }
        out_s.push_str("    <tr class=\"govuk-table__row\">\n");
        for (index, cell) in items(row).iter().enumerate() {
            let common = format!(
                "{}{}{}",
                attribute_if("colspan", get(cell, &["colspan"])),
                attribute_if("rowspan", get(cell, &["rowspan"])),
                attributes(get(cell, &["attributes"]))
            );
            if index == 0 && truthy(p.get("firstCellIsHeader")) {
                out_s.push_str(&format!(
                    r#"      <th scope="row" class="govuk-table__header{}"{}>{}</th>"#,
                    classes_if(get(cell, &["classes"])),
                    common,
                    content(cell, "html", "text")
                ));
                out_s.push('\n');
            } else {
                out_s.push_str(&format!(
                    r#"      <td class="govuk-table__cell{}{}"{}>{}</td>"#,
                    format_class("govuk-table__cell--", get(cell, &["format"])),
                    classes_if(get(cell, &["classes"])),
                    common,
                    content(cell, "html", "text")
                ));
                out_s.push('\n');
            }
        }
        out_s.push_str("    </tr>\n");
    }
    out_s.push_str("  </tbody>\n</table>");
    out_s
}

fn format_class(prefix: &str, format: &Value) -> String {
    if !truthy(format) {
        return String::new();
    }
    format!(" {prefix}{}", out(format))
}

pub fn render_tabs(p: &Params) -> String {
    let mut id_prefix = String::new();
    let prefix = p.get("idPrefix");
    if truthy(prefix) {
        id_prefix = str_val(prefix);
    }

    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<div{} class="govuk-tabs{}"{} data-module="govuk-tabs">"#,
        attribute_if("id", p.get("id")),
        classes_if(p.get("classes")),
        attributes(p.get("attributes"))
    ));
    out_s.push('\n');
    let title_fb = v_str("Contents");
    out_s.push_str("  <h2 class=\"govuk-tabs__title\">\n    ");
    out_s.push_str(&out(def(p.get("title"), &title_fb)));
    out_s.push_str("\n  </h2>\n");

    let entries = items(p.get("items"));
    if !entries.is_empty() {
        out_s.push_str("  <ul class=\"govuk-tabs__list\">\n");
        for (index, item) in entries.iter().enumerate() {
            if !truthy(item) {
                continue;
            }
            out_s.push_str(&indent(
                &trim(&tab_list_item(item, index + 1, &id_prefix)),
                4,
                true,
            ));
            out_s.push('\n');
        }
        out_s.push_str("  </ul>\n");
        for (index, item) in entries.iter().enumerate() {
            if !truthy(item) {
                continue;
            }
            out_s.push_str(&indent(
                &trim(&tab_panel(item, index + 1, &id_prefix)),
                2,
                true,
            ));
            out_s.push('\n');
        }
    }

    out_s.push_str("</div>");
    out_s
}

fn tab_panel_id(item: &Value, index: usize, id_prefix: &str) -> String {
    let id = get(item, &["id"]);
    if truthy(id) {
        str_val(id)
    } else {
        format!("{id_prefix}-{index}")
    }
}

fn tab_list_item(item: &Value, index: usize, id_prefix: &str) -> String {
    format!(
        r##"<li class="govuk-tabs__list-item{}">
  <a class="govuk-tabs__tab" href="#{}"{}>
    {}
  </a>
</li>
"##,
        flag_if(" govuk-tabs__list-item--selected", &Value::Bool(index == 1)),
        escape(&tab_panel_id(item, index, id_prefix)),
        attributes(get(item, &["attributes"])),
        out(get(item, &["label"])),
    )
}

fn tab_panel(item: &Value, index: usize, id_prefix: &str) -> String {
    let panel = get(item, &["panel"]);
    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<div class="govuk-tabs__panel{}" id="{}"{}>"#,
        flag_if(" govuk-tabs__panel--hidden", &Value::Bool(index > 1)),
        escape(&tab_panel_id(item, index, id_prefix)),
        attributes(get(panel, &["attributes"]))
    ));
    out_s.push('\n');
    let html = get(panel, &["html"]);
    let text = get(panel, &["text"]);
    if truthy(html) {
        out_s.push_str("  ");
        out_s.push_str(&indent(&trim(&str_val(html)), 2, false));
        out_s.push('\n');
    } else if truthy(text) {
        out_s.push_str(&format!(r#"  <p class="govuk-body">{}</p>"#, out(text)));
        out_s.push('\n');
    }
    out_s.push_str("</div>\n");
    out_s
}

pub fn render_task_list(p: &Params) -> String {
    let mut id_prefix = String::from("task-list");
    let prefix = p.get("idPrefix");
    if truthy(prefix) {
        id_prefix = str_val(prefix);
    }

    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"<ul class="govuk-task-list{}"{}>"#,
        classes_if(p.get("classes")),
        attributes(p.get("attributes"))
    ));
    out_s.push('\n');
    for (index, item) in items(p.get("items")).iter().enumerate() {
        if truthy(item) {
            out_s.push_str(&task_list_item(item, index + 1, &id_prefix));
            out_s.push('\n');
        } else {
            out_s.push('\n');
        }
    }
    out_s.push_str("</ul>");
    out_s
}

fn task_list_item(item: &Value, index: usize, id_prefix: &str) -> String {
    let position = index.to_string();
    let hint_id = format!("{id_prefix}-{position}-hint");
    let status_id = format!("{id_prefix}-{position}-status");
    let title = get(item, &["title"]);
    let hint = get(item, &["hint"]);
    let status = get(item, &["status"]);

    let mut out_s = String::new();
    out_s.push_str(&format!(
        r#"  <li class="govuk-task-list__item{}{}">"#,
        flag_if(" govuk-task-list__item--with-link", get(item, &["href"])),
        classes_if(get(item, &["classes"]))
    ));
    out_s.push('\n');
    out_s.push_str("    <div class=\"govuk-task-list__name-and-hint\">\n");

    let href = get(item, &["href"]);
    if truthy(href) {
        let described_by = if truthy(hint) {
            format!("{hint_id} {status_id}")
        } else {
            status_id.clone()
        };
        out_s.push_str(&format!(
            r#"      <a class="govuk-link govuk-task-list__link{}" href="{}" aria-describedby="{}">"#,
            classes_if(get(title, &["classes"])),
            out(href),
            escape(&described_by)
        ));
        out_s.push('\n');
        out_s.push_str("        ");
        out_s.push_str(&content_indent(title, "html", "text", 8));
        out_s.push_str("\n      </a>\n");
    } else {
        out_s.push_str("      <div");
        out_s.push_str(&attribute_if("class", get(title, &["classes"])));
        out_s.push_str(">\n");
        out_s.push_str("        ");
        out_s.push_str(&content_indent(title, "html", "text", 8));
        out_s.push_str("\n      </div>\n");
    }

    if truthy(hint) {
        out_s.push_str(&format!(
            r#"      <div id="{}" class="govuk-task-list__hint">"#,
            escape(&hint_id)
        ));
        out_s.push('\n');
        out_s.push_str("        ");
        out_s.push_str(&content_indent(hint, "html", "text", 8));
        out_s.push_str("\n      </div>\n");
    }
    out_s.push_str("    </div>\n");

    out_s.push_str(&format!(
        r#"    <div class="govuk-task-list__status{}" id="{}">"#,
        classes_if(get(status, &["classes"])),
        escape(&status_id)
    ));
    out_s.push('\n');
    let tag = get(status, &["tag"]);
    if truthy(tag) {
        if let Value::Object(tag_params) = tag {
            out_s.push_str("      ");
            out_s.push_str(&indent(&trim(&render_tag(tag_params)), 6, false));
            out_s.push('\n');
        }
    } else {
        out_s.push_str("      ");
        out_s.push_str(&content_indent(status, "html", "text", 6));
        out_s.push('\n');
    }
    out_s.push_str("    </div>\n  </li>");
    out_s
}
