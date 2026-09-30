//! Component renderer registry.

use crate::govuk::button::{render_button, render_exit_this_page};
use crate::govuk::chrome::{
    render_breadcrumbs, render_cookie_banner, render_footer, render_generic_header, render_header,
    render_language_navigation, render_pagination, render_service_navigation,
};
use crate::govuk::forms::{
    render_character_count, render_checkboxes, render_date_input, render_file_upload, render_input,
    render_password_input, render_radios, render_select, render_textarea,
};
use crate::govuk::lists::{
    render_accordion, render_error_summary, render_notification_banner, render_summary_list,
    render_table, render_tabs, render_task_list,
};
use crate::govuk::params::Params;
use crate::govuk::text::{
    render_back_link, render_details, render_error_message, render_feedback, render_fieldset,
    render_hint, render_inset_text, render_label, render_panel, render_phase_banner,
    render_skip_link, render_tag, render_warning_text,
};
use std::collections::BTreeMap;
use thiserror::Error;

type Renderer = fn(&Params) -> String;

fn registry() -> BTreeMap<&'static str, Renderer> {
    BTreeMap::from([
        ("accordion", render_accordion as Renderer),
        ("back-link", render_back_link as Renderer),
        ("breadcrumbs", render_breadcrumbs as Renderer),
        ("button", render_button as Renderer),
        ("character-count", render_character_count as Renderer),
        ("checkboxes", render_checkboxes as Renderer),
        ("cookie-banner", render_cookie_banner as Renderer),
        ("date-input", render_date_input as Renderer),
        ("details", render_details as Renderer),
        ("error-message", render_error_message as Renderer),
        ("error-summary", render_error_summary as Renderer),
        ("exit-this-page", render_exit_this_page as Renderer),
        ("feedback", render_feedback as Renderer),
        ("fieldset", render_fieldset as Renderer),
        ("file-upload", render_file_upload as Renderer),
        ("footer", render_footer as Renderer),
        ("generic-header", render_generic_header as Renderer),
        ("header", render_header as Renderer),
        ("hint", render_hint as Renderer),
        ("input", render_input as Renderer),
        ("inset-text", render_inset_text as Renderer),
        ("label", render_label as Renderer),
        (
            "language-navigation",
            render_language_navigation as Renderer,
        ),
        (
            "notification-banner",
            render_notification_banner as Renderer,
        ),
        ("pagination", render_pagination as Renderer),
        ("panel", render_panel as Renderer),
        ("password-input", render_password_input as Renderer),
        ("phase-banner", render_phase_banner as Renderer),
        ("radios", render_radios as Renderer),
        ("select", render_select as Renderer),
        ("service-navigation", render_service_navigation as Renderer),
        ("skip-link", render_skip_link as Renderer),
        ("summary-list", render_summary_list as Renderer),
        ("table", render_table as Renderer),
        ("tabs", render_tabs as Renderer),
        ("tag", render_tag as Renderer),
        ("task-list", render_task_list as Renderer),
        ("textarea", render_textarea as Renderer),
        ("warning-text", render_warning_text as Renderer),
    ])
}

#[derive(Debug, Error)]
#[error("govuk: {0:?} is not a GOV.UK Frontend component")]
pub struct RenderError(pub String);

/// Render one GOV.UK Frontend component. Output is trimmed like official fixtures.
pub fn render(component: &str, params: &Params) -> Result<String, RenderError> {
    let renderers = registry();
    let f = renderers
        .get(component)
        .ok_or_else(|| RenderError(component.to_string()))?;
    Ok(f(params).trim().to_string())
}

pub fn must_render(component: &str, params: &Params) -> String {
    render(component, params).unwrap_or_else(|e| panic!("{e}"))
}

pub fn components() -> Vec<&'static str> {
    registry().into_keys().collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::govuk::fixtures::{components_root, fixture_components, load_fixtures};

    fn difference(want: &str, got: &str) -> String {
        let want_lines: Vec<&str> = want.split('\n').collect();
        let got_lines: Vec<&str> = got.split('\n').collect();
        let max = want_lines.len().max(got_lines.len());
        for i in 0..max {
            let w = want_lines.get(i).copied().unwrap_or("<missing line>");
            let g = got_lines.get(i).copied().unwrap_or("<missing line>");
            if w != g {
                return format!(
                    "first difference on line {}\nwant: {w:?}\ngot:  {g:?}",
                    i + 1
                );
            }
        }
        format!("line-by-line equal but strings differ\nwant: {want:?}\ngot:  {got:?}")
    }

    #[test]
    fn render_matches_fixtures() {
        let root = components_root().expect("components root");
        let components = fixture_components(&root).expect("list components");
        assert!(!components.is_empty(), "no components with fixtures found");

        let mut total = 0;
        let mut failures = Vec::new();
        for component in &components {
            let set = load_fixtures(&root, component).expect("load fixtures");
            total += set.fixtures.len();
            for fixture in &set.fixtures {
                match render(component, &fixture.options) {
                    Ok(got) if got == fixture.html => {}
                    Ok(got) => failures.push(format!(
                        "{component} / {}: {}",
                        fixture.name,
                        difference(&fixture.html, &got)
                    )),
                    Err(e) => failures.push(format!("{component} / {}: {e}", fixture.name)),
                }
            }
        }
        if !failures.is_empty() {
            panic!(
                "{} fixture(s) failed of {total}:\n{}",
                failures.len(),
                failures.join("\n\n")
            );
        }
        eprintln!(
            "checked {total} fixtures across {} components",
            components.len()
        );
    }

    #[test]
    fn components_cover_every_fixture_component() {
        let root = components_root().expect("components root");
        let shipped = fixture_components(&root).expect("list");
        let supported: std::collections::HashSet<_> = components().into_iter().collect();
        for name in shipped {
            assert!(
                supported.contains(name.as_str()),
                "component {name:?} ships fixtures but has no Rust renderer"
            );
        }
    }

    #[test]
    fn render_rejects_unknown_component() {
        assert!(render("not-a-component", &Params::new()).is_err());
    }
}
