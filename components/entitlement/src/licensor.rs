// Copyright 2018 The Rio Advancement Inc
//

//! A module containing the middleware of the HTTP server

use config;
use config::Backend;
use db::data_store::DataStoreConn;
use entitlement::models::license;
use error::{Result, Error};
use licensecloud::LicenseCloud;
use nalperion::Nalperion;
use protocol::api::base::{MetaFields, WhoAmITypeMeta};
use protocol::api::licenses::{Licenses, LicenseStatus};
use protocol::api::schema::type_meta_url;

use softwarekey::SoftwareKey;

const ALLOWED_EXPIRY: u32 = 5;

pub trait LicenseClient: Send {
    // Returns the status of license verified with configured license tool
    // If there is a chance for starting a trial, then it does.
    // If there is the activation code then it used that to verify.
    fn verify(&self) -> Result<(LicenseStatus, String)>;

    fn hard_stop(&mut self) -> Result<String>;
}

#[derive(Debug)]
pub struct Client {
    pub backend: Backend,
    pub nalp: Nalperion,
    pub licensecloud: LicenseCloud,
    pub softwarekey: SoftwareKey,
    expiry_counter: u32,
}

impl Client {
    pub fn new<T: config::License>(config: &T) -> Self {
        Client {
            backend: config.backend(),
            nalp: Nalperion::new(config),
            licensecloud: LicenseCloud::new(config),
            softwarekey: SoftwareKey::new(config),
            expiry_counter: 0,
        }
    }

    // Returns the status of license verified with nalperion
    // If there is a chance for starting a trial, then it does.
    // If there is the activation code then it used that to verify.
    pub fn create_trial_or_verify(&self) -> Result<(LicenseStatus, String)> {
        let res = match self.backend {
            // Backend::LicenseCloud => self.licensecloud.verify(),
            Backend::SoftwareKey => self.softwarekey.verify(),
        };
        res
    }

    fn expiry_counter(&mut self) -> Result<String> {
        self.expiry_counter += 1;
        if self.expiry_counter > ALLOWED_EXPIRY {
            return Err(Error::TrialExpired);
        }
        Ok(format!(
            "{} [{}/{}]",
            Error::TrialExpired,
            self.expiry_counter,
            ALLOWED_EXPIRY
        ))
    }

    pub fn hard_stop(&mut self) -> Result<String> {
        self.expiry_counter()
    }

    pub fn update_license_status(&self, datastore: Box<DataStoreConn>, status: String, days: String) {
        let mut license = Licenses::new();

        let m = license.mut_meta(
            license.object_meta(),
            self.backend.to_string(),
            license.get_account(),
        );

        let jackie = license.who_am_i();

        license.set_meta(type_meta_url(jackie), m);

        license.set_status(status);
        license.set_expired(days);
        license::DataStore::new(&datastore).create_or_update(&license);
    }
}
