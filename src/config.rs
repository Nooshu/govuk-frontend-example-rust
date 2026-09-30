//! Repository paths, Frontend pin, PORT/HOST, demos flag.

use serde::Deserialize;
use std::env;
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

pub const SERVICE_NAME: &str = "Apply for a rod fishing licence";
pub const SERVICE_NAME_CY: &str = "Gwneud cais am drwydded bysgota";
pub const DEFAULT_PORT: u16 = 3000;

#[derive(Debug, Clone)]
pub struct Config {
    pub root: PathBuf,
    pub components_root: PathBuf,
    pub frontend_assets: PathBuf,
    pub stylesheet: PathBuf,
    pub policy_file: PathBuf,
    pub frontend_version: String,
    pub demos_enabled: bool,
    pub secure_transport: bool,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        let root = find_root(&env::current_dir().map_err(|e| e.to_string())?)?;
        Self::from_root(root)
    }

    pub fn from_root(root: PathBuf) -> Result<Self, String> {
        let pkg = root.join("node_modules/govuk-frontend/package.json");
        let version = frontend_version(&pkg)?;
        let govuk = root.join("node_modules/govuk-frontend/dist/govuk");
        Ok(Self {
            components_root: govuk.join("components"),
            frontend_assets: govuk.join("assets"),
            stylesheet: root.join("dist/stylesheets/application.css"),
            policy_file: root.join("baseline/policy.json"),
            frontend_version: version,
            demos_enabled: demos_enabled(),
            secure_transport: env::var("SECURE_TRANSPORT").ok().as_deref() == Some("true")
                || env::var("NODE_ENV").ok().as_deref() == Some("production"),
            root,
        })
    }
}

pub fn demos_enabled() -> bool {
    match env::var("DEMOS_ENABLED").ok().as_deref() {
        Some("true" | "1" | "yes") => true,
        Some("false" | "0" | "no") => false,
        _ => env::var("NODE_ENV").ok().as_deref() != Some("production"),
    }
}

pub fn listen_addr() -> Result<SocketAddr, String> {
    let port = match env::var("PORT") {
        Ok(p) if !p.is_empty() => p.parse::<u16>().map_err(|_| format!("invalid PORT: {p}"))?,
        _ => DEFAULT_PORT,
    };
    let host = env::var("HOST").unwrap_or_default();
    let host = if host.is_empty() { "0.0.0.0" } else { &host };
    format!("{host}:{port}")
        .parse()
        .map_err(|e| format!("listen addr: {e}"))
}

fn find_root(start: &Path) -> Result<PathBuf, String> {
    let mut dir = start.to_path_buf();
    loop {
        if dir.join("package.json").is_file() && dir.join("baseline/policy.json").is_file() {
            return Ok(dir);
        }
        if !dir.pop() {
            return Err("repository root not found".into());
        }
    }
}

fn frontend_version(package_json: &Path) -> Result<String, String> {
    let raw = std::fs::read_to_string(package_json)
        .map_err(|_| "govuk-frontend is not installed; run `npm install`".to_string())?;
    #[derive(Deserialize)]
    struct Meta {
        version: String,
    }
    let meta: Meta = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    if meta.version.is_empty() {
        return Err("govuk-frontend package.json has no version".into());
    }
    Ok(meta.version)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_root_and_demos() {
        let root = std::env::current_dir().unwrap();
        let cfg = Config::from_root(root).expect("config");
        assert!(!cfg.frontend_version.is_empty());
        let _ = demos_enabled();
        let addr = listen_addr().expect("addr");
        assert_eq!(addr.port(), DEFAULT_PORT);
    }
}
