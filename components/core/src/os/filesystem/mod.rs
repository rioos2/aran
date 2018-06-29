// Copyright 2018 The Rio Advancement Inc

#[allow(unused_variables)]
#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use self::windows::{chmod, chown, symlink};

#[cfg(not(windows))]
mod linux;

#[cfg(not(windows))]
pub use self::linux::{chmod, chown, symlink};
