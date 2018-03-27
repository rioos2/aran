// Copyright 2018 The Rio Advancement Inc


use events::{Event, EventHandler, InternalEvent};
use node::runtime::{RuntimeHandler, ExternalMessage};
use error::Result;

use api::audit::ledger;

impl EventHandler for RuntimeHandler {
    fn handle_event(&mut self, event: Event) -> Result<()> {
        match event {
            Event::Api(api) => self.handle_api_event(api),
            Event::Internal(internal) => self.handle_internal_event(internal),
        }
    }
}

//Send the stuff to the audit block chain server
impl RuntimeHandler {
    fn handle_api_event(&mut self, event: ExternalMessage) -> Result<()> {
        match event {
            ExternalMessage::PeerAdd(event_envl) => {
                println!("--> ledger config is {:?}", self.config);

                match ledger::from_config(&self.config) {
                    Ok(ledger) => {
                        match ledger.record(&event_envl) {
                            Ok(_) => {
                                println!("--> save success");
                                Ok(())
                            }
                            _ => {
                                println!("--> save fail. {:?}", event_envl);
                                Ok(())
                            }
                        }
                    }
                    _ => {
                        println!("--> ledger load  fail.");
                        Ok(())
                    }
                }
            }
        }
    }

    fn handle_internal_event(&mut self, event: InternalEvent) -> Result<()> {
        match event {
            InternalEvent::Shutdown => {
                match self.license.create_trial_or_verify() {
                    Ok(()) => Ok(()),
                    Err(err) => {
                        println!(
                            "---------------------------------------------------------------{:?}",
                            err
                        );
                        Ok(())
                    }
                }
            }
        }
    }
}
