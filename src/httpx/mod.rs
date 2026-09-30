//! Static assets with fingerprinting and compression.

use crate::baseline::{CacheKind, Policy};
use axum::body::Body;
use axum::http::{header, HeaderMap, HeaderValue, Request, Response, StatusCode};
use brotli::enc::BrotliEncoderParams;
use flate2::write::GzEncoder;
use flate2::Compression;
use sha2::{Digest, Sha256};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Clone)]
pub struct Assets {
    pub stylesheet_path: PathBuf,
    pub stylesheet_href: String,
    pub stylesheet_bytes: Arc<Vec<u8>>,
    pub frontend_assets: PathBuf,
    pub app_module_bytes: Arc<Vec<u8>>,
    pub app_module_href: String,
    pub policy: Arc<Policy>,
    pub secure_transport: bool,
}

impl Assets {
    pub fn load(
        stylesheet: &Path,
        frontend_assets: &Path,
        policy: Arc<Policy>,
        secure_transport: bool,
    ) -> Result<Self, String> {
        let css = std::fs::read(stylesheet).map_err(|e| {
            format!(
                "stylesheet missing at {}: {e} (run npm run build:styles)",
                stylesheet.display()
            )
        })?;
        let css_hash = short_hash(&css);
        let module = br#"import { initAll } from '/assets/govuk-frontend.min.js'; initAll();"#;
        let mod_hash = short_hash(module);
        Ok(Self {
            stylesheet_path: stylesheet.to_path_buf(),
            stylesheet_href: format!("/assets/application.{css_hash}.css"),
            stylesheet_bytes: Arc::new(css),
            frontend_assets: frontend_assets.to_path_buf(),
            app_module_bytes: Arc::new(module.to_vec()),
            app_module_href: format!("/assets/app.{mod_hash}.mjs"),
            policy,
            secure_transport,
        })
    }
}

fn short_hash(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    hex::encode(&digest[..8])
}

pub async fn serve_assets(
    axum::extract::State(state): axum::extract::State<crate::web::AppState>,
    axum::extract::Path(path): axum::extract::Path<String>,
    req: Request<Body>,
) -> Response<Body> {
    let assets = &state.assets;
    let (bytes, content_type, kind) = if path.starts_with("application.") && path.ends_with(".css")
    {
        (
            assets.stylesheet_bytes.as_ref().clone(),
            "text/css; charset=utf-8".to_string(),
            CacheKind::FingerprintedAsset,
        )
    } else if path.starts_with("app.") && path.ends_with(".mjs") {
        (
            assets.app_module_bytes.as_ref().clone(),
            "text/javascript; charset=utf-8".to_string(),
            CacheKind::FingerprintedAsset,
        )
    } else if path == "govuk-frontend.min.js"
        || (path.ends_with(".js") && path.contains("govuk-frontend"))
    {
        let file = assets
            .frontend_assets
            .parent()
            .unwrap()
            .join("govuk-frontend.min.js");
        match std::fs::read(&file) {
            Ok(b) => (
                b,
                "text/javascript; charset=utf-8".to_string(),
                CacheKind::StaticAsset,
            ),
            Err(_) => return not_found(),
        }
    } else {
        // fonts / images under assets/
        let file = assets.frontend_assets.join(&path);
        if !file.starts_with(&assets.frontend_assets) || !file.is_file() {
            return not_found();
        }
        match std::fs::read(&file) {
            Ok(b) => {
                let ct = mime_guess::from_path(&file)
                    .first_or_octet_stream()
                    .to_string();
                (b, ct, CacheKind::StaticAsset)
            }
            Err(_) => return not_found(),
        }
    };

    respond_bytes(assets, &req, bytes, &content_type, kind)
}

fn respond_bytes(
    assets: &Assets,
    req: &Request<Body>,
    bytes: Vec<u8>,
    content_type: &str,
    kind: CacheKind,
) -> Response<Body> {
    let accept = req
        .headers()
        .get(header::ACCEPT_ENCODING)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");
    let (body, encoding) = compress(bytes, accept);
    let mut res = Response::new(Body::from(body));
    *res.status_mut() = StatusCode::OK;
    let headers = res.headers_mut();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_str(content_type).unwrap(),
    );
    if let Some(enc) = encoding {
        headers.insert(header::CONTENT_ENCODING, HeaderValue::from_static(enc));
    }
    headers.insert(header::VARY, HeaderValue::from_static("Accept-Encoding"));
    apply_policy_headers(headers, &assets.policy, kind, assets.secure_transport);
    res
}

fn compress(bytes: Vec<u8>, accept: &str) -> (Vec<u8>, Option<&'static str>) {
    if accept.split(',').any(|e| e.trim().starts_with("br")) {
        let mut out = Vec::new();
        let params = BrotliEncoderParams::default();
        let mut cursor = std::io::Cursor::new(bytes.as_slice());
        if brotli::BrotliCompress(&mut cursor, &mut out, &params).is_ok() {
            return (out, Some("br"));
        }
    }
    if accept
        .split(',')
        .any(|e| e.trim().starts_with("gzip") || e.trim() == "gzip")
    {
        let mut enc = GzEncoder::new(Vec::new(), Compression::default());
        if enc.write_all(&bytes).is_ok() {
            if let Ok(out) = enc.finish() {
                return (out, Some("gzip"));
            }
        }
    }
    (bytes, None)
}

pub fn apply_policy_headers(
    headers: &mut HeaderMap,
    policy: &Policy,
    kind: CacheKind,
    secure_transport: bool,
) {
    for (k, v) in policy.build_headers(kind, secure_transport) {
        if let (Ok(name), Ok(value)) = (
            header::HeaderName::try_from(k.as_str()),
            HeaderValue::from_str(&v),
        ) {
            headers.insert(name, value);
        }
    }
    for name in &policy.remove {
        if let Ok(h) = header::HeaderName::try_from(name.as_str()) {
            headers.remove(h);
        }
    }
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from("not found"))
        .unwrap()
}
