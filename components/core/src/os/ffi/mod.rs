// Copyright 2018 The Rio Advancement Inc
//

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use self::windows::OsStrExt3 as OsStrExt;

#[cfg(not(windows))]
pub use std::os::unix::ffi::OsStrExt;
