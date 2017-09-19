// Copyright (c) 2017 RioCorp Inc.

#[allow(unused_variables)]
#[cfg(not(windows))]
pub mod linux;

#[cfg(not(windows))]
pub use self::linux::{get_uid_by_name, get_gid_by_name, get_effective_uid, get_home_for_user, get_current_username, get_current_groupname, root_level_account};
