# Onboarding

Human-oriented map of this repository. Coding agents should treat [`AGENTS.md`](../AGENTS.md) as the dense entry point; humans should also read [`CONTRIBUTING.md`](../CONTRIBUTING.md). How docs are split for both audiences: [documentation-structure.md](documentation-structure.md).

## What this repo is

A **Rust** example of a **GDS-compliant** frontend: **Axum** + **Askama** generate HTML; **GOV.UK Frontend** is the only UI library; **no frontend frameworks** for UI. Exact **HTML parity** against official Frontend fixtures. See [project-purpose.md](project-purpose.md) and [tech-stack.md](tech-stack.md).

**GOV.UK Frontend** ships as a Node package with Nunjucks macros and `fixtures.json`. Install it with npm for assets and fixtures. Node is **build tooling only** — request-time HTML is rendered in Rust.

**Official guidance:** search the URLs in [guidance-sources.md](guidance-sources.md).

**Priorities:** frontend web performance → frontend security → reduced maintenance → accessibility → inclusive design ([priorities.md](priorities.md)).

**Documentation:** every lasting change is documented for **humans and agents** ([documentation-structure.md](documentation-structure.md)).

**HTML:** native Rust component renderers track Frontend macros/`template.njk`; official fixtures enable **100% parity** tests. Do not copy-paste component HTML from each release as the long-term approach. Before Frontend upgrades, always read https://github.com/alphagov/govuk-frontend/releases/latest.

## Components vs patterns

| Kind          | What it is                                                               | How we build it                              | Fixture parity?                                                         |
| ------------- | ------------------------------------------------------------------------ | -------------------------------------------- | ----------------------------------------------------------------------- |
| **Component** | Design System building block (button, text input, …)                     | Rust renderer that emits exact Frontend HTML | **Yes** — official `fixtures.json`                                      |
| **Pattern**   | Guidance for a journey or page composition (addresses, check answers, …) | Compose shipped components into pages        | **No** — follow Design System guidance; no invented pattern HTML suites |

## Repo map

```text
AGENTS.md                 # Slim agent playbook
docs/                     # All documentation (this folder)
baseline/                 # Shared performance + OWASP header contract
styles/                   # Sass entry + govuk-overrides → dist/stylesheets/
scripts/                  # Node build helpers (styles, Render build)
src/                      # Rust library + Axum app
  govuk/                  # Component renderers + fixtures + escape/attributes
  web/                    # Routes (catalogue, journey, static pages)
  pages/                  # Askama page shell
  service/                # Fishing licence domain
templates/                # Askama templates (layout.html)
tests/                    # HTTP integration tests
```

## Run modes

| Mode    | Purpose                                                                                   |
| ------- | ----------------------------------------------------------------------------------------- |
| Preview | `npm start` / `cargo run` — journey + component catalogue                                 |
| Test    | `cargo test --all` (fixture parity + integration) + Node baseline/Sass suites             |
| Verify  | `npm run verify` — docs + styles + full test suite (CI equivalent)                        |
| Upgrade | Mechanical Frontend bump — see [upgrading-govuk-frontend.md](upgrading-govuk-frontend.md) |

Commands: [tech-stack.md](tech-stack.md).

## Testing mindset

1. **Parity checks (primary)** compare **Rust** output to fixture `html` with ordinal string equality — every fixture from the pinned Frontend release.
2. Never edit fixture `html` to make tests pass — fix the renderer.
3. Never normalise HTML in tests.
4. Nunjucks may be used as reference when reading `template.njk`; it is not a request-time dependency.

Details: [testing-components.md](testing-components.md).

## Troubleshooting

| Symptom                        | Likely cause                                                   |
| ------------------------------ | -------------------------------------------------------------- |
| Parity fails on whitespace     | Renderer ≠ Nunjucks `template.njk` / `{%-` stripping           |
| Encoding differs (`'` vs `'`)  | Used a framework encoder instead of Nunjucks-compatible escape |
| Attribute order differs        | Built attributes in code property order, not template order    |
| Preview/fixture 404            | `DEMOS_ENABLED=false` or production without demos              |
| Editing fixtures “fixes” tests | Wrong fix — update the Rust renderer                           |

## Consistency tooling

```sh
npm ci
npm start            # build styles, then cargo run
npm test             # baseline + Sass + cargo fmt/clippy/test
npm run verify:docs  # Prettier + markdownlint
npm run verify       # docs + build:styles + tests
```

See [CONTRIBUTING.md](../CONTRIBUTING.md).

## Next reads

1. [documentation-structure.md](documentation-structure.md)
2. [tech-stack.md](tech-stack.md)
3. [page-shell.md](page-shell.md) and [layout-chrome.md](layout-chrome.md)
4. [govuk-components.md](govuk-components.md)
5. [example-service.md](example-service.md)
6. [frontend-performance.md](frontend-performance.md) and [frontend-security.md](frontend-security.md)
7. [styles.md](styles.md)
8. [upgrading-govuk-frontend.md](upgrading-govuk-frontend.md)
9. [deploying-on-render.md](deploying-on-render.md)
