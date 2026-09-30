# Preview server

Local Axum server for the fishing licence journey and component catalogue.

## Start

```sh
npm ci
npm run build:styles
cargo run
# or: npm start
```

Opens at <http://127.0.0.1:3000> (binds `0.0.0.0:$PORT`, default `3000`). Set `HOST=127.0.0.1` for loopback only.

`DEMOS_ENABLED` defaults on when not in production. Set `DEMOS_ENABLED=false` to hide `/components` and example preview pages.

## Expectations

- Homepage lists the journey; when demos are on, links to the **component catalogue**.
- Catalogue index (`/components`) is **links only** — no embedded live demos.
- `/components/:name` renders the selected fixture with a parity banner that is green **only** when Rust HTML equals the fixture.
- `/components/:name/fixture` returns the Rust-rendered HTML **fragment**.
- Responses use [`baseline/`](../baseline/) headers. Local HTTP uses `secureTransport: false` so HSTS is not sent.
- `GET /health` returns `ok`.

## After code changes

Rebuild with `cargo run` (or restart the release binary). Hard-refresh the browser. Confirm focus states, header/footer, and a failing-form example during visual QA after Frontend upgrades ([upgrading-govuk-frontend.md](upgrading-govuk-frontend.md)).

See [example-service.md](example-service.md), [govuk-components.md](govuk-components.md), [testing-components.md](testing-components.md).
