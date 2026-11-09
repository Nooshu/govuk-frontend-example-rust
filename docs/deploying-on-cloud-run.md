# Deploying on Google Cloud Run

Host this Rust example on [Cloud Run](https://cloud.google.com/run/docs). The binary binds `0.0.0.0:$PORT`. Cloud Run’s default port is 8080. TLS stops at the platform. The image sets `SECURE_TRANSPORT=true` so session cookies are marked `Secure`.

You need a Google Cloud project with billing enabled and the [gcloud CLI](https://cloud.google.com/sdk/docs/install). Cloud Run can scale to zero. Build minutes, Artifact Registry storage, and requests above the free tier are billed. The first Rust image build compiles the release binary and takes longer than the other languages.

This service’s name is `govuk-frontend-example-rust`, region `europe-west2` (London).

## What this repo includes

| File                                                            | Role                                                 |
| --------------------------------------------------------------- | ---------------------------------------------------- |
| [`Dockerfile`](../Dockerfile)                                   | Node Sass, then `cargo build --release`, plus assets |
| [`scripts/deploy-cloud-run.sh`](../scripts/deploy-cloud-run.sh) | Enables APIs and deploys from source                 |
| `GET /health`                                                   | Plain `ok` for startup, readiness, and liveness      |

## Deploy

From the repo root, with a project selected:

```sh
gcloud config set project PROJECT_ID
./scripts/deploy-cloud-run.sh
```

| Variable        | Default                       |
| --------------- | ----------------------------- |
| `PROJECT`       | gcloud’s active project       |
| `REGION`        | `europe-west2`                |
| `SERVICE`       | `govuk-frontend-example-rust` |
| `MEMORY`        | `512Mi`                       |
| `CPU`           | `1`                           |
| `MIN_INSTANCES` | `0`                           |
| `MAX_INSTANCES` | `3`                           |

The script sets `DEMOS_ENABLED=true` and `SECURE_TRANSPORT=true`. Without `DEMOS_ENABLED`, `NODE_ENV=production` hides the catalogue.

### After deploy checklist

- [ ] `https://<service>-<hash>.<region>.run.app/health` returns `ok`
- [ ] Start page loads with GOV.UK styling
- [ ] `/components` works
- [ ] A form POST in the licence journey keeps the answers on the next page

## Run the image locally

```sh
docker build -t govuk-frontend-example-rust .
docker run --rm -p 8080:8080 -e PORT=8080 -e SECURE_TRANSPORT=false govuk-frontend-example-rust
```

Open <http://127.0.0.1:8080>. `SECURE_TRANSPORT=false` keeps the session cookie usable over local HTTP.
