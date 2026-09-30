//! Ordered Params and JSON decoding for macro options.
//!
//! Order is preserved because GOV.UK Frontend renders the `attributes` option by iterating
//! keys, so a HashMap would break fixture parity. Numbers keep their original spelling.

use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;

/// Sentinel for an option that was never supplied (JavaScript `undefined`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Undefined;

/// String that is already HTML and must be emitted without escaping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Safe(pub String);

impl From<&str> for Safe {
    fn from(s: &str) -> Self {
        Safe(s.to_string())
    }
}

impl From<String> for Safe {
    fn from(s: String) -> Self {
        Safe(s)
    }
}

/// JSON / macro option value kinds used by the renderers.
#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Bool(bool),
    /// Preserves original spelling (e.g. `1.50`) for JavaScript number stringification.
    Number(String),
    String(String),
    Array(Vec<Value>),
    Object(Params),
    Safe(Safe),
    Undefined,
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Null, Value::Null) | (Value::Undefined, Value::Undefined) => true,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Safe(a), Value::Safe(b)) => a.0 == b.0,
            (Value::Array(a), Value::Array(b)) => a == b,
            (Value::Object(a), Value::Object(b)) => {
                a.keys() == b.keys() && a.keys().iter().all(|k| a.get(k) == b.get(k))
            }
            _ => false,
        }
    }
}

/// Ordered set of component options (Nunjucks macro params).
#[derive(Clone, Default)]
pub struct Params {
    keys: Vec<String>,
    values: HashMap<String, Value>,
}

impl Params {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_pairs(pairs: &[(&str, Value)]) -> Self {
        let mut p = Self::new();
        for (k, v) in pairs {
            p.set(*k, v.clone());
        }
        p
    }

    pub fn set(&mut self, key: impl Into<String>, value: Value) {
        let key = key.into();
        if !self.values.contains_key(&key) {
            self.keys.push(key.clone());
        }
        self.values.insert(key, value);
    }

    pub fn get(&self, key: &str) -> &Value {
        static UNDEFINED: Value = Value::Undefined;
        self.values.get(key).unwrap_or(&UNDEFINED)
    }

    pub fn has(&self, key: &str) -> bool {
        self.values.contains_key(key)
    }

    pub fn keys(&self) -> &[String] {
        &self.keys
    }

    pub fn len(&self) -> usize {
        self.keys.len()
    }

    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }
}

impl<'de> Deserialize<'de> for Params {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::Object(p) => Ok(p),
            _ => Err(serde::de::Error::custom("expected a JSON object")),
        }
    }
}

impl<'de> Deserialize<'de> for Value {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        // Deserialize via serde_json::Value with arbitrary_precision for number strings,
        // then convert — but fixture loading uses parse_json below for spelling.
        let v = serde_json::Value::deserialize(deserializer)?;
        Ok(from_serde_json(&v))
    }
}

fn from_serde_json(v: &serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Null,
        serde_json::Value::Bool(b) => Value::Bool(*b),
        serde_json::Value::Number(n) => Value::Number(n.to_string()),
        serde_json::Value::String(s) => Value::String(s.clone()),
        serde_json::Value::Array(a) => Value::Array(a.iter().map(from_serde_json).collect()),
        serde_json::Value::Object(map) => {
            // serde_json::Map preserves insertion order (indexmap).
            let mut p = Params::new();
            for (k, val) in map {
                p.set(k.clone(), from_serde_json(val));
            }
            Value::Object(p)
        }
    }
}

/// Parse JSON preserving key order and number spelling.
pub fn parse_json(data: &[u8]) -> Result<Value, String> {
    let text = std::str::from_utf8(data).map_err(|e| e.to_string())?;
    let mut parser = Parser::new(text);
    let value = parser.parse_value()?;
    parser.skip_ws();
    if parser.pos < parser.bytes.len() {
        return Err("unexpected data after JSON value".into());
    }
    Ok(value)
}

/// Parse a JSON object into Params.
#[allow(dead_code)]
pub fn parse_params(data: &[u8]) -> Result<Params, String> {
    match parse_json(data)? {
        Value::Object(p) => Ok(p),
        other => Err(format!("expected a JSON object, got {other:?}")),
    }
}

struct Parser<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            bytes: text.as_bytes(),
            pos: 0,
        }
    }

    fn skip_ws(&mut self) {
        while self.pos < self.bytes.len() && self.bytes[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn bump(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.pos += 1;
        Some(b)
    }

    fn parse_value(&mut self) -> Result<Value, String> {
        self.skip_ws();
        match self.peek() {
            Some(b'{') => self.parse_object(),
            Some(b'[') => self.parse_array(),
            Some(b'"') => Ok(Value::String(self.parse_string()?)),
            Some(b't') => self.parse_literal(b"true", Value::Bool(true)),
            Some(b'f') => self.parse_literal(b"false", Value::Bool(false)),
            Some(b'n') => self.parse_literal(b"null", Value::Null),
            Some(b'-') | Some(b'0'..=b'9') => Ok(Value::Number(self.parse_number()?)),
            other => Err(format!(
                "unexpected token at {}: {:?}",
                self.pos,
                other.map(|c| c as char)
            )),
        }
    }

    fn parse_literal(&mut self, lit: &[u8], value: Value) -> Result<Value, String> {
        for &b in lit {
            if self.bump() != Some(b) {
                return Err(format!("invalid literal at {}", self.pos));
            }
        }
        Ok(value)
    }

    fn parse_number(&mut self) -> Result<String, String> {
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.pos += 1;
        }
        if self.peek() == Some(b'0') {
            self.pos += 1;
        } else if matches!(self.peek(), Some(b'1'..=b'9')) {
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        } else {
            return Err(format!("invalid number at {start}"));
        }
        if self.peek() == Some(b'.') {
            self.pos += 1;
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(format!("invalid fraction at {}", self.pos));
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.pos += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.pos += 1;
            }
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(format!("invalid exponent at {}", self.pos));
            }
            while matches!(self.peek(), Some(b'0'..=b'9')) {
                self.pos += 1;
            }
        }
        Ok(std::str::from_utf8(&self.bytes[start..self.pos])
            .unwrap()
            .to_string())
    }

    fn parse_string(&mut self) -> Result<String, String> {
        if self.bump() != Some(b'"') {
            return Err("expected string".into());
        }
        let mut out = String::new();
        loop {
            match self.peek() {
                None => return Err("unterminated string".into()),
                Some(b'"') => {
                    self.pos += 1;
                    return Ok(out);
                }
                Some(b'\\') => {
                    self.pos += 1;
                    match self.bump() {
                        Some(b'"') => out.push('"'),
                        Some(b'\\') => out.push('\\'),
                        Some(b'/') => out.push('/'),
                        Some(b'b') => out.push('\u{0008}'),
                        Some(b'f') => out.push('\u{000c}'),
                        Some(b'n') => out.push('\n'),
                        Some(b'r') => out.push('\r'),
                        Some(b't') => out.push('\t'),
                        Some(b'u') => {
                            let mut hex = String::new();
                            for _ in 0..4 {
                                let b = self.bump().ok_or("bad unicode escape")?;
                                hex.push(b as char);
                            }
                            let cp =
                                u32::from_str_radix(&hex, 16).map_err(|_| "bad unicode escape")?;
                            out.push(char::from_u32(cp).ok_or("bad unicode codepoint")?);
                        }
                        _ => return Err("bad escape".into()),
                    }
                }
                Some(b) if b < 0x80 => {
                    self.pos += 1;
                    out.push(b as char);
                }
                Some(_) => {
                    let rest = std::str::from_utf8(&self.bytes[self.pos..])
                        .map_err(|e| format!("invalid UTF-8 in string: {e}"))?;
                    let ch = rest.chars().next().ok_or("empty UTF-8")?;
                    self.pos += ch.len_utf8();
                    out.push(ch);
                }
            }
        }
    }

    fn parse_array(&mut self) -> Result<Value, String> {
        self.bump(); // '['
        let mut items = Vec::new();
        self.skip_ws();
        if self.peek() == Some(b']') {
            self.bump();
            return Ok(Value::Array(items));
        }
        loop {
            items.push(self.parse_value()?);
            self.skip_ws();
            match self.bump() {
                Some(b']') => return Ok(Value::Array(items)),
                Some(b',') => {
                    self.skip_ws();
                    continue;
                }
                _ => return Err("expected ',' or ']' in array".into()),
            }
        }
    }

    fn parse_object(&mut self) -> Result<Value, String> {
        self.bump(); // '{'
        let mut params = Params::new();
        self.skip_ws();
        if self.peek() == Some(b'}') {
            self.bump();
            return Ok(Value::Object(params));
        }
        loop {
            self.skip_ws();
            let key = self.parse_string()?;
            self.skip_ws();
            if self.bump() != Some(b':') {
                return Err("expected ':' in object".into());
            }
            let value = self.parse_value()?;
            params.set(key, value);
            self.skip_ws();
            match self.bump() {
                Some(b'}') => return Ok(Value::Object(params)),
                Some(b',') => continue,
                _ => return Err("expected ',' or '}' in object".into()),
            }
        }
    }
}

impl fmt::Debug for Params {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map()
            .entries(self.keys.iter().map(|k| (k, self.get(k))))
            .finish()
    }
}

/// Build Params from key/value pairs (owned Values).
pub fn params(pairs: &[(&str, Value)]) -> Params {
    Params::from_pairs(pairs)
}

/// Convenience: string option value.
pub fn v_str(s: impl Into<String>) -> Value {
    Value::String(s.into())
}

/// Convenience: safe HTML value.
pub fn v_safe(s: impl Into<String>) -> Value {
    Value::Safe(Safe(s.into()))
}

/// Convenience: object value.
pub fn v_obj(p: Params) -> Value {
    Value::Object(p)
}

/// Convenience: bool value.
pub fn v_bool(b: bool) -> Value {
    Value::Bool(b)
}

/// Alias for missing option sentinel (tests / docs).
pub const UNDEFINED: Value = Value::Undefined;

/// JSON number as string (spelling preserved). Alias keeps older imports compiling.
pub type Number = String;
