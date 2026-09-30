# GOV.UK components — architecture

## Source of truth

[GOV.UK Frontend](https://frontend.design-system.service.gov.uk/) **Nunjucks** macros and fixtures (Node package `govuk-frontend`). This repo re-implements the HTML contract in **Rust** so product pages never hand-write component markup. See [tech-stack.md](tech-stack.md).

## Architecture

```text
Page / pattern (Askama + Axum)
  → govuk::render / must_render
    → Params / Value (aligned with Nunjucks macro options)
      → Rust renderer → exact HTML string
        → Frontend CSS/JS in the page shell
```

Supporting pieces:

- **Shared HTML helpers** — Nunjucks-compatible escape + attribute serialization (`src/govuk/`).
- **Fixture loader** — `fixtures.json` via Serde for catalogue previews and parity tests.
- **Options mapper** — fixture `options` → `Params` (including edge cases).
- **Layout chrome** — Askama `templates/layout.html` with skip link / header / footer / service nav.
- **Catalogue** — `/components` (links only) and `/components/:name` (fixture preview + real parity banner).

See [layout-chrome.md](layout-chrome.md), [creating-components.md](creating-components.md), [testing-components.md](testing-components.md), [preview-server.md](preview-server.md).

## Components vs patterns

|                | Components                | Patterns               |
| -------------- | ------------------------- | ---------------------- |
| Design System  | `/components/`            | `/patterns/` and Pages |
| Implementation | Rust renderers + fixtures | Composed pages         |
| Parity suite   | Required                  | Not applicable         |

## Component set

This example ships Rust renderers for every fixture-bearing Frontend component in the pinned release (39 components), including: accordion, back link, breadcrumbs, button, character count, checkboxes, cookie banner, date input, details, error message, error summary, exit this page, feedback, fieldset, file upload, generic header, footer, header, hint, input, inset text, label, language navigation, notification banner, pagination, panel, password input, phase banner, radios, select, service navigation, skip link, summary list, table, tabs, tag, task list, textarea, warning text.

Per-component deep dives: add `docs/govuk-<kebab-name>.md` as needed. Until then use the [Design System component pages](https://design-system.service.gov.uk/components/).

## Preview contract

- Catalogue index: links + short descriptions only (no embedded demos).
- Component page: back link, demo banner, **parity banner only when Rust ≡ fixture**, Design System link, dotted preview, fixture version list with Current badge.
- Raw fixture route: Rust-rendered HTML fragment (same path as tests).
