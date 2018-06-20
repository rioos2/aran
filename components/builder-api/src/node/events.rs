// Copyright 2018 The Rio Advancement Inc

use events::{Event, EventHandler, InternalEvent};
use node::runtime::{RuntimeHandler, ExternalMessage};

use api::audit::ledger;
use api::audit::mailer::email_sender;
use api::audit::mailer::PushNotifier;

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
                let notify = email_sender::EmailNotifier::new(event_envl, *self.mailer.clone());
                if notify.should_notify() {
                    notify.notify();
                }
            }
        }
    }

    fn handle_internal_event(&mut self, event: &InternalEvent) {
        match *event {
             /*InternalEvent::EntitlementTimeout => match self.license.create_trial_or_verify() {
                 Ok(()) => info!{" ✓ All Good. You have a valid entitlement. !"},
                 Err(err) => {
                     let expiry_attempt = self.license.hard_stop();
                     if expiry_attempt.is_err() {
                         error!("{:?}", err)
                     } else {
                         warn!("{:?}, Message: {:?}", expiry_attempt.unwrap(), err)
                     }                     
                 }
             },*/
            InternalEvent::EntitlementTimeout => info!{" ✓ All Good. You have a valid entitlement. !"},
            InternalEvent::Shutdown => warn!("Shutting down...please wait!."),
        }
    }
}

