#!/usr/bin/env bash
set -euxo pipefail

cd passwd-gen
cargo build --release

#./target/release/passwd-gen --use_upper --use_lower --use_numbers --no_symbols --length 16