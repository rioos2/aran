# Rio/OS Aran API Development Environment

[Aran] is a code name for the API server and it means ``

## Overview

This document outlines the steps to start and run a Rio/OS Aran API environment for development. The environment includes the database services.

## Pre-Reqs
1. Use a Linux OS - either Ubuntu or ArchLinux.
1. Clone the aran repo to your local filesystem.
1. The sample commands below use the 'httpie' tool. Install it if not present on your system (https://github.com/jkbrzt/httpie) but `curl` is ok tool

## PostgreSQL

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

local   all            all      peer  to local  all      all          md5
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

Create the following files somewhere on your local filesystem (Note: the client_id and client_secret below are for development purposes only):

`/var/lib/rioos/api.toml`
```toml
[depot]
builds_enabled = true

[github]
url = "https://api.github.com"
client_id = "0c2f738a7d0bd300de10"
client_secret = "438223113eeb6e7edf2d2f91a232b72de72b9bdf"
```
(Note: If you want your log files to persist across restarts of your development machine, replace `/tmp` with some other directory. It *must* exist and be writable before you start the job server).

Now, modify the `Procfile` (located in your hab repo in the `support` folder) to point the api, sessionsrv, jobsrv, and worker services to the previously created config files, e.g.

```
api: target/debug/rioos-api-server  --config /home/your_alias/rioos/api.toml
```

## Run Aran
1. Open a new terminal window.
1. Export the following environment variables:

```
export RIOOS_HOME=$HOME/home
```

```

```

### In the browser

http://localhost:3000/#/pkgs, sign in with GitHub, and click My Origins in the sidebar to create an origin.


## Run a build

Incremental build  - used during development.

```
make
```

Clean build, cleans everythings and does a build.
```
make clean
```

## Testing

This should create a build job, and then dispatch it to the build worker.

You should see a response similar to the following:

* Healthz
`curl  http://localhost:9636/v1/healthz
`

```
HTTP/1.1 201 Created

{
    "name": "aran",
    "version": "2.0.dev"
    "state": "alliswell"
}
```


## Troubleshooting
1. If you get the following error when starting the api-server, check to make sure you have the database setup correctly.
`ERROR:habitat_builder_worker::runner: Unable to retrieve secret key, err=[404 Not Found]`

1. If you get a build failing with a `401 Unauthorized`, make sure the builder worker is pointed to a valid Github token (via a config.toml in the Procfile)


1. If Postgres dies when you run `systemctl start postgresql` with an error message that
   says `WARNING: out of shared memory`, edit the `postgresql.conf` file in
   `/etc/etc/postgresql/9.5/main/postgresql.conf` and add
   `max_locks_per_transaction=128` to it.
