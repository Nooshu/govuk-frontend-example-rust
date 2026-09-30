//! OWASP / cache headers from baseline/policy.json.

use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct Policy {
    #[serde(rename = "jsEnabledSnippet")]
    pub js_enabled_snippet: String,
    #[serde(rename = "jsEnabledScriptHash")]
    pub js_enabled_script_hash: String,
    pub headers: PolicyHeaders,
    #[serde(rename = "cacheControl")]
    pub cache_control: HashMap<String, String>,
    pub csp: Csp,
    #[serde(rename = "permissionsPolicy")]
    pub permissions_policy: Vec<String>,
    pub remove: Vec<String>,
    pub hsts: Hsts,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PolicyHeaders {
    pub all: HashMap<String, String>,
    pub document: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Csp {
    pub directives: HashMap<String, Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Hsts {
    #[serde(rename = "maxAge")]
    pub max_age: u64,
    #[serde(rename = "includeSubDomains")]
    pub include_sub_domains: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum CacheKind {
    Document,
    SensitiveDocument,
    FingerprintedAsset,
    StaticAsset,
}

impl CacheKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::SensitiveDocument => "sensitive-document",
            Self::FingerprintedAsset => "fingerprinted-asset",
            Self::StaticAsset => "static-asset",
        }
    }
}

impl Policy {
    pub fn load(path: &Path) -> Result<Self, String> {
        let raw = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
        serde_json::from_str(&raw).map_err(|e| e.to_string())
    }

    pub fn build_headers(&self, kind: CacheKind, secure_transport: bool) -> Vec<(String, String)> {
        let mut out = Vec::new();
        for (k, v) in &self.headers.all {
            out.push((k.clone(), v.clone()));
        }
        if matches!(kind, CacheKind::Document | CacheKind::SensitiveDocument) {
            for (k, v) in &self.headers.document {
                out.push((k.clone(), v.clone()));
            }
            out.push(("Content-Security-Policy".into(), self.csp_header()));
            out.push((
                "Permissions-Policy".into(),
                self.permissions_policy
                    .iter()
                    .map(|f| format!("{f}=()"))
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
        }
        if let Some(cc) = self.cache_control.get(kind.as_str()) {
            out.push(("Cache-Control".into(), cc.clone()));
        }
        if secure_transport {
            let mut hsts = format!("max-age={}", self.hsts.max_age);
            if self.hsts.include_sub_domains {
                hsts.push_str("; includeSubDomains");
            }
            out.push(("Strict-Transport-Security".into(), hsts));
        }
        out
    }

    fn csp_header(&self) -> String {
        let mut parts = Vec::new();
        for (name, values) in &self.csp.directives {
            if values.is_empty() {
                parts.push(name.clone());
            } else {
                let mut vals = values.clone();
                if name == "script-src" {
                    vals.push(format!("'{}'", self.js_enabled_script_hash));
                }
                parts.push(format!("{name} {}", vals.join(" ")));
            }
        }
        parts.join("; ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_policy_and_builds_headers() {
        let root = std::env::current_dir().unwrap();
        let path = root.join("baseline/policy.json");
        let policy = Policy::load(&path).expect("policy");
        assert!(!policy.js_enabled_snippet.is_empty());
        let headers = policy.build_headers(CacheKind::Document, true);
        assert!(headers.iter().any(|(k, _)| k == "Content-Security-Policy"));
        assert!(headers.iter().any(|(k, v)| {
            k == "X-Robots-Tag"
                && v.contains("noindex")
                && v.contains("nofollow")
                && v.contains("noarchive")
        }));
        assert!(headers
            .iter()
            .any(|(k, _)| k == "Strict-Transport-Security"));
        let asset = policy.build_headers(CacheKind::StaticAsset, false);
        assert!(asset
            .iter()
            .any(|(k, v)| k == "X-Robots-Tag" && v.contains("noindex")));
        let sensitive = policy.build_headers(CacheKind::SensitiveDocument, false);
        assert!(sensitive
            .iter()
            .any(|(k, v)| k == "Cache-Control" && v.contains("no-store")));
        assert_eq!(
            CacheKind::FingerprintedAsset.as_str(),
            "fingerprinted-asset"
        );
        assert_eq!(CacheKind::StaticAsset.as_str(), "static-asset");
    }
}
