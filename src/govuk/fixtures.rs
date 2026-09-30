//! Fixture loading helpers used by the parity suite.

use crate::govuk::params::{parse_json, Params, Value};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

/// One entry from a component's fixtures.json.
#[derive(Debug, Clone)]
pub struct Fixture {
    pub name: String,
    pub options: Params,
    pub hidden: bool,
    pub description: String,
    pub html: String,
}

/// A component's whole fixtures.json file.
#[derive(Debug, Clone)]
pub struct FixtureSet {
    pub component: String,
    pub fixtures: Vec<Fixture>,
}

#[derive(Deserialize)]
struct RawFixtureFile {
    component: String,
    fixtures: Vec<RawFixture>,
}

#[derive(Deserialize)]
struct RawFixture {
    name: String,
    #[serde(default)]
    hidden: bool,
    #[serde(default)]
    description: String,
    html: String,
    /// Kept as raw JSON so we can re-parse with order/number spelling preserved.
    options: serde_json::Value,
}

/// Component names under `components_dir` that ship a fixtures.json, sorted.
pub fn fixture_components(components_dir: &Path) -> Result<Vec<String>, String> {
    let mut names = Vec::new();
    let entries = fs::read_dir(components_dir)
        .map_err(|e| format!("reading {}: {e}", components_dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| e.to_string())?;
        if !entry.file_type().map(|t| t.is_dir()).unwrap_or(false) {
            continue;
        }
        let fixtures = entry.path().join("fixtures.json");
        if fixtures.is_file() {
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    names.sort();
    Ok(names)
}

/// Read one component's fixtures.json from the installed GOV.UK Frontend package.
pub fn load_fixtures(components_dir: &Path, component: &str) -> Result<FixtureSet, String> {
    let path = components_dir.join(component).join("fixtures.json");
    let raw = fs::read(&path).map_err(|e| format!("reading {}: {e}", path.display()))?;
    let file: RawFixtureFile =
        serde_json::from_slice(&raw).map_err(|e| format!("parsing {}: {e}", path.display()))?;

    let mut fixtures = Vec::with_capacity(file.fixtures.len());
    for f in file.fixtures {
        let options_json = serde_json::to_vec(&f.options)
            .map_err(|e| format!("re-serializing options for {}: {e}", f.name))?;
        let options = match parse_json(&options_json)? {
            Value::Object(p) => p,
            Value::Null | Value::Undefined => Params::new(),
            other => {
                return Err(format!(
                    "fixture {} options must be an object, got {other:?}",
                    f.name
                ))
            }
        };
        fixtures.push(Fixture {
            name: f.name,
            options,
            hidden: f.hidden,
            description: f.description,
            html: f.html,
        });
    }

    Ok(FixtureSet {
        component: file.component,
        fixtures,
    })
}

/// Resolve `node_modules/govuk-frontend/dist/govuk/components` from cwd or ancestors.
pub fn components_root() -> Result<PathBuf, String> {
    let mut dir = std::env::current_dir().map_err(|e| e.to_string())?;
    loop {
        let candidate = dir
            .join("node_modules")
            .join("govuk-frontend")
            .join("dist")
            .join("govuk")
            .join("components");
        if candidate.is_dir() {
            return Ok(candidate);
        }
        let package = dir.join("package.json");
        let policy = dir.join("baseline").join("policy.json");
        if package.is_file() && policy.is_file() {
            return Err(
                "GOV.UK Frontend is not installed; run `npm install` at the repository root".into(),
            );
        }
        if !dir.pop() {
            return Err("repository root not found".into());
        }
    }
}
