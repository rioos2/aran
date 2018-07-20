// Copyright 2018 The Rio Advancement Inc

use api::base::{MetaFields, ObjectMeta, TypeMeta, WhoAmITypeMeta};
use api::base::IdGet;
use cache::inject::LicensesFeeder;
use std::collections::BTreeMap;

pub const INVALID: &'static str = "invalid";


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Licenses {
    #[serde(default)]
    id: String,
    #[serde(default)]
    type_meta: TypeMeta,
    pub object_meta: ObjectMeta,
    #[serde(default)]
    status: String,
    #[serde(default)]
    user_activation: bool,
    product: String,
    license_id: String,
    password: String,
    #[serde(default)]
    product_options: BTreeMap<String, AllowActive>,
    #[serde(default)]
    expired_at: String,
    #[serde(default)]
    created_at: String,
}


impl MetaFields for Licenses {
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
pub struct AllowActive {
    pub maximum: i32,
    pub current: i32,
}
impl Licenses {
    pub fn new() -> Licenses {
        ::std::default::Default::default()
    }

    //Create a new service with type_meta and object_meta
    //and other defaulted.
    pub fn with(t: TypeMeta, o: ObjectMeta) -> Licenses {
        Licenses {
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

    pub fn set_status(&mut self, v: ::std::string::String) {
        self.status = v;
    }

    pub fn get_status(&self) -> ::std::string::String {
        self.status.clone()
    }

    pub fn set_product(&mut self, v: ::std::string::String) {
        self.product = v;
    }

    pub fn get_product(&self) -> ::std::string::String {
        self.product.clone()
    }

    pub fn set_password(&mut self, v: ::std::string::String) {
        self.password = v;
    }

    pub fn get_password(&self) -> ::std::string::String {
        self.password.clone()
    }

    pub fn set_license_id(&mut self, v: String) {
        self.license_id = v;
    }

    pub fn get_license_id(&self) -> String {
        self.license_id.clone()
    }

    pub fn set_expired(&mut self, v: ::std::string::String) {
        self.expired_at = v;
    }

    pub fn get_expired(&self) -> ::std::string::String {
        self.expired_at.clone()
    }

    pub fn set_user_activation(&mut self, v: bool) {
        self.user_activation = v;
    }

    pub fn get_user_activation(&self) -> bool {
        self.user_activation.clone()
    }

    pub fn set_product_options(&mut self, v: BTreeMap<String, AllowActive>) {
        self.product_options = v;
    }

    pub fn get_product_options(&self) -> &BTreeMap<String, AllowActive> {
        &self.product_options
    }

    pub fn set_created_at(&mut self, v: ::std::string::String) {
        self.created_at = v;
    }

    pub fn get_created_at(&self) -> ::std::string::String {
        self.created_at.clone()
    }
}

impl LicensesFeeder for Licenses {
    fn iget_id(&mut self) -> IdGet {
        IdGet::with_id_name(self.object_meta().name, "".to_string())
    }

    fn ifeed(&mut self, m: Option<String>) {
        match m {
            Some(status) => self.set_status(status),
            None => {}
        }
    }
}

impl WhoAmITypeMeta for Licenses {
    const MY_KIND: &'static str = "POST:licenseactivate";
}
//TRIAL => Evaluation trial for 30 days
//ACTIVE => License with FullNonExpiring
//EXPIRED => License TimeLimit is exists
//INVALID => License process failed

pub enum LicenseStatus {
    TRIAL,
    ACTIVE,
    EXPIRED,
    INVALID,
}

impl LicenseStatus {
    pub fn status(status: &str) -> LicenseStatus {
        match &status[..] {
            "active" => LicenseStatus::ACTIVE,
            "expired" => LicenseStatus::EXPIRED,
            "trial" => LicenseStatus::TRIAL,
            "invalid" => LicenseStatus::INVALID,
            _ => LicenseStatus::INVALID,
        }
    }
}


#[cfg(test)]
mod test {

    use super::*;
    use serde_json::from_str as json_decode;

    #[test]
    fn decode_license() {
        let val = r#"{
            "object_meta":{
                "name":"SoftwareKey"
                },
            "status":"trial",
            "product":"Rio/OS",
            "activation_code":"ertyuicvbnm456789dfghjk456789",
            "expired_at":"30"}"#;
        let license: Licenses = json_decode(val).unwrap();
        assert_eq!(license.status, "trial");
        assert_eq!(license.product, "Rio/OS");
        assert_eq!(license.expired_at, "30");
        assert_eq!(license.activation_code, "ertyuicvbnm456789dfghjk456789");
    }

}
