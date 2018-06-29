// Copyright 2018 The Rio Advancement Inc

#[allow(unused_variables)]
#[cfg(not(windows))]
pub mod linux;

#[cfg(not(windows))]
pub use self::linux::{
    get_current_groupname, get_current_username, get_effective_uid, get_gid_by_name,
    get_home_for_user, get_uid_by_name, root_level_account,
};
