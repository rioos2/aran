#!/bin/bash

rustfmt --version

git log HEAD~1..HEAD | grep -q '!!! Temporary Commit !!!'
is_tmp_commit=$?


# When we're on a temporary commit, don't do anything.
if [[ $is_tmp_commit = 0 ]]; then
  exit 0
fi

set -e
if [ "${TRAVIS_PULL_REQUEST}" = "false" ]; then
  make unit-all
else
  make unit-all
fi
