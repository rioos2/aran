// Copyright 2018 The Rio Advancement Inc
use api::base::{MetaFields, ObjectMeta, TypeMeta, WhoAmITypeMeta};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
pub struct ImageVulner {
    #[serde(default)]
    pub object_meta: ObjectMeta,
    #[serde(default)]
    type_meta: TypeMeta,
    data: Vulnerable,
}
impl ImageVulner {
    pub fn new() -> ImageVulner {
        ::std::default::Default::default()
    }

    //Create a new team with type_meta and object_meta
    //and other defaulted.
    pub fn with_image(t: TypeMeta, o: ObjectMeta, d: Vulnerable) -> ImageVulner {
        ImageVulner {
            type_meta: t,
            object_meta: o,
            data: d,
            ..Default::default()
        }
    }

    // pub fn set_data(&mut self, v: Vulnerable) {
    //     self.data = v;
    // }
    // pub fn get_data(&self) -> &Vulnerable {
    //     &self.data
    // }
}
impl MetaFields for ImageVulner {
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

impl WhoAmITypeMeta for ImageVulner {
    const MY_KIND: &'static str = "GET:imagevulnerablity";
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct Vulnerable {
    #[serde(rename = "imageDigest")]
    #[serde(default)]
    pub image_digest: String,
    pub vulnerabilities: Vec<VulnContent>,
    pub vulnerability_type: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq)]
pub struct VulnContent {
    pub fix: String,
    pub package: String,
    pub severity: String,
    pub url: String,
    pub vuln: String,
}
