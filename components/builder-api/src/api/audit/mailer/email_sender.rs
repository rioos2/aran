use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::EmailTransport;
use lettre::smtp::SmtpTransportBuilder;
use lettre::smtp::ClientSecurity;
use lettre::smtp::ConnectionReuseParameters;
use lettre_email::EmailBuilder;
use std::net::ToSocketAddrs;
use api::audit::config::MailerCfg;
use api::audit::mailer::{PushNotifier, Status};


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
                .to(self.email)
                .from(self.config.sender)
                .subject(self.subject)
                .html(self.content)
                .build();
            let mut addrs_iter = self.config.domain.to_socket_addrs().unwrap();
            let mut mailer = SmtpTransportBuilder::new(addrs_iter.next().unwrap(), ClientSecurity::None)
                .unwrap()
                .credentials(Credentials::new(
                    self.config.username.to_string(),
                    self.config.password.to_string(),
                ))
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
        EmailNotifier {
            envelope: envelope,
            config: config,
        }
    }
}

impl PushNotifier for EmailNotifier {
    fn should_notify(&self) -> bool {
        match Status::from_str(&self.envelope.event.reason) {
            Status::DigitalCloudRunning |
            Status::DigitalCloudFailed => true,
            Status::None => false,
        }
    }
    fn notify(&self) {
        let data = email_generator::EmailGenerator::new(
            event_envl.event.object_meta().labels,
            &event_envl.event.message,
        );
        match Status::from_str(&event_envl.event.reason) {
            Status::DigitalCloudRunning => {
                let content = data.deploy_success().unwrap();
                let mail_builder = email_sender::EmailSender::new(self.config, data.email(), content.0, content.1);
                mail_builder.send_email();
            }
            Status::DigitalCloudFailed => {
                let content = data.deploy_failed().unwrap();
                let mail_builder = email_sender::EmailSender::new(self.config, data.email(), content.0, content.1);
                mail_builder.send_email();
            }
            Status::None => {}
        }
    }
}
