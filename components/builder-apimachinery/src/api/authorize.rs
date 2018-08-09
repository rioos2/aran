// Copyright 2018 The Rio Advancement Inc

use api::base::IdGet;
use cache::inject::PermissionsFeeder;
use std::collections::BTreeMap;
use api::base::{ChildTypeMeta, TypeMeta, ObjectMeta, MetaFields, WhoAmITypeMeta};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Teams {
    #[serde(default)]
    id: String,
    name: String,
    description: String,
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: Team
    object_meta: ObjectMeta, ////Standard object metadata
    #[serde(default)]
    metadata: BTreeMap<String, String>, //Standard object's metadata. Can contain optional label selector team, origin
    #[serde(default)]
    created_at: String,
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

    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    pub fn get_name(&self) -> ::std::string::String {
        self.name.clone()
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

/*impl WhoAmITypeMeta for Teams {
    const MY_KIND: &'static str = "POST:teams";
}*/

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Permissions {
    #[serde(default)]
    id: String,
    team_id: String,
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

    pub fn set_team_id(&mut self, v: ::std::string::String) {
        self.team_id = v;
    }

    pub fn get_team_id(&self) -> ::std::string::String {
        self.team_id.clone()
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
pub struct PermissionsForAccount {
    account_email: String,
    permissions: Option<Vec<Permissions>>,
}

impl PermissionsForAccount {
    pub fn new() -> PermissionsForAccount {
        ::std::default::Default::default()
    }

    pub fn set_account_email(&mut self, v: ::std::string::String) {
        self.account_email = v;
    }

    pub fn get_account_email(&self) -> ::std::string::String {
        self.account_email.clone()
    }

    pub fn set_permissions(&mut self, v: Option<Vec<Permissions>>) {
        self.permissions = v;
    }
    pub fn get_permissions(&self) -> Option<Vec<Permissions>> {
        self.permissions.clone()
    }
}

impl PermissionsFeeder for PermissionsForAccount {
    fn iget_id(&mut self) -> IdGet {
        IdGet::with_id_name(self.get_account_email(), "".to_string())
    }

    fn ifeed(&mut self, m: Option<Vec<Permissions>>) {
        self.set_permissions(m);
    }
}

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct PermissionsForTeam {
    team: String,
    permissions: Option<Vec<Permissions>>,
}

impl PermissionsForTeam {
    pub fn new() -> PermissionsForTeam {
        ::std::default::Default::default()
    }

    pub fn set_team(&mut self, v: ::std::string::String) {
        self.team = v;
    }

    pub fn get_team(&self) -> ::std::string::String {
        self.team.clone()
    }

    pub fn set_permissions(&mut self, v: Option<Vec<Permissions>>) {
        self.permissions = v;
    }
    pub fn get_permissions(&self) -> Option<Vec<Permissions>> {
        self.permissions.clone()
    }
}

impl PermissionsFeeder for PermissionsForTeam {
    fn iget_id(&mut self) -> IdGet {
        IdGet::with_id_name(self.get_team(), "".to_string())
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
            "name": "RIOOS:SUPERUSER",
            "description":"superuser of RIO/OS. God given powers.  instance",
            "object_meta": {"account":"1043206892018475008"},"metadata": {"origin":"rioos"}}"#;
        let team: Teams = json_decode(val).unwrap();
        assert_eq!(team.name, "RIOOS:SUPERUSER");
        assert_eq!(
            team.description,
            "superuser of RIO/OS. God given powers.  instance"
        );
    }

    #[test]
    fn decode_permission() {
        let val = r#"{
            "team_id": "98765432123456",
            "name": "rioos.assembly.get",
            "description":"Read only access to all the users  VMs, Containers"
            }"#;
        let perms: Permissions = json_decode(val).unwrap();
        assert_eq!(perms.team_id, "98765432123456");
        assert_eq!(perms.name, "rioos.assembly.get");
        assert_eq!(
            perms.description,
            "Read only access to all the users  VMs, Containers"
        );
    }
}