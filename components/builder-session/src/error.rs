// Copyright 2018 The Rio Advancement Inc

//! A module containing the errors handling for the builder session

use std::error;
use std::fmt;
use std::io;
use std::result;
use std::num;
use postgres;
use db;

#[derive(Debug)]
pub enum Error {
    Db(db::error::Error),
    AccountIdFromString(num::ParseIntError),
    AccountCreate(postgres::error::Error),
    AccountGet(postgres::error::Error),
    AccountGetById(postgres::error::Error),
    SessionGet(postgres::error::Error),
    AccountOriginInvitationCreate(postgres::error::Error),
    AccountOriginInvitationList(postgres::error::Error),
    AccountOriginInvitationAccept(postgres::error::Error),
    OriginAccountList(postgres::error::Error),
    OriginCreate(postgres::error::Error),
    TeamCreate(postgres::error::Error),
    OriginGetResponse(postgres::error::Error),
    OriginGet(postgres::error::Error),
    LdapConfigCreate(postgres::error::Error),
    IO(io::Error),
    SamlProviderCreate(postgres::error::Error),
    OidcProviderCreate(postgres::error::Error),
    SamlProviderGetResponse(postgres::error::Error),
    SamlProviderGet(postgres::error::Error),
    OpenidProviderGetResponse(postgres::error::Error),
    OidcProviderGet(postgres::error::Error),
    PassTicketGet(postgres::error::Error),
    PassTicketDelete(postgres::error::Error),
    PassTicketCreate(postgres::error::Error),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Db(ref e) => format!("{}", e),
            Error::AccountIdFromString(ref e) => format!("Cannot convert from string to Account ID, {}", e),
            Error::AccountCreate(ref e) => format!("Error creating account in database, {}", e),
            Error::AccountGet(ref e) => format!("Error getting account from database, {}", e),
            Error::AccountGetById(ref e) => format!("Error getting account from database, {}", e),
            Error::SessionGet(ref e) => format!("Error getting session from database, {}", e),
            Error::AccountOriginInvitationCreate(ref e) => format!("Error creating invitation in database, {}", e),
            Error::AccountOriginInvitationList(ref e) => format!("Error listing invitation in database, {}", e),
            Error::AccountOriginInvitationAccept(ref e) => format!("Error accepting invitation in database, {}", e),
            Error::OriginAccountList(ref e) => format!("Error listing origins for account in database, {}", e),
            Error::OriginCreate(ref e) => format!("Error creating origin for account in database, {}", e),
            Error::TeamCreate(ref e) => format!("Error creating team for origin in database, {}", e),
            Error::OriginGetResponse(ref e) => format!("Error retrive origin for account in database, {}", e),
            Error::OriginGet(ref e) => format!("Error retrive origin by name, {}", e),
            Error::LdapConfigCreate(ref e) => format!("Error creating ldap config, {}", e),
            Error::IO(ref e) => format!("{}", e),
            Error::SamlProviderCreate(ref e) => format!("Error creating saml provider, {}", e),
            Error::OidcProviderCreate(ref e) => format!("Error creating open id provider, {}", e),
            Error::SamlProviderGetResponse(ref e) => format!("Error get all saml provider list, {}", e),
            Error::SamlProviderGet(ref e) => format!("Error get saml provider data, {}", e),
            Error::OpenidProviderGetResponse(ref e) => format!("Error get all open id  provider data, {}", e),
            Error::OidcProviderGet(ref e) => format!("Error get openid  provider data, {}", e),
            Error::PassTicketGet(ref e) => format!("Error get otp data, {}", e),
            Error::PassTicketDelete(ref e) => format!("Error removing otp data, {}", e),
            Error::PassTicketCreate(ref e) => format!("Error creating otp data, {}", e),
        };
        write!(f, "{}", msg)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Db(ref err) => err.description(),
            Error::AccountIdFromString(ref err) => err.description(),
            Error::AccountCreate(ref err) => err.description(),
            Error::AccountGet(ref err) => err.description(),
            Error::AccountGetById(ref err) => err.description(),
            Error::SessionGet(ref err) => err.description(),
            Error::AccountOriginInvitationCreate(ref err) => err.description(),
            Error::AccountOriginInvitationList(ref err) => err.description(),
            Error::AccountOriginInvitationAccept(ref err) => err.description(),
            Error::OriginAccountList(ref err) => err.description(),
            Error::OriginCreate(ref err) => err.description(),
            Error::TeamCreate(ref err) => err.description(),
            Error::OriginGetResponse(ref err) => err.description(),
            Error::OriginGet(ref err) => err.description(),
            Error::LdapConfigCreate(ref err) => err.description(),
            Error::IO(ref err) => err.description(),
            Error::SamlProviderCreate(ref err) => err.description(),
            Error::OidcProviderCreate(ref err) => err.description(),
            Error::SamlProviderGetResponse(ref err) => err.description(),
            Error::SamlProviderGet(ref err) => err.description(),
            Error::OpenidProviderGetResponse(ref err) => err.description(),
            Error::OidcProviderGet(ref err) => err.description(),
            Error::PassTicketGet(ref err) => err.description(),
            Error::PassTicketDelete(ref err) => err.description(),
            Error::PassTicketCreate(ref err) => err.description(),

        }
    }
}

impl From<db::error::Error> for Error {
    fn from(err: db::error::Error) -> Self {
        Error::Db(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Self {
        Error::AccountIdFromString(err)
    }
}
