// Copyright 2018 The Rio Advancement Inc
use api::base::IdGet;
use cache::inject::PermissionsFeeder;
use std::collections::BTreeMap;
use api::base::{ChildTypeMeta, TypeMeta, ObjectMeta, MetaFields, WhoAmITypeMeta};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Roles {
    #[serde(default)]
    id: String,
    name: String,
    description: String,
    #[serde(default)]
    type_meta: TypeMeta, //standard type metadata: kind: Role
    object_meta: ObjectMeta, ////Standard object metadata
    #[serde(default)]
    metadata: BTreeMap<String, String>, //Standard object's metadata. Can contain optional label selector team, origin
    #[serde(default)]
    created_at: String,
}

impl Roles {
    pub fn new() -> Roles {
        ::std::default::Default::default()
    }

    pub fn with(t: TypeMeta, o: ObjectMeta) -> Roles {
        Roles {
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

impl MetaFields for Roles {
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

/*impl WhoAmITypeMeta for Roles {
    const MY_KIND: &'static str = "POST:roles";
}*/

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Permissions {
    #[serde(default)]
    id: String,
    role_id: String,
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

    pub fn set_role_id(&mut self, v: ::std::string::String) {
        self.role_id = v;
    }

    pub fn get_role_id(&self) -> ::std::string::String {
        self.role_id.clone()
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
pub struct PermissionsForRole {
    role: String,
    permissions: Option<Vec<Permissions>>,
}

impl PermissionsForRole {
    pub fn new() -> PermissionsForRole {
        ::std::default::Default::default()
    }

    pub fn set_role(&mut self, v: ::std::string::String) {
        self.role = v;
    }

    pub fn get_role(&self) -> ::std::string::String {
        self.role.clone()
    }

    pub fn set_permissions(&mut self, v: Option<Vec<Permissions>>) {
        self.permissions = v;
    }
    pub fn get_permissions(&self) -> Option<Vec<Permissions>> {
        self.permissions.clone()
    }
}

impl PermissionsFeeder for PermissionsForRole {
    fn iget_id(&mut self) -> IdGet {
        IdGet::with_id_name(self.get_role(), "".to_string())
    }

    fn ifeed(&mut self, m: Option<Vec<Permissions>>) {
        self.set_permissions(m);
    }
}

#[cfg(test)]
mod test {
    use serde_json::from_str as json_decode;

    use super::*;

    #[test]
    fn decode_roles() {
        let val = r#"{
            "name": "RIOOS:SUPERUSER",
            "description":"superuser of RIO/OS. God given powers.  instance"
            "account":"1096543234567876",
            "origin":"dev"
            }"#;
        let role: Roles = json_decode(val).unwrap();
        assert_eq!(role.name, "RIOOS:SUPERUSER");
        assert_eq!(
            role.description,
            "superuser of RIO/OS. God given powers.  instance"
        );
    }

    #[test]
    fn decode_permission() {
        let val = r#"{
            "role_id": "98765432123456",
            "name": "rioos.assembly.get",
            "description":"Read only access to all the users  VMs, Containers"
            }"#;
        let perms: Permissions = json_decode(val).unwrap();
        assert_eq!(perms.role_id, "98765432123456");
        assert_eq!(perms.name, "rioos.assembly.get");
        assert_eq!(
            perms.description,
            "Read only access to all the users  VMs, Containers"
        );
    }
}