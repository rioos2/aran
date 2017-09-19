// Copyright (c) 2017 RioCorp Inc.

use std::fs;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::result;
use std::str::FromStr;

use regex::Regex;

use error::{Error, Result};
use util::perm;

use super::{PUBLIC_KEY_PERMISSIONS, SECRET_KEY_PERMISSIONS};

lazy_static! {
    static ref ORIGIN_NAME_RE: Regex = Regex::new(r"\A[a-z0-9][a-z0-9_-]*\z").unwrap();
}

pub mod sig_key_pair;


#[derive(Debug, Eq, PartialEq)]
pub enum PairType {
    Public,
    Secret,
}

impl fmt::Display for PairType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PairType::Public => write!(f, "public"),
            PairType::Secret => write!(f, "secret"),
        }
    }
}

impl FromStr for PairType {
    type Err = Error;

    fn from_str(value: &str) -> result::Result<Self, Self::Err> {
        match value {
            "public" => Ok(PairType::Public),
            "secret" => Ok(PairType::Secret),
            _ => {
                return Err(Error::CryptoError(
                    format!("Invalid PairType conversion from {}", value),
                ))
            }
        }
    }
}


/// A pair of related keys (public and secret) which have a name and revision.
///
/// Depending on the type of keypair, the public key may be empty or not apply, or one or both of
/// the keys may not be present due to the loading context. For example, the act of verifying a
/// signed message or artifact only requires the public key to be present, whereas the act of
/// signing will require the secret key to be present.
#[derive(Clone)]
pub struct KeyPair<P, S> {
    /// The name of the key, ex: "habitat"
    pub name: String,
    /// The public key component, if relevant
    pub public: Option<P>,
    /// The private key component, if relevant
    pub secret: Option<S>,
}

impl<P, S> KeyPair<P, S> {
    /// Creates a new `KeyPair`.
    pub fn new(name: String, p: Option<P>, s: Option<S>) -> KeyPair<P, S> {
        KeyPair {
            name: name,
            public: p,
            secret: s,
        }
    }

    pub fn public(&self) -> Result<&P> {
        match self.public.as_ref() {
            Some(s) => Ok(s),
            None => {
                let msg = format!("Public key is required but not present for {}", self.name);
                return Err(Error::CryptoError(msg));
            }
        }
    }

    pub fn secret(&self) -> Result<&S> {
        match self.secret.as_ref() {
            Some(s) => Ok(s),
            None => {
                let msg = format!("Secret key is required but not present for {}", self.name);
                return Err(Error::CryptoError(msg));
            }
        }
    }
}


fn mk_key_filename<P, S1, S2>(path: P, keyname: S1, suffix: S2) -> PathBuf
where
    P: AsRef<Path>,
    S1: AsRef<str>,
    S2: AsRef<str>,
{
    path.as_ref().join(format!(
        "{}.{}",
        keyname.as_ref(),
        suffix.as_ref()
    ))
}


/// Is the string a valid ca name?
pub fn is_valid_ca_name(name: &str) -> bool {
    name.chars().count() <= 255 && ORIGIN_NAME_RE.is_match(name)
}

fn read_key_bytes(keyfile: &Path) -> Result<Vec<u8>> {
    let mut f = try!(File::open(keyfile));
    let mut s = String::new();
    if try!(f.read_to_string(&mut s)) <= 0 {
        return Err(Error::CryptoError("Can't read key bytes".to_string()));
    }

    Ok(s.into_bytes())

}

fn write_keypair_files(public_keyfile: Option<&Path>, public_content: Option<&[u8]>, secret_keyfile: Option<&Path>, secret_content: Option<&[u8]>) -> Result<()> {
    if let Some(public_keyfile) = public_keyfile {

        let public_content = match public_content {
            Some(c) => c,
            None => panic!("Invalid calling of this function"),
        };

        if let Some(pk_dir) = public_keyfile.parent() {
            try!(fs::create_dir_all(pk_dir));
        } else {
            return Err(Error::BadKeyPath(
                public_keyfile.to_string_lossy().into_owned(),
            ));
        }

        if public_keyfile.exists() {
            return Err(Error::CryptoError(format!(
                "Public keyfile or a directory already \
                                                   exists {}",
                public_keyfile.display()
            )));
        }

        let public_file = try!(File::create(public_keyfile));
        let mut public_writer = BufWriter::new(&public_file);
        try!(public_writer.write_all(public_content));
        try!(perm::set_permissions(
            public_keyfile,
            PUBLIC_KEY_PERMISSIONS,
        ));
    }

    if let Some(secret_keyfile) = secret_keyfile {
        let secret_content = match secret_content {
            Some(c) => c,
            None => panic!("Invalid calling of this function"),
        };

        if let Some(sk_dir) = secret_keyfile.parent() {
            try!(fs::create_dir_all(sk_dir));
        } else {
            return Err(Error::BadKeyPath(
                secret_keyfile.to_string_lossy().into_owned(),
            ));
        }
        if secret_keyfile.exists() {
            return Err(Error::CryptoError(format!(
                "Secret keyfile or a directory already \
                                                   exists {}",
                secret_keyfile.display()
            )));
        }
        let secret_file = try!(File::create(secret_keyfile));
        let mut secret_writer = BufWriter::new(&secret_file);
        try!(secret_writer.write_all(secret_content));
        try!(perm::set_permissions(
            secret_keyfile,
            SECRET_KEY_PERMISSIONS,
        ));
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::fs::{self, File};
    use std::io::Write;
    use std::{thread, time};

    use hex::ToHex;
    use tempdir::TempDir;

    use super::sig_key_pair::SigKeyPair;
    use super::PairType;

    use super::super::test_support::*;

    static VALID_KEY: &'static str = "ring-key-valid-20160504220722.sym.key";
    static VALID_KEY_AS_HEX: &'static str = "\
        44215a3bce23e351a6af359d77131db17a46767de2b88cbb330df162b8cf2ec1";


    #[test]
    fn read_key_bytes() {
        let cache = TempDir::new("key_cache").unwrap();
        let keyfile = cache.path().join(VALID_KEY);
        fs::copy(fixture(&format!("keys/{}", VALID_KEY)), &keyfile).unwrap();
        println!("keyfile {:?}", keyfile);
        let result = super::read_key_bytes(keyfile.as_path()).unwrap();
        assert_eq!(result.as_slice().to_hex(), VALID_KEY_AS_HEX);
    }

    #[test]
    #[should_panic(expected = "Can\\'t read key bytes")]
    fn read_key_bytes_empty_file() {
        let cache = TempDir::new("key_cache").unwrap();
        let keyfile = cache.path().join("not-much-here");
        let _ = File::create(&keyfile).unwrap();

        super::read_key_bytes(keyfile.as_path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Malformed key contents for")]
    fn read_key_bytes_missing_newlines() {
        let cache = TempDir::new("key_cache").unwrap();
        let keyfile = cache.path().join("missing-newlines");
        let mut f = File::create(&keyfile).unwrap();
        f.write_all("SOMETHING\nELSE\n".as_bytes()).unwrap();

        super::read_key_bytes(keyfile.as_path()).unwrap();
    }

    #[test]
    #[should_panic(expected = "Can\\'t read raw key from")]
    fn read_key_bytes_malformed_base64() {
        let cache = TempDir::new("key_cache").unwrap();
        let keyfile = cache.path().join("missing-newlines");
        let mut f = File::create(&keyfile).unwrap();
        f.write_all("header\nsomething\n\nI am not base64 content".as_bytes())
            .unwrap();

        super::read_key_bytes(keyfile.as_path()).unwrap();
    }
}
