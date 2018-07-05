// Copyright 2018 The Rio Advancement Inc
//

use api::audit::config::{SlackCfg, SLACK_URL};
use api::audit::{PushNotifier, Status};
use error::{Error, Result};
use protocol::api::audit::Envelope;
use protocol::api::base::MetaFields;
use reqwest::header::{Accept, Authorization, Bearer, ContentType, Headers, UserAgent};
use reqwest::Body;
use rioos_http::ApiClient;
use serde_json;
use USER_AGENT;

const DEPLOY_SUBJECT: &'static str = "Ahoy! Kryptonite QRCode generated successfully.";
const FAILED_SUBJECT: &'static str = "Kryptonite QRCode sync failure";
const DEFAULT_SLACK_USER: &'static str = "test2";

pub struct SlackSender {
    config: SlackCfg,
    user: String,
    subject: String,
    content: String,
    // files: Vec<String>,
}

impl SlackSender {
    pub fn new(config: SlackCfg, user: String, subject: String, content: String) -> Self {
        println!("********new******{}****{}****", user, subject);
        SlackSender {
            config: config,
            user: user,
            subject: subject,
            content: content,
        }
    }
    pub fn send(self) -> Result<()> {
        // let token = self.config.token.to_string()
        println!("********new****send****{}**{}***", SLACK_URL, self.content);

        let client = ApiClient::new(&format!("{}", SLACK_URL), USER_AGENT, "v1", None).unwrap();
        let body = json!({
                "channel":"test2",
                "text": self.content,
            });
        let mut headers = Headers::new();
        headers.set(Authorization(Bearer {
            token: self.config.token.to_string(),
        }));
        headers.set(UserAgent::new(USER_AGENT.to_string()));
        headers.set(ContentType::json());
        headers.set(Accept::json());

        client
            .post("chat.PostMessage")
            .body(Body::from(serde_json::to_string(&body)?))
            .headers(headers)
            .send()
            .map_err(Error::ReqwestError)?;

        Ok({})
    }
}

pub struct SlackNotifier {
    envelope: Envelope,
    config: SlackCfg,
}

impl SlackNotifier {
    pub fn new(envelope: Envelope, config: SlackCfg) -> Self {
        SlackNotifier {
            envelope: envelope,
            config: config,
        }
    }
    fn getlabel(&self, key: String) -> String {
        self.envelope
            .get_event()
            .object_meta()
            .labels
            .get(&key.to_string())
            .unwrap_or(&"".to_string())
            .clone()
    }

    fn user(&self) -> String {
        self.getlabel("slack_user".to_string())
    }
    //
    // fn qr_image(&self) -> Vec<u8> {
    //     let mut bytes = "".as_bytes();
    //     let img = self.getlabel("qr_code_image");
    //     if assert_eq!(img.is_empty(), true) {
    //         bytes.push(img.as_bytes());
    //     }
    //     bytes
    // }
}

impl PushNotifier for SlackNotifier {
    fn should_notify(&self) -> bool {
        println!("***PushNotifier**should_notify***new****send********{}**", self.config.enabled);

        if !self.config.enabled {
            return false;
        }
        match Status::from_str(&self.envelope.get_event().reason) {
            Status::KryptoniteQRCode | Status::KryptoniteSyncFailed => true,
            _ => false,
        }
    }

    fn notify(&self) {
        println!("***PushNotifier**should_notify***notify**********{}", self.envelope.get_event().reason);

        if !self.should_notify() {
            return;
        }

        match Status::from_str(&self.envelope.get_event().reason) {
            // Status::KryptoniteQRCode => {
            //     let content = data.deploy_success().unwrap();
            //     let mail_builder = SlackSender::new(self.config.clone(), data.user(), content.0, content.1);
            //     mail_builder.send();
            // }
            Status::KryptoniteSyncFailed => {
                let sender = SlackSender::new(
                    self.config.clone(),
                    "".to_string(),
                    FAILED_SUBJECT.to_string(),
                    self.envelope.get_event().message.to_string(),
                );
                sender.send();
            }
            _ => {}
        }
    }
}
