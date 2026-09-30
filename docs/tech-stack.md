# Tech stack

**Status: Rust** (implementation language recorded)

This example is a **Rust** server-rendered GOV.UK Frontend service. Component and page HTML are produced in Rust (Askama for pages; native Rust renderers that track Frontend macros/`template.njk` for components). There is **no** React/Vue/Angular/Svelte UI layer.

## Two layers

| Layer                          | Stack                                                                                               | Notes                                                                                            |
| ------------------------------ | --------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| **GOV.UK Frontend (upstream)** | **Node** package (`govuk-frontend`), **Nunjucks** macros (`template.njk`), official `fixtures.json` | Fixed by GDS. Node is used for install, Sass, fixtures, and shared baseline/docs tests only.     |
| **This example (wrapper)**     | **Rust** + **Axum** + **Askama** + **Tokio**                                                        | Native component HTML in Rust. **Do not** shell out to Node/Nunjucks for request-time rendering. |

## Runtime and tooling

| Item                    | Value                                                                                                                                                                              |
| ----------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Implementation language | Rust (stable, see `rust-toolchain.toml`)                                                                                                                                           |
| HTTP                    | Axum on Tokio                                                                                                                                                                      |
| Page templates          | Askama                                                                                                                                                                             |
| Component HTML          | Rust renderers tracking Frontend macros; fixture parity is the contract                                                                                                            |
| Package manager         | Cargo (`Cargo.lock` committed) + npm for `govuk-frontend` / Sass                                                                                                                   |
| `govuk-frontend` (Node) | `6.5.1` — check [latest release](https://github.com/alphagov/govuk-frontend/releases/latest) before upgrades                                                                       |
| Sass pipeline           | `styles/application.scss` → `@use` Frontend → `govuk-overrides.scss` last → `npm run build:styles` → `dist/stylesheets/application.css`                                            |
| Never ship              | Frontend’s prebuilt `govuk-frontend.min.css` as the long-term source                                                                                                               |
| Escape / attributes     | Nunjucks-compatible escape and `govukAttributes` behaviour in `src/govuk/`                                                                                                         |
| Fixture loader          | `node_modules/govuk-frontend/dist/govuk/components/*/fixtures.json` via Serde + ordered Params                                                                                     |
| Parity gate             | Rust output (outer-trim only) ≡ every fixture `html` (including hidden)                                                                                                            |
| Coverage                | Node baseline+Sass gated at **100%** in `npm test`. Rust: full fixture parity suite + `cargo llvm-cov` (exclude `main.rs`). Expand HTTP/domain tests toward 100% library coverage. |
| Deploy                  | Render.com free tier — native Rust runtime, `render.yaml`, `scripts/render-build.sh`                                                                                               |

### Coverage commands

```sh
# Node baseline + Sass (already gated in npm test)
npm run test:baseline
npm run test:styles

# Rust (install once: cargo install cargo-llvm-cov)
cargo llvm-cov --all --ignore-filename-regex='main\.rs'
```

Integration tests in `tests/` exercise HTTP routes and are included in `cargo test --all`.

## Commands

```sh
npm ci
npm run build:styles
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all
npm run verify          # docs + Sass + Node baseline tests
npm start               # build styles then run the Rust server
```

Render build: `./scripts/render-build.sh` → `./target/release/govuk-frontend-example-rust`.

## Shared baseline

[`baseline/policy.json`](../baseline/policy.json) defines OWASP headers, CSP (including the `js-enabled` snippet hash), and cache kinds. The Rust HTTP layer applies the same kinds; production HTTPS uses `secureTransport: true`. Compress with Brotli (`br`); Gzip only when the client does not advertise `br`.

## HTML generation rules

- Prefer GOV.UK `text` options (escaped). Use trusted HTML only for controlled `html` options (`Safe`).
- Component options mirror Nunjucks macro options / fixture `options`.
- Patterns compose components; they have no fixture-parity suites.
- No `!important` in service CSS ([styles.md](styles.md)).

## Hard constraints

- One pinned `govuk-frontend` version; CSS, JS, and fixtures stay in lockstep.
- Rust ≡ every official fixture `html`; never edit fixture HTML to pass tests.
- Node is build/tooling only — not request-time HTML rendering.
- No frontend UI frameworks for GOV.UK chrome.

See [`AGENTS.md`](../AGENTS.md), [testing-components.md](testing-components.md), [creating-components.md](creating-components.md), [example-service.md](example-service.md), [deploying-on-render.md](deploying-on-render.md).
