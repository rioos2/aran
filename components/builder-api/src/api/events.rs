// Copyright 2018 The Rio Advancement Inc

//! A module containing events and logger

use std::fs::{self, File};
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json;

use node::runtime::ApiSender;
use protocol::api::audit::{Envelope, AuditEvent, AccessedBy};

/// The records created by the Rio/OS AuditBlockchain capture information on
/// who has performed, what action, when, and how successfully:

/// Who: account_id (or) ip address, rioos-system (organization)
///      To access the system, each administrator has authenticated identifiable throughout their session.
/// What: The action performed. Different kinds of action are tracked by Rio/OS.
///     Login in from ubuntu in Chrome 61
///     Node added to cluster, Node removed from cluster, Node failed over
///     Metrics profiling started,	Memory profiling started,	Self-signed SSL certificate regenerated
/// When: The UTC time stamp that corresponds to each recorded action.
/// How: The success or failure of the action

/// An envelope JSON payload for factory-create
/// {
///   "timestamp": "147933000013442404",
///   "address": "192.168.1.1",
///   "event": {
///     "type_meta": {},
//      "object_meta": {},
///     "reason" :  "Deploying Assembly",
///     "message" : "Deploy received for assembly",
///     "source" :  {component: "assemblycontroller", host:"192.168.1.1"},
///     "type_of_event" : "NORMAL/WARNING/SUCCESS/FAIL"
///   }
/// }
// Macros to help hooking in the event logger into an Iron chain,
// and calling into the chained event logger.
#[macro_export]
macro_rules! define_event_log {
    () => {
        pub struct EventLog;
        impl typemap::Key for EventLog {
            type Value = EventLogger;
        }
    }
}

// Macros to post in the event logger  from any request.
#[macro_export]
macro_rules! log_event {
    ($req:ident, $evt:expr) => {{
        use persistent;
        let ad = format!("{:?}", ($req).remote_addr);
        let el = ($req).get::<persistent::Read<EventLog>>().unwrap();
        el.record_event($evt, (($evt).get_account(), ad))
    }};
}

fn write_file<T: ?Sized>(parent_dir: &Path, file_path: &Path, val: &T)
where
    T: Serialize,
{
    fs::create_dir_all(parent_dir).expect("Unable to create directory");
    let mut file = File::create(&file_path).expect("Unable to create file");
    serde_json::ser::to_writer(&mut file, val).expect("Unable to write file");
}
#[derive(Debug)]
pub struct EventLogger {
    channel: ApiSender,
    log_dir: PathBuf,
    enabled: bool,
}

#[allow(unused_must_use)]
impl EventLogger {
    pub fn new<T: Into<PathBuf>>(channel: ApiSender, log_dir: T, enabled: bool) -> Self {
        EventLogger {
            channel: channel,
            log_dir: log_dir.into(),
            enabled: enabled,
        }
    }

    pub fn record_event(&self, event: AuditEvent, accessed_by: AccessedBy) {
        if self.enabled {
            let envelope = Envelope::new(&event, accessed_by);
            let file_path = self.log_dir.join("audits-blockchain.json");
            write_file(&self.log_dir, &file_path, &envelope);
            self.channel.peer_add(envelope);
        }
    }
}

#[cfg(test)]
mod test {
    use futures::sync::mpsc;

    use super::*;

    #[test]
    fn event_logger_path() {
        let api_sender = ApiSender::new(mpsc::channel(10).0);
        let event_logger: EventLogger = EventLogger::new(api_sender, "/var/lib/rioos/foo/var", true);
        let expected = r#"foo"#;
        match event_logger.log_dir.to_str() {
            Some(s) => assert!(s.contains(expected)),
            None => assert!(false),
        }
    }
}
