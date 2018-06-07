use handlebars::Handlebars;
use rio_core::fs::{read_from_file, rioconfig_config_path};
use error::{Result, Error};
use std::path::PathBuf;
use serde_json;
use std::collections::BTreeMap;
const DEPLOY_SUBJECT: &'static str = "Ahoy! Deployed successfully.";
const FAILED_SUBJECT: &'static str = "Deploy failure";

lazy_static! {
    static  ref DEPLOY_SUCCESS: PathBuf =  PathBuf::from(&*rioconfig_config_path(None).join("template/deploy_success.hbs").to_str().unwrap());
    static  ref DEPLOY_FAILED: PathBuf =  PathBuf::from(&*rioconfig_config_path(None).join("template/deploy_failed.hbs").to_str().unwrap());
}

pub struct EmailGenerator {
    labels: BTreeMap<String, String>,
    message: String,
}

impl EmailGenerator {
    pub fn new(labels: BTreeMap<String, String>, message: &str) -> Self {
        EmailGenerator {
            labels: labels,
            message: message.to_string(),
        }
    }
    pub fn deploy_success(&self) -> Result<(String, String)> {
        let r = Handlebars::new()
            .render_template(&read_from_file(&DEPLOY_SUCCESS)?, &self.content())
            .map_err(|tr| Error::MissingConfiguration(format!("{}", tr)));
        Ok((DEPLOY_SUBJECT.to_string(), r.unwrap()))
    }

    pub fn deploy_failed(&self) -> Result<(String, String)> {
        let r = Handlebars::new()
            .render_template(&read_from_file(&DEPLOY_FAILED)?, &self.content())
            .map_err(|tr| Error::MissingConfiguration(format!("{}", tr)));
        Ok((FAILED_SUBJECT.to_string(), r.unwrap()))
    }

    pub fn email(&self) -> String {
        self.labels.get("email").unwrap_or(&"".to_string()).clone()
    }

    fn content(&self) -> serde_json::Value {
        json!({
        "email": self.labels.get("username"),
        "appname": self.labels.get("app_name"),
        "category": self.labels.get("category"),
        "alert_message": self.message,
        "image_name": self.labels.get("image_name")
    })
    }
}