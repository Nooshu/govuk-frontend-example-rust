# Project purpose

This repository is a **Rust** example of a **GDS-compliant** government frontend.

## Intent

This example shows how to stand up a service that:

1. Meets [Service Standard](https://www.gov.uk/service-manual/service-standard) and [Technology Code of Practice](https://www.gov.uk/guidance/the-technology-code-of-practice) expectations for common components, accessibility, and open standards — as far as the UI layer can.
2. Uses **Rust** (Axum + Askama + Tokio) for HTML generation and application logic.
3. Uses **[GOV.UK Frontend](https://frontend.design-system.service.gov.uk/)** (latest pinned release) as the **only** frontend component library — styles, progressive-enhancement JS, and macro-aligned HTML.
4. Does **not** introduce SPA or component **frontend frameworks** (React, Vue, Angular, Svelte, etc.) for rendering GOV.UK UI.
5. Derives component HTML from **GOV.UK Frontend macros** / `template.njk` via **native Rust renderers** — never long-term copy-paste from each release — and wires **official test fixtures** for extensive **100% HTML parity** testing of Rust output.

## Priorities

1. Frontend web performance
2. Frontend security
3. Reduced maintenance
4. Accessibility
5. Inclusive design

See [priorities.md](priorities.md).

GOV.UK Frontend remains a **Node** package with **Nunjucks** macros upstream (install, fixtures, Sass). Request-time HTML is rendered in Rust. See [tech-stack.md](tech-stack.md).

## What “GDS compliant” means here

Follow official GDS guidance and the Design System / Frontend contracts rather than inventing parallel UI systems. Canonical places to search: [guidance-sources.md](guidance-sources.md).

Using this example does **not** by itself make a live service assessment-ready — see [service-assessment-readiness.md](service-assessment-readiness.md) and [assisted digital](https://www.gov.uk/service-manual/helping-people-to-use-your-service/assisted-digital-support-introduction).

## Agent skill

Coding agents should apply [`.cursor/skills/gds-compliant-frontend/SKILL.md`](../.cursor/skills/gds-compliant-frontend/SKILL.md) when scaffolding or reviewing work in this repo, and [`.cursor/rules/govuk-frontend-rust.mdc`](../.cursor/rules/govuk-frontend-rust.mdc) for Rust-specific architecture.
