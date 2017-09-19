// Copyright (c) 2017 RioCorp Inc.

use libc::c_int;
use std::path::Path;
use std::io;

use error::Result;

pub fn path_exists(path: &str) -> Result<c_int> {
    match Path::new(path).exists() {
        false => Ok(1),
        true => Ok(0),
    }
}

pub fn chown(path: &str, uid: String, gid: String) -> Result<c_int> {
    path_exists(path)
}

pub fn chmod(path: &str, mode: u32) -> Result<c_int> {
    path_exists(path)
}

pub fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> io::Result<()> {
    unimplemented!();
}
