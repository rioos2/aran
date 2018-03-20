// Copyright 2018 The Rio Advancement Inc

//! A module containing the middleware of the HTTP server

use error::Result;

use ldap3::{LdapConn, Scope, SearchEntry};
use protocol::api::session;
use rand;

#[derive(Clone)]
pub struct LDAPClient {
    config: session::LdapConfig,
}

#[derive(Debug)]
pub struct LDAPUser {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
}

impl LDAPClient {
    pub fn new(config: session::LdapConfig) -> Self {
        LDAPClient { config: config }
    }

    pub fn connection(&self) -> Result<(LdapConn)> {
        let ldap = LdapConn::new(&self.config.get_host())?;
        ldap.simple_bind(
            &self.config.get_lookup_dn(),
            &self.config.get_lookup_password(),
        )?;
        Ok(ldap)
    }

    pub fn search(&self) -> Result<(Vec<LDAPUser>)> {
        let ldap_connection = self.connection();
        match ldap_connection {
            Ok(ldap) => {
                let (rs, _res) = ldap.search(
                    &self.config.get_user_search().get_search_base(),
                    Scope::Subtree,
                    "(&(objectClass=*))",
                    vec!["*"],
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
        if let Some(givennames) = self.attrs.get("givenName") {
            user.first_name = givennames
                .iter()
                .next()
                .unwrap_or(&"none".to_string())
                .to_string();
        }
        if let Some(emails) = self.attrs.get("mail") {
            user.email = emails
                .iter()
                .next()
                .unwrap_or(&"none".to_string())
                .to_string();
        }
        user
    }
}

impl Into<session::SessionCreate> for LDAPUser {
    fn into(self) -> session::SessionCreate {
        let mut session = session::SessionCreate::new();
        session.set_email(self.email.to_owned());
        if session.get_email().is_empty() {
            session.set_email(self.first_name.to_owned());
        }
        session.set_last_name(self.last_name.to_owned());
        session.set_apikey(rand::random::<u64>().to_string());
        session
    }
}
