// Copyright (c) 2017 RioCorp Inc.

//! A module containing the middleware of the HTTP server

use error::{self, Result};
use std::error::Error as StdError;
use std::result::Result as StdResult;

use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;

use protocol::net;
use serde_json;
use ldap3::{LdapConn, Scope, SearchEntry};
use protocol::sessionsrv;

#[derive(Clone)]
pub struct LDAPClient {
    config: sessionsrv::LdapConfig,
}

pub struct LDAPUser {
    email: String,
    name: String,
    phone: String,
}

impl LDAPClient {
    pub fn new(config: sessionsrv::LdapConfig) -> Self {
        LDAPClient { config: config }
    }

    pub fn connection(&self) -> Result<(LdapConn)> {
        let ldap = LdapConn::new(&self.config.get_host())?;
        let (rs, _res) = ldap.search(
            &self.config.get_lookup_dn(),
            Scope::Subtree,
            "(&(objectClass=*))",
            vec![""],
        )?
            .success()?;
        Ok(ldap)
    }

    pub fn search(&self) -> Result<(Vec<LDAPUser>)> {
        let ldap_connection = self.connection();
        match ldap_connection {
            Ok(ldap) => {
                let (rs, _res) = ldap.search(
                    &self.config.get_lookup_dn(),
                    Scope::Subtree,
                    "(&(objectClass=*))",
                    vec![""],
                )?
                    .success()?;
                //build entries as LDAPUser struct\
                let mut user_collection = vec![];
                for entry in rs {
                    println!("{:?}", SearchEntry::construct(entry));
                    // user_collection.push(LDAPUser {});

                }
                Ok(user_collection)
            }
            Err(err) => Err(err),
        }

    }

    fn close() {
        //close ldap connection
    }
}
