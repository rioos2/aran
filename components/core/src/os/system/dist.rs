use regex::Regex;
use std::fmt;

#[derive(Debug, Clone)]
pub struct PartialTargetTriple {
    pub arch: Option<String>,
    pub os: Option<String>,
    pub env: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct TargetTriple(String);

// These lists contain the targets known to rustup, and used to build
// the PartialTargetTriple.

static LIST_ARCHS: &'static [&'static str] = &[
    "i386",
    "i586",
    "i686",
    "x86_64",
    "arm",
    "armv7",
    "armv7s",
    "aarch64",
    "mips",
    "mipsel",
    "mips64",
    "mips64el",
    "powerpc",
    "powerpc64",
    "powerpc64le",
    "s390x",
];
static LIST_OSES: &'static [&'static str] = &[
    "pc-windows",
    "unknown-linux",
    "apple-darwin",
    "unknown-netbsd",
    "apple-ios",
    "linux",
    "rumprun-netbsd",
    "unknown-freebsd",
];
static LIST_ENVS: &'static [&'static str] = &["gnu", "msvc", "gnueabi", "gnueabihf", "gnuabi64", "androideabi", "android", "musl"];

// MIPS platforms don't indicate endianness in uname, however binaries only
// run on boxes with the same endianness, as expected.
// Hence we could distinguish between the variants with compile-time cfg()
// attributes alone.
#[cfg(all(not(windows), target_endian = "big"))]
const TRIPLE_MIPS_UNKNOWN_LINUX_GNU: &'static str = "mips-unknown-linux-gnu";
#[cfg(all(not(windows), target_endian = "little"))]
const TRIPLE_MIPS_UNKNOWN_LINUX_GNU: &'static str = "mipsel-unknown-linux-gnu";

#[cfg(all(not(windows), target_endian = "big"))]
const TRIPLE_MIPS64_UNKNOWN_LINUX_GNUABI64: &'static str = "mips64-unknown-linux-gnuabi64";
#[cfg(all(not(windows), target_endian = "little"))]
const TRIPLE_MIPS64_UNKNOWN_LINUX_GNUABI64: &'static str = "mips64el-unknown-linux-gnuabi64";

impl TargetTriple {
    pub fn from_str(name: &str) -> Self {
        TargetTriple(name.to_string())
    }

    pub fn from_build() -> Self {
        TargetTriple::from_str("x86_64-unknown-freebsd")
    }

    pub fn from_host() -> Option<Self> {
        #[cfg(windows)]
        fn inner() -> Option<TargetTriple> {
            use std::mem;
            use winapi::um::sysinfoapi::GetNativeSystemInfo;

            // First detect architecture
            const PROCESSOR_ARCHITECTURE_AMD64: u16 = 9;
            const PROCESSOR_ARCHITECTURE_INTEL: u16 = 0;

            let mut sys_info;
            unsafe {
                sys_info = mem::zeroed();
                GetNativeSystemInfo(&mut sys_info);
            }

            let arch = match unsafe { sys_info.u.s() }.wProcessorArchitecture {
                PROCESSOR_ARCHITECTURE_AMD64 => "x86_64",
                PROCESSOR_ARCHITECTURE_INTEL => "i686",
                _ => return None,
            };

            // Default to msvc
            let msvc_triple = format!("{}-pc-windows-msvc", arch);
            Some(TargetTriple(msvc_triple))
        }

        #[cfg(not(windows))]
        fn inner() -> Option<TargetTriple> {
            use libc;
            use std::ffi::CStr;
            use std::mem;

            let mut sys_info;
            let (sysname, machine) = unsafe {
                sys_info = mem::zeroed();
                if libc::uname(&mut sys_info) != 0 {
                    return None;
                }

                (
                    CStr::from_ptr(sys_info.sysname.as_ptr()).to_bytes(),
                    CStr::from_ptr(sys_info.machine.as_ptr()).to_bytes(),
                )
            };

            let host_triple = match (sysname, machine) {
                (_, b"arm") if cfg!(target_os = "android") => Some("arm-linux-androideabi"),
                (_, b"armv7l") if cfg!(target_os = "android") => Some("armv7-linux-androideabi"),
                (_, b"armv8l") if cfg!(target_os = "android") => Some("armv7-linux-androideabi"),
                (_, b"aarch64") if cfg!(target_os = "android") => Some("aarch64-linux-android"),
                (_, b"i686") if cfg!(target_os = "android") => Some("i686-linux-android"),
                (_, b"x86_64") if cfg!(target_os = "android") => Some("x86_64-linux-android"),
                (b"Linux", b"x86_64") => Some("x86_64-unknown-linux-gnu"),
                (b"Linux", b"i686") => Some("i686-unknown-linux-gnu"),
                (b"Linux", b"mips") => Some(TRIPLE_MIPS_UNKNOWN_LINUX_GNU),
                (b"Linux", b"mips64") => Some(TRIPLE_MIPS64_UNKNOWN_LINUX_GNUABI64),
                (b"Linux", b"arm") => Some("arm-unknown-linux-gnueabi"),
                (b"Linux", b"armv7l") => Some("armv7-unknown-linux-gnueabihf"),
                (b"Linux", b"armv8l") => Some("armv7-unknown-linux-gnueabihf"),
                (b"Linux", b"aarch64") => Some("aarch64-unknown-linux-gnu"),
                (b"Darwin", b"x86_64") => Some("x86_64-apple-darwin"),
                (b"Darwin", b"i686") => Some("i686-apple-darwin"),
                (b"FreeBSD", b"x86_64") => Some("x86_64-unknown-freebsd"),
                (b"FreeBSD", b"i686") => Some("i686-unknown-freebsd"),
                (b"OpenBSD", b"x86_64") => Some("x86_64-unknown-openbsd"),
                (b"OpenBSD", b"i686") => Some("i686-unknown-openbsd"),
                (b"NetBSD", b"x86_64") => Some("x86_64-unknown-netbsd"),
                (b"NetBSD", b"i686") => Some("i686-unknown-netbsd"),
                (b"DragonFly", b"x86_64") => Some("x86_64-unknown-dragonfly"),
                _ => None,
            };

            host_triple.map(TargetTriple::from_str)
        }

        inner()
    }

    pub fn from_host_or_build() -> Self {
        Self::from_host().unwrap_or_else(Self::from_build)
    }
}

impl PartialTargetTriple {
    pub fn from_str(name: &str) -> Option<Self> {
        if name.is_empty() {
            return Some(PartialTargetTriple {
                arch: None,
                os: None,
                env: None,
            });
        }

        // Prepending `-` makes this next regex easier since
        // we can count  on all triple components being
        // delineated by it.
        let name = format!("-{}", name);
        let pattern = format!(
            r"^(?:-({}))?(?:-({}))?(?:-({}))?$",
            LIST_ARCHS.join("|"),
            LIST_OSES.join("|"),
            LIST_ENVS.join("|")
        );

        let re = Regex::new(&pattern).unwrap();
        re.captures(&name).map(|c| {
            fn fn_map(s: &str) -> Option<String> {
                if s == "" {
                    None
                } else {
                    Some(s.to_owned())
                }
            }

            PartialTargetTriple {
                arch: c.get(1).map(|s| s.as_str()).and_then(fn_map),
                os: c.get(2).map(|s| s.as_str()).and_then(fn_map),
                env: c.get(3).map(|s| s.as_str()).and_then(fn_map),
            }
        })
    }
}

impl fmt::Display for TargetTriple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
