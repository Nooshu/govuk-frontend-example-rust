//! Nunjucks-parity filters: escape, indent, length, and related helpers.

use crate::govuk::params::{Params, Safe, Value};

/// Escape a string the way Nunjucks' escape filter does (backslash included).
pub fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#39;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '\\' => out.push_str("&#92;"),
            _ => out.push(ch),
        }
    }
    out
}

/// Walk a chain of option names (Nunjucks member lookup).
pub fn get<'a>(value: &'a Value, names: &[&str]) -> &'a Value {
    let mut current = value;
    for name in names {
        match current {
            Value::Object(p) => current = p.get(name),
            _ => return &Value::Undefined,
        }
    }
    current
}

#[allow(dead_code)]
pub fn get_params<'a>(params: &'a Params, names: &[&str]) -> &'a Value {
    if names.is_empty() {
        return &Value::Undefined;
    }
    let mut current = params.get(names[0]);
    for name in &names[1..] {
        match current {
            Value::Object(p) => current = p.get(name),
            _ => return &Value::Undefined,
        }
    }
    current
}

pub fn items(value: &Value) -> &[Value] {
    match value {
        Value::Array(a) => a,
        _ => &[],
    }
}

#[allow(dead_code)]
pub fn at(value: &Value, index: usize) -> &Value {
    items(value).get(index).unwrap_or(&Value::Undefined)
}

/// JavaScript truthiness (empty array/object are truthy).
pub fn truthy(value: &Value) -> bool {
    match value {
        Value::Null | Value::Undefined => false,
        Value::Bool(b) => *b,
        Value::String(s) => !s.is_empty(),
        Value::Number(n) => n.parse::<f64>().map(|f| f != 0.0).unwrap_or(false),
        Value::Safe(Safe(s)) => !s.is_empty(),
        Value::Array(_) | Value::Object(_) => true,
    }
}

pub fn is_undefined(value: &Value) -> bool {
    matches!(value, Value::Undefined)
}

/// Convert a value to the string JavaScript would produce, before escaping.
pub fn str_val(value: &Value) -> String {
    match value {
        Value::Null | Value::Undefined => String::new(),
        Value::String(s) => s.clone(),
        Value::Safe(Safe(s)) => s.clone(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => number_str(n),
        Value::Array(a) => a.iter().map(str_val).collect::<Vec<_>>().join(","),
        Value::Object(_) => "[object Object]".to_string(),
    }
}

fn number_str(raw: &str) -> String {
    if let Ok(i) = raw.parse::<i64>() {
        return i.to_string();
    }
    if let Ok(f) = raw.parse::<f64>() {
        // Format like JavaScript (no trailing zeros from FormatFloat 'f' -1)
        let mut s = format!("{f}");
        if s.contains('e') || s.contains('E') {
            // use non-scientific
            s = format!("{f:.16}")
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string();
            if s.is_empty() || s == "-" {
                s = "0".to_string();
            }
        }
        return s;
    }
    raw.to_string()
}

/// Autoescaping template output: Safe passes through, everything else is escaped.
pub fn out(value: &Value) -> String {
    match value {
        Value::Safe(Safe(s)) => s.clone(),
        _ => escape(&str_val(value)),
    }
}

pub fn trim(text: &str) -> String {
    text.trim().to_string()
}

/// Nunjucks indent filter.
pub fn indent(text: &str, width: usize, first: bool) -> String {
    if text.is_empty() {
        return String::new();
    }
    let padding = " ".repeat(width);
    let lines: Vec<&str> = text.split('\n').collect();
    lines
        .iter()
        .enumerate()
        .map(|(i, line)| {
            if i == 0 && !first {
                (*line).to_string()
            } else {
                format!("{padding}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// `default(fallback)` — only when undefined.
pub fn def<'a>(value: &'a Value, fallback: &'a Value) -> &'a Value {
    if is_undefined(value) {
        fallback
    } else {
        value
    }
}

/// `default(fallback, true)` — also replaces falsy.
pub fn def_truthy<'a>(value: &'a Value, fallback: &'a Value) -> &'a Value {
    if truthy(value) {
        value
    } else {
        fallback
    }
}

pub fn length(value: &Value) -> usize {
    match value {
        Value::Null | Value::Undefined | Value::Bool(_) => 0,
        Value::Array(a) => a.len(),
        Value::Object(p) => p.len(),
        Value::String(s) => s.len(),
        Value::Safe(Safe(s)) => s.len(),
        Value::Number(_) => 0,
    }
}

pub fn loose_eq(left: &Value, right: &Value) -> bool {
    let left_nil = matches!(left, Value::Null | Value::Undefined);
    let right_nil = matches!(right, Value::Null | Value::Undefined);
    if left_nil || right_nil {
        return left_nil && right_nil;
    }
    match (left, right) {
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Bool(a), _) => loose_eq(&bool_to_number(*a), right),
        (_, Value::Bool(b)) => loose_eq(left, &bool_to_number(*b)),
        (Value::Number(a), Value::Number(b)) => numeric(a) == numeric(b),
        (Value::Number(a), _) => same_number(a, &str_val(right)),
        (_, Value::Number(b)) => same_number(b, &str_val(left)),
        _ => str_val(left) == str_val(right),
    }
}

pub fn strict_eq(left: &Value, right: &Value) -> bool {
    if is_undefined(left) || is_undefined(right) {
        return is_undefined(left) && is_undefined(right);
    }
    if matches!(left, Value::Null) || matches!(right, Value::Null) {
        return matches!(left, Value::Null) && matches!(right, Value::Null);
    }
    match (left, right) {
        (Value::Number(a), Value::Number(b)) => numeric(a) == numeric(b),
        (Value::Number(_), _) | (_, Value::Number(_)) => false,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        _ => false,
    }
}

pub fn contains(needle: &Value, haystack: &Value) -> bool {
    match haystack {
        Value::String(s) => s.contains(&str_val(needle)),
        Value::Safe(Safe(s)) => s.contains(&str_val(needle)),
        Value::Array(a) => a.iter().any(|item| strict_eq(needle, item)),
        Value::Object(p) => p.has(&str_val(needle)),
        _ => false,
    }
}

fn bool_to_number(value: bool) -> Value {
    Value::Number(if value { "1" } else { "0" }.to_string())
}

fn numeric(text: &str) -> f64 {
    text.parse().unwrap_or(0.0)
}

fn same_number(left: &str, right: &str) -> bool {
    let trimmed = right.trim();
    let trimmed = if trimmed.is_empty() { "0" } else { trimmed };
    match trimmed.parse::<f64>() {
        Ok(v) => numeric(left) == v,
        Err(_) => false,
    }
}

pub fn concat_if(prefix: &str, value: &Value) -> String {
    if !truthy(value) {
        return String::new();
    }
    format!("{prefix}{}", str_val(value))
}

/// Heading level option, falling back when unset.
pub fn heading(level: &Value, fallback: &str) -> String {
    if truthy(level) {
        str_val(level)
    } else {
        fallback.to_string()
    }
}
