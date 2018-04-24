// Copyright 2018 The Rio Advancement Inc

use std::fs;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::result;
use std::str::FromStr;

use fs::open_from;
use error::{Error, Result};
use util::perm;

use super::REGULAR_KEY_PERMISSIONS;

//Note: This file beautifies using rust-fmt if the below line is commented.
//
pub mod sig_key_pair;

//The pair saver extn is used to provide inputs on the extension the secret pair will be
//saved as.
#[derive(Clone, Debug)]
pub enum PairSaverExtn {
    PubRSA,
    PemX509,
    PfxPKCS12,
    DSA,
    ED25519,
}

impl fmt::Display for PairSaverExtn {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PairSaverExtn::PubRSA => write!(f, "rsa"),
            PairSaverExtn::PemX509 => write!(f, "x509"),
            PairSaverExtn::PfxPKCS12 => write!(f, "pkcs12"),
            PairSaverExtn::DSA => write!(f, "dsa"),
            PairSaverExtn::ED25519 => write!(f, "ed25519"),
        }
    }
}

//The pair configuration provides a wrapper to the generator of keypair about details
//governing the configuration needed during the save, if the pair needs to be saved as file or not
//and the extn types the file needs to be saved
#[derive(Debug)]
pub struct PairConf {
    save: bool,
    bit_len: Option<u32>,
    save_as_extn: PairSaverExtn,
}

impl PairConf {
    //Default bit lenght for RSA keys and `X509_REQ`
    const DEFAULT_BIT_LENGTH: u32 = 2048;

    //Default Pair configuration which does a save,
    //generates pair with default bit length and
    //save extn is PEM_RSA.
    pub fn new() -> Self {
        PairConf {
            save: true,
            bit_len: None,
            save_as_extn: PairSaverExtn::PubRSA,
        }
    }

    //Pair configuration which allows to change the saved extn.
    //generates pair with default bit length and
    //save extn is as per input
    pub fn with_extn(extn: PairSaverExtn) -> Self {
        PairConf {
            save: true,
            bit_len: None,
            save_as_extn: extn,
        }
    }

    //Pair configuration which allows to change the saved extn, save, and the save as extn.
    //generates pair with input bit length, saved in a file (or) not,
    //and a save extn as per input
    pub fn with_save(save: bool, bit_len: Option<u32>, extn: PairSaverExtn) -> Self {
        PairConf {
            save: save,
            bit_len: bit_len,
            save_as_extn: extn,
        }
    }

    fn save(&self) -> bool {
        self.save
    }

    fn bit_len(&self) -> u32 {
        self.bit_len.unwrap_or(Self::DEFAULT_BIT_LENGTH)
    }

    fn save_as_extn(&self) -> PairSaverExtn {
        self.save_as_extn.clone()
    }
}

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
                return Err(Error::CryptoError(format!(
                    "Invalid PairType conversion from {}",
                    value
                )))
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
#[derive(Clone, Debug)]
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
    path.as_ref()
        .join(format!("{}.{}", keyname.as_ref(), suffix.as_ref()))
}

fn read_key_bytes(keyfile: &Path) -> Result<Vec<u8>> {
    let mut f = try!(open_from(keyfile));
    let mut s = String::new();
    if try!(f.read_to_string(&mut s)) <= 0 {
        return Err(Error::CryptoError("Can't read key bytes".to_string()));
    }

    Ok(s.into_bytes())
}

pub fn read_key_in_bytes(keyfile: &Path) -> Result<Vec<u8>> {
    let mut f = try!(open_from(keyfile));
    let mut v = vec![];
    if try!(f.read_to_end(&mut v)) <= 0 {
        return Err(Error::CryptoError("Can't read key bytes".to_string()));
    }

    Ok(v)
}

//This is a common write to wite a file.
//Change the name public to keyfile & PUBLIC_KEY_PERMISSION to COMMON_KEY_PERMISSION
fn write_key_file(regular_keyfile: Option<&Path>, regular_content: Option<&[u8]>) -> Result<()> {
    if let Some(regular_keyfile) = regular_keyfile {
        let regular_content = match regular_content {
            Some(c) => c,
            None => panic!("Invalid calling of this function"),
        };

        if let Some(pk_dir) = regular_keyfile.parent() {
            try!(fs::create_dir_all(pk_dir));
        } else {
            return Err(Error::BadKeyPath(
                regular_keyfile.to_string_lossy().into_owned(),
            ));
        }

        if regular_keyfile.exists() {
            return Err(Error::CryptoError(format!(
                "Keyfile or a directory already \
                 exists {}",
                regular_keyfile.display()
            )));
        }

        let regular_file = try!(File::create(regular_keyfile));
        let mut regular_writer = BufWriter::new(&regular_file);
        try!(regular_writer.write_all(regular_content));
        try!(perm::set_permissions(
            regular_keyfile,
            REGULAR_KEY_PERMISSIONS,
        ));
    }

    Ok(())
}

///Write a pair public and secret.
//Calls and write_key_file for public and secret separately.
fn write_keypair_files(public_keyfile: Option<&Path>, public_content: Option<&[u8]>, secret_keyfile: Option<&Path>, secret_content: Option<&[u8]>) -> Result<()> {
    write_key_file(public_keyfile, public_content)?;
    write_key_file(secret_keyfile, secret_content)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use std::fs::{self, File};

    use hex;
    use tempdir::TempDir;

    use super::super::test_support::*;

    static VALID_KEY: &'static str = "ring-key-valid-20160504220722.sym.key";
    static VALID_KEY_AS_HEX: &'static str = "\
                                             53594d2d5345432d310a72696e672d6b65792d76616c69642d32303136303530343232303732320a0a524346614f38346a3431476d727a576464784d6473587047646e3369754979374d77337859726a504c73453d";

    #[test]
    fn read_key_bytes() {
        let cache = TempDir::new("key_cache").unwrap();
        let keyfile = cache.path().join(VALID_KEY);
        fs::copy(fixture(&format!("keys/{}", VALID_KEY)), &keyfile).unwrap();
        println!("keyfile {:?}", keyfile);
        let result = super::read_key_bytes(keyfile.as_path()).unwrap();
        assert_eq!(hex::encode(result.as_slice()), VALID_KEY_AS_HEX);
    }

    #[test]
    #[should_panic(expected = "Can\\'t read key bytes")]
    fn read_key_bytes_empty_file() {
        let cache = TempDir::new("key_cache").unwrap();
        let keyfile = cache.path().join("not-much-here");
        let _ = File::create(&keyfile).unwrap();

        super::read_key_bytes(keyfile.as_path()).unwrap();
    }

}
