// Copyright (c) 2017 RioCorp Inc.


use std::path::PathBuf;

use linux_users;
use linux_users::os::unix::UserExt;

pub fn get_uid_by_name(owner: &str) -> Option<u32> {
    linux_users::get_user_by_name(owner).map(|u| u.uid())
}

pub fn get_gid_by_name(group: &str) -> Option<u32> {
    linux_users::get_group_by_name(&group.as_ref()).map(|g| g.gid())
}

pub fn get_current_username() -> Option<String> {
    linux_users::get_current_username()
}

pub fn get_current_groupname() -> Option<String> {
    linux_users::get_current_groupname()
}

pub fn get_effective_uid() -> u32 {
    linux_users::get_effective_uid()
}

pub fn get_home_for_user(username: &str) -> Option<PathBuf> {
    linux_users::get_user_by_name(username).map(|u| PathBuf::from(u.home_dir()))
}

pub fn root_level_account() -> String {
    "root".to_string()
}
