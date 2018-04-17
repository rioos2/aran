# Rio/OS Aran API Development Environment

[Aran] is a code name for the API server and it means ``

## Overview

This document outlines the steps to start and run a Rio/OS Aran API environment for development. The environment includes the database services.

## Pre-Reqs
1. Use a Linux OS - either Ubuntu or ArchLinux.
1. Clone the aran repo to your local filesystem.
1. The sample commands below use the `curl` tool.

## Git hooks

After you clone  `aran.git`, please do the following.

```
	cp .hooks/* ./.git/hooks
    chmod 755 ./.git/hooks/pre-commit

```

## PostgreSQL - Ubuntu

### Install postgres
```
	sudo apt-get install postgresql

```

### Change as postgres user
```
    sudo -s
    su postgres
```

### To open postgresql from user `postgres`
```

psql

```

### To list all users
```       	
  postgres=# select * from user;
 	current_user
--------------
 postgres
```

### To create user `rioos` with password  `rioos`

```
postgres=# create user rioos with password 'rioos';
CREATE ROLE
```

### To grant all access to user `rioos`

```
postgres=# ALTER USER rioos WITH SUPERUSER;
ACCESS GRANTED
```

### To create new database  `rioosdb`
```
postgres=# create database rioosdb;
CREATE DATABASE
```

### To grant all permissions for db `rioodb` to user `rioos`
```
postgres=# grant all privileges on database rioosdb to rioos;
GRANT
```

### To exit from postgres
```
postgres=#  \q
```

### To configure postgresql to use password auth.
modify `/etc/postgresql/9.5/main/pg_hba.conf`
```
nano  /etc/postgresql/9.5/main/pg_hba.conf					
#from this line
local   all            all      peer  
#to line
local  all      all          md5
```

### Restart
```
systemctl stop postgresql
systemctl start postgresql
systemctl status postgresql
```

### psql to database rioosdb user `rioos`
```
psql -U rioos -W rioos -d rioosdb

set search_path TO shard_0 ,public;

```

### To change postgresql default port

By default postgresql will run on port: 5432

```
	nano  /etc/postgresql/9.5/main/postgresql.conf
  Port = <your port>
  systemctl stop postgresql
  systemctl start postgresql
```

### To delete database
```
drop database rioosdb;
```

## Bootstrap the OS with required packages

You need to make sure you have the required packages installed.
Run the appropriate shell script under the `support/linux` folder to install the packages.
Refer to [BUILDING.md](./BUILDING.md) doc for the detailed steps.

## Create configuration files *optional*

Some capabilities (such as configuring database, turning on bioshield.

Create the following files somewhere on your local filesystem)

`/var/lib/rioos/api.toml`
```toml

```
### Procfile *optional*

Now, modify the `Procfile` (located in your aran repo in the `support` folder) to point the api, and worker services to the previously created config files, e.g.

```
api: target/debug/rioos-api-server  --config /home/your_alias/rioos/api.toml
```
# Managing migrations for Builder services

All builder migrations are run with [Diesel](http://diesel.rs). This document describes how to create and manage those migrations.

## Install the Diesel client

```
cargo install diesel_cli --no-default-features --features postgres
```

## Generating new migrations

Every time you need to make a change to the Builder schema you will be required to generate a new migration

For the service `builder-SERVICE` you will need to run:

* `cd components/builder-SERVICE/src`
* `diesel migration generate <your migration name>`

The migration name should describe what you are doing. Ex:

* create-posts
* add-user-select-v4
* remove-user-select-43

This will generate something like

```
Creating migrations/20160815133237_create_posts/up.sql
Creating migrations/20160815133237_create_posts/down.sql
```

You can then edit `up.sql` to create your migration steps.
You should ignore, but not delete, `down.sql` as we don't use it since we rely on transactions for our rollback logic.

## Testing your changes

You will need to compile your service and restart it to test your changes. You should see:

`Running Migration <your-migration-name>`

## Run a build

Incremental builds  - used during development.

```
# Builds the api server only
make buildapi

# Builds the blockchain audit server only
make buildaud

# Builds the marketplace server only
make buildmkt
```
Builds for release - used during production.
For release builds append `r` to the above targets

```
make rbuildapi

```

Clean build, cleans everythings and does a build.
```
make clean
```

## Testing

This should show the status of the api server.

You should see a response similar to the following:

* Healthz
`curl  https://localhost:7443/v1/healthz
`

```
HTTP/1.1 201 Created

{
    "name": "aran",
    "version": "2.0.dev"
    "state": "alliswell"
}
```
To verify pfx

* The pfx password `TEAMRIOADVANCEMENT123`

```
openssl pkcs12 -info -in serving-rioos-apiserver.pfx

```
To verify pub/key

```
openssl verify -CAfile server-ca.crt serving-rioos-apiserver.crt

```

## Run Aran

1. Open a new terminal window.
2. Export the following environment variables:

```
export RIOOS_HOME=$HOME/home
```

```

./rioos_apiserver start

```

### Blockchain

We want to do audit logging in blockchain rioos-api-blockchainserver. We will use [rocksdb](https://rocksdb.org) as the storage.


Ubuntu
```
sudo apt-get install librocksdb-dev librocksdb4.5

export ROCKSDB_LIB_DIR=/usr/lib/x86_64-linux-gnu
export SNAPPY_LIB_DIR=/usr/lib/x86_64-linux-gnu
```

ArchLinux
```
yaourt rocksdb

```

## Performance testing

We want to do load testing on the api-server. We will use [locust](https://locust.io)

### Install locust.io

Watch this asciinema

```
https://asciinema.org/a/LtzjvzEWOkxqPZmMo6UbhujsY

```

### Run scripts


```
cd tools/perf

locust -f auth.py --host=http://<rioos_aran_api_server>

```

Locust's web:  [http://127.0.0.1:8089](http://127.0.0.1:8089)

## Troubleshooting
1. If you get the following error when starting the api-server, check to make sure you have the database setup correctly.
`ERROR:r2d2: Error opening a connection: Error communicating with the server: Connection refused (os error 111)`

2. If Postgres dies when you run `systemctl start postgresql` with an error message that
   says `WARNING: out of shared memory`, edit the `postgresql.conf` file in
   `/etc/etc/postgresql/10.1/main/postgresql.conf` and add
   `max_locks_per_transaction=128` to it.

3. If you receive the following error after installing openssl 1.1 from source
   `rioos-apiserver: error while loading shared libraries: libssl.so.1.1: cannot open shared object file: No such file or directory`
   then add the following

```
echo $LD_LIBRARY_PATH
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib
```

To make the above *LD_LIBRARY_PATH* permanent add `export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:/usr/local/lib` in `~/.bashrc`
