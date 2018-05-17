// Copyright 2018 The Rio Advancement Inc

use events::{Event, EventHandler, InternalEvent};
use node::runtime::{RuntimeHandler, ExternalMessage};

use api::audit::ledger;
use api::audit::mailer::{email_generator, email_notifier, Status};
use protocol::api::base::MetaFields;

impl EventHandler for RuntimeHandler {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Api(api) => self.handle_api_event(api),
            Event::Internal(internal) => {
                self.handle_internal_event(&internal);
            }
        }
    }
}

//Send the stuff to the audit block chain server
impl RuntimeHandler {
    fn handle_api_event(&mut self, event: ExternalMessage) {
        match event {
            ExternalMessage::PeerAdd(event_envl) => {
                println!("--> ledger config is {:?}", self.config);

                match ledger::from_config(&self.config) {
                    Ok(ledger) => {
                        match ledger.record(&event_envl) {
                            Ok(_) => println!("--> save success"),

                            _ => println!("--> save fail. {:?}", event_envl),
                        };
                    }
                    _ => println!("--> ledger load  fail."),
                }
            }
            ExternalMessage::EmailNotification(event_envl) => {
                let data = email_generator::EmailGenerator::new(
                    event_envl.event.object_meta().labels,
                    &event_envl.event.message,
                );
                match Status::from_str(&event_envl.event.reason) {
                    Status::DigitalCloudRunning => {
                        let content = data.deploy_success().unwrap();
                        let mail_builder = email_notifier::EmailNotifier::new(*self.mailer.clone(), data.email(), content.0, content.1);
                        mail_builder.send_email();
                    }
                    Status::DigitalCloudFailed => {
                        let content = data.deploy_failed().unwrap();
                        let mail_builder = email_notifier::EmailNotifier::new(*self.mailer.clone(), data.email(), content.0, content.1);
                        mail_builder.send_email();
                    }
                    Status::None => {}
                }
            }
        }
    }

    fn handle_internal_event(&mut self, event: &InternalEvent) {
        match *event {
            // InternalEvent::EntitlementTimeout => match self.license.create_trial_or_verify() {
            //     Ok(()) => info!{" ✓ All Good. You have a valid entitlement. !"},
            //     Err(err) => {
            //         let expiry_attempt = self.license.hard_stop();
            //         if expiry_attempt.is_err() {
            //             error!("{:?}", err)
            //         }
            //         warn!("{:?}, Message: {:?}", expiry_attempt.unwrap(), err)
            //     }
            // },
            InternalEvent::EntitlementTimeout => info!{" ✓ All Good. You have a valid entitlement. !"},
            InternalEvent::Shutdown => warn!("Shutting down...please wait!."),
        }
    }
}
