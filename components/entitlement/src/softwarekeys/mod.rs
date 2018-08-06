use error::{Error, ResultCode, Result};
use std::os::raw::*;
pub mod licensor;


pub type SK_ApiContext = c_long;
pub type SK_XmlDoc = c_long;
pub type SK_StringPointer = *const c_char;
pub type SK_DatePointer = *const c_char;
pub type SK_IntPointer = c_int;

//Initializes a new API Context, which may be used to open and manipulate a license file.
pub const SK_INIT: &'static str = "SK_ApiContextInitialize";
//Retrieves the raw contents of the License File.
pub const SK_GET_LICENSE: &'static str = "SK_PLUS_LicenseGetXmlDocument";
//Disposes an SK_ApiContext, which clears it from memory and sets it's pointer to NULL (0).
pub const SK_DISPOSE: &'static str = "SK_ApiContextDispose";
//Adds an element to a given XML document.
pub const SK_NEW: &'static str = "SK_XmlElementAddNew";
//Sets an string field in the API Context.
pub const SK_SET: &'static str = "SK_ApiContextSetFieldString";
//Loads a License from a file or Windows Registry key.
pub const SK_LOAD_LICENSE: &'static str = "SK_PLUS_LicenseFileLoad";
//Sets an integer field in the API Context.
pub const SK_SET_INT: &'static str = "SK_ApiContextSetFieldInt";
//Creates a new License.
pub const SK_CREATE_NEW_LICENSE: &'static str = "SK_PLUS_LicenseCreateNew";
//Saves a License to a file or Windows Registry key.
pub const SK_SAVE: &'static str = "SK_PLUS_LicenseFileSave";
//Retrieves an integer value from a given XML node.
pub const SK_NODE_GET_INT: &'static str = "SK_XmlNodeGetValueInt";
//Retrieves a string value from a given XML node.
pub const SK_NODE_GET_STRING: &'static str = "SK_XmlNodeGetValueString";
//Determines the number of days left until a specified date is reached.
pub const SK_DATEREMAINING: &'static str = "SK_DateTimeDaysRemaining";
//Retrieves a date-time string value from a given XML node.
pub const SK_NODE_GET_DATE: &'static str = "SK_XmlNodeGetValueDateTimeString";
//Adds SystemIdentifier elements to the list of current system identifiers using the specified, built-in algorithm.
pub const SK_IDNTIFIER_ALGORITHAM: &'static str = "SK_PLUS_SystemIdentifierAlgorithmAddCurrentIdentifiers";
//Verifies that the operating system clock APIs have not been compromised. This prevents tools like "Time Stopper" or "RunAsDate" from being used to trick your protected applications.
pub const SK_DATETIME_VALIDATE: &'static str = "SK_DateTimeValidateApi";
//Compares the current system's identifiers against the identifiers authorized in the license.
pub const SK_SYSTEM_IDENTIFIER_COMPARE: &'static str = "SK_PLUS_SystemIdentifierCompare";
//Retrieves the current date-time as a string from the system clock.
pub const SK_DATETIME_GET_CURRENT_STRING: &'static str = "SK_DateTimeGetCurrentString";
//Compares two date-time strings in ISO-8601 format (YYYY-MM-DDTHH:MM:SSZ).
pub const SK_DATETIME_COMPARE_STRING: &'static str = "SK_DateTimeCompareStrings";
//Creates an XML document from a string representation.
pub const SK_XML_DOC_CREATE_FROM_STRING: &'static str = "SK_XmlDocumentCreateFromString";
//Decrypts an XML document using the RSA Algorithm.
pub const SK_XML_DOC_DECRYPT_RSA: &'static str = "SK_XmlDocumentDecryptRsa";
//Creates an new XML document from a sub-document in the specified XML document.
pub const SK_XML_NODE_GET_DOC: &'static str = "SK_XmlNodeGetDocument";
//Disposes an XML document, which clears it from memory and sets it's pointer to NULL (0).
pub const SK_XML_DOC_DISPOSE: &'static str = "SK_XmlDocumentDispose";
//Builds a request to send to the XmlActivationService web service's ActivateInstallationLicenseFile method in SOLO Server.
pub const SK_ActivateInstallationGetRequest: &'static str = "SK_SOLO_ActivateInstallationGetRequest";
//Calls a SOLO Server XML web service method.
pub const SK_CALL_XM_WEB_SERVICE: &'static str = "SK_CallXmlWebService";

//Activate url

pub const SK_CONST_WEBSERVICE_ACTIVATEINSTALLATION_URL: &'static str = "secure.softwarekey.com/solo/webservices/XmlActivationService.asmx/ActivateInstallationLicenseFile";

pub enum LicenseType {
    Unlicensed = 0,
    FullNonExpiring = 1,
    TimeLimited = 10,
}

//Enumeration for built-in system identifier algorithms.
pub enum SK_SystemIdentifierAlgorithm {
    //Use for adding current identifiers for all relevant Network Interface Cards (NICs) using their MAC/physical addresses.
    SK_SYSTEM_IDENTIFIER_ALGORITHM_NIC = 10,
    //Use for adding a current identifier for the computer's name.
    SK_SYSTEM_IDENTIFIER_ALGORITHM_COMPUTER_NAME = 20,
    //Use for adding current identifiers for the volume format serials on all accessible partitions on all available hard drives which report themselves as "fixed." (Note that some removable drives such as some USB drives or hard drives placed in USB enclosures may still report themselves as being "fixed" drives.)
    SK_SYSTEM_IDENTIFIER_ALGORITHM_HARD_DISK_VOLUME_SERIAL = 30,
    //Use for adding identifiers a network share location that use being used to store the applications license file and/or network semaphore files.
    SK_SYSTEM_IDENTIFIER_ALGORITHM_NETWORK_NAME = 40,
    //Use for adding current identifiers for all relevant usernames
    SK_SYSTEM_IDENTIFIER_ALGORITHM_USER_NAME = 50,
}
