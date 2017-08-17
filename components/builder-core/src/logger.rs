// Copyright (c) 2017 RioCorp Inc.

use std::fs::File;
use std::io::Write;
use std::path::Path;
use chrono::prelude::*;

pub struct Logger {
    file: File,
}

impl Logger {
    pub fn init<T: AsRef<Path>>(log_path: T, filename: &str) -> Self {
        let filepath = log_path.as_ref().to_path_buf().join(filename);
        Logger { file: File::create(filepath).expect("Failed to initialize log file") }
    }

    pub fn log(&mut self, msg: &str) {
        let dt: DateTime<UTC> = UTC::now();
        let fmt_msg = format!("{},{}\n", dt.format("%Y-%m-%d %H:%M:%S"), msg);

        self.file.write_all(fmt_msg.as_bytes()).expect(&format!(
            "Logger unable to write to {:?}",
            self.file
        ));
    }

    // Log format (fields are comma-separated)
    //   Log entry datetime (UTC)
    //   Entry type - G (group), J (job), P (project), W (worker), I (ident)
    //   Id (group or job id)
    //   State
    //   Project name (for job or project)
    //   Start datetime (UTC) (only for jobs)
    //   End datetime (UTC) (only for jobs)
    //   Start offset (offset from group creation, in seconds, only for jobs)
    //   Duration (job duration, in seconds, only for jobs)
    //   Error (if applicable)

    pub fn log_ident(&mut self, ident: &str) {
        let msg = format!("I,{}", ident);
        self.log(&msg);
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.file.sync_all().expect("Unable to sync log file");
    }
}
