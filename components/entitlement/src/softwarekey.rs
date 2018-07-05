// Copyright 2018 The Rio Advancement Inc
//

//! A module containing the middleware of the HTTP server


use config;
use error::{Error, Result};
use handlebars::Handlebars;
use lib_load;
use licensor::LicenseStatus;
use rand::{self, Rng};

use rio_core::fs::{read_from_file, rioconfig_config_path, rioconfig_license_path};
use std::ffi::{CString, CStr};
use std::fs::File;
use std::os::raw::*;
use std::path::PathBuf;
use std::ptr;
use std::str;

/// These values should be set to your customer ID and
/// product ID. They are used to verify that the library
/// being accessed corresponds to your product.


type SK_ApiContext = c_long;
type SK_XmlDoc = c_long;
type SK_StringPointer = *const c_char;
type SK_DatePointer = *const c_char;

lazy_static! {
    static ref LICENSEFILE: PathBuf =
        PathBuf::from(&*rioconfig_license_path(None)
            .join("LicenseFile.lfx")
            .to_str()
            .unwrap());
}


const ENVELOPE: &'static str = "_EVALUATION_EXPIRES_2018-09-20_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";

const ENVELOPE_KEY: &'static str = "_EVALUATION_EXPIRES_2018-09-20_nlZW/s6JCUNiKeKvwqKBH5siPNxGFcNZdfdOZhaETsL1kG0uV3xHHiY7Vm06Oipn";
const VERSION: &'static str = "";

const SOFT_LIB_INIT: &'static str = "SK_ApiContextInitialize";
const SOFT_LIB_GET_LICENSE: &'static str = "SK_PLUS_LicenseGetXmlDocument";
const SOFT_LIB_DISPOSE: &'static str = "SK_ApiContextDispose";
const SOFT_LIB_NEW: &'static str = "SK_XmlElementAddNew";
const SOFT_LIB_SET: &'static str = "SK_ApiContextSetFieldString";
const SOFT_LIB_LOAD_LICENSE: &'static str = "SK_PLUS_LicenseFileLoad";
const SOFT_LIB_SET_INT: &'static str = "SK_ApiContextSetFieldInt";
const SOFT_LIB_CREATE_NEW_LICENSE: &'static str = "SK_PLUS_LicenseCreateNew";
const SOFT_LIB_SAVE: &'static str = "SK_PLUS_LicenseFileSave";
const SOFT_LIB_NODE_GET_INT: &'static str = "SK_XmlNodeGetValueInt";
const SOFT_LIB_NODE_GET_STRING: &'static str = "SK_XmlNodeGetValueString";
const SOFT_LIB_DATEREMAINING: &'static str = "SK_DateTimeDaysRemaining";
const SOFT_LIB_NODE_GET_DATE: &'static str = "SK_XmlNodeGetValueDateTimeString";

const PRODUCT_ID: c_int = 408040;
const PRODUCT_OPTION_ID: c_int = 30787;

const SK_FLAGS_NONE: c_int = 0x00000000;
const SK_FLAGS_USE_SSL: c_int = 0x00040000;
const SK_FLAGS_USE_ENCRYPTION: c_int = 0x00010000;
const SK_FLAGS_USE_SIGNATURE: c_int = 0x00020000;
const SK_FLAGS_APICONTEXTDISPOSE_SHUTDOWN: c_int = 0x00000001;

const INTERNAL: &'static str = "Internal_error";
const LICENSE_ERROR: &'static str = "License_error";

#[derive(Debug)]
pub struct SoftwareKey {
    fascade: API,
}

impl SoftwareKey {
    pub fn new<T: config::License>(config: &T) -> Self {
        SoftwareKey { fascade: API::new(config.so_file().to_string(), config.activation_code()) }
    }

    // Returns the status of license verified with nalperion
    pub fn verify(&self) -> Result<(LicenseStatus, String)> {
        Ok(self.fascade.check_license()?)
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

    fn check_license(&self) -> Result<(LicenseStatus, String)> {
        Ok(Self::call_dynamic(
            self.so_file.clone(),
            self.activation_code.clone(),
        )?)
    }

    fn call_dynamic(so_file: String, activation_code: Option<String>) -> Result<(LicenseStatus, String)> {
        let lib = lib_load::Library::new(&rioconfig_license_path(None).join(so_file))?;
        unsafe {

            let context: &mut SK_ApiContext = &mut 0;
            let license: &mut SK_XmlDoc = &mut 0;
            let licensePtr: &mut SK_XmlDoc = &mut 0;
            let valuePtr: &mut c_int = &mut 0;

            let mut license_type = 0;

            // open the softewarekey library and initialize the lib
            let init_fn = lib.get::<fn(c_int,
                  bool,
                  c_int,
                  c_int,
                  *const c_char,
                  *const c_char,
                  *mut SK_ApiContext)
                  -> c_int>(SOFT_LIB_INIT.as_bytes())?;

            //set the string filed to api_context
            let set_fn_str = lib.get::<fn(SK_ApiContext, c_int, c_int, *const c_char) -> c_int>(SOFT_LIB_SET.as_bytes())?;

            //set the int filed to api_context
            let set_fn_int = lib.get::<fn(SK_ApiContext, c_int, c_int, c_int) -> c_int>(
                SOFT_LIB_SET_INT.as_bytes(),
            )?;

            //load the license file
            let license_load_fn = lib.get::<fn(SK_ApiContext, c_int, *const c_char) -> c_int>(SOFT_LIB_LOAD_LICENSE.as_bytes())?;

            //save the license file
            let license_save_fn = lib.get::<fn(SK_ApiContext, c_int, *const c_char, SK_XmlDoc) -> c_int>(SOFT_LIB_SAVE.as_bytes())?;

            //get the license xml document
            let license_get_xml_doc = lib.get::<fn(SK_ApiContext, c_int, *mut SK_XmlDoc) -> c_int>(SOFT_LIB_GET_LICENSE.as_bytes())?;

            //get the license node int type
            let node_get_int = lib.get::<fn(c_int, SK_XmlDoc, *const c_char, *mut c_int) -> c_int>(SOFT_LIB_NODE_GET_INT.as_bytes())?;

            //get the license node date type
            let node_get_date = lib.get::<fn(c_int, SK_XmlDoc, *const c_char, *mut SK_DatePointer) -> c_int>(SOFT_LIB_NODE_GET_DATE.as_bytes())?;

            //get the license node string type
            let node_get_string = lib.get::<fn(c_int,
                  SK_XmlDoc,
                  *const c_char,
                  bool,
                  *mut SK_StringPointer)
                  -> c_int>(SOFT_LIB_NODE_GET_STRING.as_bytes())?;

            //get remain days of license
            let license_remaining_day = lib.get::<fn(c_int, *const c_char, *mut c_int) -> c_int>(
                SOFT_LIB_DATEREMAINING.as_bytes(),
            )?;

            //dispose the library
            let free_fn = lib.get::<fn(c_int, *mut SK_ApiContext) -> c_int>(
                SOFT_LIB_DISPOSE.as_bytes(),
            )?;

            //start the license process
            let mut result = init_fn(
                SK_FLAGS_USE_SSL | SK_FLAGS_USE_ENCRYPTION | SK_FLAGS_USE_SIGNATURE,
                false,
                PRODUCT_ID,
                PRODUCT_OPTION_ID,
                CString::new(VERSION).unwrap().into_raw(),
                CString::new(ENVELOPE).unwrap().into_raw(),
                context,
            );

            debug!("=> open_lib: {:?}", result);

            if result != 0 {
                return check_resut(result, SOFT_LIB_INIT, INTERNAL);
            }

            //set the ENVELOPE_KEY to the api_context
            result = set_fn_str(
                *context,
                SK_FLAGS_NONE,
                1,
                CString::new(ENVELOPE_KEY).unwrap().into_raw(),
            );


            if result != 0 && result != 9019 {
                return check_resut(result, SOFT_LIB_SET, INTERNAL);
            }

            try!(File::create(
                rioconfig_license_path(None).join("LicenseFile.lfx"),
            ));

            result = set_fn_int(*context, SK_FLAGS_NONE, 4, 0);

            debug!("=> set_fn_int: {:?}", result);

            if result != 0 {
                return check_resut(result, SOFT_LIB_SET_INT, INTERNAL);
            }

            result = license_load_fn(
                *context,
                SK_FLAGS_NONE,
                CString::new(LICENSEFILE.to_str().unwrap())
                    .unwrap()
                    .into_raw(),
            );

            debug!("=> before set write access license_load_fn: {:?}", result);

            if result == 9200 || result == 9005 {

                result = set_fn_int(*context, SK_FLAGS_NONE, 4, 1);

                debug!("=> set_fn_int: {:?}", result);

                if result != 0 {
                    return check_resut(result, SOFT_LIB_SET_INT, INTERNAL);
                }

                result = license_load_fn(
                    *context,
                    SK_FLAGS_NONE,
                    CString::new(LICENSEFILE.to_str().unwrap())
                        .unwrap()
                        .into_raw(),
                );

                debug!("=> after set write access license_load_fn: {:?}", result);

                if result == 9200 {
                    let license_create_fn = lib.get::<fn(SK_ApiContext, c_int, c_int, *mut SK_XmlDoc) -> c_int>(SOFT_LIB_CREATE_NEW_LICENSE.as_bytes())?;
                    result = license_create_fn(*context, SK_FLAGS_NONE, 30, license);
                    debug!("=> license_create_fn: {:?}", result);

                    if result != 0 {
                        return check_resut(result, SOFT_LIB_CREATE_NEW_LICENSE, INTERNAL);
                    }

                    result = license_save_fn(
                        *context,
                        SK_FLAGS_NONE,
                        CString::new(LICENSEFILE.to_str().unwrap())
                            .unwrap()
                            .into_raw(),
                        *license,
                    );

                    debug!("=> license_save_fn: {:?}", result);

                    if result != 0 {
                        return check_resut(result, SOFT_LIB_SAVE, INTERNAL);
                    }
                }
                if result != 0 {
                    return check_resut(result, SOFT_LIB_LOAD_LICENSE, INTERNAL);
                }
            }


            result = license_get_xml_doc(*context, SK_FLAGS_NONE, licensePtr);

            debug!("=> license_get_xml_doc: {:?}", result);

            if result != 0 {
                return check_resut(result, SOFT_LIB_GET_LICENSE, INTERNAL);
            }


            result = node_get_int(
                SK_FLAGS_NONE,
                *licensePtr,
                CString::new("/SoftwareKey/PrivateData/License/TriggerCode")
                    .unwrap()
                    .into_raw(),
                valuePtr,
            );


            debug!("=> node_get_int: {:?}", result);

            if result != 0 {
                license_type = 0
            }

            // if LicenseType::TimeLimited as u32 == license_type {
            //
            // }

            let licensePtr_1: &mut SK_XmlDoc = &mut 0;

            let strvaluePtr: &mut SK_StringPointer = &mut (0 as *const c_char);

            result = license_get_xml_doc(*context, SK_FLAGS_NONE, licensePtr_1);

            debug!("=> license_get_xml_doc: {:?}", result);

            if result != 0 {
                return check_resut(result, SOFT_LIB_GET_LICENSE, INTERNAL);
            }

            result = node_get_string(
                SK_FLAGS_NONE,
                *licensePtr_1,
                CString::new("/SoftwareKey/PrivateData/License/InstallationID")
                    .unwrap()
                    .into_raw(),
                false,
                strvaluePtr,
            );

            debug!("=> node_get_string: {:?}", result);

            if result != 0 && result != 9014 {
                return check_resut(result, SOFT_LIB_NODE_GET_STRING, INTERNAL);
            }


            if *strvaluePtr == 0 as *const c_char {

                let licensePtr_2: &mut SK_XmlDoc = &mut 0;

                let datevaluePtr: &mut SK_DatePointer = &mut (0 as *const c_char);

                result = license_get_xml_doc(*context, SK_FLAGS_NONE, licensePtr_2);

                debug!("=> license_get_xml_doc: {:?}", result);

                if result != 0 {
                    return check_resut(result, SOFT_LIB_GET_LICENSE, INTERNAL);
                }

                result = node_get_date(
                    SK_FLAGS_NONE,
                    *licensePtr_2,
                    CString::new("/SoftwareKey/PrivateData/License/EffectiveEndDate")
                        .unwrap()
                        .into_raw(),
                    datevaluePtr,
                );

                debug!("=> node_get_int: {:?}", result);

                if result != 0 {
                    return check_resut(result, SOFT_LIB_NODE_GET_DATE, INTERNAL);
                }

                let dayPtr: &mut c_int = &mut 0;

                result = license_remaining_day(SK_FLAGS_NONE, *datevaluePtr, dayPtr);

                if result != 0 {
                    return check_resut(result, SOFT_LIB_DATEREMAINING, INTERNAL);
                }

                debug!("=> done trial");

                return Ok((LicenseStatus::TRIAL, (*dayPtr).to_string()));
            }

            result = free_fn(SK_FLAGS_NONE, context);

            debug!("=> free_lib: {:?}", result);

            if result != 0 {
                return check_resut(result, SOFT_LIB_GET_LICENSE, INTERNAL);
            }

            Ok((LicenseStatus::ACTIVE, "".to_string()))

        }
    }
}

enum SoftWareKeyResult {}

impl SoftWareKeyResult {
    pub fn from_err(value: i32, name: &str, err_type: &str) -> Result<(LicenseStatus, String)> {
        match ErrorType::get_error_type(err_type) {
            ErrorType::INTERNAL => {
                match name {
                    SOFT_LIB_INIT |
                    SOFT_LIB_DISPOSE |
                    SOFT_LIB_SET |
                    SOFT_LIB_GET_LICENSE |
                    SOFT_LIB_LOAD_LICENSE |
                    SOFT_LIB_SET_INT |
                    SOFT_LIB_CREATE_NEW_LICENSE |
                    SOFT_LIB_SAVE |
                    SOFT_LIB_NODE_GET_STRING |
                    SOFT_LIB_NODE_GET_DATE |
                    SOFT_LIB_DATEREMAINING => {
                        debug!(
                            "=> Internal Error in Processing SoftwareKey License. Error Code : {}",
                            value
                        );
                        Ok((LicenseStatus::UNKNOWN, "".to_string()))
                    }
                    _ => Ok((LicenseStatus::UNKNOWN, "".to_string())),
                }
            }
            ErrorType::LICENSE_ERROR => {
                match name {
                    _ => Ok((LicenseStatus::UNKNOWN, "".to_string())),
                }
            }

        }


    }
}

enum LicenseType {
    Unlicensed = 0,
    FullNonExpiring = 1,
    TimeLimited = 10,
}

fn check_resut(value: i32, name: &str, err_type: &str) -> Result<(LicenseStatus, String)> {
    return SoftWareKeyResult::from_err(value, name, err_type);
}

enum ErrorType {
    INTERNAL,
    LICENSE_ERROR,
}

impl ErrorType {
    fn get_error_type(err_type: &str) -> ErrorType {
        match err_type {
            INTERNAL => ErrorType::INTERNAL,
            LICENSE_ERROR => ErrorType::LICENSE_ERROR,
            _ => ErrorType::INTERNAL,
        }
    }
}
