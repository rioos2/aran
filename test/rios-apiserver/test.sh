#!/bin/bash

echo "**** THESE TESTS WILL START YOUR COCKROACHDB ****"

pushd ../../
env RIOS_FUNC_TEST=1 ./support/linux/bin/forego start -f support/Procfile -e support/bldr.env 2>&1 > ./test/rios-apiserver/rios-apiserver.log &
forego_pid=$!
popd

echo "**** Services ready **** $forego_pid"
npm run mocha
mocha_exit_code=$?
echo "**** Stopping services ****"
kill -INT $forego_pid
exit $mocha_exit_code
