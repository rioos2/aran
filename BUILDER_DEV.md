# Rio/OS Aran API Development Environment

[Aran] is a code name for the API server and it means ``

## Overview

This document outlines the steps to start and run a Rio/OS Aran API environment for development. The environment includes the database services.

## Pre-Reqs

1. Use a Linux OS - either Ubuntu or ArchLinux.
2. Clone the aran repo to your local filesystem.
3. The sample commands below use the `curl` (or) `rioos` CLI tool.


## Do's & Don'ts for Developers

**Recommendation to developers who work on code.**

### 1. Setup Git hooks

After you clone  `aran.git`, please do the following.

```
	cp .hooks/* ./.git/hooks
    chmod 755 ./.git/hooks/pre-commit

```

### 2. Install an editor (atom.io/vscode)

### 3. Document code submitted - [example](https://gitlab.com/rioos/aran/blob/2-0-stable/components/builder-deployment/src/stacks.rs)

### 4. Make sure any needed debug is logged using `info!..` macros

### 5. Remove `println!` when you commit.

***

The below section describes setting up a development environment. 


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

All the 3 statements

```
drop database rioosdb; create database rioosdb; grant all privileges on database  rioosdb to rioos;
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

Some capabilities (such as configuring database, turning on features.


```
cp  ./tools/config/api.toml $RIOOS_HOME/config

```
### Procfile *optional*

Now, modify the `Procfile` (located in your aran repo in the `support` folder) to point the api, and worker services to the previously created config files, e.g.

```
api: target/debug/rioos-api-server  --config /home/your_alias/rioos/api.toml
```
# Managing migrations for Rio/OS

All builder migrations are run with [Diesel](http://diesel.rs). This document describes how to create and manage those migrations.

## Install the Diesel client

```
cargo install diesel_cli --no-default-features --features postgres

```

## Generating new migrations

Every time you need to make a change to the Rio/OS schema you will be required to generate a new migration

For the service `rioos-apiserver` you will need to run:

* `cd components/builder-db/api`
* `diesel migration generate <your migration name>` --database-url postgres://rioos:rioosd@localhost/rioodb

The migration name should describe what you are doing. Ex:

* create_auth_providers
* add_user_select_v4

This will generate something like

```
Creating migrations/2018-06-08-123529_create_auth_providers/up.sql
Creating migrations/2018-06-08-123529_create_auth_providers/down.sql
```

Watch this [asciicast](https://asciinema.org/a/rW6ypal1wTEjZ9ONbXU8fooKR)

You can then edit `up.sql` to create your migration steps.

Recommed to use the [SQLFormatter here](https://www.freeformatter.com/sql-formatter.html#ad-output) to format the `.sql` files. Other formatters tend to disrupt your sql.

You should ignore, but not delete, `down.sql` as we don't use it since we rely on transactions for our rollback logic.

## Testing your changes

You will need to compile your service and `rioos-api-server migrate` to test your changes. You should see:

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

## Builds for release - used during production.

For release builds append `r` to the above targets

```
make rbuildapi

```

## Clean build, cleans everythings and does a build.

```
make clean
```

## Testing

This should show the status of the api server.

You should see a response similar to the following:

* Ping

`curl  https://localhost:7443/api/v1/ping --insecure

```json
{
   "master":[
      {
         "name":"API Server",
         "status":"up",
         "description":"Service is operating normally"
      },
      {
         "name":"Postgres",
         "status":"up",
         "description":"Service is operating normally"
      },
      {
         "name":"Controller",
         "status":"up",
         "description":"Service is operating normally"
      },
      {
         "name":"Scheduler",
         "status":"up",
         "description":"Service is operating normally"
      },
      {
         "name":"Blockchain",
         "status":"up",
         "description":"Service is operating normally"
      },
      {
         "name":"Logs",
         "status":"down",
         "description":"Service is currently down"
      },
      {
         "name":"Telemetry",
         "status":"up",
         "description":"Service is operating normally"
      },
      {
         "name":"VNC Console",
         "status":"down",
         "description":"Service is currently down"
      },
      {
         "name":"Rio.Marketplace",
         "status":"down",
         "description":"Service is currently down"
      },
      {
         "name":"Vaults",
         "status":"down",
         "description":"Service is currently down"
      },
      {
         "name":"Anchore",
         "status":"down",
         "description":"Service is currently down"
      }
   ],
   "nodes":[
      {
         "name":"216.126.195.154",
         "status":"up",
         "description":"Service is operating normally"
      },
      {
         "name":"192.168.1.3",
         "status":"up",
         "description":"Service is operating normally"
      },
      {
         "name":"107.152.143.242",
         "status":"up",
         "description":"Service is operating normally"
      }
   ]
}

```

## To verify pfx

* The pfx password `TEAMRIOADVANCEMENT123`

```
openssl pkcs12 -info -in serving-rioos-apiserver.pfx

```

## To verify pub/key

```
openssl verify -CAfile server-ca.crt serving-rioos-apiserver.crt

```

## Run Aran

1. Open a new terminal window.
2. Export the following environment variables:

```
export RIOOS_HOME=$HOME/home

```

You must have a valid $RIOOS_HOME/config directory.



| Description | Location | Type |
|-------------|----------|------|
| Configuration for API | $RIOOS_HOME/config/api.toml | File |
| Template for generating yaml configuration (nodelet, storlet, controller, scheduler)  | $RIOOS_HOME/config/template/rioconfig.hbs | File |
| Template for generating xml in licensing check  | $RIOOS_HOME/config/template/shafer_filechk.xml | File |
| Configuration templates | $RIOOS_HOME/config/template/rioconfig.hbs | File |
| Directory to cache of pulls| $RIOOS_HOME/config/pullcache | Dir |
| License so | $RIOOS_HOME/license/ShaferFilechck.so | File |




```

./rioos_apiserver setup

./rioos_apiserver migrate

# Use the marketplaces.rioos.xyz (userid/pw)
./rioos_apiserver sync

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

FreeBSD

```
pkg install rocksdb-lite-5.11.3_1

```

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

4. If get the following error

```
Compiling migrations_macros v1.2.0
error: linking with `cc` failed: exit code: 1
  |
  = note: "cc" "-Wl,--as-needed" "-Wl,-z,noexecstack" "-m64" "-L" "/home/suganya/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.migrations_macros0.rcgu.o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.migrations_macros1.rcgu.o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.migrations_macros10.rcgu.o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.migrations_macros11.rcgu.o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.migrations_macros12.rcgu.o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.migrations_macros13.rcgu.o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.migrations_macros14.rcgu.o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.migrations_macros15.rcgu.o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.migrations_macros2.rcgu.o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.migrations_macros3.rcgu.o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.migrations_macros4.rcgu.o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.migrations_macros5.rcgu.o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.migrations_macros6.rcgu.o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.migrations_macros7.rcgu.o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.migrations_macros8.rcgu.o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.migrations_macros9.rcgu.o" "-o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/libmigrations_macros-9bb400830b2371ff.so" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.crate.metadata.rcgu.o" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/migrations_macros-9bb400830b2371ff.crate.allocator.rcgu.o" "-Wl,--gc-sections" "-Wl,-z,relro,-z,now" "-nodefaultlibs" "-L" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps" "-L" "/home/suganya/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bstatic" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/libsyn-e21de1e71d53587a.rlib" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/libsynom-ac93423e9047e7c8.rlib" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/libunicode_xid-155fd845ac784c02.rlib" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/libquote-304777cb3696ca3c.rlib" "-L" "/home/suganya/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bdynamic" "-l" "proc_macro-7d531857c463fdf1" "-L" "/home/suganya/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "syntax-a0f8084e7d2e8f23" "-L" "/home/suganya/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "rustc_errors-0442a147c1fbea3f" "-L" "/home/suganya/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "syntax_pos-a3c77f19bf623ee8" "-L" "/home/suganya/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "rustc_data_structures-6c5726675bdb8f04" "-L" "/home/suganya/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "term-74d3aea795746522" "-L" "/home/suganya/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "serialize-df58869bc5612287" "-L" "/home/suganya/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-l" "rustc_cratesio_shim-f858a3ee752a9bb9" "-Wl,-Bstatic" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/libmigrations_internals-af748bb30310c744.rlib" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/libdiesel-c6f9187c0064765c.rlib" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/libpq_sys-917183de6852f6fc.rlib" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/libbyteorder-04a4459d454f1cde.rlib" "/home/suganya/code/megam/workspace/go/src/gitlab.com/rioos/aran/target/debug/deps/libbitflags-2247395e388e5ef5.rlib" "-L" "/home/suganya/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-Wl,-Bdynamic" "-l" "std-c10c01f750e28d27" "-Wl,-Bstatic" "/home/suganya/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/lib/libcompiler_builtins-4e85b1507e729192.rlib" "-Wl,-Bdynamic" "-l" "util" "-l" "util" "-l" "pq" "-l" "dl" "-l" "rt" "-l" "pthread" "-l" "pthread" "-l" "gcc_s" "-l" "c" "-l" "m" "-l" "rt" "-l" "pthread" "-l" "util" "-l" "util" "-shared"
  = note: /usr/bin/ld: cannot find -lpq
```  
As migrations are handled by diesel-cli, install libpqdev

Ubuntu
```
sudo apt-get install libpq-dev

```
