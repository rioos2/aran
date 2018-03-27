// Copyright 2018 The Rio Advancement Inc

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Roles {
    #[serde(default)]
    id: String,
    name: String,
    description: String,
    #[serde(default)]
    created_at: String,
}

impl Roles {
    pub fn new() -> Roles {
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

#[cfg(test)]
mod test {
    use serde_json::from_str as json_decode;

    use super::*;

    #[test]
    fn decode_roles() {
        let val = r#"{
            "name": "role_rios:superuser",
            "description":"superuser of RIO/OS. God given powers.  instance"
            }"#;
        let role: Roles = json_decode(val).unwrap();
        assert_eq!(role.name, "role_rios:superuser");
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
