//! Attribute and i18n helpers matching Nunjucks macros.

use crate::govuk::escape::{escape, get, indent, out, str_val, trim, truthy};
use crate::govuk::params::{Params, Safe, Value};

/// Renders the `attributes` option like Frontend's `govukAttributes` macro.
pub fn attributes(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Safe(Safe(s)) => s.clone(),
        Value::Object(p) => {
            let mut out = String::new();
            for name in p.keys() {
                out.push_str(&attribute(name, p.get(name)));
            }
            out
        }
        _ => String::new(),
    }
}

fn attribute(name: &str, item: &Value) -> String {
    let (value, optional) = match item {
        Value::Object(options) => {
            let optional = matches!(options.get("optional"), Value::Bool(true));
            (options.get("value"), optional)
        }
        other => (other, false),
    };

    let empty = matches!(value, Value::Null | Value::Undefined);
    let escaped = if empty {
        String::new()
    } else if let Value::Safe(Safe(s)) = value {
        s.clone()
    } else {
        escape(&str_val(value))
    };

    if optional {
        if let Value::Bool(true) = value {
            return format!(" {}", escape(name));
        }
        if empty || matches!(value, Value::Bool(false)) {
            return String::new();
        }
    }
    format!(" {}=\"{}\"", escape(name), escaped)
}

pub fn i18n_attributes(key: &str, message: &Value, messages: &Value) -> String {
    if truthy(messages) {
        let Value::Object(object) = messages else {
            return String::new();
        };
        let mut out = String::new();
        for rule in object.keys() {
            out.push_str(&format!(
                " data-i18n.{key}.{rule}=\"{}\"",
                escape(&str_val(object.get(rule)))
            ));
        }
        return out;
    }
    if truthy(message) {
        return format!(" data-i18n.{key}=\"{}\"", escape(&str_val(message)));
    }
    String::new()
}

pub fn attribute_if(name: &str, value: &Value) -> String {
    if !truthy(value) {
        return String::new();
    }
    format!(" {name}=\"{}\"", out(value))
}

pub fn classes_if(value: &Value) -> String {
    if !truthy(value) {
        return String::new();
    }
    format!(" {}", out(value))
}

pub fn flag_if(suffix: &str, value: &Value) -> String {
    if !truthy(value) {
        return String::new();
    }
    suffix.to_string()
}

pub fn content(params: &Value, html_key: &str, text_key: &str) -> String {
    let html = get(params, &[html_key]);
    if truthy(html) {
        return str_val(html);
    }
    out(get(params, &[text_key]))
}

pub fn content_params(params: &Params, html_key: &str, text_key: &str) -> String {
    let html = params.get(html_key);
    if truthy(html) {
        return str_val(html);
    }
    out(params.get(text_key))
}

pub fn content_indent(params: &Value, html_key: &str, text_key: &str, width: usize) -> String {
    let html = get(params, &[html_key]);
    if truthy(html) {
        return indent(&trim(&str_val(html)), width, false);
    }
    out(get(params, &[text_key]))
}

pub fn content_indent_params(
    params: &Params,
    html_key: &str,
    text_key: &str,
    width: usize,
) -> String {
    let html = params.get(html_key);
    if truthy(html) {
        return indent(&trim(&str_val(html)), width, false);
    }
    out(params.get(text_key))
}
