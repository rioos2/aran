// Copyright 2018 The Rio Advancement Inc

use std::ffi::CStr;
use std::mem;

use libc;

use os::system::Uname;
use errno::errno;
use error::{Error, Result};

pub fn uname() -> Result<Uname> {
    unsafe { uname_libc() }
}

unsafe fn uname_libc() -> Result<Uname> {
    let mut utsname: libc::utsname = mem::uninitialized();
    let rv = libc::uname(&mut utsname);
    if rv < 0 {
        let errno = errno();
        let code = errno.0 as i32;
        return Err(Error::UnameFailed(format!(
            "Error {} when calling uname: {}",
            code, errno
        )));
    }
    Ok(Uname {
        sys_name: CStr::from_ptr(utsname.sysname.as_ptr())
            .to_string_lossy()
            .into_owned(),
        node_name: CStr::from_ptr(utsname.nodename.as_ptr())
            .to_string_lossy()
            .into_owned(),
        release: CStr::from_ptr(utsname.release.as_ptr())
            .to_string_lossy()
            .into_owned(),
        version: CStr::from_ptr(utsname.version.as_ptr())
            .to_string_lossy()
            .into_owned(),
        machine: CStr::from_ptr(utsname.machine.as_ptr())
            .to_string_lossy()
            .into_owned(),
    })
}
