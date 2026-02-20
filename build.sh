#!/usr/bin/env bash
set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FLAMEBOLT_DIR="${SCRIPT_DIR}/flamebolt"

cd "$FLAMEBOLT_DIR"
trunk build

cd "$SCRIPT_DIR"
cargo build
