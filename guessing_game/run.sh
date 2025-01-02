#!/usr/bin/env bash
set -euxo pipefail

cargo build
cargo run
cargo check
# cargo update