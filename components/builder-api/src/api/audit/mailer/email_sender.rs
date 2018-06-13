use api::audit::config::MailerCfg;
use api::audit::mailer::{email_generator, PushNotifier, Status};
use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::smtp::ClientSecurity;
use lettre::smtp::ConnectionReuseParameters;
use lettre::smtp::SmtpTransportBuilder;
use lettre::EmailTransport;
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
            let email = EmailBuilder::new()
                .to(self.email.clone())
                .from(self.config.sender)
                .subject(self.subject.clone())
                .html(self.content)
                .build();
            let mut addrs_iter = self.config.domain.to_socket_addrs().unwrap();
            let mut mailer =
                SmtpTransportBuilder::new(addrs_iter.next().unwrap(), ClientSecurity::None)
                    .unwrap()
                    .credentials(Credentials::new(
                        self.config.username.to_string(),
                        self.config.password.to_string(),
                    ))
                    .authentication_mechanism(Mechanism::Plain)
                    .connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
                    .build();

            match mailer.send(&email.unwrap()) {
                Ok(_) => info!("{} {} {} → SENT", "✔", self.email, self.subject),
                Err(_) => info!("{} {} {} → FAIL", "✘", self.email, self.subject),
            };

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
        EmailNotifier {
            envelope: envelope,
            config: config,
        }
    }
}

impl PushNotifier for EmailNotifier {
    fn should_notify(&self) -> bool {
        match Status::from_str(&self.envelope.get_event().reason) {
            Status::DigitalCloudRunning | Status::DigitalCloudFailed => true,
            Status::None => false,
        }
    }
    fn notify(&self) {
        let data = email_generator::EmailGenerator::new(
            self.envelope.get_event().object_meta().labels,
            &self.envelope.get_event().message,
        );
        match Status::from_str(&self.envelope.get_event().reason) {
            Status::DigitalCloudRunning => {
                let content = data.deploy_success().unwrap();
                let mail_builder =
                    EmailSender::new(self.config.clone(), data.email(), content.0, content.1);
                mail_builder.send_email();
            }
            Status::DigitalCloudFailed => {
                let content = data.deploy_failed().unwrap();
                let mail_builder =
                    EmailSender::new(self.config.clone(), data.email(), content.0, content.1);
                mail_builder.send_email();
            }
            Status::None => {}
        }
    }
}
