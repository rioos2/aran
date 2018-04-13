#!/bin/sh
set -eux

sudo -E pacman -Syyu --noconfirm

sudo -E pacman -S --noconfirm \
  base-devel \
  cmake \
  curl \
  gdb \
  libsodium \
  man \
  npm \
  openssl \
  pkg-config \
  postgresql \
  wget
