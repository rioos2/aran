#!/bin/bash

# Copyright 2017 RioCorp Inc.

set -o errexit
set -o nounset
set -o pipefail

# The root of the build/dist directory
RIOOS_ROOT="$(cd "$(dirname "${BASH_SOURCE}")/../.." && pwd -P)"

source "${RIOOS_ROOT}/tools/lib/util.sh"
source "${RIOOS_ROOT}/tools/lib/logging.sh"

rioos::log::install_errexit
