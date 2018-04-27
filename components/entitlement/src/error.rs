use std::result;
use std::io::Error as IoError;
use std::fmt;
use rio_core;

macro_rules! impl_error {
    ($from:ty, $to:path) => {
        impl From<$from> for Error {
            fn from(e: $from) -> Self {
                $to(format!("{:?}", e))
            }
        }
    }
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
        };
        write!(f, "{}", msg)
    }
}
