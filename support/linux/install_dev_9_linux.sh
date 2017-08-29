#!/bin/sh
set -eux

# Install Rust and musl libc target
curl -sSf https://sh.rustup.rs \
  | env -u CARGO_HOME sh -s -- -y --default-toolchain stable
. $HOME/.cargo/env
env -u CARGO_HOME rustup target add x86_64-unknown-linux-musl
env -u CARGO_HOME cargo install protobuf
rustc --version
cargo --version

if [ ! -f /usr/bin/node ] && [ -f /usr/bin/nodejs ]; then
  sudo -E ln -snf /usr/bin/nodejs /usr/bin/node
fi
sudo -E npm install -g docco
echo "Node $(node --version)"
echo "npm $(npm --version)"

if [ ! -f /usr/local/bin/rq ]; then
  curl -sSLf https://sh.dflemstr.name/rq | bash -s -- --yes false
fi

### TODO we run in user rioos/group rioos
###if command -v useradd > /dev/null; then
###  sudo -E useradd --system --no-create-home rioos || true
###else
###  sudo -E adduser --system rioos || true
###fi
###if command -v groupadd > /dev/null; then
###  sudo -E groupadd --system rioos || true
###else
###  sudo -E addgroup --system rioos || true
###fi
