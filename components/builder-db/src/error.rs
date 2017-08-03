// Copyright (c) 2017 RioCorp Inc.

use hab_net;

use std::error;
use std::fmt;
use std::result;

use r2d2;
use postgres;

#[derive(Debug)]
pub enum Error {
    AsyncListen(postgres::error::Error),
    AsyncNotification(postgres::error::Error),
    AsyncMalformedChannel(String),
    AsyncMalformedShardId(String),
    AsyncFunctionCheck(postgres::error::Error),
    AsyncFunctionUpdate(postgres::error::Error),
    ConnectionTimeout(r2d2::GetTimeout),
    FunctionCreate(postgres::error::Error),
    FunctionDrop(postgres::error::Error),
    FunctionRun(postgres::error::Error),
    NetError(hab_net::Error),
    PostgresConnect(postgres::error::ConnectError),
    SchemaCreate(postgres::error::Error),
    SchemaDrop(postgres::error::Error),
    SchemaSwitch(postgres::error::Error),
    SetSearchPath(postgres::error::Error),
    TransactionCreate(postgres::error::Error),
    TransactionCommit(postgres::error::Error),
    DbPoolTimeout(r2d2::GetTimeout),
    DbTransaction(postgres::error::Error),
    DbTransactionStart(postgres::error::Error),
    DbTransactionCommit(postgres::error::Error),
    Migration(postgres::error::Error),
    MigrationCheck(postgres::error::Error),
    MigrationTable(postgres::error::Error),
    MigrationTracking(postgres::error::Error),
    MigrationLock(postgres::error::Error),
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
            Error::ConnectionTimeout(ref e) => format!("Connection timeout, {}", e),
            Error::FunctionCreate(ref e) => format!("Error creating a function: {}", e),
            Error::FunctionDrop(ref e) => format!("Error dropping a function: {}", e),
            Error::FunctionRun(ref e) => format!("Error running a function: {}", e),
            Error::NetError(ref e) => format!("{}", e),
            Error::PostgresConnect(ref e) => format!("Postgres connection error: {}", e),
            Error::SchemaCreate(ref e) => format!("Error creating schema: {}", e),
            Error::SchemaDrop(ref e) => format!("Error dropping schema: {}", e),
            Error::SchemaSwitch(ref e) => format!("Error switching schema: {}", e),
            Error::SetSearchPath(ref e) => format!("Error setting local search path: {}", e),
            Error::TransactionCreate(ref e) => format!("Error creating transaction: {}", e),
            Error::TransactionCommit(ref e) => format!("Error committing transaction: {}", e),
            Error::DbPoolTimeout(ref e) => format!("Timeout getting connection from the database pool, {}", e),
            Error::DbTransaction(ref e) => format!("Database transaction error, {}", e),
            Error::DbTransactionStart(ref e) => format!("Failed to start database transaction, {}", e),
            Error::DbTransactionCommit(ref e) => format!("Failed to commit database transaction, {}", e),
            Error::Migration(ref e) => format!("Error executing migration: {}", e),
            Error::MigrationCheck(ref e) => format!("Error checking if a migration has run: {}", e),
            Error::MigrationTable(ref e) => format!("Error creating migration tracking table: {}", e),
            Error::MigrationTracking(ref e) => format!("Error updating migration tracking table: {}", e),
            Error::MigrationLock(ref e) => format!("Error getting migration lock: {}", e),
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
            Error::ConnectionTimeout(ref e) => e.description(),
            Error::FunctionCreate(_) => "Error creating database function",
            Error::FunctionDrop(_) => "Error dropping database function",
            Error::FunctionRun(_) => "Error running a database function",
            Error::NetError(ref err) => err.description(),
            Error::PostgresConnect(ref e) => e.description(),
            Error::SchemaCreate(_) => "Error creating a schema",
            Error::SchemaDrop(_) => "Error dropping a schema",
            Error::SchemaSwitch(_) => "Error switching schema",
            Error::SetSearchPath(_) => "Error setting local search path",
            Error::TransactionCreate(_) => "Error creating a transaction",
            Error::TransactionCommit(_) => "Error committing a transaction",
            Error::DbPoolTimeout(ref err) => err.description(),
            Error::DbTransaction(ref err) => err.description(),
            Error::DbTransactionCommit(ref err) => err.description(),
            Error::DbTransactionStart(ref err) => err.description(),
            Error::Migration(_) => "Error executing migration",
            Error::MigrationCheck(_) => "Error checking if a migration has run",
            Error::MigrationTable(_) => "Error creat2ing migration tracking table",
            Error::MigrationTracking(_) => "Error updating migration tracking table",
            Error::MigrationLock(_) => "Error getting migration lock",
        }
    }
}

impl From<hab_net::Error> for Error {
    fn from(err: hab_net::Error) -> Self {
        Error::NetError(err)
    }
}

impl From<r2d2::GetTimeout> for Error {
    fn from(err: r2d2::GetTimeout) -> Self {
        Error::ConnectionTimeout(err)
    }
}

impl From<postgres::error::ConnectError> for Error {
    fn from(err: postgres::error::ConnectError) -> Self {
        Error::PostgresConnect(err)
    }
}
