// Copyright (c) 2017 RioCorp Inc.

use std::fs::File;
use std::io::Write;
use std::path::Path;
use chrono::prelude::*;

use protocol::jobsrv::Job;
use protocol::scheduler::{Group, Project};

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

    pub fn log_group(&mut self, group: &Group) {
        let msg = format!("G,{},{:?},", group.get_id(), group.get_state());
        self.log(&msg);
    }

    pub fn log_group_project(&mut self, group: &Group, project: &Project) {
        let msg = format!(
            "P,{},{:?},{},",
            group.get_id(),
            project.get_state(),
            project.get_name()
        );
        self.log(&msg);
    }

    pub fn log_group_job(&mut self, group: &Group, job: &Job) {
        let suffix = if job.has_build_started_at() && job.has_build_finished_at() {
            let start = job.get_build_started_at().parse::<DateTime<UTC>>().unwrap();
            let stop = job.get_build_finished_at()
                .parse::<DateTime<UTC>>()
                .unwrap();
            let group_start = group.get_created_at().parse::<DateTime<UTC>>().unwrap();

            let offset = start
                .signed_duration_since(group_start)
                .to_std()
                .unwrap()
                .as_secs();
            let duration = stop.signed_duration_since(start)
                .to_std()
                .unwrap()
                .as_secs();

            format!(
                "{},{},{},{}",
                offset,
                duration,
                start.format("%Y-%m-%d %H:%M:%S").to_string(),
                stop.format("%Y-%m-%d %H:%M:%S").to_string()
            )
        } else {
            "".to_string()
        };

        let error = if job.has_error() {
            format!("{:?}", job.get_error())
        } else {
            "".to_string()
        };

        let msg = format!(
            "J,{},{:?},{},{},{}",
            job.get_owner_id(),
            job.get_state(),
            job.get_project().get_name(),
            suffix,
            error
        );

        self.log(&msg);
    }

    pub fn log_worker_job(&mut self, job: &Job) {
        let start = if job.has_build_started_at() {
            job.get_build_started_at()
                .parse::<DateTime<UTC>>()
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        } else {
            "".to_string()
        };

        let stop = if job.has_build_finished_at() {
            job.get_build_finished_at()
                .parse::<DateTime<UTC>>()
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
                .to_string()
        } else {
            "".to_string()
        };

        let msg = format!(
            "W,{},{:?},{},,,{},{},{:?}",
            job.get_id(),
            job.get_state(),
            job.get_project().get_name(),
            start,
            stop,
            job.get_error()
        );
        self.log(&msg);
    }
}

impl Drop for Logger {
    fn drop(&mut self) {
        self.file.sync_all().expect("Unable to sync log file");
    }
}
