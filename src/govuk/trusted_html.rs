//! Trusted HTML that must be emitted without escaping (Nunjucks SafeString).

use std::fmt;

/// HTML that is already trusted and must not be escaped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrustedHtml(pub String);

impl TrustedHtml {
    pub fn new(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for TrustedHtml {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

impl From<String> for TrustedHtml {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl fmt::Display for TrustedHtml {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trusted_html_round_trips() {
        let t = TrustedHtml::new("<em>ok</em>");
        assert_eq!(t.as_str(), "<em>ok</em>");
        assert_eq!(TrustedHtml::from("x").to_string(), "x");
        assert_eq!(TrustedHtml::from(String::from("y")).as_str(), "y");
    }
}
