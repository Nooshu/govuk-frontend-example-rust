//! Native GOV.UK Frontend component HTML renderers with fixture parity.

mod attributes;
mod button;
mod chrome;
mod escape;
mod fixtures;
mod forms;
mod lists;
mod params;
mod render;
mod text;
mod trusted_html;

pub use escape::escape;
pub use fixtures::{components_root, fixture_components, load_fixtures, Fixture, FixtureSet};
pub use params::{
    params, parse_json, v_bool, v_obj, v_safe, v_str, Number, Params, Safe, Value, UNDEFINED,
};
pub use render::{components, must_render, render, RenderError};
pub use trusted_html::TrustedHtml;
