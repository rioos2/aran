// Copyright (c) 2017 RioCorp Inc.

//! A module containing the middleware of the HTTP server

use super::super::error::{self, Result};
use std::error::Error as StdError;
use std::result::Result as StdResult;

use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;

use protocol::net;
use serde_json;


#[derive(Clone)]
pub struct LDAPClient {
    pub url: String,
}

impl LDAPClient {
    pub fn new(config: &T) -> Self
    where
        T: config::LDAPConfiguration,
    {
        LDAPClient { config: config }
    }

    fn connection() -> Result<(LdapConn)> {
        let ldap = LdapConn::new(&host)?;
        let (rs, _res) = ldap.search(&lookup_dn, Scope::Subtree, "(&(objectClass=*))", vec![""])?
            .success()?;
        Ok(ldap)
    }

    fn search() -> Result<(Vec<LDAPUser>)> {
        let ldap_connection =  connection();
        match ldap_connection {
            Ok(ldap) =>  {
                let (rs, _res) = ldap.search(&lookup_dn, Scope::Subtree, "(&(objectClass=*))", vec![""])?
                    .success()?;
                    //build entries as LDAPUser struct
                    //for entry in rs {
                        println!("{:?}", SearchEntry::construct(entry));
                    //}
            }
            Err(err) =>
        }

    }

    fn close () {
        //close ldap connection
    }

}
