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

function stop_pg {
  sudo systemctl stop postgresql
  exit 0
}

trap stop_pg SIGHUP SIGINT SIGTERM

pwd

running=0;

echo "Waiting for postgresql to start"
while [ $running -eq 0 ]; do
  if sudo -E TERM=vt100 psql -lqt --host 127.0.0.1 -U rioos; then
    running=1
  fi
  sleep 2
done

for dbname in rioosdb; do
  if sudo -E TERM=vt100 psql -lqt --host 127.0.0.1 -U rioos -W rioos | cut -d \| -f 1 | grep -qw $dbname; then
    echo "Database $dbname exists"
  else
    echo "Creating database $dbname"
    sudo -u hab -E TERM=vt100 createdb -O rioos -h 127.0.0.1 $dbname
  fi
done

while true; do
  sleep 1
done
