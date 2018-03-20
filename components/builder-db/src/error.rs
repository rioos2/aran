// Copyright 2018 The Rio Advancement Inc
use std::io;
use std::error;
use std::fmt;
use std::result;

use r2d2;
use postgres;
use rcore;
use jwt;
use serde_json;
use serde_yaml;

#[derive(Debug)]
pub enum Error {
    AsyncListen(postgres::error::Error),
    AsyncNotification(postgres::error::Error),
    AsyncMalformedChannel(String),
    AsyncMalformedShardId(String),
    AsyncFunctionCheck(postgres::error::Error),
    AsyncFunctionUpdate(postgres::error::Error),
    RecordsNotFound,
    ConnectionTimeout(r2d2::Error),
    FunctionCreate(postgres::error::Error),
    FunctionDrop(postgres::error::Error),
    FunctionRun(postgres::error::Error),
    PostgresConnect(postgres::error::Error),
    SchemaCreate(String),
    SchemaDrop(postgres::error::Error),
    SchemaSwitch(postgres::error::Error),
    SetSearchPath(postgres::error::Error),
    TransactionCreate(postgres::error::Error),
    TransactionCommit(postgres::error::Error),
    DbTransaction(postgres::error::Error),
    DbTransactionStart(postgres::error::Error),
    DbTransactionCommit(postgres::error::Error),
    Migration(postgres::error::Error),
    MigrationCheck(postgres::error::Error),
    MigrationTable(postgres::error::Error),
    MigrationTracking(postgres::error::Error),
    MigrationLock(postgres::error::Error),
    IO(io::Error),
    RioosAranCore(rcore::Error),
    Jwt(jwt::errors::Error),
    Json(serde_json::Error),
    Yaml(serde_yaml::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::AsyncListen(ref e) => format!("Error setting up async listen, {}", e),
            Error::AsyncNotification(ref e) => format!("Error getting async notification, {}", e),
            Error::AsyncMalformedChannel(ref e) => format!("Notification received, but the channel is malformed, {}", e),
            Error::AsyncMalformedShardId(ref e) => {
                format!(
                    "Notification received, but the channels shard id is malformed, {}",
                    e
                )
            }
            Error::AsyncFunctionCheck(ref e) => format!("Async function database check failed, {}", e),
            Error::AsyncFunctionUpdate(ref e) => format!("Async function database update failed, {}", e),
            Error::RecordsNotFound => format!("No Record Found"),
            Error::ConnectionTimeout(ref e) => format!("Connection timeout, {}", e),
            Error::FunctionCreate(ref e) => format!("Error creating a function: {}", e),
            Error::FunctionDrop(ref e) => format!("Error dropping a function: {}", e),
            Error::FunctionRun(ref e) => format!("Error running a function: {}", e),
            Error::PostgresConnect(ref e) => format!("Postgres connection error: {}", e),
            Error::SchemaCreate(ref e) => format!("Error creating schema: {}", e),
            Error::SchemaDrop(ref e) => format!("Error dropping schema: {}", e),
            Error::SchemaSwitch(ref e) => format!("Error switching schema: {}", e),
            Error::SetSearchPath(ref e) => format!("Error setting local search path: {}", e),
            Error::TransactionCreate(ref e) => format!("Error creating transaction: {}", e),
            Error::TransactionCommit(ref e) => format!("Error committing transaction: {}", e),
            Error::DbTransaction(ref e) => format!("Database transaction error, {}", e),
            Error::DbTransactionStart(ref e) => format!("Failed to start database transaction, {}", e),
            Error::DbTransactionCommit(ref e) => format!("Failed to commit database transaction, {}", e),
            Error::Migration(ref e) => format!("Error executing migration: {}", e),
            Error::MigrationCheck(ref e) => format!("Error checking if a migration has run: {}", e),
            Error::MigrationTable(ref e) => format!("Error creating migration tracking table: {}", e),
            Error::MigrationTracking(ref e) => format!("Error updating migration tracking table: {}", e),
            Error::MigrationLock(ref e) => format!("Error getting migration lock: {}", e),
            Error::IO(ref e) => format!("{}", e),
            Error::RioosAranCore(ref e) => format!("{}", e),
            Error::Jwt(ref e) => format!("{}", e),
            Error::Json(ref e) => format!("{}", e),
            Error::Yaml(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::AsyncListen(ref e) => e.description(),
            Error::AsyncNotification(ref e) => e.description(),
            Error::AsyncMalformedChannel(_) => "Error parsing a channel string",
            Error::AsyncMalformedShardId(_) => "Error parsing a channel strings shard id",
            Error::AsyncFunctionCheck(ref e) => e.description(),
            Error::AsyncFunctionUpdate(ref e) => e.description(),
            Error::RecordsNotFound => "RecordsNotFound",
            Error::ConnectionTimeout(ref e) => e.description(),
            Error::FunctionCreate(_) => "Error creating database function",
            Error::FunctionDrop(_) => "Error dropping database function",
            Error::FunctionRun(_) => "Error running a database function",
            Error::PostgresConnect(ref e) => e.description(),
            Error::SchemaCreate(_) => "Error creating a schema",
            Error::SchemaDrop(_) => "Error dropping a schema",
            Error::SchemaSwitch(_) => "Error switching schema",
            Error::SetSearchPath(_) => "Error setting local search path",
            Error::TransactionCreate(_) => "Error creating a transaction",
            Error::TransactionCommit(_) => "Error committing a transaction",
            Error::DbTransaction(ref err) => err.description(),
            Error::DbTransactionCommit(ref err) => err.description(),
            Error::DbTransactionStart(ref err) => err.description(),
            Error::Migration(_) => "Error executing migration",
            Error::MigrationCheck(_) => "Error checking if a migration has run",
            Error::MigrationTable(_) => "Error creat2ing migration tracking table",
            Error::MigrationTracking(_) => "Error updating migration tracking table",
            Error::MigrationLock(_) => "Error getting migration lock",
            Error::IO(ref err) => err.description(),
            Error::RioosAranCore(ref err) => err.description(),
            Error::Jwt(ref err) => err.description(),
            Error::Json(ref err) => err.description(),
            Error::Yaml(ref err) => err.description(),
        }
    }
}

impl From<r2d2::Error> for Error {
    fn from(err: r2d2::Error) -> Self {
        Error::ConnectionTimeout(err)
    }
}

impl From<postgres::error::Error> for Error {
    fn from(err: postgres::error::Error) -> Self {
        Error::PostgresConnect(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<rcore::Error> for Error {
    fn from(err: rcore::Error) -> Error {
        Error::RioosAranCore(err)
    }
}

impl From<jwt::errors::Error> for Error {
    fn from(err: jwt::errors::Error) -> Error {
        Error::Jwt(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error::Json(err)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(err: serde_yaml::Error) -> Error {
        Error::Yaml(err)
    }
}
