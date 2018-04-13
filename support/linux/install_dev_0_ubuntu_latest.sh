#!/bin/sh
set -eux

sudo -E apt-get update

sudo -E apt-get install -y --no-install-recommends \
  build-essential \
  ca-certificates \
  cmake \
  curl \
  file \
  gdb \
  iproute2 \
  libsodium-dev \
  libssl-dev \
  man \
  musl-tools \
  net-tools \
  pkg-config \
  software-properties-common \
  postgresql \
  sudo \
  tmux \
  vim \
  wget
