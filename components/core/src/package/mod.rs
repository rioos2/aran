pub mod ident;
pub mod install;
pub mod metadata;
pub mod plan;
pub mod target;

pub use self::ident::{Identifiable, PackageIdent};
pub use self::install::PackageInstall;
pub use self::plan::Plan;
pub use self::target::{Target, PackageTarget};

#[cfg(test)]
pub mod test_support {
    use std::path::PathBuf;

    pub fn fixture_path(name: &str) -> PathBuf {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("fixtures")
            .join(name);
        path
    }
}
