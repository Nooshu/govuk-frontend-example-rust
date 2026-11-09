# Cloud Run image.
# Stage 1: Node — pin govuk-frontend and compile Sass.
# Stage 2: Rust — release binary (Askama templates compiled in).
# Stage 3: Runtime — binary plus Frontend assets and baseline policy.

FROM node:22-bookworm AS styles
WORKDIR /build
COPY package.json package-lock.json ./
RUN npm ci
COPY styles ./styles
COPY scripts ./scripts
RUN npm run build:styles

FROM rust:1-bookworm AS build
WORKDIR /src
COPY rust-toolchain.toml Cargo.toml Cargo.lock ./
COPY src ./src
COPY templates ./templates
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update \
  && apt-get install -y --no-install-recommends ca-certificates \
  && rm -rf /var/lib/apt/lists/*
COPY --from=build /src/target/release/govuk-frontend-example-rust /app/server
COPY --from=styles /build/node_modules/govuk-frontend ./node_modules/govuk-frontend
COPY --from=styles /build/dist ./dist
COPY package.json ./
COPY baseline ./baseline
ENV DEMOS_ENABLED=true \
    SECURE_TRANSPORT=true
EXPOSE 8080
CMD ["/app/server"]
