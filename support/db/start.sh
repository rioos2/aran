#!/bin/bash
#
# Oh RIO/OS, how you bring me back to my most hack-worthy roots. I love you for it.
#
# What this does - we trap our own exit, and at exit, we send a SIGINT to all the
# children in our process group - this brings habitat down. When we run tests, we
# start this script, and it will take care of setting up the test database on your
# behalf, no matter what.
#
# The gpid stuff below is because we need to track the parent process ID of the
# sudo command that executes us.

function stop_cockroach {
  sudo /usr/bin/killall cockroach
  exit 0
}

trap stop_pg SIGHUP SIGINT SIGTERM

pwd

running=0;

echo "Waiting for cockroachdb to start"
while [ $running -eq 0 ]; do
  cd $MEGAM_HOME/cockroach
  #  cockroach start --certs-dir=certs --host=localhost --http-host=localhost
  if cockroach start --insecure --host=localhost --http-host=localhost; then
    running=1
  fi
  sleep 2
done

while true; do
  sleep 1
done
