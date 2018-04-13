#!/bin/sh
set -eux

# Install Rust and musl libc target
curl -sSf https://sh.rustup.rs \
  | env -u CARGO_HOME sh -s -- -y --default-toolchain stable
. $HOME/.cargo/env
env -u CARGO_HOME rustup target add x86_64-unknown-linux-musl

# Always use the latest version of rust lang
rustup update stable

rustc --version
cargo --version

if [ ! -f /usr/bin/node ] && [ -f /usr/bin/nodejs ]; then
  sudo -E ln -snf /usr/bin/nodejs /usr/bin/node
fi

sudo -E npm install -g docco

echo "Node $(node --version)"
echo "npm $(npm --version)"

