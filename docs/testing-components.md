# Testing components

How we keep **100% HTML parity** with GOV.UK Frontend — now and after upgrades.

Authoritative upstream: https://frontend.design-system.service.gov.uk/testing-your-html/

## What must be compared

| Compare                                 | Required?              | Purpose                                                      |
| --------------------------------------- | ---------------------- | ------------------------------------------------------------ |
| **Rust HTML → official fixture `html`** | **Yes — primary gate** | Proves the Rust renderer matches the pinned Frontend release |

Nunjucks macros are the upstream reference when reading `template.njk`. This example does **not** run a separate Nunjucks freshness suite at request or CI time; fixtures are taken from the pinned `govuk-frontend` package.

## Why fixtures exist

Official `fixtures.json` files (one set per component per Frontend release) are the contract for **extensive 100% HTML parity testing** of Rust-rendered markup. Load them from `node_modules/govuk-frontend/dist/govuk/components/*/fixtures.json`. Set them up from day one so every shipped component proves byte-for-byte equality — and so upgrades catch drift automatically.

Do **not** treat copy-pasted HTML from Design System examples or release notes as the source of truth; macros + fixtures are.

## Coverage gate (100% code)

Application and library code under test must maintain **100%** coverage of:

- **functions**
- **branches**
- **statements**

Node baseline and Sass scripts already gate at 100% via `npm test`. Rust library coverage is measured with `cargo llvm-cov` (see [tech-stack.md](tech-stack.md)); exclude only the binary entry (`main`). Do **not** normalise fixture HTML or skip parity cases to inflate coverage.

## Layers

| Layer                      | What it proves                           | How                                                                                                                      |
| -------------------------- | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| **Parity suite (primary)** | **Rust HTML** matches fixture `html`     | Map fixture `options` → `govuk::render` → ordinal string equality vs fixture `html` — cover **all** fixtures extensively |
| **HTTP integration**       | Routes, catalogue, assets, journey shell | `tests/http_integration.rs`                                                                                              |
| **Structural**             | Nav lists components; no demos on index  | Catalogue index is links only                                                                                            |

## Hard rules

1. **Never** edit fixture `html` to make tests pass.
2. **Never** normalise or pretty-print HTML before compare (trim only outer whitespace of Rust output).
3. **Never** mix Frontend versions between CSS/JS and fixtures.
4. Prefer in-process parity for the bulk of cases (fast, no HTTP); exercise **every** fixture for each shipped component through the Rust renderer.
5. After upgrade, the **parity suite** must be green — see [upgrading-govuk-frontend.md](upgrading-govuk-frontend.md). Always read https://github.com/alphagov/govuk-frontend/releases/latest first.

## Encoding

Parity depends on **Nunjucks-compatible `escape`** (`&#39;` for `'`, real newlines in textarea values). Implemented in `src/govuk/escape.rs`.

## Preview UI parity banner

On `/components/:name`, show the green **“HTML matches the fixture”** notification banner **only** when Rust output equals that fixture’s `html`. If it does not match, show a failure banner — never claim parity for a failing render.

## Commands

```sh
cargo test --all
npm test
```

See [creating-components.md](creating-components.md), [example-service.md](example-service.md), [preview-server.md](preview-server.md).
