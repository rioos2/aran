use error::{Error, ResultCode, Result};
use std::os::raw::*;
pub mod load_library;


pub type SK_ApiContext = c_long;
pub type SK_XmlDoc = c_long;
pub type SK_StringPointer = *const c_char;
pub type SK_DatePointer = *const c_char;
pub type SK_IntPointer = c_int;


pub const SK_INIT: &'static str = "SK_ApiContextInitialize";
pub const SK_GET_LICENSE: &'static str = "SK_PLUS_LicenseGetXmlDocument";
pub const SK_DISPOSE: &'static str = "SK_ApiContextDispose";
pub const SK_NEW: &'static str = "SK_XmlElementAddNew";
pub const SK_SET: &'static str = "SK_ApiContextSetFieldString";
pub const SK_LOAD_LICENSE: &'static str = "SK_PLUS_LicenseFileLoad";
pub const SK_SET_INT: &'static str = "SK_ApiContextSetFieldInt";
pub const SK_CREATE_NEW_LICENSE: &'static str = "SK_PLUS_LicenseCreateNew";
pub const SK_SAVE: &'static str = "SK_PLUS_LicenseFileSave";
pub const SK_NODE_GET_INT: &'static str = "SK_XmlNodeGetValueInt";
pub const SK_NODE_GET_STRING: &'static str = "SK_XmlNodeGetValueString";
pub const SK_DATEREMAINING: &'static str = "SK_DateTimeDaysRemaining";
pub const SK_NODE_GET_DATE: &'static str = "SK_XmlNodeGetValueDateTimeString";
pub const SK_IDNTIFIER_ALGORITHAM: &'static str = "SK_PLUS_SystemIdentifierAlgorithmAddCurrentIdentifiers";
pub const SK_DATETIME_VALIDATE: &'static str = "SK_DateTimeValidateApi";
pub const SK_SYSTEM_IDENTIFIER_COMPARE: &'static str = "SK_PLUS_SystemIdentifierCompare";
pub const SK_DATETIME_GET_CURRENT_STRING: &'static str = "SK_DateTimeGetCurrentString";
pub const SK_DATETIME_COMPARE_STRING: &'static str = "SK_DateTimeCompareStrings";


pub enum LicenseType {
    Unlicensed = 0,
    FullNonExpiring = 1,
    TimeLimited = 10,
}
