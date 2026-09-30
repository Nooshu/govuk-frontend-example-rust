//! Ports for form components (input, radios, date-input, …).

use crate::govuk::attributes::{
    attribute_if, attributes, classes_if, content_indent, flag_if, i18n_attributes,
};
use crate::govuk::button::render_button;
use crate::govuk::escape::{
    concat_if, contains, def, def_truthy, escape, get, indent, is_undefined, items, loose_eq, out,
    str_val, trim, truthy,
};
use crate::govuk::params::{v_bool, v_obj, v_safe, v_str, Params, Value};
use crate::govuk::text::{render_error_message, render_fieldset, render_hint, render_label};

fn form_group_open(p: &Params) -> String {
    let form_group = p.get("formGroup");
    format!(
        r#"<div class="govuk-form-group{}{}"{}>"#,
        flag_if(" govuk-form-group--error", p.get("errorMessage")),
        classes_if(get(form_group, &["classes"])),
        attributes(get(form_group, &["attributes"]))
    ) + "\n"
}

fn described_by_append(described_by: &str, id: &str) -> String {
    if described_by.is_empty() {
        id.to_string()
    } else {
        format!("{described_by} {id}")
    }
}

fn hint_block(p: &Params, id: &str, described_by: &str, width: usize) -> (String, String) {
    let hint = p.get("hint");
    if !truthy(hint) {
        return (String::new(), described_by.to_string());
    }
    let hint_id = format!("{id}-hint");
    let described_by = described_by_append(described_by, &hint_id);
    let html = render_hint(&Params::from_pairs(&[
        ("id", v_str(hint_id)),
        ("classes", get(hint, &["classes"]).clone()),
        ("attributes", get(hint, &["attributes"]).clone()),
        ("html", get(hint, &["html"]).clone()),
        ("text", get(hint, &["text"]).clone()),
    ]));
    (
        format!(
            "{}{}\n",
            " ".repeat(width),
            indent(&trim(&html), width, false)
        ),
        described_by,
    )
}

fn error_block(p: &Params, id: &str, described_by: &str, width: usize) -> (String, String) {
    let message = p.get("errorMessage");
    if !truthy(message) {
        return (String::new(), described_by.to_string());
    }
    let error_id = format!("{id}-error");
    let described_by = described_by_append(described_by, &error_id);
    let html = render_error_message(&Params::from_pairs(&[
        ("id", v_str(error_id)),
        ("classes", get(message, &["classes"]).clone()),
        ("attributes", get(message, &["attributes"]).clone()),
        ("html", get(message, &["html"]).clone()),
        ("text", get(message, &["text"]).clone()),
        (
            "visuallyHiddenText",
            get(message, &["visuallyHiddenText"]).clone(),
        ),
    ]));
    (
        format!(
            "{}{}\n",
            " ".repeat(width),
            indent(&trim(&html), width, false)
        ),
        described_by,
    )
}

fn label_block(p: &Params, id: &Value, width: usize) -> String {
    let label = p.get("label");
    let html = render_label(&Params::from_pairs(&[
        ("html", get(label, &["html"]).clone()),
        ("text", get(label, &["text"]).clone()),
        ("classes", get(label, &["classes"]).clone()),
        ("isPageHeading", get(label, &["isPageHeading"]).clone()),
        ("attributes", get(label, &["attributes"]).clone()),
        ("for", id.clone()),
    ]));
    format!(
        "{}{}\n",
        " ".repeat(width),
        indent(&trim(&html), width, false)
    )
}

fn slot_content(slot: &Value, width: usize, indent_first: bool) -> String {
    let html = get(slot, &["html"]);
    if truthy(html) {
        return indent(&trim(&str_val(html)), width, indent_first);
    }
    out(get(slot, &["text"]))
}

fn component_id(p: &Params) -> Value {
    let id = p.get("id");
    if truthy(id) {
        id.clone()
    } else {
        p.get("name").clone()
    }
}

pub fn render_input(p: &Params) -> String {
    let mut class_names = String::from("govuk-input");
    let classes = p.get("classes");
    if truthy(classes) {
        class_names.push(' ');
        class_names.push_str(&str_val(classes));
    }
    if truthy(p.get("errorMessage")) {
        class_names.push_str(" govuk-input--error");
    }

    let id = component_id(p);
    let mut described_by = String::new();
    let supplied = p.get("describedBy");
    if truthy(supplied) {
        described_by = str_val(supplied);
    }

    let form_group = p.get("formGroup");
    let prefix = p.get("prefix");
    let suffix = p.get("suffix");
    let before_input = get(form_group, &["beforeInput"]);
    let after_input = get(form_group, &["afterInput"]);
    let has_prefix =
        truthy(prefix) && (truthy(get(prefix, &["text"])) || truthy(get(prefix, &["html"])));
    let has_suffix =
        truthy(suffix) && (truthy(get(suffix, &["text"])) || truthy(get(suffix, &["html"])));
    let has_before = truthy(before_input)
        && (truthy(get(before_input, &["text"])) || truthy(get(before_input, &["html"])));
    let has_after = truthy(after_input)
        && (truthy(get(after_input, &["text"])) || truthy(get(after_input, &["html"])));

    let mut out_s = String::new();
    out_s.push_str(&form_group_open(p));
    out_s.push_str(&label_block(p, &id, 2));

    let (hint, described_by) = hint_block(p, &str_val(&id), &described_by, 2);
    out_s.push_str(&hint);
    let (error_message, described_by) = error_block(p, &str_val(&id), &described_by, 2);
    out_s.push_str(&error_message);

    let element = input_element(p, &class_names, &id, &described_by);
    if has_prefix || has_suffix || has_before || has_after {
        let wrapper = p.get("inputWrapper");
        out_s.push_str(&format!(
            r#"  <div class="govuk-input__wrapper{}"{}>"#,
            classes_if(get(wrapper, &["classes"])),
            attributes(get(wrapper, &["attributes"]))
        ));
        out_s.push('\n');
        if has_before {
            out_s.push_str(&slot_content(before_input, 4, true));
            out_s.push('\n');
        }
        if has_prefix {
            out_s.push_str(&indent(&affix_item(prefix, "prefix"), 2, true));
            out_s.push('\n');
        }
        out_s.push_str("    ");
        out_s.push_str(&element);
        out_s.push('\n');
        if has_suffix {
            out_s.push_str(&indent(&affix_item(suffix, "suffix"), 2, true));
            out_s.push('\n');
        }
        if has_after {
            out_s.push_str(&slot_content(after_input, 4, true));
            out_s.push('\n');
        }
        out_s.push_str("  </div>\n");
    } else {
        out_s.push_str("  ");
        out_s.push_str(&element);
        out_s.push('\n');
    }

    out_s.push_str("</div>");
    out_s
}

fn input_element(p: &Params, class_names: &str, id: &Value, described_by: &str) -> String {
    let spellcheck = match p.get("spellcheck") {
        Value::Bool(flag) => v_str(if *flag { "true" } else { "false" }),
        _ => v_bool(false),
    };
    let aria_described_by = if described_by.is_empty() {
        Value::Undefined
    } else {
        v_str(described_by)
    };
    let text_type = v_str("text");
    let attributes_p = Params::from_pairs(&[
        ("class", v_str(class_names)),
        ("id", id.clone()),
        ("name", p.get("name").clone()),
        ("type", def_truthy(p.get("type"), &text_type).clone()),
        (
            "spellcheck",
            v_obj(Params::from_pairs(&[
                ("value", spellcheck),
                ("optional", v_bool(true)),
            ])),
        ),
        (
            "value",
            v_obj(Params::from_pairs(&[
                ("value", p.get("value").clone()),
                ("optional", v_bool(true)),
            ])),
        ),
        (
            "disabled",
            v_obj(Params::from_pairs(&[
                ("value", p.get("disabled").clone()),
                ("optional", v_bool(true)),
            ])),
        ),
        (
            "aria-describedby",
            v_obj(Params::from_pairs(&[
                ("value", aria_described_by),
                ("optional", v_bool(true)),
            ])),
        ),
        (
            "autocomplete",
            v_obj(Params::from_pairs(&[
                ("value", p.get("autocomplete").clone()),
                ("optional", v_bool(true)),
            ])),
        ),
        (
            "autocapitalize",
            v_obj(Params::from_pairs(&[
                ("value", p.get("autocapitalize").clone()),
                ("optional", v_bool(true)),
            ])),
        ),
        (
            "pattern",
            v_obj(Params::from_pairs(&[
                ("value", p.get("pattern").clone()),
                ("optional", v_bool(true)),
            ])),
        ),
        (
            "inputmode",
            v_obj(Params::from_pairs(&[
                ("value", p.get("inputmode").clone()),
                ("optional", v_bool(true)),
            ])),
        ),
    ]);
    format!(
        "<input{}{}>",
        attributes(&v_obj(attributes_p)),
        attributes(p.get("attributes"))
    )
}

fn affix_item(affix: &Value, kind: &str) -> String {
    format!(
        r#"  <div class="govuk-input__{}{}" aria-hidden="true"{}>{}</div>"#,
        kind,
        classes_if(get(affix, &["classes"])),
        attributes(get(affix, &["attributes"])),
        content_indent(affix, "html", "text", 4)
    )
}

pub fn render_textarea(p: &Params) -> String {
    let id = component_id(p);
    let mut described_by = String::new();
    let supplied = p.get("describedBy");
    if truthy(supplied) {
        described_by = str_val(supplied);
    }
    let form_group = p.get("formGroup");

    let mut out_s = String::new();
    out_s.push_str(&form_group_open(p));
    out_s.push_str(&label_block(p, &id, 2));

    let (hint, described_by) = hint_block(p, &str_val(&id), &described_by, 2);
    out_s.push_str(&hint);
    let (error_message, described_by) = error_block(p, &str_val(&id), &described_by, 2);
    out_s.push_str(&error_message);

    let before = get(form_group, &["beforeInput"]);
    if truthy(before) {
        out_s.push_str("  ");
        out_s.push_str(&slot_content(before, 2, false));
        out_s.push('\n');
    }

    let spellcheck = match p.get("spellcheck") {
        Value::Bool(flag) => format!(r#" spellcheck="{}""#, if *flag { "true" } else { "false" }),
        _ => String::new(),
    };
    let rows = v_str("5");
    let described_by_val = v_str(&described_by);
    out_s.push_str(&format!(
        r#"  <textarea class="govuk-textarea{}{}" id="{}" name="{}" rows="{}"{}{}{}{}{}>{}</textarea>"#,
        flag_if(" govuk-textarea--error", p.get("errorMessage")),
        classes_if(p.get("classes")),
        out(&id),
        out(p.get("name")),
        out(def_truthy(p.get("rows"), &rows)),
        spellcheck,
        flag_if(" disabled", p.get("disabled")),
        attribute_if("aria-describedby", &described_by_val),
        attribute_if("autocomplete", p.get("autocomplete")),
        attributes(p.get("attributes")),
        out(p.get("value"))
    ));
    out_s.push('\n');

    let after = get(form_group, &["afterInput"]);
    if truthy(after) {
        out_s.push_str("  ");
        out_s.push_str(&slot_content(after, 2, false));
        out_s.push('\n');
    }

    out_s.push_str("</div>");
    out_s
}

pub fn render_select(p: &Params) -> String {
    let id = component_id(p);
    let mut described_by = String::new();
    let supplied = p.get("describedBy");
    if truthy(supplied) {
        described_by = str_val(supplied);
    }
    let form_group = p.get("formGroup");

    let mut out_s = String::new();
    out_s.push_str(&form_group_open(p));
    out_s.push_str(&label_block(p, &id, 2));

    let (hint, described_by) = hint_block(p, &str_val(&id), &described_by, 2);
    out_s.push_str(&hint);
    let (error_message, described_by) = error_block(p, &str_val(&id), &described_by, 2);
    out_s.push_str(&error_message);

    let before = get(form_group, &["beforeInput"]);
    if truthy(before) {
        out_s.push_str("  ");
        out_s.push_str(&slot_content(before, 2, false));
        out_s.push('\n');
    }

    let described_by_val = v_str(&described_by);
    out_s.push_str(&format!(
        r#"  <select class="govuk-select{}{}" id="{}" name="{}"{}{}{}>"#,
        classes_if(p.get("classes")),
        flag_if(" govuk-select--error", p.get("errorMessage")),
        out(&id),
        out(p.get("name")),
        flag_if(" disabled", p.get("disabled")),
        attribute_if("aria-describedby", &described_by_val),
        attributes(p.get("attributes"))
    ));
    out_s.push('\n');

    let selected = p.get("value");
    for item in items(p.get("items")) {
        if !truthy(item) {
            continue;
        }
        let value = get(item, &["value"]);
        let effective = def(value, get(item, &["text"]));
        let mut is_selected = truthy(get(item, &["selected"]));
        if !is_selected && truthy(selected) {
            is_selected = loose_eq(effective, selected)
                && !loose_eq(get(item, &["selected"]), &v_bool(false));
        }

        out_s.push_str("    <option");
        if !is_undefined(value) {
            out_s.push_str(&format!(r#" value="{}""#, out(value)));
        }
        out_s.push_str(&flag_if(" selected", &v_bool(is_selected)));
        out_s.push_str(&flag_if(" disabled", get(item, &["disabled"])));
        out_s.push_str(&attributes(get(item, &["attributes"])));
        out_s.push('>');
        out_s.push_str(&out(get(item, &["text"])));
        out_s.push_str("</option>\n");
    }
    out_s.push_str("  </select>\n");

    let after = get(form_group, &["afterInput"]);
    if truthy(after) {
        out_s.push_str("  ");
        out_s.push_str(&slot_content(after, 2, false));
        out_s.push('\n');
    }

    out_s.push_str("</div>");
    out_s
}

pub fn render_file_upload(p: &Params) -> String {
    let id = component_id(p);
    let mut described_by = String::new();
    let supplied = p.get("describedBy");
    if truthy(supplied) {
        described_by = str_val(supplied);
    }
    let form_group = p.get("formGroup");

    let mut out_s = String::new();
    out_s.push_str(&form_group_open(p));
    out_s.push_str(&label_block(p, &id, 2));

    let (hint, described_by) = hint_block(p, &str_val(&id), &described_by, 2);
    out_s.push_str(&hint);
    let (error_message, described_by) = error_block(p, &str_val(&id), &described_by, 2);
    out_s.push_str(&error_message);

    let before = get(form_group, &["beforeInput"]);
    if truthy(before) {
        out_s.push_str("  ");
        out_s.push_str(&slot_content(before, 2, false));
        out_s.push('\n');
    }

    let javascript = truthy(p.get("javascript"));
    if javascript {
        out_s.push_str("  <div\n    class=\"govuk-file-upload-wrapper");
        out_s.push_str(&classes_if(p.get("wrapperClasses")));
        out_s.push_str("\"\n    data-module=\"govuk-file-upload\"");
        out_s.push_str(&i18n_attributes(
            "choose-files-button",
            p.get("chooseFilesButtonText"),
            &Value::Undefined,
        ));
        out_s.push_str(&i18n_attributes(
            "no-file-chosen",
            p.get("noFileChosenText"),
            &Value::Undefined,
        ));
        out_s.push_str(&i18n_attributes(
            "multiple-files-chosen",
            &Value::Undefined,
            p.get("multipleFilesChosenText"),
        ));
        out_s.push_str(&i18n_attributes(
            "drop-instruction",
            p.get("dropInstructionText"),
            &Value::Undefined,
        ));
        out_s.push_str(&i18n_attributes(
            "entered-drop-zone",
            p.get("enteredDropZoneText"),
            &Value::Undefined,
        ));
        out_s.push_str(&i18n_attributes(
            "left-drop-zone",
            p.get("leftDropZoneText"),
            &Value::Undefined,
        ));
        out_s.push_str(&attributes(p.get("wrapperAttributes")));
        out_s.push_str("\n  >\n");
    }

    let described_by_val = v_str(&described_by);
    out_s.push_str(&format!(
        r#"  <input class="govuk-file-upload{}{}" id="{}" name="{}" type="file"{}{}{}{}>"#,
        classes_if(p.get("classes")),
        flag_if(" govuk-file-upload--error", p.get("errorMessage")),
        out(&id),
        out(p.get("name")),
        flag_if(" disabled", p.get("disabled")),
        flag_if(" multiple", p.get("multiple")),
        attribute_if("aria-describedby", &described_by_val),
        attributes(p.get("attributes"))
    ));
    out_s.push('\n');

    if javascript {
        out_s.push_str("  </div>\n");
    }
    let after = get(form_group, &["afterInput"]);
    if truthy(after) {
        out_s.push_str("  ");
        out_s.push_str(&slot_content(after, 2, false));
        out_s.push('\n');
    }

    out_s.push_str("</div>");
    out_s
}

pub fn render_character_count(p: &Params) -> String {
    let maxwords = p.get("maxwords");
    let maxlength = p.get("maxlength");
    let has_no_limit = !truthy(maxwords) && !truthy(maxlength);
    let id = component_id(p);

    let description_no_limit = if !has_no_limit {
        let limit = if truthy(maxwords) {
            maxwords
        } else {
            maxlength
        };
        let unit = if truthy(maxwords) {
            "words"
        } else {
            "characters"
        };
        let mut description = format!("You can enter up to %{{count}} {unit}");
        let supplied = p.get("textareaDescriptionText");
        if truthy(supplied) {
            description = str_val(supplied);
        }
        v_str(description.replace("%{count}", &str_val(limit)))
    } else {
        Value::Undefined
    };

    let count_message = p.get("countMessage");
    let mut count_message_html = format!(
        "{}\n",
        trim(&render_hint(&Params::from_pairs(&[
            ("text", description_no_limit),
            ("id", v_str(format!("{}-info", str_val(&id)))),
            (
                "classes",
                v_str(format!(
                    "govuk-character-count__message{}",
                    concat_if(" ", get(count_message, &["classes"]))
                ))
            ),
        ])))
    );

    let form_group = p.get("formGroup");
    let after = get(form_group, &["afterInput"]);
    if truthy(after) {
        let html = get(after, &["html"]);
        if truthy(html) {
            count_message_html.push_str(&trim(&str_val(html)));
            count_message_html.push('\n');
        } else {
            count_message_html.push_str(&out(get(after, &["text"])));
            count_message_html.push('\n');
        }
    }

    let mut attributes_html = attributes(&v_obj(Params::from_pairs(&[
        ("data-module", v_str("govuk-character-count")),
        (
            "data-maxlength",
            v_obj(Params::from_pairs(&[
                ("value", maxlength.clone()),
                ("optional", v_bool(true)),
            ])),
        ),
        (
            "data-threshold",
            v_obj(Params::from_pairs(&[
                ("value", p.get("threshold").clone()),
                ("optional", v_bool(true)),
            ])),
        ),
        (
            "data-maxwords",
            v_obj(Params::from_pairs(&[
                ("value", maxwords.clone()),
                ("optional", v_bool(true)),
            ])),
        ),
    ])));
    let description = p.get("textareaDescriptionText");
    if has_no_limit && truthy(description) {
        attributes_html.push_str(&i18n_attributes(
            "textarea-description",
            &Value::Undefined,
            &v_obj(Params::from_pairs(&[("other", description.clone())])),
        ));
    }
    attributes_html.push_str(&i18n_attributes(
        "characters-under-limit",
        &Value::Undefined,
        p.get("charactersUnderLimitText"),
    ));
    attributes_html.push_str(&i18n_attributes(
        "characters-at-limit",
        p.get("charactersAtLimitText"),
        &Value::Undefined,
    ));
    attributes_html.push_str(&i18n_attributes(
        "characters-over-limit",
        &Value::Undefined,
        p.get("charactersOverLimitText"),
    ));
    attributes_html.push_str(&i18n_attributes(
        "words-under-limit",
        &Value::Undefined,
        p.get("wordsUnderLimitText"),
    ));
    attributes_html.push_str(&i18n_attributes(
        "words-at-limit",
        p.get("wordsAtLimitText"),
        &Value::Undefined,
    ));
    attributes_html.push_str(&i18n_attributes(
        "words-over-limit",
        &Value::Undefined,
        p.get("wordsOverLimitText"),
    ));
    attributes_html.push_str(&appended_attributes(get(form_group, &["attributes"])));

    let label = p.get("label");
    trim(&render_textarea(&Params::from_pairs(&[
        ("id", id.clone()),
        ("name", p.get("name").clone()),
        ("describedBy", v_str(format!("{}-info", str_val(&id)))),
        ("rows", p.get("rows").clone()),
        ("spellcheck", p.get("spellcheck").clone()),
        ("value", p.get("value").clone()),
        (
            "formGroup",
            v_obj(Params::from_pairs(&[
                (
                    "classes",
                    v_str(format!(
                        "govuk-character-count{}",
                        concat_if(" ", get(form_group, &["classes"]))
                    )),
                ),
                ("attributes", v_safe(attributes_html)),
                ("beforeInput", get(form_group, &["beforeInput"]).clone()),
                (
                    "afterInput",
                    v_obj(Params::from_pairs(&[("html", v_safe(count_message_html))])),
                ),
            ])),
        ),
        (
            "classes",
            v_str(format!(
                "govuk-js-character-count{}",
                concat_if(" ", p.get("classes"))
            )),
        ),
        (
            "label",
            v_obj(Params::from_pairs(&[
                ("html", get(label, &["html"]).clone()),
                ("text", get(label, &["text"]).clone()),
                ("classes", get(label, &["classes"]).clone()),
                ("isPageHeading", get(label, &["isPageHeading"]).clone()),
                ("attributes", get(label, &["attributes"]).clone()),
                ("for", id.clone()),
            ])),
        ),
        ("hint", p.get("hint").clone()),
        ("errorMessage", p.get("errorMessage").clone()),
        ("attributes", p.get("attributes").clone()),
    ])))
}

fn appended_attributes(attributes_v: &Value) -> String {
    let Value::Object(object) = attributes_v else {
        return String::new();
    };
    let mut out_s = String::new();
    for name in object.keys() {
        out_s.push_str(&format!(
            r#" {}="{}""#,
            escape(name),
            escape(&str_val(object.get(name)))
        ));
    }
    out_s
}

pub fn render_password_input(p: &Params) -> String {
    let id = component_id(p);
    let form_group = p.get("formGroup");

    let mut attributes_html = String::from(r#" data-module="govuk-password-input""#);
    attributes_html.push_str(&i18n_attributes(
        "show-password",
        p.get("showPasswordText"),
        &Value::Undefined,
    ));
    attributes_html.push_str(&i18n_attributes(
        "hide-password",
        p.get("hidePasswordText"),
        &Value::Undefined,
    ));
    attributes_html.push_str(&i18n_attributes(
        "show-password-aria-label",
        p.get("showPasswordAriaLabelText"),
        &Value::Undefined,
    ));
    attributes_html.push_str(&i18n_attributes(
        "hide-password-aria-label",
        p.get("hidePasswordAriaLabelText"),
        &Value::Undefined,
    ));
    attributes_html.push_str(&i18n_attributes(
        "password-shown-announcement",
        p.get("passwordShownAnnouncementText"),
        &Value::Undefined,
    ));
    attributes_html.push_str(&i18n_attributes(
        "password-hidden-announcement",
        p.get("passwordHiddenAnnouncementText"),
        &Value::Undefined,
    ));
    attributes_html.push_str(&appended_attributes(get(form_group, &["attributes"])));

    let button = p.get("button");
    let show = v_str("Show");
    let show_label = v_str("Show password");
    let mut button_html = format!(
        "{}\n",
        trim(&render_button(&Params::from_pairs(&[
            ("type", v_str("button")),
            (
                "classes",
                v_str(format!(
                    "govuk-button--secondary govuk-password-input__toggle govuk-js-password-input-toggle{}",
                    concat_if(" ", get(button, &["classes"]))
                ))
            ),
            (
                "text",
                def(p.get("showPasswordText"), &show).clone()
            ),
            (
                "attributes",
                v_obj(Params::from_pairs(&[
                    ("aria-controls", id.clone()),
                    (
                        "aria-label",
                        def(p.get("showPasswordAriaLabelText"), &show_label).clone(),
                    ),
                    (
                        "hidden",
                        v_obj(Params::from_pairs(&[
                            ("value", v_bool(true)),
                            ("optional", v_bool(true)),
                        ])),
                    ),
                ])),
            ),
        ])))
    );
    let after = get(form_group, &["afterInput"]);
    if truthy(after) {
        let html = get(after, &["html"]);
        if truthy(html) {
            button_html.push_str(&trim(&str_val(html)));
            button_html.push('\n');
        } else {
            button_html.push_str(&out(get(after, &["text"])));
            button_html.push('\n');
        }
    }

    let current_password = v_str("current-password");
    trim(&render_input(&Params::from_pairs(&[
        (
            "formGroup",
            v_obj(Params::from_pairs(&[
                (
                    "classes",
                    v_str(format!(
                        "govuk-password-input{}",
                        concat_if(" ", get(form_group, &["classes"]))
                    )),
                ),
                ("attributes", v_safe(attributes_html)),
                ("beforeInput", get(form_group, &["beforeInput"]).clone()),
                (
                    "afterInput",
                    v_obj(Params::from_pairs(&[("html", v_safe(button_html))])),
                ),
            ])),
        ),
        (
            "inputWrapper",
            v_obj(Params::from_pairs(&[(
                "classes",
                v_str("govuk-password-input__wrapper"),
            )])),
        ),
        ("label", p.get("label").clone()),
        ("hint", p.get("hint").clone()),
        (
            "classes",
            v_str(format!(
                "govuk-password-input__input govuk-js-password-input-input{}",
                concat_if(" ", p.get("classes"))
            )),
        ),
        ("errorMessage", p.get("errorMessage").clone()),
        ("id", id.clone()),
        ("name", p.get("name").clone()),
        ("type", v_str("password")),
        ("spellcheck", v_bool(false)),
        ("autocapitalize", v_str("none")),
        (
            "autocomplete",
            def_truthy(p.get("autocomplete"), &current_password).clone(),
        ),
        ("value", p.get("value").clone()),
        ("disabled", p.get("disabled").clone()),
        ("describedBy", p.get("describedBy").clone()),
        ("attributes", p.get("attributes").clone()),
    ])))
}

pub fn render_checkboxes(p: &Params) -> String {
    let mut id_prefix = p.get("idPrefix").clone();
    if !truthy(&id_prefix) {
        id_prefix = p.get("name").clone();
    }
    let fieldset = p.get("fieldset");
    let mut described_by = String::new();
    let supplied = p.get("describedBy");
    if truthy(supplied) {
        described_by = str_val(supplied);
    }
    let supplied = get(fieldset, &["describedBy"]);
    if truthy(supplied) {
        described_by = str_val(supplied);
    }
    let has_fieldset = truthy(fieldset);

    let mut inner = String::new();
    let (hint, described_by) = hint_block(p, &str_val(&id_prefix), &described_by, 2);
    inner.push_str(&hint);
    let (error_message, described_by) = error_block(p, &str_val(&id_prefix), &described_by, 2);
    inner.push_str(&error_message);

    let form_group = p.get("formGroup");
    inner.push_str(&format!(
        r#"  <div class="govuk-checkboxes{}"{} data-module="govuk-checkboxes">"#,
        classes_if(p.get("classes")),
        attributes(p.get("attributes"))
    ));
    inner.push('\n');
    let before = get(form_group, &["beforeInputs"]);
    if truthy(before) {
        inner.push_str("    ");
        inner.push_str(&slot_content(before, 4, false));
        inner.push('\n');
    }
    for (index, item) in items(p.get("items")).iter().enumerate() {
        if !truthy(item) {
            continue;
        }
        inner.push_str(&checkbox_item(
            p,
            item,
            index + 1,
            &str_val(&id_prefix),
            &described_by,
            has_fieldset,
        ));
    }
    let after = get(form_group, &["afterInputs"]);
    if truthy(after) {
        inner.push_str("    ");
        inner.push_str(&slot_content(after, 4, false));
        inner.push('\n');
    }
    inner.push_str("  </div>\n");

    fieldset_wrapper(p, &inner, &described_by, &Value::Undefined, false)
}

fn fieldset_wrapper(
    p: &Params,
    inner: &str,
    described_by: &str,
    role: &Value,
    indent_fieldset: bool,
) -> String {
    let fieldset = p.get("fieldset");
    let mut body = trim(inner);
    if truthy(fieldset) {
        let html = render_fieldset(&Params::from_pairs(&[
            ("describedBy", v_str(described_by)),
            ("classes", get(fieldset, &["classes"]).clone()),
            ("role", role.clone()),
            ("attributes", get(fieldset, &["attributes"]).clone()),
            ("legend", get(fieldset, &["legend"]).clone()),
            ("html", v_safe(body)),
        ]));
        body = trim(&html);
        if indent_fieldset {
            body = indent(&body, 2, false);
        }
    }
    format!("{}  {}\n</div>", form_group_open(p), body)
}

fn checkbox_item(
    p: &Params,
    item: &Value,
    index: usize,
    id_prefix: &str,
    described_by: &str,
    has_fieldset: bool,
) -> String {
    let mut item_id = id_prefix.to_string();
    if index > 1 {
        item_id.push('-');
        item_id.push_str(&index.to_string());
    }
    let supplied = get(item, &["id"]);
    if truthy(supplied) {
        item_id = str_val(supplied);
    }
    let mut item_name = p.get("name").clone();
    let supplied = get(item, &["name"]);
    if truthy(supplied) {
        item_name = supplied.clone();
    }
    let conditional_id = format!("conditional-{item_id}");

    let divider = get(item, &["divider"]);
    if truthy(divider) {
        return format!(
            r#"    <div class="govuk-checkboxes__divider">{}</div>"#,
            out(divider)
        ) + "\n";
    }

    let mut checked = truthy(get(item, &["checked"]));
    if !checked && truthy(p.get("values")) {
        checked = contains(get(item, &["value"]), p.get("values"))
            && !loose_eq(get(item, &["checked"]), &v_bool(false));
    }
    let hint = get(item, &["hint"]);
    let has_hint = truthy(get(hint, &["text"])) || truthy(get(hint, &["html"]));
    let item_hint_id = if has_hint {
        format!("{item_id}-item-hint")
    } else {
        String::new()
    };
    let mut item_described_by = if !has_fieldset {
        described_by.to_string()
    } else {
        String::new()
    };
    item_described_by = trim(&format!("{item_described_by} {item_hint_id}"));

    let conditional = get(item, &["conditional"]);
    let label = get(item, &["label"]);

    let mut out_s = String::from("    <div class=\"govuk-checkboxes__item\">\n");
    out_s.push_str(&format!(
        r#"      <input class="govuk-checkboxes__input" id="{}" name="{}" type="checkbox" value="{}"{}{}{}{}{}{}>"#,
        escape(&item_id),
        out(&item_name),
        out(get(item, &["value"])),
        flag_if(" checked", &v_bool(checked)),
        flag_if(" disabled", get(item, &["disabled"])),
        attribute_if(
            "data-aria-controls",
            &if_truthy(get(conditional, &["html"]), &conditional_id),
        ),
        attribute_if("data-behaviour", get(item, &["behaviour"])),
        attribute_if("aria-describedby", &v_str(&item_described_by)),
        attributes(get(item, &["attributes"]))
    ));
    out_s.push('\n');
    out_s.push_str("      ");
    out_s.push_str(&indent(
        &trim(&render_label(&Params::from_pairs(&[
            ("html", get(item, &["html"]).clone()),
            ("text", get(item, &["text"]).clone()),
            (
                "classes",
                v_str(format!(
                    "govuk-checkboxes__label{}",
                    concat_if(" ", get(label, &["classes"]))
                )),
            ),
            ("attributes", get(label, &["attributes"]).clone()),
            ("for", v_str(&item_id)),
        ]))),
        6,
        false,
    ));
    out_s.push('\n');
    if has_hint {
        out_s.push_str("      ");
        out_s.push_str(&indent(
            &trim(&render_hint(&Params::from_pairs(&[
                ("id", v_str(&item_hint_id)),
                (
                    "classes",
                    v_str(format!(
                        "govuk-checkboxes__hint{}",
                        concat_if(" ", get(hint, &["classes"]))
                    )),
                ),
                ("attributes", get(hint, &["attributes"]).clone()),
                ("html", get(hint, &["html"]).clone()),
                ("text", get(hint, &["text"]).clone()),
            ]))),
            6,
            false,
        ));
        out_s.push('\n');
    }
    out_s.push_str("    </div>\n");
    let html = get(conditional, &["html"]);
    if truthy(html) {
        out_s.push_str(&format!(
            r#"    <div class="govuk-checkboxes__conditional{}" id="{}">"#,
            flag_if(" govuk-checkboxes__conditional--hidden", &v_bool(!checked)),
            escape(&conditional_id)
        ));
        out_s.push_str("\n      ");
        out_s.push_str(&trim(&str_val(html)));
        out_s.push_str("\n    </div>\n");
    }
    out_s
}

fn if_truthy(condition: &Value, value: &str) -> Value {
    if truthy(condition) {
        v_str(value)
    } else {
        Value::Undefined
    }
}

fn if_truthy_bool(condition: bool, value: &str) -> Value {
    if condition {
        v_str(value)
    } else {
        Value::Undefined
    }
}

pub fn render_radios(p: &Params) -> String {
    let mut id_prefix = p.get("idPrefix").clone();
    if !truthy(&id_prefix) {
        id_prefix = p.get("name").clone();
    }
    let fieldset = p.get("fieldset");
    let mut described_by = String::new();
    let supplied = get(fieldset, &["describedBy"]);
    if truthy(supplied) {
        described_by = str_val(supplied);
    }

    let mut inner = String::new();
    let (hint, described_by) = hint_block(p, &str_val(&id_prefix), &described_by, 2);
    inner.push_str(&hint);
    let (error_message, described_by) = error_block(p, &str_val(&id_prefix), &described_by, 2);
    inner.push_str(&error_message);

    let form_group = p.get("formGroup");
    inner.push_str(&format!(
        r#"  <div class="govuk-radios{}"{} data-module="govuk-radios">"#,
        classes_if(p.get("classes")),
        attributes(p.get("attributes"))
    ));
    inner.push('\n');
    let before = get(form_group, &["beforeInputs"]);
    if truthy(before) {
        inner.push_str("    ");
        inner.push_str(&slot_content(before, 4, false));
        inner.push('\n');
    }
    for (index, item) in items(p.get("items")).iter().enumerate() {
        if !truthy(item) {
            continue;
        }
        inner.push_str(&radio_item(p, item, index + 1, &str_val(&id_prefix)));
    }
    let after = get(form_group, &["afterInputs"]);
    if truthy(after) {
        inner.push_str("    ");
        inner.push_str(&slot_content(after, 4, false));
        inner.push('\n');
    }
    inner.push_str("  </div>\n");

    fieldset_wrapper(p, &inner, &described_by, &Value::Undefined, false)
}

fn radio_item(p: &Params, item: &Value, index: usize, id_prefix: &str) -> String {
    let mut item_id = id_prefix.to_string();
    if index > 1 {
        item_id.push('-');
        item_id.push_str(&index.to_string());
    }
    let supplied = get(item, &["id"]);
    if truthy(supplied) {
        item_id = str_val(supplied);
    }
    let conditional_id = format!("conditional-{item_id}");

    let divider = get(item, &["divider"]);
    if truthy(divider) {
        return format!(
            r#"    <div class="govuk-radios__divider">{}</div>"#,
            out(divider)
        ) + "\n";
    }

    let mut checked = truthy(get(item, &["checked"]));
    if !checked && truthy(p.get("value")) {
        checked = loose_eq(get(item, &["value"]), p.get("value"))
            && !loose_eq(get(item, &["checked"]), &v_bool(false));
    }
    let hint = get(item, &["hint"]);
    let has_hint = truthy(get(hint, &["text"])) || truthy(get(hint, &["html"]));
    let item_hint_id = format!("{item_id}-item-hint");
    let conditional = get(item, &["conditional"]);
    let label = get(item, &["label"]);

    let mut out_s = String::from("    <div class=\"govuk-radios__item\">\n");
    out_s.push_str(&format!(
        r#"      <input class="govuk-radios__input" id="{}" name="{}" type="radio" value="{}"{}{}{}{}{}>"#,
        escape(&item_id),
        out(p.get("name")),
        out(get(item, &["value"])),
        flag_if(" checked", &v_bool(checked)),
        flag_if(" disabled", get(item, &["disabled"])),
        attribute_if(
            "data-aria-controls",
            &if_truthy(get(conditional, &["html"]), &conditional_id),
        ),
        attribute_if("aria-describedby", &if_truthy_bool(has_hint, &item_hint_id)),
        attributes(get(item, &["attributes"]))
    ));
    out_s.push('\n');
    out_s.push_str("      ");
    out_s.push_str(&indent(
        &trim(&render_label(&Params::from_pairs(&[
            ("html", get(item, &["html"]).clone()),
            ("text", get(item, &["text"]).clone()),
            (
                "classes",
                v_str(format!(
                    "govuk-radios__label{}",
                    concat_if(" ", get(label, &["classes"]))
                )),
            ),
            ("attributes", get(label, &["attributes"]).clone()),
            ("for", v_str(&item_id)),
        ]))),
        6,
        false,
    ));
    out_s.push('\n');
    if has_hint {
        out_s.push_str("      ");
        out_s.push_str(&indent(
            &trim(&render_hint(&Params::from_pairs(&[
                ("id", v_str(&item_hint_id)),
                (
                    "classes",
                    v_str(format!(
                        "govuk-radios__hint{}",
                        concat_if(" ", get(hint, &["classes"]))
                    )),
                ),
                ("attributes", get(hint, &["attributes"]).clone()),
                ("html", get(hint, &["html"]).clone()),
                ("text", get(hint, &["text"]).clone()),
            ]))),
            6,
            false,
        ));
        out_s.push('\n');
    }
    out_s.push_str("    </div>\n");
    let html = get(conditional, &["html"]);
    if truthy(html) {
        out_s.push_str(&format!(
            r#"    <div class="govuk-radios__conditional{}" id="{}">"#,
            flag_if(" govuk-radios__conditional--hidden", &v_bool(!checked)),
            escape(&conditional_id)
        ));
        out_s.push_str("\n      ");
        out_s.push_str(&trim(&str_val(html)));
        out_s.push_str("\n    </div>\n");
    }
    out_s
}

pub fn render_date_input(p: &Params) -> String {
    let fieldset = p.get("fieldset");
    let mut described_by = String::new();
    let supplied = get(fieldset, &["describedBy"]);
    if truthy(supplied) {
        described_by = str_val(supplied);
    }
    let values = p.get("values");

    let day_default = v_obj(Params::from_pairs(&[
        ("name", v_str("day")),
        ("value", get(values, &["day"]).clone()),
        ("classes", v_str("govuk-input--width-2")),
    ]));
    let month_default = v_obj(Params::from_pairs(&[
        ("name", v_str("month")),
        ("value", get(values, &["month"]).clone()),
        ("classes", v_str("govuk-input--width-2")),
    ]));
    let year_default = v_obj(Params::from_pairs(&[
        ("name", v_str("year")),
        ("value", get(values, &["year"]).clone()),
        ("classes", v_str("govuk-input--width-4")),
    ]));
    let day = def(p.get("day"), &day_default).clone();
    let month = def(p.get("month"), &month_default).clone();
    let year = def(p.get("year"), &year_default).clone();

    // Match default day/month/year by identity (like Go pointers), not deep equality —
    // fixtures often pass `{error: true}` for each part with no distinguishing name.
    let custom = items(p.get("items"));
    let using_defaults = custom.is_empty();
    let date_item_refs: Vec<&Value> = if using_defaults {
        vec![&day, &month, &year]
    } else {
        custom.iter().collect()
    };

    let any_item_has_error = date_item_refs.iter().any(|item| date_item_has_error(item));

    let mut inner = String::new();
    let (hint, described_by) = hint_block(p, &str_val(p.get("id")), &described_by, 2);
    inner.push_str(&hint);
    let (error_message, described_by) = error_block(p, &str_val(p.get("id")), &described_by, 2);
    inner.push_str(&error_message);

    let form_group = p.get("formGroup");
    inner.push_str(&format!(
        r#"  <div class="govuk-date-input{}"{}{}>"#,
        classes_if(p.get("classes")),
        attributes(p.get("attributes")),
        attribute_if("id", p.get("id"))
    ));
    inner.push('\n');
    let before = get(form_group, &["beforeInputs"]);
    if truthy(before) {
        inner.push_str("    ");
        inner.push_str(&slot_content(before, 4, false));
        inner.push('\n');
    }
    for item in &date_item_refs {
        if !truthy(item) {
            continue;
        }
        let is_day = std::ptr::eq(*item, &day);
        let is_month = std::ptr::eq(*item, &month);
        let is_year = std::ptr::eq(*item, &year);
        inner.push_str(&indent(
            &trim(&date_input_item(
                p,
                item,
                any_item_has_error,
                &day,
                &month,
                &year,
                is_day,
                is_month,
                is_year,
            )),
            4,
            true,
        ));
        inner.push('\n');
    }
    let after = get(form_group, &["afterInputs"]);
    if truthy(after) {
        inner.push_str("    ");
        inner.push_str(&slot_content(after, 4, false));
        inner.push('\n');
    }
    inner.push_str("  </div>\n");

    fieldset_wrapper(p, &inner, &described_by, &v_str("group"), true)
}

fn date_item_has_error(item: &Value) -> bool {
    if truthy(get(item, &["error"])) {
        return true;
    }
    let classes = get(item, &["classes"]);
    truthy(classes) && contains(&v_str("govuk-input--error"), classes)
}

#[allow(clippy::too_many_arguments)]
fn date_input_item(
    p: &Params,
    item: &Value,
    any_item_has_error: bool,
    day: &Value,
    month: &Value,
    year: &Value,
    is_day: bool,
    is_month: bool,
    is_year: bool,
) -> String {
    let mut item_name = get(item, &["name"]).clone();
    let mut item_value = get(item, &["value"]).clone();
    let mut item_width = "2".to_string();
    let mut item_classes = String::new();
    let item_has_error = date_item_has_error(item);

    let name = get(item, &["name"]);
    let day_names = Value::Array(vec![v_str("day"), get(day, &["name"]).clone()]);
    let month_names = Value::Array(vec![v_str("month"), get(month, &["name"]).clone()]);
    let year_names = Value::Array(vec![v_str("year"), get(year, &["name"]).clone()]);

    if is_day || (truthy(name) && contains(name, &day_names)) {
        let day_fb = v_str("day");
        item_name = def(name, &day_fb).clone();
        item_value = def(&item_value, get(day, &["value"])).clone();
    } else if is_month || (truthy(name) && contains(name, &month_names)) {
        let month_fb = v_str("month");
        item_name = def(name, &month_fb).clone();
        item_value = def(&item_value, get(month, &["value"])).clone();
    } else if is_year || (truthy(name) && contains(name, &year_names)) {
        let year_fb = v_str("year");
        item_name = def(name, &year_fb).clone();
        item_value = def(&item_value, get(year, &["value"])).clone();
        item_width = "4".to_string();
    }

    let classes = get(item, &["classes"]);
    let has_error_class = truthy(classes) && contains(&v_str("govuk-input--error"), classes);
    if !has_error_class
        && (item_has_error
            || (!loose_eq(get(item, &["error"]), &v_bool(false))
                && truthy(p.get("errorMessage"))
                && !any_item_has_error))
    {
        item_classes = trim(&format!("{item_classes} govuk-input--error"));
    }
    if !truthy(classes) || !contains(&v_str("govuk-input--width-"), classes) {
        item_classes = trim(&format!("{item_classes} govuk-input--width-{item_width}"));
    }
    if truthy(classes) {
        item_classes = trim(&format!("{item_classes} {}", str_val(classes)));
    }

    let mut name_prefix = String::new();
    let prefix = p.get("namePrefix");
    if truthy(prefix) {
        name_prefix = format!("{}-", str_val(prefix));
    }

    let mut label = get(item, &["label"]).clone();
    if !truthy(&label) {
        label = v_str(capitalise(&str_val(&item_name)));
    }
    let mut id = get(item, &["id"]).clone();
    if !truthy(&id) {
        id = v_str(format!("{}-{}", str_val(p.get("id")), str_val(&item_name)));
    }
    let mut value = item_value;
    if is_undefined(&value) {
        let key = format!("{name_prefix}{}", str_val(&item_name));
        value = match p.get("values") {
            Value::Object(obj) => obj.get(&key).clone(),
            _ => Value::Undefined,
        };
    }

    let numeric = v_str("numeric");
    let input = render_input(&Params::from_pairs(&[
        (
            "label",
            v_obj(Params::from_pairs(&[
                ("text", label),
                ("classes", v_str("govuk-date-input__label")),
            ])),
        ),
        ("id", id),
        (
            "classes",
            v_str(format!(
                "govuk-date-input__input{}",
                concat_if(" ", &v_str(&item_classes))
            )),
        ),
        (
            "name",
            v_str(format!("{name_prefix}{}", str_val(&item_name))),
        ),
        ("value", value),
        ("type", v_str("text")),
        (
            "inputmode",
            def_truthy(get(item, &["inputmode"]), &numeric).clone(),
        ),
        ("autocomplete", get(item, &["autocomplete"]).clone()),
        ("pattern", get(item, &["pattern"]).clone()),
        ("attributes", get(item, &["attributes"]).clone()),
    ]));

    format!(
        "<div class=\"govuk-date-input__item\">\n  {}\n</div>",
        indent(&trim(&input), 2, false)
    )
}

fn capitalise(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }
    let lowered = text.to_lowercase();
    let mut chars = lowered.chars();
    let first = chars.next().unwrap().to_uppercase().to_string();
    first + chars.as_str()
}
