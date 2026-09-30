//! GOV.UK Frontend example — Rust library.
//!
//! Component HTML is rendered natively to match official fixtures byte-for-byte.

pub mod baseline;
pub mod config;
pub mod govuk;
pub mod httpx;
pub mod pages;
pub mod service;
pub mod session;
pub mod web;

pub use govuk::{
    components, must_render, render, Fixture, FixtureSet, Params, RenderError, Safe, Value,
};
