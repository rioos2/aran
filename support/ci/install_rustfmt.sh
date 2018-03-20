#!/bin/bash
set -eu

if command -v rustfmt >/dev/null; then
  echo "--> Detected rustfmt, skipping install"
  exit 0
fi

echo "--> Installing rustfmt"
rustup component add rustfmt-preview
