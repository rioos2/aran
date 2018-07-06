// Copyright 2018 The Rio Advancement Inc
use api::base::IdGet;
use cache::inject::LicensesFeeder;
use api::base::{MetaFields, ObjectMeta, TypeMeta};


#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct Licenses {
    #[serde(default)]
    id: String,
    #[serde(default)]
    type_meta: TypeMeta,
    pub object_meta: ObjectMeta,
    status: String,
    product: String,
    activation_code: String,
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

    pub fn set_activation_code(&mut self, v: ::std::string::String) {
        self.activation_code = v;
    }

    pub fn get_activation_code(&self) -> ::std::string::String {
        self.activation_code.clone()
    }

    pub fn set_expired(&mut self, v: ::std::string::String) {
        self.expired_at = v;
    }

    pub fn get_expired(&self) -> ::std::string::String {
        self.expired_at.clone()
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
        assert_eq!(license.expired_at,"30");
        assert_eq!(license.activation_code,"ertyuicvbnm456789dfghjk456789");
    }

}
