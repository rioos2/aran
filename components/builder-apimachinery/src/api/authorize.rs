// Copyright 2018 The Rio Advancement Inc

use api::base::IdGet;
use cache::inject::PermissionsFeeder;
use std::collections::BTreeMap;
use api::base::{ChildTypeMeta, TypeMeta, ObjectMeta, MetaFields, WhoAmITypeMeta};
use cache::inject::MembersFeeder;
use cache::inject::{PoliciesFeeder, PolicyMembersFeeder};
use cache::inject::TeamsFeeder;
use api::invitations::Invitations;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Teams {
    #[serde(default)]
    id: String,
    #[serde(default)]
    full_name: String,
    description: String,
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: Team
    object_meta: ObjectMeta, ////Standard object metadata
    #[serde(default)]
    metadata: BTreeMap<String, String>, //Standard object's metadata. Can contain optional label selector team, origin
    #[serde(default)]
    created_at: String,
    #[serde(default)]
    members: Option<Vec<Invitations>>,
    #[serde(default)]
    policies: Option<Vec<PolicyMembers>>,
}

impl Teams {
    pub fn new() -> Teams {
        ::std::default::Default::default()
    }

    pub fn with(t: TypeMeta, o: ObjectMeta) -> Teams {
        Teams {
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

    pub fn set_full_name(&mut self, v: ::std::string::String) {
        self.full_name = v;
    }

    pub fn get_full_name(&self) -> ::std::string::String {
        self.full_name.clone()
    }

    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    pub fn get_description(&self) -> ::std::string::String {
        self.description.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }

    pub fn set_metadata(&mut self, v: BTreeMap<String, String>) {
        self.metadata = v;
    }

    pub fn get_metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    // Mutable pointer to the field metadata.
    pub fn get_mut_metadata(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.metadata
    }

    pub fn set_members(&mut self, v: Option<Vec<Invitations>>) {
        self.members = v;
    }

    pub fn set_policies(&mut self, v: Option<Vec<PolicyMembers>>) {
        self.policies = v;
    }

    pub fn get_policies(&self) -> Option<Vec<PolicyMembers>> {
        self.policies.clone()
    }
}

impl MetaFields for Teams {
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

// The service feeder, which gets called from an expander cache.
// The expander cache is ttl and loads the service the first time.
impl MembersFeeder for Teams {
    fn eget_id(&mut self) -> IdGet {
        IdGet::with_id(self.get_id().clone())
    }

    fn efeed(&mut self, s: Option<Vec<Invitations>>) {
        self.set_members(s);
    }
}


impl PolicyMembersFeeder for Teams {
    fn eget_id(&mut self) -> IdGet {
        IdGet::with_id(self.get_id().clone())
    }

    fn efeed(&mut self, s: Option<Vec<PolicyMembers>>) {
        self.set_policies(s);
    }
}

// The service feeder, which gets called from an expander cache.
// The expander cache is ttl and loads the service the first time.
impl TeamsFeeder for Teams {
    fn eget_id(&mut self) -> IdGet {
        IdGet::with_id(self.get_id().clone())
    }

    fn efeed(&mut self, s: Option<Teams>) {       
        match s {
            Some(acc) => {            
                self.set_policies(acc.get_policies());           
            },
            None => {}
       }
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct TeamMembers {
    #[serde(default)]
    id: String,
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: Team
    object_meta: ObjectMeta, ////Standard object metadata
    #[serde(default)]
    metadata: BTreeMap<String, String>, //Standard object's metadata. Can contain optional label selector team, origin
    #[serde(default)]
    created_at: String,
    #[serde(default)]
    updated_at: String,
    #[serde(default)]
    team: Option<Teams>,
}

impl TeamMembers {
    pub fn new() -> TeamMembers {
        ::std::default::Default::default()
    }

    pub fn with(t: TypeMeta, o: ObjectMeta) -> TeamMembers {
        TeamMembers {
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

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }

    pub fn set_metadata(&mut self, v: BTreeMap<String, String>) {
        self.metadata = v;
    }

    pub fn get_metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    pub fn set_updated_at(&mut self, v: ::std::string::String) {
        self.updated_at = v;
    }

    pub fn get_updated_at(&self) -> ::std::string::String {
        self.updated_at.clone()
    }

     pub fn set_team(&mut self, v: Option<Teams>) {
        self.team = v;
    }

}

// The service feeder, which gets called from an expander cache.
// The expander cache is ttl and loads the service the first time.
impl TeamsFeeder for TeamMembers {
    fn eget_id(&mut self) -> IdGet {
        let empty = "".to_string();
        let id = self.get_metadata().get("team").unwrap_or(&empty);
        IdGet::with_id(id.clone())
    }

    fn efeed(&mut self, s: Option<Teams>) {
        self.set_team(s);
    }
}

impl MetaFields for TeamMembers {
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


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Permissions {
    #[serde(default)]
    id: String,
    policy_id: String,
    name: String,
    description: String,
    #[serde(default)]
    created_at: String,
}

impl Permissions {
    pub fn new() -> Permissions {
        ::std::default::Default::default()
    }

    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }
    pub fn get_id(&self) -> ::std::string::String {
        self.id.clone()
    }

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
    }

    pub fn set_policy_id(&mut self, v: ::std::string::String) {
        self.policy_id = v;
    }

    pub fn get_policy_id(&self) -> ::std::string::String {
        self.policy_id.clone()
    }

    pub fn set_description(&mut self, v: ::std::string::String) {
        self.description = v;
    }

    pub fn get_description(&self) -> ::std::string::String {
        self.description.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct PermissionsForPolicy {
    policy: String,
    permissions: Option<Vec<Permissions>>,
}

impl PermissionsForPolicy {
    pub fn new() -> PermissionsForPolicy {
        ::std::default::Default::default()
    }

    pub fn set_policy(&mut self, v: ::std::string::String) {
        self.policy = v;
    }

    pub fn get_policy(&self) -> ::std::string::String {
        self.policy.clone()
    }

    pub fn set_permissions(&mut self, v: Option<Vec<Permissions>>) {
        self.permissions = v;
    }
    pub fn get_permissions(&self) -> Option<Vec<Permissions>> {
        self.permissions.clone()
    }
}

impl PermissionsFeeder for PermissionsForPolicy {
    fn iget_id(&mut self) -> IdGet {
        IdGet::with_id_name(self.get_policy(), "".to_string())
    }

    fn ifeed(&mut self, m: Option<Vec<Permissions>>) {
        self.set_permissions(m);
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct PoliciesForLevel {
    level: String,
    policies: Option<Vec<Policies>>,
}

impl PoliciesForLevel {
    pub fn new() -> PoliciesForLevel {
        ::std::default::Default::default()
    }

     pub fn set_level(&mut self, v: ::std::string::String) {
        self.level = v;
    }

    pub fn get_level(&self) -> ::std::string::String {
        self.level.clone()
    }

    pub fn set_policies(&mut self, v: Option<Vec<Policies>>) {
        self.policies = v;
    }

    pub fn get_policies(&self) -> Option<Vec<Policies>> {
        self.policies.clone()
    }
  
}

impl PoliciesFeeder for PoliciesForLevel {
    fn eget_id(&mut self) -> IdGet {
        IdGet::with_id_name(self.get_level(), "".to_string())
    }

    fn efeed(&mut self, m: Option<Vec<Policies>>) {
        self.set_policies(m);
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Policies {
    #[serde(default)]
    id: String,
    metadata: BTreeMap<String, String>,
    object_meta: ObjectMeta,
    #[serde(default)]
    type_meta: TypeMeta,
    #[serde(default)]
    created_at: String,
}

impl Policies {
    pub fn new() -> Policies {
        ::std::default::Default::default()
    }

    pub fn with(t: TypeMeta, o: ObjectMeta) -> Policies {
        Policies {
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

    pub fn set_metadata(&mut self, v: BTreeMap<String, String>) {
        self.metadata = v;
    }

    pub fn get_metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

impl MetaFields for Policies {
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

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct PolicyMembersList {
    pub policies: Vec<PolicyMembers>,
}

impl PolicyMembersList {    

    pub fn set_policies(&mut self, v: ::std::vec::Vec<PolicyMembers>) {
        self.policies = v;
    }

    pub fn get_policies(&self) -> ::std::vec::Vec<PolicyMembers> {
        self.policies.clone()
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct PolicyMemberInputs {
    account_id: String,
    origin_id: String,   
    team_id: String, 
    allowed_policies: Vec<String>,    
    denied_policies: Vec<String>, 
}

impl PolicyMemberInputs {
    pub fn new() -> PolicyMemberInputs {
        ::std::default::Default::default()
    }   

    pub fn set_account_id(&mut self, v: ::std::string::String) {
        self.account_id = v;
    }
    pub fn get_account_id(&self) -> ::std::string::String {
        self.account_id.clone()
    }

    pub fn set_origin_id(&mut self, v: ::std::string::String) {
        self.origin_id = v;
    }
    pub fn get_origin_id(&self) -> ::std::string::String {
        self.origin_id.clone()
    }

    pub fn set_team_id(&mut self, v: ::std::string::String) {
        self.team_id = v;
    }
    pub fn get_team_id(&self) -> ::std::string::String {
        self.team_id.clone()
    }

    pub fn set_allowed_policies(&mut self, v: ::std::vec::Vec<String>) {
        self.allowed_policies = v;
    }
    pub fn get_allowed_policies(&self) -> ::std::vec::Vec<String> {
        self.allowed_policies.clone()
    }

    pub fn set_denied_policies(&mut self, v: ::std::vec::Vec<String>) {
        self.denied_policies = v;
    }
    pub fn get_denied_policies(&self) -> ::std::vec::Vec<String> {
        self.denied_policies.clone()
    }
}

impl ChildTypeMeta for PolicyMemberInputs {
    const CHILD_KIND: &'static str = "POST:policymembers";
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct PolicyMembers {
    #[serde(default)]
    id: String,
    policy_name: String,
    is_allow: String,
    #[serde(default)]
    metadata: BTreeMap<String, String>,
    #[serde(default)]
    object_meta: ObjectMeta,
    #[serde(default)]
    type_meta: TypeMeta,
    #[serde(default)]
    created_at: String,
    #[serde(default)]
    permissions: Option<Vec<Permissions>>,
}

impl PolicyMembers {
    pub fn new() -> PolicyMembers {
        ::std::default::Default::default()
    }

    pub fn with(t: TypeMeta, o: ObjectMeta) -> PolicyMembers {
        PolicyMembers {
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

    pub fn set_policy_name(&mut self, v: ::std::string::String) {
        self.policy_name = v;
    }
    pub fn get_policy_name(&self) -> ::std::string::String {
        self.policy_name.clone()
    }

    pub fn set_is_allow(&mut self, v: ::std::string::String) {
        self.is_allow = v;
    }
    pub fn get_is_allow(&self) -> ::std::string::String {
        self.is_allow.clone()
    }

    pub fn set_metadata(&mut self, v: BTreeMap<String, String>) {
        self.metadata = v;
    }

    pub fn get_metadata(&self) -> &BTreeMap<String, String> {
        &self.metadata
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }

    pub fn set_permissions(&mut self, v: Option<Vec<Permissions>>) {
        self.permissions = v;
    }
    pub fn get_permissions(&self) -> Option<Vec<Permissions>> {
        self.permissions.clone()
    }
}

impl MetaFields for PolicyMembers {
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

impl PermissionsFeeder for PolicyMembers {
    fn iget_id(&mut self) -> IdGet {
        IdGet::with_id_name(self.get_policy_name(), "".to_string())
    }

    fn ifeed(&mut self, m: Option<Vec<Permissions>>) {
        self.set_permissions(m);
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use serde_json::from_str as json_decode;

    #[test]
    fn decode_teams() {
        let val = r#"{
            "full_name": "RIOOS:SUPERUSER",
            "description":"superuser of RIO/OS. God given powers.  instance",
            "object_meta": {"account":"1043206892018475008"},"metadata": {"origin":"rioos"}}"#;
        let team: Teams = json_decode(val).unwrap();
        assert_eq!(team.full_name, "RIOOS:SUPERUSER");
        assert_eq!(
            team.description,
            "superuser of RIO/OS. God given powers.  instance"
        );
    }

    #[test]
    fn decode_permission() {
        let val = r#"{
            "policy_id": "98765432123456",
            "name": "rioos.assembly.get",
            "description":"Read only access to all the users  VMs, Containers"
            }"#;
        let perms: Permissions = json_decode(val).unwrap();
        assert_eq!(perms.policy_id, "98765432123456");
        assert_eq!(perms.name, "rioos.assembly.get");
        assert_eq!(
            perms.description,
            "Read only access to all the users  VMs, Containers"
        );
    }
}
