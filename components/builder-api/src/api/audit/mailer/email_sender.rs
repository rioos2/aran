// Copyright 2018 The Rio Advancement Inc
//

use api::audit::config::MailerCfg;
use api::audit::mailer::email_generator;
use api::audit::{PushNotifier, Status};
use lettre::EmailTransport;
use lettre::smtp::ClientSecurity;
use lettre::smtp::ConnectionReuseParameters;
use lettre::smtp::SmtpTransportBuilder;
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre_email::EmailBuilder;
use protocol::api::audit::Envelope;
use protocol::api::base::MetaFields;
use std::net::ToSocketAddrs;

pub struct EmailSender {
    config: MailerCfg,
    email: String,
    subject: String,
    content: String,
}

impl EmailSender {
    pub fn new(config: MailerCfg, email: String, subject: String, content: String) -> Self {
        EmailSender {
            config: config,
            email: email,
            subject: subject,
            content: content,
        }
    }
    pub fn send_email(self) {
        if self.config.enabled {
            let email = EmailBuilder::new().to(self.email).from(self.config.sender).subject(self.subject).html(self.content).build();
            let mut addrs_iter = self.config.domain.to_socket_addrs().unwrap();
            let mut mailer = SmtpTransportBuilder::new(addrs_iter.next().unwrap(), ClientSecurity::None)
                .unwrap()
                .credentials(Credentials::new(self.config.username.to_string(), self.config.password.to_string()))
                .authentication_mechanism(Mechanism::Plain)
                .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
                .build();

            mailer.send(&email.unwrap());
            mailer.close();
        }
    }
}
pub struct EmailNotifier {
    envelope: Envelope,
    config: MailerCfg,
}

impl EmailNotifier {
    pub fn new(envelope: Envelope, config: MailerCfg) -> Self {
        EmailNotifier { envelope: envelope, config: config }
    }
}

impl PushNotifier for EmailNotifier {
    fn should_notify(&self) -> bool {
        match Status::from_str(&self.envelope.event.reason) {
            Status::DigitalCloudRunning | Status::DigitalCloudFailed => true,
            _ => false,
        }
    }
    fn notify(&self) {
        let data = email_generator::EmailGenerator::new(self.envelope.event.object_meta().labels, &self.envelope.event.message);
        match Status::from_str(&self.envelope.event.reason) {
            Status::DigitalCloudRunning => {
                let content = data.deploy_success().unwrap();
                let mail_builder = EmailSender::new(self.config.clone(), data.email(), content.0, content.1);
                mail_builder.send_email();
            }
            Status::DigitalCloudFailed => {
                let content = data.deploy_failed().unwrap();
                let mail_builder = EmailSender::new(self.config.clone(), data.email(), content.0, content.1);
                mail_builder.send_email();
            }
            _ => {}
        }
    }
}
