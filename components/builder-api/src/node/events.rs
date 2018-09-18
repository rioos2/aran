// Copyright 2018 The Rio Advancement Inc

#![allow(non_snake_case)]

use api::blockchain::PushNotifier;
use api::blockchain::ledger;
use api::blockchain::mailer::email_sender as mailer;
use api::blockchain::slack::slack_sender as slack;
use events::{Event, EventHandler, InternalEvent};
use node::runtime::{ExternalMessage, RuntimeHandler};

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
            ExternalMessage::EmitEvent(event_envl) => {
                debug!("--> ledger config is {:?}", self.config);

                match ledger::from_config(&self.config) {
                    Ok(ledger) => {
                        match ledger.record_event(&event_envl) {
                            Ok(_) => debug!("--> event save success"),

                            _ => debug!("--> event save fail. {:?}", event_envl),
                        };
                    }
                    _ => debug!("--> ledger load  fail."),
                }
            }
            ExternalMessage::EmitAudit(event_envl) => {
                debug!("--> ledger config is {:?}", self.config);

                match ledger::from_config(&self.config) {
                    Ok(ledger) => {
                        match ledger.record_audit(&event_envl) {
                            Ok(_) => debug!("--> audit save success"),

                            _ => debug!("--> audit save fail. {:?}", event_envl),
                        };
                    }
                    _ => debug!("--> ledger load  fail."),
                }
            }
            ExternalMessage::PushNotification(event_envl) => {
                let e = event_envl.clone();
                mailer::EmailNotifier::new(e, *self.mailer.clone()).notify();
                slack::SlackNotifier::new(event_envl, *self.slack.clone()).notify();
            }

            ExternalMessage::ActivateLicense(license_id, password, product) => {
                match &product[..] {
                    NINJAS => {
                        match self.ninjas_license.activate(license_id, &password) {
                            Ok(_) => {
                                self.ninjas_license.update(
                                    &license_id.to_string(),
                                    &password,
                                )
                            }
                            Err(err) => self.ninjas_license.persist_error(format!("{}", err)),
                        }
                    }
                    SENSEIS => {
                        match self.senseis_license.activate(license_id, &password) {
                            Ok(_) => {
                                self.senseis_license.update(
                                    &license_id.to_string(),
                                    &password,
                                )
                            }
                            Err(err) => self.senseis_license.persist_error(format!("{}", err)),
                        }
                    }
                    _ => {}

                }
            }

            ExternalMessage::DeActivateLicense(license_id, password, product) => {
                match &product[..] {
                    NINJAS => {
                        match self.ninjas_license.deactivate() {
                            Ok(_) => {
                                self.ninjas_license.update(
                                    &license_id.to_string(),
                                    &password,
                                )
                            }
                            Err(err) => self.ninjas_license.persist_error(format!("{}", err)),
                        }
                    }
                    SENSEIS => {
                        match self.senseis_license.deactivate() {
                            Ok(_) => {
                                self.senseis_license.update(
                                    &license_id.to_string(),
                                    &password,
                                )
                            }
                            Err(err) => self.senseis_license.persist_error(format!("{}", err)),
                        }
                    }
                    _ => {}
                }
            }

        }
    }

    fn handle_internal_event(&mut self, event: &InternalEvent) {
        match *event {
            InternalEvent::EntitlementTimeToVerify => {
                self.ninjas_license.live_verify().unwrap();
                self.ninjas_license.update_status();
                self.senseis_license.live_verify().unwrap();
                self.senseis_license.update_status();
            }
            InternalEvent::Shutdown => warn!("Shutting down...please wait!."),
        }
    }
}
