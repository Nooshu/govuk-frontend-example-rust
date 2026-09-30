# Deploying on Render.com

Host this Rust example service on [Render](https://render.com) as a public demo. This line is a normal Rust HTTP process — not a static site — so use a **Web Service**, not Static Sites.

Authoritative Render docs: [Language support](https://render.com/docs/language-support), [Blueprints](https://render.com/docs/blueprint-spec), [Native runtimes](https://render.com/docs/native-runtimes).

## What this repo already includes

| File                                                    | Role                                                      |
| ------------------------------------------------------- | --------------------------------------------------------- |
| [`render.yaml`](../render.yaml)                         | Blueprint: free web service, build/start, `/health` check |
| [`scripts/render-build.sh`](../scripts/render-build.sh) | `npm ci` → Sass → `cargo build --release`                 |
| `src/main.rs`                                           | Binds all interfaces by default; honours Render’s `PORT`  |
| `GET /health`                                           | Plain `ok` for Render health checks                       |

Build artefacts kept at runtime: `target/release/govuk-frontend-example-rust`, `node_modules/govuk-frontend`, `dist/stylesheets/application.css`.

## Prerequisites

1. A GitHub account with this repo (or a fork) pushed to `main`.
2. A [Render](https://render.com) account (free tier is enough for a demo).
3. Local checks green before you deploy: `npm ci && npm run verify` (optional but recommended).

## Option A — Blueprint (recommended)

Uses the committed [`render.yaml`](../render.yaml).

### Steps

1. **Push this repo to GitHub**, including `render.yaml` and `scripts/render-build.sh`.
2. Open the [Render Dashboard](https://dashboard.render.com/) and sign in (connect GitHub when asked).
3. Click **New +** → **Blueprint**.
4. Select this repository (or your fork).
5. Confirm Render detects `render.yaml` at the repo root.
6. Review the service:
   - **Name:** `govuk-frontend-example-rust`
   - **Runtime:** Rust
   - **Plan:** Free
   - **Region:** Oregon (or closer to you)
   - **Build:** `./scripts/render-build.sh`
   - **Start:** `./target/release/govuk-frontend-example-rust`
   - **Health check:** `/health`
   - **Env:** `DEMOS_ENABLED=true`, `NODE_VERSION=22`
7. Create the Blueprint and wait for the first deploy.

## After deploy — verify

- [ ] `GET /health` returns `ok`
- [ ] Start page loads with compiled CSS (not a broken layout)
- [ ] Component catalogue is visible (`DEMOS_ENABLED=true`)
- [ ] A component preview shows a real parity banner (green only when HTML matches)
- [ ] Licence journey: Start now → task list → a question → check answers path works
- [ ] `/robots.txt` disallows crawling
- [ ] HTML responses include `X-Robots-Tag: noindex…` and a matching `<meta name="robots">`

## Free-tier behaviour

- The service **sleeps** after about 15 minutes idle; the first request after sleep can take ~1 minute.
- **In-memory sessions** reset when the process sleeps or restarts.
- Hobby build minutes are limited; prefer release builds and avoid cleaning `target/` unnecessarily.

## Option B — Manual web service

Create a Web Service, choose the Rust runtime, set the same build/start commands and env vars as in `render.yaml`.
