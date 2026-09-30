#!/usr/bin/env bash
# Build the example service for Render (and similar hosts).
# Installs the Frontend pin, compiles Sass, then builds the Rust release binary.
set -euo pipefail

cd "$(dirname "$0")/.."

npm ci
npm run build:styles
cargo build --release
