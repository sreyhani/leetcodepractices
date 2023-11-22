#!/usr/bin/env bash
set -eu

cargo clippy --workspace --all-targets -- --deny warnings
