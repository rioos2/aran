// Copyright 2018 The Rio Advancement Inc
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use std::result;
use api::base::{TypeMeta, ObjectMeta, MetaFields};
use iron::headers::UserAgent;
use iron::prelude::*;
use woothee::parser::{WootheeResult, Parser};
pub const DEFAULT_AGENT: &'static str = "Rio Bulldog";


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct SessionCreate {
    #[serde(default)]
    id: String, //Id an unique identifier in systems of record. Generated during creation of the session.
    #[serde(default)]
    object_meta: ObjectMeta, //Standard object metadata
    #[serde(default)]
    type_meta: TypeMeta, //Standard type metadata: kind: SesssionCreate
    email: String, //email of the user
    first_name: String, //first name of the user
    last_name: String, //last name of the user
    phone: String, //contact number of the user
    #[serde(default)]
    avatar: Option<Vec<u8>>, //Avatar picture to identity the user
    company_name: String, //Company name is where the user works.
    #[serde(default)]
    provider: ::std::option::Option<OAuthProvider>,
    #[serde(default)]
    api_key: String, //A persistenant personal access token is required to authenticate to Rio/OS  in the following situations:  1. When you don't want to login and use the ephermeal authorization tokens. This should be used with caution.
    #[serde(default)]
    token: String, //tolen for individual user
    password: String, //user password
    #[serde(default)]
    approval: bool, //approved user or not
    #[serde(default)]
    roles: Vec<String>, //Roles are Rio/OS role label that applies to the user
    #[serde(default)]
    suspend: bool, //user suspend or not   If true, the user is suspended. Defaults to false
    #[serde(default)]
    registration_ip_address: String, //Registration ip address of the user
    #[serde(default)]
    trust_level: String,
    #[serde(default)]
    created_at: String, //when session created
}

impl MetaFields for SessionCreate {
    /// Returns the latest self with built ObjectMeta and Type_meta
    /// Wipes out the old meta.
    /// Should be handled externally by doing Meta::with(by mutating the old ObjectMeta)
    fn set_meta(&mut self, t: TypeMeta, v: ObjectMeta) {
        self.type_meta = t;
        self.object_meta = v;
    }

    fn object_meta(&self) -> ObjectMeta {
        self.object_meta.clone()
    }

    fn type_meta(&self) -> TypeMeta {
        self.type_meta.clone()
    }
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

    pub fn set_approval(&mut self, v: bool) {
        self.approval = v;
    }

    pub fn get_approval(&self) -> bool {
        self.approval.clone()
    }

    pub fn set_suspend(&mut self, v: bool) {
        self.suspend = v;
    }

    pub fn get_suspend(&self) -> bool {
        self.suspend.clone()
    }

    pub fn set_roles(&mut self, v: ::std::vec::Vec<String>) {
        self.roles = v;
    }

    pub fn get_roles(&self) -> ::std::vec::Vec<String> {
        self.roles.clone()
    }

    pub fn set_registration_ip_address(&mut self, v: ::std::string::String) {
        self.registration_ip_address = v;
    }

    pub fn get_registration_ip_address(&self) -> ::std::string::String {
        self.registration_ip_address.clone()
    }

    pub fn set_trust_level(&mut self, v: ::std::string::String) {
        self.trust_level = v;
    }

    pub fn get_trust_level(&self) -> ::std::string::String {
        self.trust_level.clone()
    }

    pub fn set_company_name(&mut self, v: ::std::string::String) {
        self.company_name = v;
    }

    pub fn get_company_name(&self) -> ::std::string::String {
        self.company_name.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }

    pub fn set_avatar(&mut self, v: Option<Vec<u8>>) {
        self.avatar = v;
    }

    pub fn get_avatar(&self) -> &Option<Vec<u8>> {
        &self.avatar
    }
}

impl Into<Session> for SessionCreate {
    fn into(self) -> Session {
        let mut session = Session::new();
        session.set_id(self.get_id());
        session.set_email(self.get_email().to_owned());
        session.set_token(self.get_token().to_owned());
        session.set_apikey(self.get_apikey().to_owned());
        session
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Session {
    #[serde(default)]
    id: String, //Id an unique identifier in systems of record. Generated during creation of the session.
    email: String, //email of the user
    first_name: String, //first name of the user
    last_name: String, //last name of the user
    roles: Vec<String>, //Roles are Rio/OS role label that applies to the user
    token: String, //tolen for individual user
    api_key: String, //A persistenant personal access token is required to authenticate to Rio/OS  in the following situations:  1. When you don't want to login and use the ephermeal authorization tokens. This should be used with caution.
    flags: u32,
    #[serde(default)]
    object_meta: ObjectMeta, //Standard object metadata
    #[serde(default)]
    type_meta: TypeMeta, //Standard Type metadata
}

impl MetaFields for Session {
    /// Returns the latest self with built ObjectMeta and Type_meta
    /// Wipes out the old meta.
    /// Should be handled externally by doing Meta::with(by mutating the old ObjectMeta)
    fn set_meta(&mut self, t: TypeMeta, v: ObjectMeta) {
        self.type_meta = t;
        self.object_meta = v;
    }

    fn object_meta(&self) -> ObjectMeta {
        self.object_meta.clone()
    }

    fn type_meta(&self) -> TypeMeta {
        self.type_meta.clone()
    }
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

    pub fn set_apikey(&mut self, v: ::std::string::String) {
        self.api_key = v;
    }

    pub fn get_apikey(&self) -> ::std::string::String {
        self.api_key.clone()
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

    pub fn set_roles(&mut self, v: ::std::vec::Vec<String>) {
        self.roles = v;
    }

    pub fn get_roles(&self) -> ::std::vec::Vec<String> {
        self.roles.clone()
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

#[derive(PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct SessionGet {
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
    email: String,
    password: String,
}

impl AccountGet {
    pub fn new() -> AccountGet {
        ::std::default::Default::default()
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


#[derive(PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct AccountTokenGet {
    email: String,
    token: String,
}

impl AccountTokenGet {
    pub fn new() -> AccountTokenGet {
        ::std::default::Default::default()
    }

    pub fn get_email(&self) -> ::std::string::String {
        self.email.clone()
    }

    pub fn set_email(&mut self, v: ::std::string::String) {
        self.email = v;
    }

    pub fn get_token(&self) -> ::std::string::String {
        self.token.clone()
    }

    pub fn set_token(&mut self, v: ::std::string::String) {
        self.token = v;
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

impl<'de> Deserialize<'de> for OAuthProvider {
    fn deserialize<D>(deserializer: D) -> Result<OAuthProvider, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(match u64::deserialize(deserializer)? {
            0 => OAuthProvider::PasswordAuth,
            1 => OAuthProvider::LDAP,
            2 => OAuthProvider::OpenID,
            _ => OAuthProvider::PasswordAuth,
        })
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Account {
    #[serde(default)]
    id: String, //Id an unique identifier in systems of record. Generated during creation of the Account.
    #[serde(default)]
    object_meta: ObjectMeta, //Standard object metadata
    #[serde(default)]
    type_meta: TypeMeta, //Standard type metadata: kind: Account
    email: String, //email of the user
    first_name: String, //first name of the user
    last_name: String, //last name of the user
    phone: String, //contact number of the user
    #[serde(default)]
    avatar: Option<Vec<u8>>, //Avatar picture to identity the user
    company_name: String, //Company name is where the user works.
    #[serde(default)]
    provider: ::std::option::Option<OAuthProvider>,
    #[serde(default)]
    api_key: String, //A persistenant personal access token is required to authenticate to Rio/OS  in the following situations:  1. When you don't want to login and use the ephermeal authorization tokens. This should be used with caution.
    password: String, //user password
    #[serde(default)]
    approval: bool, //approved user or not
    #[serde(default)]
    roles: Vec<String>, //Roles are Rio/OS role label that applies to the user
    #[serde(default)]
    suspend: bool, //user suspend or not   If true, the user is suspended. Defaults to false
    registration_ip_address: String, //Registration ip address of the user
    #[serde(default)]
    trust_level: String,
    #[serde(default)]
    created_at: String, //when account created
}

impl Into<Session> for Account {
    fn into(self) -> Session {
        let mut session = Session::new();
        session.set_id(self.get_id());
        session.set_email(self.get_email().to_owned());
        session.set_apikey(self.get_apikey().to_owned());
        session.set_first_name(self.get_first_name().to_owned());
        session.set_last_name(self.get_last_name().to_owned());
        session.set_roles(self.get_roles().to_owned());
        session.set_meta(self.type_meta(), self.object_meta());
        session
    }
}

impl MetaFields for Account {
    /// Returns the latest self with built ObjectMeta and Type_meta
    /// Wipes out the old meta.
    /// Should be handled externally by doing Meta::with(by mutating the old ObjectMeta)
    fn set_meta(&mut self, t: TypeMeta, v: ObjectMeta) {
        self.type_meta = t;
        self.object_meta = v;
    }

    fn object_meta(&self) -> ObjectMeta {
        self.object_meta.clone()
    }

    fn type_meta(&self) -> TypeMeta {
        self.type_meta.clone()
    }
}

impl Account {
    pub fn new() -> Account {
        ::std::default::Default::default()
    }

    pub fn with(t: TypeMeta, o: ObjectMeta) -> Account {
        Account {
            type_meta: t,
            object_meta: o,
            ..Default::default()
        }
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

    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    pub fn get_password(&self) -> ::std::string::String {
        self.password.clone()
    }

    pub fn set_approval(&mut self, v: bool) {
        self.approval = v;
    }

    pub fn get_approval(&self) -> bool {
        self.approval.clone()
    }

    pub fn set_suspend(&mut self, v: bool) {
        self.suspend = v;
    }

    pub fn get_suspend(&self) -> bool {
        self.suspend.clone()
    }

    pub fn set_roles(&mut self, v: ::std::vec::Vec<String>) {
        self.roles = v;
    }

    pub fn get_roles(&self) -> ::std::vec::Vec<String> {
        self.roles.clone()
    }

    pub fn set_registration_ip_address(&mut self, v: ::std::string::String) {
        self.registration_ip_address = v;
    }

    pub fn get_registration_ip_address(&self) -> ::std::string::String {
        self.registration_ip_address.clone()
    }

    pub fn set_trust_level(&mut self, v: ::std::string::String) {
        self.trust_level = v;
    }

    pub fn set_company_name(&mut self, v: ::std::string::String) {
        self.company_name = v;
    }

    pub fn get_trust_level(&self) -> ::std::string::String {
        self.trust_level.clone()
    }

    pub fn get_company_name(&self) -> ::std::string::String {
        self.company_name.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }

    pub fn set_avatar(&mut self, v: Option<Vec<u8>>) {
        self.avatar = v;
    }

    pub fn get_avatar(&self) -> &Option<Vec<u8>> {
        &self.avatar
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct LdapConfig {
    #[serde(default)]
    id: String,
    host: String,
    port: String,
    enforce_starttls: bool,
    use_ldaps: bool,
    lookup_dn: String,
    lookup_password: String,
    user_search: UserSearch,
    group_search: GroupSearch,
    ca_certs: String,
    client_cert: String,
    #[serde(default)]
    created_at: String,
}

impl LdapConfig {
    pub fn new() -> LdapConfig {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
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

    pub fn set_enforce_starttls(&mut self, v: bool) {
        self.enforce_starttls = v;
    }

    pub fn get_enforce_starttls(&self) -> bool {
        self.enforce_starttls.clone()
    }

    pub fn set_use_ldaps(&mut self, v: bool) {
        self.use_ldaps = v;
    }

    pub fn get_use_ldaps(&self) -> bool {
        self.use_ldaps.clone()
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
    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
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

    pub fn set_search_filter_template(&mut self, v: ::std::string::String) {
        self.search_filter_template = v;
    }

    pub fn set_member_attributes(&mut self, v: ::std::vec::Vec<String>) {
        self.member_attributes = v;
    }
}
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct SamlProvider {
    #[serde(default)]
    id: String,
    description: String,
    idp_metadata: String,
    sp_base_url: String,
    #[serde(default)]
    created_at: String,
}

impl SamlProvider {
    pub fn new() -> SamlProvider {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    pub fn get_description(&self) -> ::std::string::String {
        self.description.clone()
    }

    pub fn set_idp_metadata(&mut self, v: ::std::string::String) {
        self.idp_metadata = v;
    }

    pub fn get_idp_metadata(&self) -> ::std::string::String {
        self.idp_metadata.clone()
    }
    pub fn set_sp_base_url(&mut self, v: ::std::string::String) {
        self.sp_base_url = v;
    }

    pub fn get_sp_base_url(&self) -> ::std::string::String {
        self.sp_base_url.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct Success {
    result: String,
}

impl Success {
    pub fn new() -> Success {
        ::std::default::Default::default()
    }
    pub fn set_result(&mut self, v: ::std::string::String) {
        self.result = v;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ImportResult {
    result: String,
    imported: Vec<String>,
}

impl ImportResult {
    pub fn new() -> ImportResult {
        ::std::default::Default::default()
    }
    pub fn set_result(&mut self, v: ::std::string::String) {
        self.result = v;
    }
    pub fn set_users(&mut self, v: Vec<String>) {
        self.imported = v;
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct OidcProvider {
    #[serde(default)]
    id: String,
    description: String,
    issuer: String,
    base_url: String,
    client_secret: String,
    client_id: String,
    verify_server_certificate: bool,
    ca_certs: String,
    #[serde(default)]
    created_at: String,
}

impl OidcProvider {
    pub fn new() -> OidcProvider {
        ::std::default::Default::default()
    }
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    pub fn get_description(&self) -> ::std::string::String {
        self.description.clone()
    }

    pub fn set_issuer(&mut self, v: ::std::string::String) {
        self.issuer = v;
    }

    pub fn get_issuer(&self) -> ::std::string::String {
        self.issuer.clone()
    }
    pub fn set_base_url(&mut self, v: ::std::string::String) {
        self.base_url = v;
    }

    pub fn get_base_url(&self) -> ::std::string::String {
        self.base_url.clone()
    }
    pub fn set_client_secret(&mut self, v: ::std::string::String) {
        self.client_secret = v;
    }

    pub fn get_client_secret(&self) -> ::std::string::String {
        self.client_secret.clone()
    }
    pub fn set_client_id(&mut self, v: ::std::string::String) {
        self.client_id = v;
    }

    pub fn get_client_id(&self) -> ::std::string::String {
        self.client_id.clone()
    }
    pub fn set_ca_certs(&mut self, v: ::std::string::String) {
        self.ca_certs = v;
    }

    pub fn get_ca_certs(&self) -> ::std::string::String {
        self.ca_certs.clone()
    }

    pub fn set_verify_server_certificate(&mut self, v: bool) {
        self.verify_server_certificate = v;
    }

    pub fn get_verify_server_certificate(&self) -> bool {
        self.verify_server_certificate.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Device {
    name: String,
    category: String,
    os: String,
    os_version: String,
    browser_type: String,
    version: String,
    vendor: String,
    ip: String,
}

impl Device {
    pub fn new() -> Device {
        ::std::default::Default::default()
    }
    pub fn with(name: String, category: String, os: String, os_version: String, browser_type: String, version: String, vendor: String) -> Device {
        Device {
            name: name,
            category: category,
            os: os,
            os_version: os_version,
            browser_type: browser_type,
            version: version,
            vendor: vendor,
            ..Default::default()
        }
    }

    pub fn set_ip(&mut self, v: ::std::string::String) {
        self.ip = v;
    }
}

//convert the PromResponse into OSUsages value
impl Into<Device> for WootheeResult {
    fn into(self) -> Device {
        Device::with(
            self.name,
            self.category,
            self.os,
            self.os_version,
            self.browser_type,
            self.version,
            self.vendor,
        )
    }
}

pub fn user_agent(req: &Request) -> WootheeResult {
   let default_agent = UserAgent(DEFAULT_AGENT.to_owned());
   let user_agent = req.headers.get::<UserAgent>().unwrap_or(&default_agent);
   let parser = Parser::new();
   let result = parser.parse(user_agent).unwrap_or(
       WootheeResult {
   name: DEFAULT_AGENT.to_string(),
   category: "cli".to_string(),
   os: "Linux".to_string(),
   os_version: "0".to_string(),
   browser_type: "CLI".to_string(),
   version: "0".to_string(),
   vendor: "Rio/OS".to_string()
}
);
   result
}
