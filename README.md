# GOV.UK Frontend example (Rust)

> [!IMPORTANT]
> You are free to fork, modify, and maintain this repository for your own use.
>
> This includes using and adapting it within your department, organisation, or project.

<!-- Separate alerts. Prettier collapses the two blank lines markdownlint MD028 would accept. -->

> [!WARNING]
>
> **🚨 Example repository only**
>
> This repository is a **demonstration only**. It will not be actively maintained or supported.
>
> **It is not an official UK government project.** It is not endorsed, maintained, or supported by any UK government department, the Government Digital Service (GDS), or the GOV.UK Design System team.
>
> **No ongoing support or updates will be provided.** This includes maintenance, dependency updates, security fixes, or technical support.
>
> **Use this code at your own risk.** You are responsible for reviewing, testing, securing, and maintaining the code, and for determining whether it is suitable for use in a service or production environment.
>
> **You are free to fork, modify, and maintain this repository for your own use.**
>
> This repository is released under the [MIT Licence](LICENSE). See the licence for the full terms.

Server-rendered UK government-style service using **Rust**, **Axum**, and **Askama** with **[GOV.UK Frontend](https://frontend.design-system.service.gov.uk/)** as the only UI library — **no** React/Vue/Angular/Svelte.

Component HTML is produced by native Rust renderers that track Frontend macros/`template.njk`. Official `fixtures.json` from the pinned release are the parity contract (**100%** match for every fixture).

Stack details: [`docs/tech-stack.md`](docs/tech-stack.md). Example journey: [`docs/example-service.md`](docs/example-service.md). Deploy: [`docs/deploying-on-render.md`](docs/deploying-on-render.md).

## Priorities

Frontend web performance → frontend security → reduced maintenance → accessibility → inclusive design.

## Who should read what

| You are…            | Start here                                                                                                                                                                                               |
| ------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Human developer** | [`docs/onboarding.md`](docs/onboarding.md) → [`CONTRIBUTING.md`](CONTRIBUTING.md) → [`docs/`](docs/README.md)                                                                                            |
| **AI coding agent** | [`AGENTS.md`](AGENTS.md) → [`.cursor/skills/gds-compliant-frontend/`](.cursor/skills/gds-compliant-frontend/SKILL.md) → [`.cursor/rules/govuk-frontend-rust.mdc`](.cursor/rules/govuk-frontend-rust.mdc) |

## Quick start

```sh
npm ci
npm start
```

Builds styles, then runs the Axum server (default <http://127.0.0.1:3000>). Use `npm run start:release` for an optimised binary.

```sh
npm test          # Node baseline + Sass + cargo fmt/clippy/test
npm run verify    # docs + styles + tests
```

## Licence and security

- Code in this repository: [MIT License](LICENSE)
- How to report vulnerabilities: [SECURITY.md](SECURITY.md)
- GOV.UK Design System and Frontend are maintained by GDS; Crown copyright / OGL apply to GOV.UK content patterns as documented on GOV.UK.
