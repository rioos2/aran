// Copyright 2018 The Rio Advancement Inc


use api::audit::PushNotifier;
use api::audit::ledger;
use api::audit::mailer::email_sender as mailer;
use api::audit::slack::slack_sender as slack;
use events::{Event, EventHandler, InternalEvent};
use node::runtime::{ExternalMessage, RuntimeHandler};
const SUB_PRODUCTS: [&'static str; 2] = ["senseis", "ninjas"];


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
                debug!("--> ledger config is {:?}", self.config);

                match ledger::from_config(&self.config) {
                    Ok(ledger) => {
                        match ledger.record(&event_envl) {
                            Ok(_) => debug!("--> save success"),

                            _ => debug!("--> save fail. {:?}", event_envl),
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
                match self.license.activate(license_id, &password, &product) {
                    Ok(_) => {
                        self.license.update(
                            &product,
                            &license_id.to_string(),
                            &password,
                        )
                    }
                    Err(err) => self.license.persist_error(&product, format!("{}", err)),
                }
            }

            ExternalMessage::DeActivateLicense(license_id, password, product) => {
                match self.license.deactivate(&product) {
                    Ok(_) => {
                        self.license.update(
                            &product,
                            &license_id.to_string(),
                            &password,
                        )
                    }
                    Err(err) => self.license.persist_error(&product, format!("{}", err)),
                }
            }

        }
    }

    fn handle_internal_event(&mut self, event: &InternalEvent) {
        match *event {
            InternalEvent::EntitlementTimeToVerify => {
                SUB_PRODUCTS
                    .iter()
                    .map(|x| {
                        self.license.live_verify(x).unwrap();
                        self.license.update(x, "", "");
                    })
                    .collect::<Vec<_>>();
            }
            InternalEvent::Shutdown => warn!("Shutting down...please wait!."),
        }
    }
}
