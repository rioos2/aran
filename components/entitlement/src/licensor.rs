// Copyright 2018 The Rio Advancement Inc
//

//! A module containing the middleware of the HTTP server

use error::{Result, Error};
use nalperion::Nalperion;
use config;

const ALLOWED_EXPIRY: u32 = 5;

#[derive(Debug)]
pub struct Client {
    pub nalp: Nalperion,
    expiry_counter: u32,
}

impl Client {
    pub fn new<T: config::License>(config: &T) -> Self {
        Client {
            nalp: Nalperion::new(config),
            expiry_counter: 0,
        }
    }

    // Returns the status of license verified with nalperion
    // If there is a chance for starting a trial, then it does.
    // If there is the activation code then it used that to verify.
    pub fn create_trial_or_verify(&self) -> Result<()> {
        self.nalp.verify()
    }

    fn expiry_counter(&mut self) -> Result<String> {
        self.expiry_counter += 1;
        if self.expiry_counter > ALLOWED_EXPIRY {
            return Err(Error::TrialExpired);
        }
        Ok(format!(
            "You License Trial Attempt {:?}",
            self.expiry_counter
        ))
    }

    pub fn hard_stop(&mut self) -> Result<String> {
        self.expiry_counter()
    }
}
