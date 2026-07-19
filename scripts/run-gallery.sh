#!/usr/bin/env bash
set -euo pipefail
export RUST_BACKTRACE=1
cargo run -p acme-gallery
