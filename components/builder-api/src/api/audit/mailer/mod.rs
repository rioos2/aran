pub mod email_notifier;
pub mod email_generator;

const LAUNCHSUCCESS: &'static str = "LaunchSuccess";
const LAUNCHFAILED: &'static str = "LaunchFailed";

pub enum Status {
    DigitalCloudRunning,
    DigitalCloudFailed,
    None,
}

//convert level string to TrustLevel enum value
impl Status {
    pub fn from_str(value: &str) -> Status {
        match &value[..] {
            LAUNCHSUCCESS => Status::DigitalCloudRunning,
            LAUNCHFAILED => Status::DigitalCloudFailed,
            _ => Status::None,
        }
    }
}
