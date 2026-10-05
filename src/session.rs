//! In-memory session store for the example service.

use crate::service::Application;
use getrandom::fill;
use hex::encode as hex_encode;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Default)]
pub struct SessionData {
    pub csrf: String,
    pub application: Application,
    pub cookie_choice: Option<String>,
    pub flash_errors: Vec<(String, String)>,
    pub flash_notice: Option<String>,
}

#[derive(Clone, Default)]
pub struct SessionStore {
    inner: Arc<Mutex<HashMap<String, SessionData>>>,
}

impl SessionStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_or_create(&self, id: Option<&str>) -> (String, SessionData) {
        let mut map = self.inner.lock().unwrap();
        if let Some(id) = id {
            if let Some(data) = map.get(id) {
                return (id.to_string(), data.clone());
            }
        }
        let id = random_hex(16);
        let data = SessionData {
            csrf: random_hex(16),
            application: Application::default(),
            cookie_choice: None,
            flash_errors: Vec::new(),
            flash_notice: None,
        };
        map.insert(id.clone(), data.clone());
        (id, data)
    }

    pub fn save(&self, id: &str, data: SessionData) {
        self.inner.lock().unwrap().insert(id.to_string(), data);
    }

    pub fn reset_application(&self, id: &str) {
        let mut map = self.inner.lock().unwrap();
        if let Some(data) = map.get_mut(id) {
            data.application = Application::default();
            data.flash_errors.clear();
            data.flash_notice = None;
        }
    }
}

pub fn reference_for(session_id: &str) -> String {
    let hex = if session_id.len() >= 8 {
        &session_id[..8]
    } else {
        session_id
    };
    let number = u64::from_str_radix(hex, 16).unwrap_or(0) % 100_000_000;
    format!("FR{number:08}")
}

fn random_hex(size: usize) -> String {
    let mut bytes = vec![0u8; size];
    let _ = fill(&mut bytes);
    hex_encode(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_and_loads() {
        let store = SessionStore::new();
        let (id, data) = store.get_or_create(None);
        assert!(!data.csrf.is_empty());
        let (id2, _) = store.get_or_create(Some(&id));
        assert_eq!(id, id2);
        assert!(reference_for(&id).starts_with("FR"));
        assert_eq!(reference_for(&id).len(), 10);
    }
}
