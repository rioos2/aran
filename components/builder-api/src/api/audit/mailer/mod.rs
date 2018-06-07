pub mod email_sender;
pub mod email_generator;

const DIGITALCLOUDRUNNING: &'static str = "DigitalCloudRunning";
const CONTAINERRUNNING: &'static str = "ContainerRunning";
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
            DIGITALCLOUDRUNNING | CONTAINERRUNNING => Status::DigitalCloudRunning,
            LAUNCHFAILED => Status::DigitalCloudFailed,
            _ => Status::None,
        }
    }
}

pub trait PushNotifier {
    fn should_notify(&self) -> bool;
    fn notify(&self);
}