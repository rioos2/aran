// Copyright (c) 2017 RioCorp Inc.


use std::env;
use std::path::{Path, PathBuf};

use users;

use env as renv;

/// The default root path of the Rio/OS filesystem
pub const ROOT_PATH: &'static str = "/var/lib/rioos";
/// The default path where TLS-related keys are placed
pub const KEY_PATH: &'static str = "config";
/// The default path where TLS-related artifacts are placed
pub const CONFIG_PATH: &'static str = "config";
/// The default path where etc config are placed
pub const ETC_PATH: &'static str = "etc";

/// The environment variable pointing to the filesystem root. This exists for internal
/// team usage and is not intended to be used by consumers.
pub const FS_ROOT_ENVVAR: &'static str = "RIOOS_HOME";

lazy_static! {
    /// The default filesystem root path.
    ///
    pub static ref FS_ROOT_PATH: PathBuf = {
            match renv::var(FS_ROOT_ENVVAR) {
                Ok(path) =>  PathBuf::from(path),
                Err(_) => {
                    if *EUID == 0u32 {
                        PathBuf::from(renv::var(FS_ROOT_ENVVAR).unwrap_or(ROOT_PATH.to_string()))
                    } else {
                       env::home_dir().unwrap()
                    }
                },
            }
     };

    static ref EUID: u32 = users::get_effective_uid();

    static ref MY_KEY_PATH: PathBuf = {
        FS_ROOT_PATH.join(format!("{}", KEY_PATH))
    };

    static ref MY_CONFIG_PATH: PathBuf = {
        FS_ROOT_PATH.join(format!("{}", CONFIG_PATH))
    };

    static ref MY_ETC_PATH: PathBuf = {
        FS_ROOT_PATH.join(format!("{}", ETC_PATH))
    };
}


/// Returns the path to the keys cache, optionally taking a custom filesystem root.
/// TO-DO: Not used currently, needed for `setup` command.
pub fn rioconfig_key_path(fs_root_path: Option<&Path>) -> PathBuf {
    match fs_root_path {
        Some(fs_root_path) => Path::new(fs_root_path).join(&*MY_KEY_PATH),
        None => Path::new(&*FS_ROOT_PATH).join(&*MY_KEY_PATH),
    }
}

/// Returns the path to the config cache ssl, optionally taking a custom filesystem root.
/// This is the same directory like $RIOOS_HOME/config
pub fn rioconfig_ssl_path(fs_root_path: Option<&Path>) -> PathBuf {
    match fs_root_path {
        Some(fs_root_path) => Path::new(fs_root_path).join(&*MY_CONFIG_PATH),
        None => Path::new(&*FS_ROOT_PATH).join(&*MY_CONFIG_PATH),
    }
}

/// Returns the path to the config cache, optionally taking a custom filesystem root.
pub fn rioconfig_config_path(fs_root_path: Option<&Path>) -> PathBuf {
    match fs_root_path {
        Some(fs_root_path) => Path::new(fs_root_path).join(&*MY_CONFIG_PATH),
        None => Path::new(&*FS_ROOT_PATH).join(&*MY_CONFIG_PATH),
    }
}


/// Returns the path to the config cache, optionally taking a custom filesystem root.
pub fn rioconfig_etc_path(fs_root_path: Option<&Path>) -> PathBuf {
    match fs_root_path {
        Some(fs_root_path) => Path::new(fs_root_path).join(&*MY_ETC_PATH),
        None => Path::new(&*FS_ROOT_PATH).join(&*MY_ETC_PATH),
    }
}

/// Returns the absolute path for a given command, if it exists, by searching the `PATH`
/// environment variable.
///
/// If the command represents an absolute path, then the `PATH` searching will not be performed. If
/// no absolute path can be found for the command, then `None` is returned.
///
/// On Windows, the PATHEXT environment variable contains common extensions for commands,
/// for example allowing "docker.exe" to be found when searching for "docker".
///
/// # Examples
///
/// Behavior when the command exists on PATH:
///
/// ```
///
/// use std::env;
/// use std::fs;
/// use habitat_core::fs::find_command;
///
/// let first_path = fs::canonicalize("./tests/fixtures").unwrap();
/// let second_path = fs::canonicalize("./tests/fixtures/bin").unwrap();
/// let path_bufs = vec![first_path, second_path];
/// let new_path = env::join_paths(path_bufs).unwrap();
/// env::set_var("PATH", &new_path);
///
/// let result = find_command("bin_with_no_extension");
/// assert_eq!(result.is_some(), true);
/// ```
///
/// Behavior when the command does not exist on PATH:
///
/// ```
///
/// use std::env;
/// use std::fs;
/// use habitat_core::fs::find_command;
///
/// let first_path = fs::canonicalize("./tests/fixtures").unwrap();
/// let second_path = fs::canonicalize("./tests/fixtures/bin").unwrap();
/// let path_bufs = vec![first_path, second_path];
/// let new_path = env::join_paths(path_bufs).unwrap();
/// env::set_var("PATH", &new_path);
///
/// let result = find_command("missing");
/// assert_eq!(result.is_some(), false);
/// ```
///
pub fn find_command(command: &str) -> Option<PathBuf> {
    // If the command path is absolute and a file exists, then use that.
    let candidate = PathBuf::from(command);
    if candidate.is_absolute() && candidate.is_file() {
        return Some(candidate);
    }

    // Find the command by checking each entry in `PATH`. If we still can't find it, give up and
    // return `None`.
    match renv::var_os("PATH") {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                let candidate = PathBuf::from(&path).join(command);
                if candidate.is_file() {
                    return Some(candidate);
                } else {
                    match find_command_with_pathext(&candidate) {
                        Some(result) => return Some(result),
                        None => {}
                    }
                }
            }
            None
        }
        None => None,
    }
}

// Windows relies on path extensions to resolve commands like `docker` to `docker.exe`
// Path extensions are found in the PATHEXT environment variable.
// We should only search with PATHEXT if the file does not already have an extension.
fn find_command_with_pathext(candidate: &PathBuf) -> Option<PathBuf> {
    if candidate.extension().is_none() {
        match renv::var_os("PATHEXT") {
            Some(pathexts) => {
                for pathext in env::split_paths(&pathexts) {
                    let mut source_candidate = candidate.to_path_buf();
                    let extension = pathext.to_str().unwrap().trim_matches('.');
                    source_candidate.set_extension(extension);
                    let current_candidate = source_candidate.to_path_buf();
                    if current_candidate.is_file() {
                        return Some(current_candidate);
                    }
                }
            }
            None => {}
        };
    }
    None
}

/// Returns whether or not the current process is running with a root effective user id or not.
pub fn am_i_root() -> bool {
    *EUID == 0u32
}

#[cfg(test)]
mod test_find_command {

    use std::env;
    use std::fs;
    use std::path::PathBuf;
    pub use super::find_command;

    #[allow(dead_code)]
    fn setup_pathext() {
        let path_bufs = vec![PathBuf::from(".COM"), PathBuf::from(".EXE")];
        let new_path = env::join_paths(path_bufs).unwrap();
        env::set_var("PATHEXT", &new_path);
    }

    fn setup_empty_pathext() {
        if env::var("PATHEXT").is_ok() {
            env::remove_var("PATHEXT")
        }
    }

    fn setup_path() {
        let first_path = fs::canonicalize("./tests/fixtures").unwrap();
        let second_path = fs::canonicalize("./tests/fixtures/bin").unwrap();
        let path_bufs = vec![first_path, second_path];
        let new_path = env::join_paths(path_bufs).unwrap();
        env::set_var("PATH", &new_path);
    }

    mod without_pathext_set {
        use super::{setup_path, setup_empty_pathext};
        pub use super::find_command;

        fn setup_environment() {
            setup_path();
            setup_empty_pathext();
        }

        mod argument_without_extension {
            use super::{setup_environment, find_command};

            #[test]
            fn command_exists() {
                setup_environment();
                let result = find_command("bin_with_no_extension");
                assert_eq!(result.is_some(), true);
            }

            #[test]
            fn command_does_not_exist() {
                setup_environment();
                let result = find_command("missing");
                assert_eq!(result.is_some(), false);
            }

            #[test]
            fn command_exists_with_extension() {
                setup_environment();
                let result = find_command("win95_dominator");
                assert_eq!(result.is_some(), false);
            }
        }

        mod argument_with_extension {
            use std::fs::canonicalize;
            use super::{setup_environment, find_command};

            #[test]
            fn command_exists() {
                setup_environment();
                let result = find_command("bin_with_extension.exe");
                assert_eq!(result.is_some(), true);
            }

            #[test]
            fn command_does_not_exist() {
                setup_environment();
                let result = find_command("missing.com");
                assert_eq!(result.is_some(), false);
            }

            #[test]
            fn command_different_extension_does_exist() {
                setup_environment();
                let result = find_command("bin_with_extension.com");
                assert_eq!(result.is_some(), false);
            }

            #[test]
            fn first_command_on_path_found() {
                setup_environment();
                let target_path = canonicalize("./tests/fixtures/plan.sh").unwrap();
                let result = find_command("plan.sh");
                let found_path = result.unwrap();
                assert_eq!(found_path, target_path);
            }
        }
    }

    #[cfg(target_os = "windows")]
    mod with_pathext_set {
        use super::{setup_path, setup_pathext};
        pub use super::find_command;

        fn setup_environment() {
            setup_path();
            setup_pathext();
        }

        mod argument_without_extension {
            use super::{setup_environment, find_command};

            #[test]
            fn command_exists() {
                setup_environment();
                let result = find_command("bin_with_no_extension");
                assert_eq!(result.is_some(), true);
            }

            #[test]
            fn command_does_not_exist() {
                setup_environment();
                let result = find_command("missing");
                assert_eq!(result.is_some(), false);
            }

            #[test]
            #[allow(non_snake_case)]
            fn command_exists_with_extension_in_PATHEXT() {
                setup_environment();
                let result = find_command("bin_with_extension");
                assert_eq!(result.is_some(), true);
            }

            #[test]
            #[allow(non_snake_case)]
            fn command_exists_with_extension_not_in_PATHEXT() {
                setup_environment();
                let result = find_command("win95_dominator");
                assert_eq!(result.is_some(), false);
            }
        }

        mod argument_with_extension {
            use std::fs::canonicalize;
            use super::{setup_environment, find_command};

            #[test]
            fn command_exists() {
                setup_environment();
                let result = find_command("bin_with_extension.exe");
                assert_eq!(result.is_some(), true);
            }

            #[test]
            fn command_does_not_exist() {
                setup_environment();
                let result = find_command("missing.com");
                assert_eq!(result.is_some(), false);
            }

            #[test]
            fn command_different_extension_does_exist() {
                setup_environment();
                let result = find_command("bin_with_extension.com");
                assert_eq!(result.is_some(), false);
            }

            #[test]
            fn first_command_on_path_found() {
                setup_environment();
                let target_path = canonicalize("./tests/fixtures/plan.sh").unwrap();
                let result = find_command("plan.sh");
                let found_path = result.unwrap();
                assert_eq!(found_path, target_path);
            }
        }
    }
}
