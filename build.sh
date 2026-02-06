#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

cd "$ROOT_DIR"

cargo build --release

echo "Built binary: $ROOT_DIR/target/release/workspace-cli"
