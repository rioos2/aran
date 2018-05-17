use lettre::smtp::authentication::{Credentials, Mechanism};
use lettre::EmailTransport;
use lettre::smtp::SmtpTransportBuilder;
use lettre::smtp::ClientSecurity;
use lettre::smtp::ConnectionReuseParameters;
use lettre_email::EmailBuilder;
use std::net::ToSocketAddrs;
use api::audit::config::MailerCfg;

pub struct EmailNotifier {
    config: MailerCfg,
    email: String,
    subject: String,
    content: String,
}
impl EmailNotifier {
    pub fn new(config: MailerCfg, email: String, subject: String, content: String) -> Self {
        EmailNotifier {
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
