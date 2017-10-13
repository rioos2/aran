// Copyright (c) 2017 RioCorp Inc.

//! A module containing the middleware of the HTTP server

use error::{self, Result};
use std::error::Error as StdError;

use ldap3::{LdapConn, Scope, SearchEntry};
use protocol::sessionsrv;

#[derive(Clone)]
pub struct LDAPClient {
    config: sessionsrv::LdapConfig,
}

#[derive(Debug)]
pub struct LDAPUser {
    email: String,
    first_name: String,
    last_name: String,
    phone: String,
}

impl LDAPClient {
    pub fn new(config: sessionsrv::LdapConfig) -> Self {
        LDAPClient { config: config }
    }

    pub fn connection(&self) -> Result<(LdapConn)> {
        let ldap = LdapConn::new(&self.config.get_host())?;
        let (_, _res) = ldap.search(
            &self.config.get_lookup_dn(),
            Scope::Subtree,
            "(&(objectClass=*))",
            vec![""],
        )?
            .success()?;
        ldap.unbind()?;

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
                let mut ldap_users = vec![];
                for entry in rs {
                    let l = SearchEntry::construct(entry).into();
                    ldap_users.push(l);
                }
                ldap.unbind()?;

                Ok(ldap_users)
            }
            Err(err) => Err(err),
        }

    }
}

impl Into<LDAPUser> for SearchEntry {
    fn into(self) -> LDAPUser {
        println!("--- searched user {:?}", self);
        let mut user = LDAPUser {
            email: "".to_string(),
            first_name: "".to_string(),
            last_name: "".to_string(),
            phone: "".to_string(),
        };
        if let Some(cns) = self.attrs.get("cn") {
            user.first_name = cns.iter().next().unwrap_or(&"none".to_string()).to_string();
        }
        if let Some(sns) = self.attrs.get("sn") {
            user.last_name = sns.iter().next().unwrap_or(&"none".to_string()).to_string();
        }
        if let Some(givennames) = self.attrs.get("givenname") {
            user.first_name = givennames.iter().next().unwrap_or(&"none".to_string()).to_string();
        }
        if let Some(emails) = self.attrs.get("mail") {
            user.email = emails.iter().next().unwrap_or(&"none".to_string()).to_string();
        }
        println!("--- converted user {:?}", user);
        user
    }
}
