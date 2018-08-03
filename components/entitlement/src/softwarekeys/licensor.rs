// Copyright 2018 The Rio Advancement Inc

//! An entitlement for the Rio/OS using SoftwareKey.com


use super::*;
use auth::rbac::license::LicensesFascade;
use entitlement::models::license;
use error::{Result, Error, ResultCode};
use lib_load;
use lib_load::{Symbol, Library};

use protocol::api::base::{MetaFields, WhoAmITypeMeta};
use protocol::api::licenses::{Licenses, INVALID, TRIAL, ACTIVE, EXPIRY};
use protocol::api::schema::type_meta_url;
use rio_core::fs::rioconfig_license_path;
use std::collections::BTreeMap;
use std::ffi::{CString, CStr};
use std::fs::File;
use std::os::raw::*;
use std::path::PathBuf;

//The LicenseFile.lfx is generated upon registration in SoftwareKey.com
lazy_static! {
    static ref LICENSEFILE: PathBuf =
        PathBuf::from(&*rioconfig_license_path(None)
            .join("LicenseFile.lfx")
            .to_str()
            .unwrap());
}

const PRODUCT_ID: c_int = 409264;

const PRODUCT_OPTION_ID: c_int = 0;

const ENVELOPE: &'static str = "_EVALUATION_EXPIRES_2018-09-20_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";

const ENVELOPE_KEY: &'static str = "_EVALUATION_EXPIRES_2018-09-20_nlZW/s6JCUNiKeKvwqKBH5siPNxGFcNZdfdOZhaETsL1kG0uV3xHHiY7Vm06Oipn";

const VERSION: &'static str = "";

const SK_FLAGS_NONE: c_int = 0x00000000;
const SK_FLAGS_USE_SSL: c_int = 0x00040000;
const SK_FLAGS_USE_ENCRYPTION: c_int = 0x00010000;
const SK_FLAGS_USE_SIGNATURE: c_int = 0x00020000;
const SK_FLAGS_APICONTEXTDISPOSE_SHUTDOWN: c_int = 0x00000001;

pub struct NativeSDK {
    lib: Library, //sdk file for the licensor
    context: SK_ApiContext, //The API Context may only represent a single License File.
    cache: LicensesFascade, // A cache backed by database to pull License
    license_file: SK_XmlDoc, //Handle to an XML document in memory.
    isLoaded: bool, //represent the license file is loaded or not
    isWritable: bool, //set read and write permission for license file
    licenseFilePath: String, //file path of license which is loaded in directory
    provider: String, //license provider name
    activation: i32,
    status: String,
    remaining_days: String,
}

impl NativeSDK {
    pub fn new(lib: Library, license: LicensesFascade) -> Self {
        NativeSDK {
            lib: lib,
            cache: license,
            context: 0,
            license_file: 0,
            isLoaded: false,
            isWritable: false,
            licenseFilePath: "".to_string(),
            provider: "SoftwareKey".to_string(),
            activation: 0,
            status: TRIAL.to_string(),
            remaining_days: "".to_string(),
        }
    }
    //Initializes a new API Context, which may be used to open and manipulate a license file.
    //1.init_fn initial the SK_INIT and verifies if a symbol by name SK_INIT exists
    //2.set_fn_str set the SK_SET string value to the API context (link)
    //3.invoke init_fn funtion with as per(link)
    //
    pub fn initialize_api_context(&mut self) -> Result<()> {
        let context: &mut SK_ApiContext = &mut 0;
        unsafe {
            let init_fn = *self.lib.get::<fn(c_int,
                       bool,
                       c_int,
                       c_int,
                       *const c_char,
                       *const c_char,
                       *mut SK_ApiContext)
                       -> c_int>(SK_INIT.as_bytes())?;

            let set_fn_str = *self.lib
                .get::<fn(SK_ApiContext, c_int, c_int, *const c_char) -> c_int>(SK_SET.as_bytes())?;

            self.check_result(init_fn(
                SK_FLAGS_USE_SSL | SK_FLAGS_USE_ENCRYPTION |
                    SK_FLAGS_USE_SIGNATURE,
                false,
                PRODUCT_ID,
                PRODUCT_OPTION_ID,
                CString::new(VERSION).unwrap().into_raw(),
                CString::new(ENVELOPE).unwrap().into_raw(),
                context,
            ))?;

            //set the ENVELOPE_KEY to the api_context
            let result = set_fn_str(
                *context,
                SK_FLAGS_NONE,
                1,
                CString::new(ENVELOPE_KEY).unwrap().into_raw(),
            );

            self.context = *context;

            if result != ResultCode::SK_ERROR_NONE as i32 && result != ResultCode::SK_ERROR_PLUS_EVALUATION_WARNING as i32 {
                self.check_result(result)?;
            }
            self.initialize_system_identitifers()?;
            Ok(())
        }
    }

    //identify the system license_file is load
    fn initialize_system_identitifers(&mut self) -> Result<()> {
        unsafe {
            //identify the current system
            let system_identifiers = self.lib.get::<fn(SK_ApiContext,
                       c_int,
                       c_int,
                       *const c_char,
                       *mut c_int)
                       -> c_int>(SK_IDNTIFIER_ALGORITHAM.as_bytes())?;
            //system_identifiers
            let countPtr: &mut c_int = &mut 0;
            // Make sure we have a computer name identifier
            self.check_result(system_identifiers(
                self.context,
                SK_FLAGS_NONE,
                20,
                0 as *const c_char,
                countPtr,
            ))?;

            if 0 == *countPtr {
                return self.check_result(ResultCode::SK_ERROR_INVALID_DATA as i32);
            }
            // Make sure we have a hard disk volume serial identifier
            self.check_result(system_identifiers(
                self.context,
                SK_FLAGS_NONE,
                30,
                0 as *const c_char,
                countPtr,
            ))?;
            if 0 == *countPtr {
                return self.check_result(ResultCode::SK_ERROR_INVALID_DATA as i32);
            }
            self.check_result(system_identifiers(
                self.context,
                SK_FLAGS_NONE,
                10,
                0 as *const c_char,
                countPtr,
            ))?;
            Ok(())
        }
    }

    //load the license file in the local directory
    pub fn load_license(&mut self) -> Result<()> {
        try!(File::create(
            rioconfig_license_path(None).join("LicenseFile.lfx"),
        ));
        self.set_license_path(LICENSEFILE.to_str().unwrap().to_string());
        self.reload()?;
        Ok(())
    }

    fn set_license_path(&mut self, licenseFilePath: String) {
        self.licenseFilePath = licenseFilePath
    }
    //
    pub fn reload(&mut self) -> Result<()> {
        self.isLoaded = false;
        self.set_writable(false)?;
        //load the license file
        unsafe {
            let license_load_fn = *self.lib
                .get::<fn(SK_ApiContext, c_int, *const c_char) -> c_int>(SK_LOAD_LICENSE.as_bytes())?;

            let mut result = license_load_fn(
                self.context,
                SK_FLAGS_NONE,
                CString::new(self.licenseFilePath.clone())
                    .unwrap()
                    .into_raw(),
            );
            if result == ResultCode::SK_ERROR_COULD_NOT_LOAD_LICENSE as i32 || result == ResultCode::SK_ERROR_VERIFICATION_FAILED as i32 {
                self.set_writable(true)?;
                result = license_load_fn(
                    self.context,
                    SK_FLAGS_NONE,
                    CString::new(self.licenseFilePath.clone())
                        .unwrap()
                        .into_raw(),
                );
            }

            if result == ResultCode::SK_ERROR_COULD_NOT_LOAD_LICENSE as i32 {
                self.create_trial(30)?;
            } else if result == ResultCode::SK_ERROR_NONE as i32 {
                self.isLoaded = true;
            } else {
                return self.check_result(result);
            }
            self.live_verify()?;
        }
        Ok(())
    }

    fn set_writable(&mut self, isWritable: bool) -> Result<()> {
        unsafe {
            self.isWritable = isWritable;
            let mut val: c_int = 0;

            let set_fn_int = self.lib
                .get::<fn(SK_ApiContext, c_int, c_int, c_int) -> c_int>(SK_SET_INT.as_bytes())?;

            if isWritable {
                val = 1;
            } else {
                val = 0;
            }

            self.check_result(
                set_fn_int(self.context, SK_FLAGS_NONE, 4, val),
            )?;

            Ok(())
        }
    }

    //create 30 days of trial
    fn create_trial(&mut self, days: c_int) -> Result<()> {
        let license: &mut SK_XmlDoc = &mut 0;
        unsafe {
            let license_create_fn = self.lib
                .get::<fn(SK_ApiContext, c_int, c_int, *mut SK_XmlDoc) -> c_int>(SK_CREATE_NEW_LICENSE.as_bytes())?;

            let license_save_fn = self.lib
                .get::<fn(SK_ApiContext, c_int, *const c_char, SK_XmlDoc) -> c_int>(SK_SAVE.as_bytes())?;

            self.check_result(license_create_fn(
                self.context,
                SK_FLAGS_NONE,
                days,
                license,
            ))?;

            self.check_result(license_save_fn(
                self.context,
                SK_FLAGS_NONE,
                CString::new(self.licenseFilePath.clone())
                    .unwrap()
                    .into_raw(),
                *license,
            ))?;

            self.isLoaded = true;
            self.license_file = *license;

            //TO-DO clean xml doc
            Ok(())
        }
    }
    //Verifies license valid or not and update in database
    pub fn live_verify(&mut self) -> Result<()> {

        let is_valid_remote: bool = self.validate()?;
        if self.is_evaluation()? {
            if is_valid_remote {
                let name = vec!["senseis", "ninjas"];
                for x in name {
                    self.create_trial_in_db(TRIAL.to_string(), self.get_days_remaining()?.to_string(), x);
                }
            } else {
                self.status = EXPIRY.to_string();
            }
            return Ok(());
        }
        if is_valid_remote {
            if self.get_type()? as i32 == LicenseType::TimeLimited as i32 {
                self.status = ACTIVE.to_string();
                self.remaining_days = self.get_days_remaining()?.to_string();
            } else {
                self.status = ACTIVE.to_string();
            }
        } else {
            self.status = INVALID.to_string();
        }
        Ok(())
    }
    //validate the license_file and update the status in database
    fn validate(&mut self) -> Result<bool> {
        unsafe {
            if !self.isLoaded {
                return Ok(false);
            }
            let valuePtr: &mut SK_IntPointer = &mut 0;
            let daysPtr: &mut SK_IntPointer = &mut 0;
            let countPtr: &mut SK_IntPointer = &mut 0;
            let matchesPtr: &mut SK_IntPointer = &mut 0;

            let date_time_validate = *self.lib
                .get::<fn(c_int, c_int, c_int, *mut SK_IntPointer) -> c_int>(SK_DATETIME_VALIDATE.as_bytes())?;

            let compare_system_identifier = *self.lib.get::<fn(SK_ApiContext,
                       c_int,
                       *const c_char,
                       *mut SK_IntPointer,
                       *mut SK_IntPointer)
                       -> c_int>(SK_SYSTEM_IDENTIFIER_COMPARE.as_bytes())?;

            self.check_result(
                date_time_validate(SK_FLAGS_NONE, 0, 0, valuePtr),
            )?;

            self.check_result(compare_system_identifier(
                self.context,
                SK_FLAGS_NONE,
                CString::new("").unwrap().into_raw(),
                countPtr,
                matchesPtr,
            ))?;

            if *matchesPtr < 1 {
                return Ok(false);
            }
            if !(self.is_date_time_past(
                "/SoftwareKey/PrivateData/License/EffectiveStartDate",
            )?)
            {
                debug!(
                    "{:?}",
                    ResultCode::err_description(ResultCode::SK_ERROR_LICENSE_NOT_EFFECTIVE_YET as i32)
                );
                return Ok(false);
            }
            if self.is_evaluation()? || self.get_type()? as i32 == LicenseType::TimeLimited as i32 {
                if !self.is_date_time_past(
                    "/SoftwareKey/PrivateData/License/EffectiveStartDate",
                )?
                {
                    debug!(
                        "{:?}",
                        ResultCode::err_description(ResultCode::SK_ERROR_LICENSE_NOT_EFFECTIVE_YET as i32).to_string()
                    );
                    return Ok(false);
                }
                if self.get_days_remaining()? <= 0 {
                    debug!(
                        "{:?}",
                            ResultCode::err_description(
                                ResultCode::SK_ERROR_LICENSE_EXPIRED as i32,
                            ).to_string(),
                    );
                    return Ok(false);
                }
            }
            Ok(true)
        }
    }
    //(link)
    fn is_evaluation(&self) -> Result<bool> {
        if self.get_string_value(
            "/SoftwareKey/PrivateData/License/InstallationID",
        )? == "".to_string()
        {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    fn is_date_time_past(&mut self, xpath: &str) -> Result<bool> {
        let mut ret_val: bool = true;
        unsafe {
            let nowPtr: &mut SK_StringPointer = &mut (0 as *const c_char);
            let comparisonPrt: &mut SK_IntPointer = &mut 0;
            let current_datetime = *self.lib.get::<fn(c_int, *mut SK_StringPointer) -> c_int>(
                SK_DATETIME_GET_CURRENT_STRING.as_bytes(),
            )?;
            let compare_datetime = *self.lib.get::<fn(c_int,
                       *const c_char,
                       SK_StringPointer,
                       *mut SK_IntPointer)
                       -> c_int>(SK_DATETIME_COMPARE_STRING.as_bytes())?;

            self.check_result(current_datetime(SK_FLAGS_NONE, nowPtr))?;

            self.check_result(compare_datetime(
                SK_FLAGS_NONE,
                CString::new(self.get_date_time_string_value(xpath)?)
                    .unwrap()
                    .into_raw(),
                *nowPtr,
                comparisonPrt,
            ))?;

            if *comparisonPrt <= 0 {
                ret_val = true;
            } else {
                ret_val = false;
            }

            Ok(ret_val)
        }
    }
    //returns the requested date time value from the license file
    fn get_date_time_string_value(&self, xpath: &str) -> Result<String> {
        unsafe {
            if !self.isLoaded {
                return Ok("".to_string());
            }
            let licensePtr: &mut SK_XmlDoc = &mut 0;
            let valuePtr: &mut SK_StringPointer = &mut (0 as *const c_char);

            let license_get_xml_doc = self.lib
                .get::<fn(SK_ApiContext, c_int, *mut SK_XmlDoc) -> c_int>(SK_GET_LICENSE.as_bytes())?;

            let node_get_date = self.lib.get::<fn(c_int,
                       SK_XmlDoc,
                       *const c_char,
                       *mut SK_DatePointer)
                       -> c_int>(SK_NODE_GET_DATE.as_bytes())?;

            if self.check_result(license_get_xml_doc(self.context, SK_FLAGS_NONE, licensePtr))
                .is_ok() &&
                self.check_result(node_get_date(
                    SK_FLAGS_NONE,
                    *licensePtr,
                    CString::new(xpath).unwrap().into_raw(),
                    valuePtr,
                )).is_ok()
            {
                // SK_XmlDocumentDispose(SK_FLAGS_NONE, licensePtr);
                let val = CStr::from_ptr(*valuePtr).to_str().unwrap();
                return Ok(val.to_string());
            } else {
                return Ok("".to_string());
            }
        }

    }

    //returns the requested string value from the license file
    fn get_string_value(&self, xpath: &str) -> Result<String> {
        unsafe {
            if !self.isLoaded {
                return Ok("".to_string());
            }
            let licensePtr: &mut SK_XmlDoc = &mut 0;
            let valuePtr: &mut SK_StringPointer = &mut (0 as *const c_char);

            let license_get_xml_doc = self.lib
                .get::<fn(SK_ApiContext, c_int, *mut SK_XmlDoc) -> c_int>(SK_GET_LICENSE.as_bytes())?;

            let node_get_string = self.lib.get::<fn(c_int,
                       SK_XmlDoc,
                       *const c_char,
                       bool,
                       *mut SK_StringPointer)
                       -> c_int>(SK_NODE_GET_STRING.as_bytes())?;

            if self.check_result(license_get_xml_doc(self.context, SK_FLAGS_NONE, licensePtr))
                .is_ok() &&
                self.check_result(node_get_string(
                    SK_FLAGS_NONE,
                    *licensePtr,
                    CString::new(xpath).unwrap().into_raw(),
                    false,
                    valuePtr,
                )).is_ok()
            {
                // SK_XmlDocumentDispose(SK_FLAGS_NONE, licensePtr);
                let val = CStr::from_ptr(*valuePtr).to_str().unwrap();
                return Ok(val.to_string());
            } else {
                return Ok("".to_string());
            }
        }
    }

    fn get_type(&self) -> Result<LicenseType> {
        unsafe {
            let mut license_type = LicenseType::Unlicensed;
            let licensePtr: &mut SK_XmlDoc = &mut 0;
            let license_get_xml_doc = *self.lib
                .get::<fn(SK_ApiContext, c_int, *mut SK_XmlDoc) -> c_int>(SK_GET_LICENSE.as_bytes())?;
            self.check_result(license_get_xml_doc(
                self.context,
                SK_FLAGS_NONE,
                licensePtr,
            ))?;
            license_type = self.determine_type(*licensePtr)?;
            // SK_XmlDocumentDispose(SK_FLAGS_NONE, licensePtr);
            return Ok(license_type);
        }

    }

    //determine the type of license used
    fn determine_type(&self, licensePtr: SK_XmlDoc) -> Result<LicenseType> {
        unsafe {
            let valuePtr: &mut SK_IntPointer = &mut 0;
            let node_get_int = self.lib
                .get::<fn(c_int, SK_XmlDoc, *const c_char, *mut c_int) -> c_int>(SK_NODE_GET_INT.as_bytes())?;

            let result = node_get_int(
                SK_FLAGS_NONE,
                licensePtr,
                CString::new("/SoftwareKey/PrivateData/License/TriggerCode")
                    .unwrap()
                    .into_raw(),
                valuePtr,
            );

            if result != ResultCode::SK_ERROR_NONE as i32 {
                return Ok(LicenseType::Unlicensed);
            }

            match *valuePtr {
                1 | 18 | 28 => Ok(LicenseType::FullNonExpiring),
                10 | 11 | 29 => Ok(LicenseType::TimeLimited),
                _ => Ok(LicenseType::Unlicensed),
            }
        }

    }
    //calculate the number of days remain for the license
    fn get_days_remaining(&self) -> Result<c_int> {
        unsafe {
            let daysLeftPtr: &mut SK_IntPointer = &mut 0;
            let license_remaining_day = *self.lib
                .get::<fn(c_int, *const c_char, *mut c_int) -> c_int>(SK_DATEREMAINING.as_bytes())?;

            license_remaining_day(
                SK_FLAGS_NONE,
                CString::new(self.get_date_time_string_value(
                    "/SoftwareKey/PrivateData/License/EffectiveEndDate",
                )?).unwrap()
                    .into_raw(),
                daysLeftPtr,
            );
            return Ok(*daysLeftPtr);
        }
    }

    fn manual_reponse(&mut self, activation_code: &str) -> Result<bool> {
        unsafe {
            let sessionCode: &mut SK_StringPointer = &mut (0 as *const c_char);
            let response: &mut SK_XmlDoc = &mut 0;
            let encryptedResponse: &mut SK_XmlDoc = &mut 0;
            let license: &mut SK_XmlDoc = &mut 0;
            let decLicense: &mut SK_XmlDoc = &mut 0;
            let xpath = "/ActivateInstallationLicenseFile/PrivateData/SessionCode";
            let xpath1 = "/ActivateInstallationLicenseFile/PrivateData/License";

            let create_string = *self.lib
                .get::<fn(c_int, *const c_char, *mut SK_XmlDoc) -> c_int>(SK_XML_DOC_CREATE_FROM_STRING.as_bytes())?;

            let result = create_string(
                SK_FLAGS_NONE,
                CString::new(activation_code).unwrap().into_raw(),
                encryptedResponse,
            );

            if ResultCode::SK_ERROR_NONE as i32 != result {
                if ResultCode::SK_ERROR_XML_PARSER_FAILED as i32 == result {
                    self.check_result(ResultCode::SK_ERROR_INVALID_DATA as i32)?;
                } else {
                    self.check_result(result)?;
                }
            }
            let decrypt_doc = *self.lib.get::<fn(SK_ApiContext,
                       c_int,
                       bool,
                       SK_XmlDoc,
                       *mut SK_XmlDoc)
                       -> c_int>(SK_XML_DOC_DECRYPT_RSA.as_bytes())?;

            self.check_result(decrypt_doc(
                self.context,
                SK_FLAGS_NONE,
                false,
                *encryptedResponse,
                response,
            ))?;

            let node_get_string = *self.lib.get::<fn(c_int,
                       SK_XmlDoc,
                       *const c_char,
                       bool,
                       *mut SK_StringPointer)
                       -> c_int>(SK_NODE_GET_STRING.as_bytes())?;

            self.check_result(node_get_string(
                SK_FLAGS_NONE,
                *response,
                CString::new(xpath).unwrap().into_raw(),
                false,
                sessionCode,
            ))?;

            let xml_get = *self.lib
                .get::<fn(c_int, SK_XmlDoc, *const c_char, *mut SK_XmlDoc) -> c_int>(SK_XML_NODE_GET_DOC.as_bytes())?;

            self.check_result(xml_get(
                SK_FLAGS_NONE,
                *response,
                CString::new(xpath1).unwrap().into_raw(),
                license,
            ))?;

            self.check_result(decrypt_doc(
                self.context,
                SK_FLAGS_NONE,
                false,
                *license,
                decLicense,
            ))?;

            let license_type = self.determine_type(*decLicense)? as i32;
            self.set_writable(
                license_type != LicenseType::FullNonExpiring as i32,
            )?;

            let license_save_fn = *self.lib
                .get::<fn(SK_ApiContext, c_int, *const c_char, SK_XmlDoc) -> c_int>(SK_SAVE.as_bytes())?;

            self.check_result(license_save_fn(
                self.context,
                SK_FLAGS_NONE,
                CString::new(self.licenseFilePath.clone())
                    .unwrap()
                    .into_raw(),
                *license,
            ))?;

            let dispose = *self.lib.get::<fn(c_int, *mut SK_XmlDoc) -> c_int>(
                SK_XML_DOC_DISPOSE.as_bytes(),
            )?;

            dispose(SK_FLAGS_NONE, encryptedResponse);
            dispose(SK_FLAGS_NONE, response);
            dispose(SK_FLAGS_NONE, license);
            dispose(SK_FLAGS_NONE, decLicense);
            Ok(true)
        }
    }

    //activate the license in solo server with license_id and password
    pub fn activate_online(&mut self, license_id: u32, password: &str) -> Result<()> {
        debug!("activate_online");
        debug!("{:?}", license_id);
        debug!("{:?}", password);
        unsafe {
            let resultCodePtr: &mut SK_IntPointer = &mut 0;
            let statusCodePtr: &mut SK_IntPointer = &mut 0;
            let ptr: &mut SK_StringPointer = &mut (0 as *const c_char);
            let activationleftPtr: &mut SK_IntPointer = &mut 0;

            let requestPtr: &mut SK_XmlDoc = &mut 0;
            let responsePtr: &mut SK_XmlDoc = &mut 0;
            let licensePtr: &mut SK_XmlDoc = &mut 0;
            let decLicensePtr: &mut SK_XmlDoc = &mut 0;

            let errorMsgPtr: &mut SK_StringPointer = &mut (0 as *const c_char);

            let activate_request = *self.lib.get::<fn(SK_ApiContext,
                       c_int,
                       c_int,
                       *const c_char,
                       *const c_char,
                       c_int,
                       c_int,
                       bool,
                       *const c_char,
                       *const c_char,
                       *mut SK_XmlDoc,
                       *mut SK_StringPointer)
                       -> c_int>(SK_ActivateInstallationGetRequest.as_bytes())?;

            self.check_result(activate_request(
                self.context,
                SK_FLAGS_NONE,
                license_id as c_int,
                CString::new(password).unwrap().into_raw(),
                0 as *const c_char,
                1000,
                1000,
                false,
                //The optional, human-readable name of this installation. This may be a entered by your users, or may be a value of your choosing.
                CString::new("My Computer").unwrap().into_raw(),
                0 as *const c_char,
                requestPtr,
                ptr,
            ))?;

            debug!("activate_request");

            let call_xml_service = *self.lib.get::<fn(SK_ApiContext,
                       c_int,
                       *const c_char,
                       SK_XmlDoc,
                       *mut SK_XmlDoc,
                       *mut SK_IntPointer,
                       *mut SK_IntPointer)
                       -> c_int>(SK_CALL_XM_WEB_SERVICE.as_bytes())?;

            let result = call_xml_service(
                self.context,
                SK_FLAGS_NONE,
                CString::new(SK_CONST_WEBSERVICE_ACTIVATEINSTALLATION_URL)
                    .unwrap()
                    .into_raw(),
                *requestPtr,
                responsePtr,
                resultCodePtr,
                statusCodePtr,
            );
            debug!("call_xml_service");

            if ResultCode::SK_ERROR_NONE as i32 != result {
                if ResultCode::SK_ERROR_WEBSERVICE_RETURNED_FAILURE as i32 == result {
                    let node_get_string = self.lib.get::<fn(c_int,
                               SK_XmlDoc,
                               *const c_char,
                               bool,
                               *mut SK_StringPointer)
                               -> c_int>(SK_NODE_GET_STRING.as_bytes())?;

                    self.check_result(node_get_string(
                        SK_FLAGS_NONE,
                        *responsePtr,
                        CString::new(
                            "/ActivateInstallationLicenseFile/PrivateData/ErrorMessage",
                        ).unwrap()
                            .into_raw(),
                        false,
                        errorMsgPtr,
                    ))?;
                    debug!("{:?}", *errorMsgPtr);
                }
                self.check_result(result);
            }

            let xml_get = *self.lib
                .get::<fn(c_int, SK_XmlDoc, *const c_char, *mut SK_XmlDoc) -> c_int>(SK_XML_NODE_GET_DOC.as_bytes())?;

            self.check_result(xml_get(
                SK_FLAGS_NONE,
                *responsePtr,
                CString::new(
                    "/ActivateInstallationLicenseFile/PrivateData/License",
                ).unwrap()
                    .into_raw(),
                licensePtr,
            ))?;

            let node_get_int = *self.lib
                .get::<fn(c_int, SK_XmlDoc, *const c_char, *mut c_int) -> c_int>(SK_NODE_GET_INT.as_bytes())?;

            let result = node_get_int(
                SK_FLAGS_NONE,
                *responsePtr,
                CString::new(
                    "/ActivateInstallationLicenseFile/PrivateData/ActivationsLeft",
                ).unwrap()
                    .into_raw(),
                activationleftPtr,
            );

            self.activation = *activationleftPtr;

            let decrypt_doc = *self.lib.get::<fn(SK_ApiContext,
                       c_int,
                       bool,
                       SK_XmlDoc,
                       *mut SK_XmlDoc)
                       -> c_int>(SK_XML_DOC_DECRYPT_RSA.as_bytes())?;

            self.check_result(decrypt_doc(
                self.context,
                SK_FLAGS_NONE,
                false,
                *licensePtr,
                decLicensePtr,
            ))?;

            let license_type = self.determine_type(*decLicensePtr)? as i32;
            self.set_writable(
                license_type != LicenseType::FullNonExpiring as i32,
            )?;

            let license_save_fn = *self.lib
                .get::<fn(SK_ApiContext, c_int, *const c_char, SK_XmlDoc) -> c_int>(SK_SAVE.as_bytes())?;

            self.check_result(license_save_fn(
                self.context,
                SK_FLAGS_NONE,
                CString::new(self.licenseFilePath.clone())
                    .unwrap()
                    .into_raw(),
                *licensePtr,
            ))?;

            let dispose = *self.lib.get::<fn(c_int, *mut SK_XmlDoc) -> c_int>(
                SK_XML_DOC_DISPOSE.as_bytes(),
            )?;

            dispose(SK_FLAGS_NONE, requestPtr);
            dispose(SK_FLAGS_NONE, responsePtr);
            dispose(SK_FLAGS_NONE, licensePtr);
            dispose(SK_FLAGS_NONE, decLicensePtr);
            self.reload()?;
            Ok(())
        }
    }

    fn create_trial_in_db(&self, status: String, days: String, name: &str) {
        let mut license = Licenses::new();

        let m = license.mut_meta(
            license.object_meta(),
            name.to_string(),
            license.get_account(),
        );

        let jackie = license.who_am_i();

        license.set_meta(type_meta_url(jackie), m);

        license.set_status(status);
        license.set_expired(days);

        let mut activation = BTreeMap::new();

        //temp fix
        if name == "senseis" {
            activation.insert("limit".to_string(), 5);
            activation.insert("remain".to_string(), 5);
        } else {
            activation.insert("limit".to_string(), 10);
            activation.insert("remain".to_string(), 10);
        }

        license.set_activation(activation);
        license.set_provider_name(self.provider.clone());

        license::DataStore::new(&self.cache.conn).create_or_update(&license);
    }

    pub fn update_license(&self, name: &str, license_id: &str, password: &str, err: String) {
        let mut license = Licenses::new();
        let mut activation = BTreeMap::new();
        activation.insert("limit".to_string(), 5);
        activation.insert("remain".to_string(), self.activation);
        license.set_provider_name(name.to_string());
        license.set_activation(activation);
        license.set_status(self.status.clone());
        license.set_expired(self.remaining_days.clone());
        license.set_license_id(license_id.to_string());
        license.set_password(password.to_string());
        license.set_error(err);
        license::DataStore::new(&self.cache.conn).update(&license);
    }

    fn check_result(&self, value: i32) -> Result<()> {
        if value != ResultCode::SK_ERROR_NONE as i32 {
            return Err(Error::EntitlementError(
                ResultCode::err_description(value).to_string(),
            ));
        }
        Ok(())
    }
}
