// Copyright (c) 2017 RioCorp Inc.
use serde::{Serialize, Serializer};
use std::result;

#[derive(Debug, PartialEq, Clone, Default, Serialize)]
pub struct SessionCreate {
    id: String,
    name: String,
    email: String,
    first_name: String,
    last_name: String,
    phone: String,
    provider: ::std::option::Option<OAuthProvider>,
    api_key: String,
    token: String,
    password: String,
    states: String,
    approval: String,
    suspend: String,
    registration_ip_address: String,
    created_at: String,
}

impl SessionCreate {
    pub fn new() -> SessionCreate {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }


    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    pub fn get_email(&self) -> ::std::string::String {
        self.email.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_provider(&mut self, v: OAuthProvider) {
        self.provider = ::std::option::Option::Some(v);
    }

    pub fn get_provider(&self) -> OAuthProvider {
        self.provider.clone().unwrap_or(OAuthProvider::PasswordAuth)
    }

    pub fn set_first_name(&mut self, v: ::std::string::String) {
        self.first_name = v;
    }

    pub fn get_first_name(&self) -> ::std::string::String {
        self.first_name.clone()
    }

    pub fn set_last_name(&mut self, v: ::std::string::String) {
        self.last_name = v;
    }

    pub fn get_last_name(&self) -> ::std::string::String {
        self.last_name.clone()
    }

    pub fn set_phone(&mut self, v: ::std::string::String) {
        self.phone = v;
    }

    pub fn get_phone(&self) -> ::std::string::String {
        self.phone.clone()
    }

    pub fn set_apikey(&mut self, v: ::std::string::String) {
        self.api_key = v;
    }

    pub fn get_apikey(&self) -> ::std::string::String {
        self.api_key.clone()
    }

    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = v;
    }

    pub fn get_token(&self) -> ::std::string::String {
        self.token.clone()
    }

    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    pub fn get_password(&self) -> ::std::string::String {
        self.password.clone()
    }

    pub fn set_states(&mut self, v: ::std::string::String) {
        self.states = v;
    }

    pub fn get_states(&self) -> ::std::string::String {
        self.states.clone()
    }

    pub fn set_approval(&mut self, v: ::std::string::String) {
        self.approval = v;
    }

    pub fn get_approval(&self) -> ::std::string::String {
        self.approval.clone()
    }

    pub fn set_suspend(&mut self, v: ::std::string::String) {
        self.suspend = v;
    }

    pub fn get_suspend(&self) -> ::std::string::String {
        self.suspend.clone()
    }

    pub fn set_registration_ip_address(&mut self, v: ::std::string::String) {
        self.registration_ip_address = v;
    }

    pub fn get_registration_ip_address(&self) -> ::std::string::String {
        self.registration_ip_address.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

impl Into<Session> for SessionCreate {
    fn into(self) -> Session {
        let mut session = Session::new();
        session.set_id(self.get_id());
        session.set_email(self.get_email().to_owned());
        session.set_name(self.get_name().to_owned());
        session.set_token(self.get_token().to_owned());
        session.set_apikey(self.get_apikey().to_owned());
        session
    }
}

#[derive(PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Session {
    id: String,
    email: String,
    name: String,
    token: String,
    api_key: String,
    flags: u32,
}

impl Session {
    pub fn new() -> Session {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    pub fn get_email(&self) -> ::std::string::String {
        self.email.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_apikey(&mut self, v: ::std::string::String) {
        self.api_key = v;
    }

    pub fn get_apikey(&self) -> ::std::string::String {
        self.api_key.clone()
    }


    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = v;
    }

    pub fn get_token(&self) -> ::std::string::String {
        self.token.clone()
    }

    pub fn set_flags(&mut self, v: u32) {
        self.flags = v;
    }
    pub fn get_flags(&self) -> u32 {
        self.flags
    }
}

#[derive(PartialEq, Clone, Default)]
pub struct SessionGet {
    name: String,
    email: String,
    token: String,
    api_key: String,
}


impl SessionGet {
    pub fn new() -> SessionGet {
        ::std::default::Default::default()
    }

    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    pub fn get_email(&self) -> ::std::string::String {
        self.email.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_apikey(&mut self, v: ::std::string::String) {
        self.api_key = v;
    }

    pub fn get_apikey(&self) -> ::std::string::String {
        self.api_key.clone()
    }

    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = v;
    }

    pub fn get_token(&self) -> ::std::string::String {
        self.token.clone()
    }
}

#[derive(PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AccountGet {
    name: String,
    email: String,
    password: String,
}

impl AccountGet {
    pub fn new() -> AccountGet {
        ::std::default::Default::default()
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_email(&self) -> ::std::string::String {
        self.email.clone()
    }

    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    pub fn get_password(&self) -> ::std::string::String {
        self.password.clone()
    }

    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AccountGetId {
    id: String,
}


impl AccountGetId {
    pub fn new() -> AccountGetId {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum OAuthProvider {
    PasswordAuth = 0,
    LDAP = 1,
    OpenID = 2,
}

impl Serialize for OAuthProvider {
    fn serialize<S>(&self, serializer: S) -> result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u64(self.to_owned() as u64)
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Account {
    id: String,
    name: String,
    email: String,
    first_name: String,
    last_name: String,
    phone: String,
    token: String,
    api_key: String,
    password: String,
    states: String,
    approval: String,
    suspend: String,
    registration_ip_address: String,
    created_at: String,
}

impl Into<Session> for Account {
    fn into(self) -> Session {
        let mut session = Session::new();
        session.set_id(self.get_id());
        session.set_email(self.get_email().to_owned());
        session.set_name(self.get_name().to_owned());
        session.set_apikey(self.get_apikey().to_owned());
        session
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AccountGetResponse {
    results: Vec<Account>,
}


impl AccountGetResponse {
    pub fn new() -> AccountGetResponse {
        ::std::default::Default::default()
    }

    pub fn set_assemblys(&mut self, v: Vec<Account>) {
        self.results = v;
    }
}


impl Account {
    pub fn new() -> Account {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    pub fn get_email(&self) -> ::std::string::String {
        self.email.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_first_name(&mut self, v: ::std::string::String) {
        self.first_name = v;
    }

    pub fn get_first_name(&self) -> ::std::string::String {
        self.first_name.clone()
    }

    pub fn set_last_name(&mut self, v: ::std::string::String) {
        self.last_name = v;
    }

    pub fn get_last_name(&self) -> ::std::string::String {
        self.last_name.clone()
    }

    pub fn set_apikey(&mut self, v: ::std::string::String) {
        self.api_key = v;
    }

    pub fn get_apikey(&self) -> ::std::string::String {
        self.api_key.clone()
    }

    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = v;
    }

    pub fn get_token(&self) -> ::std::string::String {
        self.token.clone()
    }

    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    pub fn get_password(&self) -> ::std::string::String {
        self.password.clone()
    }

    pub fn set_states(&mut self, v: ::std::string::String) {
        self.states = v;
    }

    pub fn get_states(&self) -> ::std::string::String {
        self.states.clone()
    }

    pub fn set_approval(&mut self, v: ::std::string::String) {
        self.approval = v;
    }

    pub fn get_approval(&self) -> ::std::string::String {
        self.approval.clone()
    }

    pub fn set_suspend(&mut self, v: ::std::string::String) {
        self.suspend = v;
    }

    pub fn get_suspend(&self) -> ::std::string::String {
        self.suspend.clone()
    }

    pub fn set_registration_ip_address(&mut self, v: ::std::string::String) {
        self.registration_ip_address = v;
    }

    pub fn get_registration_ip_address(&self) -> ::std::string::String {
        self.registration_ip_address.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct LdapConfig {
    host: String,
    port: String,
    enforce_starttls: String,
    lookup_dn: String,
    lookup_password: String,
    user_search: UserSearch,
    group_search: GroupSearch,
    ca_certs: String,
    client_cert: String,
}

impl LdapConfig {
    pub fn new() -> LdapConfig {
        ::std::default::Default::default()
    }

    pub fn set_host(&mut self, v: ::std::string::String) {
        self.host = v;
    }

    pub fn get_host(&self) -> ::std::string::String {
        self.host.clone()
    }

    pub fn set_port(&mut self, v: ::std::string::String) {
        self.port = v;
    }

    pub fn get_port(&self) -> ::std::string::String {
        self.port.clone()
    }

    pub fn set_enforce_starttls(&mut self, v: ::std::string::String) {
        self.enforce_starttls = v;
    }

    pub fn get_enforce_starttls(&self) -> ::std::string::String {
        self.enforce_starttls.clone()
    }

    pub fn set_lookup_dn(&mut self, v: ::std::string::String) {
        self.lookup_dn = v;
    }

    pub fn get_lookup_dn(&self) -> ::std::string::String {
        self.lookup_dn.clone()
    }

    pub fn set_lookup_password(&mut self, v: ::std::string::String) {
        self.lookup_password = v;
    }

    pub fn get_lookup_password(&self) -> ::std::string::String {
        self.lookup_password.clone()
    }

    pub fn set_ca_certs(&mut self, v: ::std::string::String) {
        self.ca_certs = v;
    }

    pub fn get_ca_certs(&self) -> ::std::string::String {
        self.ca_certs.clone()
    }

    pub fn set_client_cert(&mut self, v: ::std::string::String) {
        self.client_cert = v;
    }

    pub fn get_client_cert(&self) -> ::std::string::String {
        self.client_cert.clone()
    }

    pub fn set_user_search(&mut self, v: UserSearch) {
        self.user_search = v;
    }

    pub fn get_user_search(&self) -> &UserSearch {
        &self.user_search
    }

    pub fn set_group_search(&mut self, v: GroupSearch) {
        self.group_search = v;
    }

    pub fn get_group_search(&self) -> &GroupSearch {
        &self.group_search
    }


}


#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct UserSearch {
    search_base: String,
    search_filter_template: String,
}

impl UserSearch {
    pub fn new() -> UserSearch {
        ::std::default::Default::default()
    }

    pub fn set_search_base(&mut self, v: ::std::string::String) {
        self.search_base = v;
    }

    pub fn get_search_base(&self) -> ::std::string::String {
        self.search_base.clone()
    }

    pub fn set_search_filter_template(&mut self, v: ::std::string::String) {
        self.search_filter_template = v;
    }

    pub fn get_search_filter_template(&self) -> ::std::string::String {
        self.search_filter_template.clone()
    }
}


#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct GroupSearch {
    search_base: String,
    search_filter_template: String,
    member_attributes: Vec<String>,
}

impl GroupSearch {
    pub fn new() -> GroupSearch {
        ::std::default::Default::default()
    }

    pub fn set_search_base(&mut self, v: ::std::string::String) {
        self.search_base = v;
    }

    pub fn get_search_base(&self) -> ::std::string::String {
        self.search_base.clone()
    }

    pub fn set_search_filter_template(&mut self, v: ::std::string::String) {
        self.search_filter_template = v;
    }

    pub fn get_search_filter_template(&self) -> ::std::string::String {
        self.search_filter_template.clone()
    }

    pub fn set_member_attributes(&mut self, v: ::std::vec::Vec<String>) {
        self.member_attributes = v;
    }

    pub fn get_member_attributes(&self) -> ::std::vec::Vec<String> {
        self.member_attributes.clone()
    }
}
