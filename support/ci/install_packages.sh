#!/bin/bash
set -eu

echo "install packages"

apt-get -y update
apt-get install -y libsodium-dev
