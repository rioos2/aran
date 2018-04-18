// Copyright 2018 The Rio Advancement Inc

use events::{Event, EventHandler, InternalEvent};
use node::runtime::{RuntimeHandler, ExternalMessage};
use futures::Stream;
use std::sync::Arc;
use tokio_core::reactor::Core;

use api::audit::ledger;
use tokio_timer;
use std::time::Duration;

impl EventHandler for RuntimeHandler {
    fn handle_event(&mut self, event: Event) {
        match event {
            Event::Api(api) => self.handle_api_event(api),
            Event::Internal(internal) => {
                let mut core = Core::new().unwrap();
                let rx = Arc::new(internal);
                let rx = &*rx.clone();
                let duration = Duration::new(3600, 0); // 10 minutes
                let builder = tokio_timer::wheel().max_timeout(duration);
                let wakeups = builder.build().interval(duration);
                let task = wakeups.for_each(|_| {
                    self.handle_internal_event(rx);
                    Ok(())
                });
                core.run(task).unwrap();
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
        }
    }

    fn handle_internal_event(&mut self, event: &InternalEvent) {
        match *event {
            InternalEvent::EntitlementTimeout => match self.license.create_trial_or_verify() {
                Ok(()) => info!{" ✓ All Good. You have a valid entitlement. !"},
                Err(err) => {
                    let expiry_attempt = self.license.hard_stop();
                    if expiry_attempt.is_err() {
                        error!("{:?}", err)
                    }
                    warn!("{:?}, Message: {:?}", expiry_attempt.unwrap(), err)
                }
            },
            InternalEvent::Shutdown => warn!("Shutting down...please wait!."),
        }
    }
}
