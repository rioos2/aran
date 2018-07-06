// Copyright 2018 The Rio Advancement Inc


use api::audit::PushNotifier;
use api::audit::ledger;
use api::audit::mailer::email_sender as mailer;
use api::audit::slack::slack_sender as slack;
use db::data_store::DataStoreConn;
use events::{Event, EventHandler, InternalEvent};
use node::runtime::{ExternalMessage, RuntimeHandler};
use protocol::api::licenses::LicenseStatus;

const EXPIRY: &'static str = "expired";
const ACTIVE: &'static str = "active";
const TRIAL: &'static str = "trial";

impl EventHandler for RuntimeHandler {
    fn handle_event(&mut self, event: Event, ds: Box<DataStoreConn>) {
        match event {
            Event::Api(api) => self.handle_api_event(api),
            Event::Internal(internal) => {
                self.handle_internal_event(&internal, ds);
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
        }
    }

    fn handle_internal_event(&mut self, event: &InternalEvent, ds: Box<DataStoreConn>) {

        match *event {
            InternalEvent::EntitlementTimeout => {
                let get_license = self.license.create_trial_or_verify().unwrap();
                match get_license.0 {
                    LicenseStatus::TRIAL => {
                        info!{" ✓ All Good. You have a trial entitlement. !"}
                        self.license.update_license_status(
                            ds.clone(),
                            TRIAL.to_string(),
                            get_license.1,
                        );
                    }
                    LicenseStatus::ACTIVE => {
                        info!{" ✓ All Good. You have Rio/OS - Sensei entitlement. !"}
                        self.license.update_license_status(
                            ds.clone(),
                            ACTIVE.to_string(),
                            get_license.1,
                        );
                    }
                    LicenseStatus::EXPIRED => {
                        let expiry_attempt = self.license.hard_stop();
                        if expiry_attempt.is_err() {
                            self.license.update_license_status(
                                ds.clone(),
                                EXPIRY.to_string(),
                                get_license.1,
                            );
                        } else {
                            warn!("Expired, attempt: {:?}", expiry_attempt.unwrap())
                        }
                    }
                    LicenseStatus::INVALID => {
                        error!(
                            "Something going wrong with License,License Error:{:?}",
                            get_license.1
                        )
                    }
                }
            }
            InternalEvent::Shutdown => warn!("Shutting down...please wait!."),
        }
    }
}
