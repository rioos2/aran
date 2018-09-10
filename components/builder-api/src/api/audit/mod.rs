// Copyright 2018 The Rio Advancement Inc
//

//! Audits  - Ledger part of the Rioos rest api.

pub mod audit_api;
pub mod event_api;
pub mod config;
pub mod ledger;
pub mod log_api;
pub mod mailer;
pub mod slack;
pub mod vuln_api;

const DIGITALCLOUDRUNNING: &'static str = "DigitalCloudRunning";
const CONTAINERRUNNING: &'static str = "ContainerRunning";
const LAUNCHFAILED: &'static str = "LaunchFailed";
const KRYPTONITEQRCODE: &'static str = "SecurityQRCode";
const KRYPTONITESYNCFAILED: &'static str = "SecurityFailed";
const INVITE: &'static str = "Invite";

pub enum Status {
    DigitalCloudRunning,
    DigitalCloudFailed,
    KryptoniteQRCode,
    KryptoniteSyncFailed,
    Invite,
    None,
}

//convert level string to TrustLevel enum value
impl Status {
    pub fn from_str(value: &str) -> Status {
        match &value[..] {
            DIGITALCLOUDRUNNING | CONTAINERRUNNING => Status::DigitalCloudRunning,
            LAUNCHFAILED => Status::DigitalCloudFailed,
            KRYPTONITEQRCODE => Status::KryptoniteQRCode,
            KRYPTONITESYNCFAILED => Status::KryptoniteSyncFailed,
            INVITE => Status::Invite,
            _ => Status::None,
        }
    }
}

pub trait PushNotifier {
    fn should_notify(&self) -> bool;
    fn notify(&self);
}
