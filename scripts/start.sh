#!/usr/bin/env bash
# Ensure rustup/cargo are on PATH even when npm runs with a minimal env.
set -euo pipefail

cd "$(dirname "$0")/.."

if [[ -f "${HOME}/.cargo/env" ]]; then
  # shellcheck source=/dev/null
  source "${HOME}/.cargo/env"
fi
export PATH="${HOME}/.cargo/bin:${PATH}"

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo was not found. Install Rust from https://rustup.rs/ then retry." >&2
  echo "After install, ensure ~/.cargo/bin is on your PATH (source ~/.cargo/env)." >&2
  exit 1
fi

npm run build:styles
exec cargo run "$@"
