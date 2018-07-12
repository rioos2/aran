use rio_core;
use std::fmt;
use std::io::Error as IoError;
use std::result;

macro_rules! impl_error {
    ($from:ty, $to:path) => {
        impl From<$from> for Error {
            fn from(e: $from) -> Self {
                $to(format!("{:?}", e))
            }
        }
    };
}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    IO(String),
    RioosAranCore(String),
    LicenseAPINotFound,
    LicenseAPIMustBeValid,
    LicenseCodeMustBeValid,
    LicenseAPIMustBeInConsistentState,
    TrialExpired,
    ProductExpired,
    SubscriptionExpired,
    EntitlementError(String),
}

impl_error!{IoError, Error::IO}
impl_error!{rio_core::Error, Error::RioosAranCore}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let msg = match *self {
            Error::IO(ref e) => format!("{}", e),
            Error::RioosAranCore(ref e) => format!("{}", e),
            Error::LicenseAPINotFound => format!("Entitlement guru is hallucinating. License can’t be verified."),
            Error::LicenseAPIMustBeValid => format!("Entitlement library is tampered. License can’t be verified"),
            Error::LicenseCodeMustBeValid => format!("Entitlement library activate_code is tampered. License can’t be verified"),
            Error::LicenseAPIMustBeInConsistentState => format!("Entitlement library is not in consistent state. Can happen when library is not freed upon use. License can’t be verified."),
            Error::TrialExpired => format!("Entitlement trial expired. Contact sales@rio.company to buy license."),
            Error::ProductExpired => format!("Entitlement trial expired. Contact sales@rio.company to buy license."),
            Error::SubscriptionExpired => format!("Entitlement activation code invalid. Contact sales@rio.company to buy license (or) provide a valid code."),
            Error::EntitlementError(ref e) => format!("{}", e),
        };
        write!(f, "{}", msg)
    }
}


pub enum ResultCode {
    SK_ERROR_NONE = 0,
    SK_ERROR_INVALID_DATA = 9001,
    SK_ERROR_INVALID_SERVER_KEY = 9002,
    SK_ERROR_INVALID_CLIENT_KEY = 9003,
    SK_ERROR_DECRYPTION_FAILED = 9004,
    SK_ERROR_VERIFICATION_FAILED = 9005,
    SK_ERROR_ENCRYPTION_FAILED = 9006,
    SK_ERROR_SIGNING_FAILED = 9007,
    SK_ERROR_SESSION_VERIFICATION_FAILED = 9008,
    SK_ERROR_INSTALLATIONID_REQUIRED = 9009,
    SK_ERROR_TRIGGER_CODE_INVALID = 9010,
    SK_ERROR_TRIGGER_CODE_EVENT_DATA_INVALID = 9011,
    SK_ERROR_INVALID_LICENSE_TYPE = 9012,
    SK_ERROR_XML_PARSER_FAILED = 9013,
    SK_ERROR_XML_NODE_MISSING = 9014,
    SK_ERROR_INVALID_ARGUMENTS = 9015,
    SK_ERROR_CONTEXT_INVALID = 9016,
    SK_ERROR_STRING_CONVERSION_FAILED = 9017,
    SK_ERROR_DATETIME_CONVERSION_FAILED = 9018,
    SK_ERROR_PLUS_EVALUATION_WARNING = 9019,
    SK_ERROR_PLUS_EVALUATION_INVALID = 9020,
    SK_ERROR_INVALID_PRODUCTID = 9021,
    SK_ERROR_INVALID_PRODUCTOPTIONID = 9022,
    SK_ERROR_ENVELOPE_TYPE_INVALID = 9023,
    SK_ERROR_INSUFFICIENT_IMAGE_SIZE = 9024,
    SK_ERROR_INVALID_IMAGE = 9025,
    SK_ERROR_WEBSERVICE_INVALID_CONFIGURATION = 9100,
    SK_ERROR_WEBSERVICE_CALL_FAILED = 9101,
    SK_ERROR_WEBSERVICE_RETURNED_FAILURE = 9102,
    SK_ERROR_REQUIRED_SERVER_VALIDATION_FAILED = 9103,
    SK_ERROR_HTTP_INITIALIZATION_FAILED = 9104,
    SK_ERROR_HTTP_CONNECTION_FAILED = 9105,
    SK_ERROR_HTTP_COULD_NOT_RESOLVE_HOST = 9106,
    SK_ERROR_SSL_FAILED = 9107,
    SK_ERROR_COULD_NOT_LOAD_LICENSE = 9200,
    SK_ERROR_COULD_NOT_SAVE_LICENSE = 9201,
    SK_ERROR_LICENSE_NOT_EFFECTIVE_YET = 9202,
    SK_ERROR_LICENSE_EXPIRED = 9203,
    SK_ERROR_LICENSE_ALIAS_VALIDATION_FAILED = 9204,
    SK_ERROR_LICENSE_ALIAS_VALIDATION_TIME_MISMATCH = 9205,
    SK_ERROR_COULD_NOT_SAVE_NETWORK_CERTIFICATE = 9206,
    SK_ERROR_NETWORK_CERTIFICATE_INVALID_PATH = 9207,
    SK_ERROR_NETWORK_CERTIFICATE_REQUIRED = 9208,
    SK_ERROR_COULD_NOT_DELETE_FILE = 9209,
    SK_ERROR_NETWORK_SEMAPHORE_INVALID_PATH = 9210,
    SK_ERROR_NETWORK_LICENSE_FULL = 9211,
    SK_ERROR_NETWORK_SEMAPHORE_LOCK_FAILED = 9212,
    SK_ERROR_MODULE_NOT_ACTIVE = 9213,
    SK_ERROR_COULD_NOT_OPEN_FILE = 9214,
    SK_ERROR_COULD_NOT_READ_FILE = 9215,
    SK_ERROR_COULD_NOT_WRITE_FILE = 9216,
    SK_ERROR_COULD_NOT_OPEN_REGISTRY_KEY = 9217,
    SK_ERROR_COULD_NOT_READ_REGISTRY_KEY = 9218,
    SK_ERROR_COULD_NOT_WRITE_REGISTRY_KEY = 9219,
    SK_ERROR_IO_OPERATION_FAILED = 9220,
    SK_ERROR_COULD_NOT_READ_PERMISSIONS = 9221,
    SK_ERROR_COULD_NOT_SET_PERMISSIONS = 9222,
    SK_ERROR_SSL_CERTIFICATE_EXPORT_FAILED = 9223,
    SK_ERROR_SSL_CERTIFICATE_UNAVAILABLE = 9224,
    SK_ERROR_COULD_NOT_LOAD_VOLUME_DOWNLOADABLE_LICENSE = 9225,
    SK_ERROR_SYSTEM_TIME_VERIFICATION_FAILED = 9300,
    SK_ERROR_SYSTEM_TIME_INVALID = 9301,
    SK_ERROR_VIRTUAL_MACHINE_DETECTED = 9302,
    SK_ERROR_REMOTE_SESSION_DETECTED = 9303,
    SK_ERROR_LICENSE_SYSTEM_IDENTIFIERS_DONT_MATCH = 9400,
    SK_ERROR_PLATFORM_ERROR = 9401,
    SK_ERROR_UNSUPPORTED_OS = 9402,
    SK_ERROR_MEMORY_ALLOCATION = 9403,
    SK_ERROR_LIBRARY_UNAVAILABLE = 9404,
    SK_ERROR_LIBRARY_FUNCTION_UNAVAILABLE = 9405,
}

impl ResultCode {
    pub fn err_description(value: i32) -> &'static str {
        match value {
            9001 => "The presence of invalid data has been detected.",
            9002 => "The presence of an invalid server key has been detected.",
            9003 => "The presence of an invalid client key has been detected.",
            9004 => "The requested decryption operation has failed.",
            9005 => "The requested verification operation has failed.",
            9006 => "The requested encryption operation has failed",
            9007 => "The requested signing operation has failed.",
            9008 => "The requested session code verification has failed.",
            9009 => "An Installation ID is required but is not present.",
            9010 => "An invalid Activation Code 1 value was entered by the user.",
            9011 => "An invalid Activation Code 2 value was entered by the user.",
            9012 => "The license type is invalid or not supported.",
            9013 => "The XML parser encountered an error.",
            9014 => "The requested XML node could not be found.",
            9015 => "Some or all of the arguments are invalid.",
            9016 => "The API context passed into the function call is not valid.",
            9017 => "A string conversion operation failed.",
            9018 => "A date-time conversion operation failed.",
            9019 => "No error actually occurred but that an evaluation envelope is being used.",
            9020 => "The Protection PLUS 5 SDK evaluation is invalid or expired.",
            9021 => "The Product ID is not valid.",
            9022 => "The Product Option ID is not valid.",
            9023 => "The envelope is not valid.",
            9024 => "The license image is either too small to hold the license data or is under the minimum image size required.",
            9025 => "The license image is not valid.",
            9100 => "The configuration of the requested Web Service is invalid.",
            9101 => "An unexpected failure occurred during an attempt to call a Web Service.",
            9102 => "A call to a Web Service succeeded but the functionality of the Web Service returned an indicator of failure.",
            9103 => "Validation against SOLO Server is required but could not be completed.",
            9104 => "The HTTP client failed to initialize.",
            9105 => "The server could not be reached. Verify that the computer is connected to the Internet and that the firewall/proxy is set-up properly.",
            9106 => "The server could not be located. Verify that the computer is connected to the Internet and that the firewall/proxy is set-up properly.",
            9107 => "The HTTPS request failed due to an SSL related error.",
            9200 => "License could not be loaded.",
            9201 => "License could not be saved.",
            9202 => "License is not yet effective.",
            9203 => "License has expired.",
            9204 => "Validation of license alias has failed.",
            9205 => "Validation of license alias time has failed due to mismatch.",
            9206 => "Network certificate could not be saved.",
            9207 => "The Network Certificate path does not match the path specified during checkout.",
            9208 => "A valid network session certificate is required but is not present.",
            9209 => "Could not delete file.",
            9210 => "The network path is not valid or is unavailable.",
            9211 => "The number of allowed concurrent users has been reached.",
            9212 => "Failed to create network semaphore.",
            9213 => "The activation was successful; however another activation is required to enable use of this application.",
            9214 => "An attempt to open a file failed.",
            9215 => "An attempt to read a file failed.",
            9216 => "An attempt to write a file failed.",
            9217 => "An attempt to open a registry key failed.",
            9218 => "An attempt to read a registry key value failed.",
            9219 => "An attempt to write a registry key value failed.",
            9220 => "An attempt to perform an I/O operation failed.",
            9221 => "An attempt to read a file or registry key's permissions failed.",
            9222 => "An attempt to set a file or registry key's permissions failed.",
            9223 => "Failed to export the SSL client certificate bundle.",
            9224 => "The client certificate for SSL communication could not be found.",
            9225 => "The volume or downloadable license file could not be found or loaded.",
            9300 => "Verification of system time has failed.",
            9301 => "System time is not valid.",
            9302 => "The application determined it is running in a virtual machine.",
            9303 => "The application determined it is running in a remote session.",
            9400 => "License system identifiers do not match.",
            9401 => "Platform specific API or system call fails.",
            9402 => "The current operating system is not supported by this feature or function.",
            9403 => "Memory could not be allocated.",
            9404 => "Required system library is missing for failed to load.",
            9405 => "Required library function is missing.",
            _ => "Invalid Error Code",
        }
    }
}
