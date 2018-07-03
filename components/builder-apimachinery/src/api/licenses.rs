// Copyright 2018 The Rio Advancement Inc
use api::base::IdGet;
use cache::inject::LicensesFeeder;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Licenses {
    #[serde(default)]
    id: String,
    name: String,
    status: String,
    #[serde(default)]
    created_at: String,
    updated_at: String,
}

impl Licenses {
    pub fn new() -> Licenses {
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

    pub fn set_status(&mut self, v: ::std::string::String) {
        self.status = v;
    }

    pub fn get_status(&self) -> ::std::string::String {
        self.status.clone()
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }

    pub fn set_updated_at(&mut self, v: ::std::string::String) {
        self.updated_at = v;
    }

    pub fn get_updated_at(&self) -> ::std::string::String {
        self.updated_at.clone()
    }
}

impl LicensesFeeder for Licenses {
    fn iget_id(&mut self) -> IdGet {
        IdGet::with_id_name(self.get_name(), "".to_string())
    }

    fn ifeed(&mut self, m: Option<String>) {
        match m {
            Some(status) => self.set_status(status),
            None => {},
        }
    }
}

#[cfg(test)]
mod test {
    use serde_json::from_str as json_decode;

    use super::*;

    // #[test]
    fn decode_roles() {
        let val = r#"{
            "name": "LICENSECLOUD",
            "description":"superuser of RIO/OS. God given powers.  instance"
            }"#;
        let license: Licenses = json_decode(val).unwrap();
        assert_eq!(license.name, "LICENSECLOUD");
        assert_eq!(license.status,"ACTIVE");
    }

}
