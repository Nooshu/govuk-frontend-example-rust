<!-- ============================================================
  GOV.UK Design System — Agent instructions
  ============================================================
  Base template: GDS-compliant frontends (backend + GOV.UK Frontend)
  Detail lives in /docs and .cursor/skills/ (gds-compliant-frontend, safe-dependency-updates)
  ============================================================ -->

♛ GOV.UK

# GOV.UK Frontend example

**Rust** example of a **GDS-compliant** government frontend: **Axum** + **Askama** generate HTML; **[GOV.UK Frontend](https://frontend.design-system.service.gov.uk/)** (latest pinned version) is the **only** UI component library. **No frontend frameworks** (React, Vue, Angular, Svelte, etc.) for UI.

All component HTML is produced by **native Rust renderers** that track **GOV.UK Frontend macros** / `template.njk`. Do not shell out to Node for request-time rendering. Never long-term copy-paste release HTML. Official **test fixtures** from each Frontend release are the contract: **Rust HTML** must match every fixture `html` byte-for-byte.

**LIVE guidance** — [Design System feedback](https://design-system.service.gov.uk/community/feedback/).

Skills: [`.cursor/skills/gds-compliant-frontend/SKILL.md`](.cursor/skills/gds-compliant-frontend/SKILL.md), [`.cursor/skills/safe-dependency-updates/SKILL.md`](.cursor/skills/safe-dependency-updates/SKILL.md). Rust architecture: [`.cursor/rules/govuk-frontend-rust.mdc`](.cursor/rules/govuk-frontend-rust.mdc). Purpose: [`docs/project-purpose.md`](docs/project-purpose.md).

## Priorities (in order)

1. Frontend web performance
2. Frontend security
3. Reduced maintenance
4. Accessibility
5. Inclusive design

Details: [`docs/priorities.md`](docs/priorities.md).

## Start here

| Audience                  | Doc                                                                                                                                                                        |
| ------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Human developers**      | [`docs/onboarding.md`](docs/onboarding.md), [`CONTRIBUTING.md`](CONTRIBUTING.md)                                                                                           |
| **AI agents (this file)** | Keep reading; skills: [gds-compliant-frontend](.cursor/skills/gds-compliant-frontend/SKILL.md), [safe-dependency-updates](.cursor/skills/safe-dependency-updates/SKILL.md) |
| Dual-audience docs map    | [`docs/documentation-structure.md`](docs/documentation-structure.md), [`docs/README.md`](docs/README.md)                                                                   |
| Project purpose           | [`docs/project-purpose.md`](docs/project-purpose.md)                                                                                                                       |
| Official guidance URLs    | [`docs/guidance-sources.md`](docs/guidance-sources.md)                                                                                                                     |
| Stack / language          | [`docs/tech-stack.md`](docs/tech-stack.md)                                                                                                                                 |

**Language rule:** This example is **Rust**. Every feature and code change must follow current Rust best practices (project layout, typing, modules, tests, packaging, CI, Clippy) — without weakening the non-negotiables below. Record stack-specific conventions in [`docs/tech-stack.md`](docs/tech-stack.md).

**HTML generation:** Generate component HTML **natively** in Rust — do not require Node at request time for rendering. Always track Frontend’s macros/`template.njk` as the behaviour reference and prove Rust ≡ fixtures. Never long-term copy-paste static HTML from each release.

**GOV.UK Frontend’s own stack:** Frontend ships as a **Node** package with **Nunjucks** macros, official `fixtures.json`, and `template.njk` sources. Use Node for install, fixtures, and Sass. Refer to Nunjucks for macro options and escape behaviour when the Rust renderer reimplements them.

**Guidance rule:** Prefer searching the URLs in [`docs/guidance-sources.md`](docs/guidance-sources.md) over inventing local policy.

**Documentation rule:** Every prompt, feature, and code change must leave **comprehensive dual-audience documentation** (humans + AI agents) in the right place. Detail: [`docs/documentation-structure.md`](docs/documentation-structure.md).

## Non-negotiables

1. **GOV.UK Frontend macros are the HTML source of truth** — implement native Rust renderers that track those macros. Do **not** copy-paste component HTML from release notes or the Design System site as the long-term approach, and do **not** shell out to Node just to render HTML.
2. **Rust HTML must match every official fixture** — for each shipped component, Rust output is compared byte-for-byte to the `html` in that release’s `fixtures.json`, for **every** fixture. That is the primary parity gate. No normalisation; never edit fixture `html` to pass tests. See [`docs/testing-components.md`](docs/testing-components.md).
3. **No frontend UI frameworks** — no React/Vue/Angular/Svelte (or similar) for GOV.UK UI; Rust + GOV.UK Frontend only.
4. **No ad-hoc custom CSS** — ship styles through the Sass pipeline in [`styles/`](styles/) (`application.scss` → GOV.UK Frontend `@use` → [`govuk-overrides.scss`](styles/govuk-overrides.scss) last). Prefer component options and Design System patterns; do not paste or serve Frontend’s prebuilt `govuk-frontend.min.css` as the long-term source. See [`docs/styles.md`](docs/styles.md).
5. **No `!important` in service CSS** — overrides must win with cascade order and specificity only. Frontend’s own `govuk-!-…` utilities are upstream; do not copy that pattern into service styles.
6. **Components via library API** — never hand-paste component `govuk-*` markup into pages; call `govuk::render` / `must_render`.
7. **Patterns compose components** — Design System patterns are pages/journeys, not new low-level components, and have no fixture-parity suites.
8. **WCAG 2.2 AA baseline** — skip link, one `h1`, visible focus (never override yellow focus), keyboard paths, Error summary + field errors, `novalidate`.
9. **Progressive enhancement** — core tasks work without Frontend JS; keep `js-enabled` / `govuk-frontend-supported` and `initAll()`.
10. **Do not ship unreleased GOV.UK chrome** — wait for Frontend release + fixtures. See [`docs/govuk-frontend-roadmap.md`](docs/govuk-frontend-roadmap.md).
11. **100% code coverage** — functions, branches, and statements at **100%** for application/library code under test; CI must fail below that. Do not weaken fixture HTML equality to chase coverage. See [`docs/testing-components.md`](docs/testing-components.md).
12. **Dependency and Frontend upgrades need a green pipeline** — follow [`.cursor/skills/safe-dependency-updates/SKILL.md`](.cursor/skills/safe-dependency-updates/SKILL.md). For `govuk-frontend`, also read https://github.com/alphagov/govuk-frontend/releases/latest and [`docs/upgrading-govuk-frontend.md`](docs/upgrading-govuk-frontend.md).
13. **Performance and security baseline** — every response uses [`baseline/`](baseline/) (cache kind, OWASP headers, CSP hash for the `js-enabled` snippet). Do not invent a weaker set. See [`docs/frontend-performance.md`](docs/frontend-performance.md) and [`docs/frontend-security.md`](docs/frontend-security.md).
14. **Split finished work into focused commits** — once a coherent piece of code or docs is complete, create **specific** commits with **comprehensive** messages (why, contract impact, how to verify). Do not leave a large mixed working tree; do not squash unrelated concerns into one commit.
15. **Document every change for humans and agents** — no feature, prompt-driven change, or behaviour lands without dual-audience docs updated in the right place (`/docs` detail, `AGENTS.md` / skill / rules links when contracts change, onboarding or CONTRIBUTING when workflow changes). See [`docs/documentation-structure.md`](docs/documentation-structure.md).
16. **Follow the latest Rust best practices** — all new and changed code must match current Rust idioms (see [`docs/tech-stack.md`](docs/tech-stack.md)). Shared Node tooling (Sass, baseline) follows current Node/ESM practice. Never weaken Frontend, parity, security, or performance non-negotiables to chase a fad.

Using this repo does **not** make a service assessment-ready. See [`docs/service-assessment-readiness.md`](docs/service-assessment-readiness.md).

## Agent playbooks

| Task                                | Doc                                                                                                                          |
| ----------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| Upgrade GOV.UK Frontend             | [`docs/upgrading-govuk-frontend.md`](docs/upgrading-govuk-frontend.md)                                                       |
| Safe dependency updates             | [`.cursor/skills/safe-dependency-updates/SKILL.md`](.cursor/skills/safe-dependency-updates/SKILL.md) — **green CI required** |
| Add a component                     | [`docs/creating-components.md`](docs/creating-components.md)                                                                 |
| Add a pattern                       | [`docs/creating-patterns.md`](docs/creating-patterns.md)                                                                     |
| Layout / chrome                     | [`docs/layout-chrome.md`](docs/layout-chrome.md)                                                                             |
| Fixture / parity testing            | [`docs/testing-components.md`](docs/testing-components.md)                                                                   |
| Page shell                          | [`docs/page-shell.md`](docs/page-shell.md)                                                                                   |
| Frontend performance                | [`docs/frontend-performance.md`](docs/frontend-performance.md)                                                               |
| Frontend security                   | [`docs/frontend-security.md`](docs/frontend-security.md)                                                                     |
| Accessibility                       | [`docs/accessibility.md`](docs/accessibility.md)                                                                             |
| Content & forms                     | [`docs/content-and-forms.md`](docs/content-and-forms.md)                                                                     |
| Design tokens (colour, type, space) | [`docs/design-tokens.md`](docs/design-tokens.md)                                                                             |
| Styles / Sass cascade               | [`docs/styles.md`](docs/styles.md)                                                                                           |
| Dual-audience documentation         | [`docs/documentation-structure.md`](docs/documentation-structure.md)                                                         |
| Guidance sources                    | [`docs/guidance-sources.md`](docs/guidance-sources.md)                                                                       |
| Authoritative links                 | [`docs/authoritative-references.md`](docs/authoritative-references.md)                                                       |

## Quick page review

Before finishing a page change:

- [ ] Page template shell / before-content / single `h1` / title
- [ ] Macros / library API only for GOV.UK UI blocks (not pasted HTML)
- [ ] Back link **or** breadcrumbs — not both
- [ ] Forms: `novalidate`, Error summary + messages, values retained
- [ ] Focus styles untouched; no `outline: none`
- [ ] Trusted/sanitised HTML only; prefer plain text options
- [ ] HTML responses use the baseline security headers; assets use the matching cache kind
- [ ] CSS from the Sass pipeline in `<head>` (not prebuilt `govuk-frontend.min.css`); Frontend JS is an external `type="module"`; the `js-enabled` snippet matches the pinned CSP hash
- [ ] No `!important` in service styles; overrides only via `govuk-overrides.scss` specificity
- [ ] Pattern guidance followed; out-of-scope widgets called out with inset text
- [ ] Coverage remains 100% functions / branches / statements for touched library code
- [ ] Rust parity suite green: library HTML ≡ every fixture `html`
- [ ] Fixture parity still green for any touched components
- [ ] Dual-audience docs updated (humans in `/docs` or CONTRIBUTING; agents via `AGENTS.md` / skill / playbook links if contracts changed)
- [ ] Code follows current Rust best practices ([`docs/tech-stack.md`](docs/tech-stack.md))

## Watching upstream

Dependency bumps (npm, language packages, Dependabot) and Frontend pin changes: [`.cursor/skills/safe-dependency-updates/SKILL.md`](.cursor/skills/safe-dependency-updates/SKILL.md) (**CI must be green** before start and before complete). `govuk-frontend` still uses [`docs/upgrading-govuk-frontend.md`](docs/upgrading-govuk-frontend.md).
