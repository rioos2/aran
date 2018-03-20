use std::fmt;
use std::cmp::Ordering;
use std::str::FromStr;
use std::result;
use error::Error;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
pub struct ServiceIdent {
    pub name: String,
}

impl ServiceIdent {
    /// Creates a new service identifier
    pub fn new<T: Into<String>>(name: T) -> Self {
        ServiceIdent { name: name.into() }
    }
}

impl Default for ServiceIdent {
    fn default() -> ServiceIdent {
        ServiceIdent::new("")
    }
}

impl fmt::Display for ServiceIdent {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl FromStr for ServiceIdent {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        Ok(ServiceIdent::new(value))
    }
}

impl PartialOrd for ServiceIdent {
    fn partial_cmp(&self, other: &ServiceIdent) -> Option<Ordering> {
        if self.name != other.name {
            return None;
        }
        Some(self.name.cmp(&other.name))
    }
}

impl Ord for ServiceIdent {
    fn cmp(&self, other: &ServiceIdent) -> Ordering {
        if self.name != other.name {
            return self.name.cmp(&other.name);
        }
        self.name.cmp(&other.name)
    }
}
