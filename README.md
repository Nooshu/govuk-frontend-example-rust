# GOV.UK Frontend example (Rust)

> [!IMPORTANT]
> You are free to fork this repository and use it for your own purposes, and to modify and maintain it as you see fit.

> [!WARNING]
> 🚨 **Example repository only**
>
> This repository was created as a demonstration and will not be actively maintained or supported. It is not an official UK government project and is not endorsed, maintained, or supported by any UK government department, the Government Digital Service (GDS), or the GOV.UK Design System team.
>
> I will not be providing ongoing maintenance, updates, security fixes, or technical support.
>
> Use this code at your own risk. You are responsible for reviewing, testing, securing, maintaining, and ensuring the suitability of the code before using it in any service or production environment. I accept no responsibility or liability for any loss, damage, security issue, service failure, or other consequence resulting from its use.
>
> This repository is released under the MIT Licence. See the [LICENSE](LICENSE) file for the full licence terms.

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
