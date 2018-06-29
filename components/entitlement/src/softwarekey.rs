// Copyright 2018 The Rio Advancement Inc
//

//! A module containing the middleware of the HTTP server
use std::str;
use std::path::PathBuf;
use std::ptr;

use error::{Error, Result};
use handlebars::Handlebars;
use rand::{self, Rng};
use std::os::raw::*;
use std::ffi::CString;

use rio_core::fs::{read_from_file, rioconfig_config_path, rioconfig_license_path};

use config;
use lib_load;

/// These values should be set to your customer ID and
/// product ID. They are used to verify that the library
/// being accessed corresponds to your product.
const PRODUCT_ID: i32 = 408040;
const PRODUCT_OPTION_ID: i32 = 0;
const ENVELOPE_KEY: &'static str = "_EVALUATION_EXPIRES_2018-09-20_uvHZpaKXu4ZgA9qrYESVLhQe6o1JLEmecP/51yyLDW7oQ1Rs7vzQcCSe0VYPr8eL";

const SOFT_LIB_OPEN: &'static str = "SK_ApiContextInitialize";
const SOFT_LIB_DISPOSE: &'static str = "SK_ApiContextDispose";

const SK_FLAGS_NONE: i32 = 0;
const SK_FLAGS_APICONTEXTDISPOSE_SHUTDOWN: u32 = 1;

#[derive(Debug)]
pub struct SoftwareKey {
    fascade: API,
}

impl SoftwareKey {
    pub fn new<T: config::License>(config: &T) -> Self {
        SoftwareKey { fascade: API::new(config.so_file().to_string(), config.activation_code()) }
    }

    // Returns the status of license verified with nalperion
    pub fn verify(&self) -> Result<()> {
        self.fascade.check_license()?;
        Ok(())
    }
}

#[derive(Debug)]
struct API {
    so_file: String,
    activation_code: Option<String>,
}

impl API {
    fn new(so_file: String, activation_code: Option<String>) -> Self {
        API {
            so_file: so_file,
            activation_code: activation_code,
        }
    }

    fn check_license(&self) -> Result<()> {
        Self::call_dynamic(self.so_file.clone(), self.activation_code.clone())?;
        Ok(())
    }

    fn call_dynamic(so_file: String, activation_code: Option<String>) -> Result<()> {
        let lib = lib_load::Library::new(&rioconfig_license_path(None).join(so_file))?;
        unsafe {
            // open the nsl library and initialize the lib
            let open_fn = lib.get::<fn(c_int,
                  bool,
                  c_int,
                  c_int,
                  *const c_char,
                  *const c_char,
                  *mut c_void)
                  -> c_int>(SOFT_LIB_OPEN.as_bytes())?;

            let cb_raw = 0 as *mut c_void;

            let ret_val = open_fn(
                SK_FLAGS_NONE,
                false,
                PRODUCT_ID,
                PRODUCT_OPTION_ID,
                b"\0".as_ptr() as *const c_char,
                b"_EVALUATION_EXPIRES_2018-09-20_uvHZpaKXu4ZgA9qrYESVLhQe6o1JLEmecP/51yyLDW7oQ1Rs7vzQcCSe0VYPr8eL\0".as_ptr() as *const c_char,
                cb_raw,
            );

            debug!("=> open_lib: {:?}", ret_val);

            if ret_val != 0 {

                return SoftWareKeyResult::from_err(SOFT_LIB_OPEN);
            }


            // lib must be close (if not close `core dump` error occurs)

            let free_fn = lib.get::<fn(u32, Option<u32>) -> i32>(
                SOFT_LIB_DISPOSE.as_bytes(),
            )?;
            let ret_val = free_fn(SK_FLAGS_APICONTEXTDISPOSE_SHUTDOWN, None);
            debug!("=> free_lib: {:?}", ret_val);
            if ret_val != 0 {
                return SoftWareKeyResult::from_err(SOFT_LIB_DISPOSE);
            }

            Ok(())
        }
    }
}

enum SoftWareKeyResult {}

impl SoftWareKeyResult {
    // pub fn from_value(v: i32) -> Result<()> {
    //     match v {
    //         // Error can Generated based on nalperion error code refer link: https://naldoc.atlassian.net/wiki/spaces/NND/pages/426049/Developers+API+Latest
    //         -1 => Err(Error::ProductExpired),
    //         -113 => Err(Error::TrialExpired),
    //         -116 => Err(Error::SubscriptionExpired),
    //         _ => Ok(()),
    //     }
    // }
    pub fn from_err(name: &str) -> Result<()> {
        match name {
            SOFT_LIB_OPEN => Err(Error::LicenseAPINotFound),
            SOFT_LIB_DISPOSE => Err(Error::LicenseAPINotFound),
            _ => Ok(()),
        }
    }
}
