// Copyright 2018 The Rio Advancement Inc


use api::audit::PushNotifier;
use api::audit::ledger;
use api::audit::mailer::email_sender as mailer;
use api::audit::slack::slack_sender as slack;
use db::data_store::DataStoreConn;
use events::{Event, EventHandler, InternalEvent};
use node::runtime::{ExternalMessage, RuntimeHandler};
const INVALID: &'static str = "invalid";


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
            ExternalMessage::PushNotification(event_envl) => {
                let e = event_envl.clone();
                mailer::EmailNotifier::new(e, *self.mailer.clone()).notify();
                slack::SlackNotifier::new(event_envl, *self.slack.clone()).notify();
            }

            ExternalMessage::ActivateLicense(license_id, password) => {
                println!("--> ActivateLicense");
                match self.license.activate_online(license_id, &password) {
                    Ok(_) => self.license.reload().unwrap(),
                    Err(err) => {
                        self.license.update_license_status(
                            INVALID.to_string(),
                            "".to_string(),
                        );
                    }
                }
            }

        }
    }

    fn handle_internal_event(&mut self, event: &InternalEvent) {
        match *event {
            InternalEvent::EntitlementTimeToVerify => self.license.live_verify().unwrap(),
            InternalEvent::Shutdown => warn!("Shutting down...please wait!."),
        }
    }
}
