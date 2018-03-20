// Copyright 2018 The Rio Advancement Inc

use libc::{self, c_int, c_char, mode_t};

pub use std::os::unix::fs::symlink;
use std::ffi::CString;

use error::{Result, Error};

fn validate_raw_path(path: &str) -> Result<*mut c_char> {
    let c_path = match CString::new(path) {
        Ok(c) => c,
        Err(e) => {
            return Err(Error::PermissionFailed(format!(
                "Can't create string from path {:?}: {}",
                path, e
            )))
        }
    };
    Ok(c_path.into_raw())
}

pub fn chown(path: &str, uid: u32, gid: u32) -> Result<c_int> {
    let r_path = match validate_raw_path(path) {
        Ok(r) => r,
        Err(e) => return Err(e),
    };

    unsafe {
        let res = libc::chown(r_path, uid, gid);
        CString::from_raw(r_path); // necessary to prevent leaks
        Ok(res)
    }
}

pub fn chmod(path: &str, mode: u32) -> Result<c_int> {
    let c_path = match CString::new(path) {
        Ok(c) => c,
        Err(e) => {
            return Err(Error::PermissionFailed(format!(
                "Can't create string from path {:?}: {}",
                path, e
            )))
        }
    };
    let r_path = c_path.into_raw();

    unsafe {
        let res = libc::chmod(r_path, mode as mode_t);
        CString::from_raw(r_path); // necessary to prevent leaks
        Ok(res)
    }
}
