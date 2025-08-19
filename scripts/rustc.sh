#!/usr/bin/env bash
set -euxo pipefail

clear
rustc $* --color always 2>&1 | more