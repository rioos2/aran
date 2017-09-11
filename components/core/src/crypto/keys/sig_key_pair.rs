use std::path::{Path, PathBuf};

use base64;

use openssl::pkey::PKey;
use openssl::rsa::Rsa;

use error::{Error, Result};
use super::{mk_key_filename, read_key_bytes, write_keypair_files, KeyPair, KeyType, PairType};
use super::super::{PUBLIC_KEY_SUFFIX, SECRET_SIG_KEY_SUFFIX};

pub type SigKeyPair = KeyPair<Vec<u8>, Vec<u8>>;

//probably make it as a default
fn generate_rsa_pair() -> Result<(Vec<u8>, Vec<u8>)> {
    let rsa = Rsa::generate(2048).unwrap();

    let in_rsa: PKey = PKey::from_rsa(rsa).unwrap();

    let public: Result<Vec<u8>> = match in_rsa.public_key_to_pem() {
        Ok(p) => Ok(p),
        Err(e) => return Err(Error::CryptoError(format!("[pub]: {}", e))),
    };


    let secret: Result<Vec<u8>> = match in_rsa.private_key_to_pem() {
        Ok(s) => Ok(s),
        Err(e) => return Err(Error::CryptoError(format!("[secret]: {}", e))),
    };

    Ok((public?, secret?))
}

impl SigKeyPair {
    pub fn generate_pair_for_origin<P: AsRef<Path> + ?Sized>(name: &str, cache_key_path: &P) -> Result<Self> {
        debug!("new sig key name = {}", &name);

        let (public, secret) = try!(Self::generate_pair_files(&name, cache_key_path.as_ref()));

        Ok(Self::new(name.to_string(), Some(public), Some(secret)))
    }

    fn generate_pair_files(name_with_rev: &str, cache_key_path: &Path) -> Result<(Vec<u8>, Vec<u8>)> {
        let (public, secret) = generate_rsa_pair()?;

        let public_keyfile = mk_key_filename(cache_key_path, name_with_rev, PUBLIC_KEY_SUFFIX);
        let secret_keyfile = mk_key_filename(cache_key_path, name_with_rev, SECRET_SIG_KEY_SUFFIX);

        debug!("public keyfile = {}", public_keyfile.display());
        debug!("secret keyfile = {}", secret_keyfile.display());

        try!(write_keypair_files(
            KeyType::Sig,
            &name_with_rev,
            Some(&public_keyfile),
            Some(&public[..]),
            Some(&secret_keyfile),
            Some(&secret[..]),
        ));
        Ok((public, secret))
    }

    /// Return a Vec of origin keys with a given name.
    /// The newest key is listed first in the Vec.
    pub fn get_pairs_for<P: AsRef<Path> + ?Sized>(name: &str, cache_key_path: &P, pair_type: Option<&PairType>) -> Result<Vec<Self>> {
        let mut key_pairs = Vec::new();

        debug!("Attempting to read key name {}", name);

        let kp = try!(Self::get_pair_for(name, cache_key_path));
        key_pairs.push(kp);

        Ok(key_pairs)
    }

    pub fn get_pair_for<P: AsRef<Path> + ?Sized>(name_with_rev: &str, cache_key_path: &P) -> Result<Self> {
        let pk = match Self::get_public_key(name_with_rev, cache_key_path.as_ref()) {
            Ok(k) => Some(k),
            Err(e) => {
                // Not an error, just continue
                debug!(
                    "Can't find public key for name_with_rev {}: {}",
                    name_with_rev,
                    e
                );
                None
            }
        };
        let sk = match Self::get_secret_key(name_with_rev, cache_key_path.as_ref()) {
            Ok(k) => Some(k),
            Err(e) => {
                // Not an error, just continue
                debug!(
                    "Can't find secret key for name_with_rev {}: {}",
                    name_with_rev,
                    e
                );
                None
            }
        };
        if pk == None && sk == None {
            let msg = format!(
                "No public or secret keys found for name_with_rev {}",
                name_with_rev
            );
            return Err(Error::CryptoError(msg));
        }
        Ok(SigKeyPair::new(name_with_rev.to_string(), pk, sk))
    }

    pub fn get_latest_pair_for<P: AsRef<Path> + ?Sized>(name: &str, cache_key_path: &P, pair_type: Option<&PairType>) -> Result<Self> {
        let mut all = try!(Self::get_pairs_for(name, cache_key_path, pair_type));
        match all.len() {
            0 => {
                let msg = format!("No revisions found for {} sig key", name);
                return Err(Error::CryptoError(msg));
            }
            _ => Ok(all.remove(0)),
        }
    }

    pub fn get_public_key_path<P: AsRef<Path> + ?Sized>(key_with_rev: &str, cache_key_path: &P) -> Result<PathBuf> {
        let path = mk_key_filename(cache_key_path.as_ref(), key_with_rev, PUBLIC_KEY_SUFFIX);
        if !path.is_file() {
            return Err(Error::CryptoError(
                format!("No public key found at {}", path.display()),
            ));
        }
        Ok(path)
    }

    pub fn get_secret_key_path<P: AsRef<Path> + ?Sized>(key_with_rev: &str, cache_key_path: &P) -> Result<PathBuf> {
        let path = mk_key_filename(cache_key_path.as_ref(), key_with_rev, SECRET_SIG_KEY_SUFFIX);
        if !path.is_file() {
            return Err(Error::CryptoError(
                format!("No secret key found at {}", path.display()),
            ));
        }
        Ok(path)
    }

    fn get_public_key(key_with_rev: &str, cache_key_path: &Path) -> Result<Vec<u8>> {
        let public_keyfile = mk_key_filename(cache_key_path, key_with_rev, PUBLIC_KEY_SUFFIX);
        let bytes = try!(read_key_bytes(&public_keyfile));

        match PKey::public_key_from_pem(&bytes) {
            Ok(sk) => Ok(sk.public_key_to_pem().unwrap()),
            Err(e) => {
                return Err(Error::CryptoError(format!(
                    "Can't read sig public key for {}\nErrorStack: {}",
                    key_with_rev,
                    e
                )))
            }
        }
    }

    fn get_secret_key(key_with_rev: &str, cache_key_path: &Path) -> Result<Vec<u8>> {
        let secret_keyfile = mk_key_filename(cache_key_path, key_with_rev, SECRET_SIG_KEY_SUFFIX);
        let bytes = try!(read_key_bytes(&secret_keyfile));

        match PKey::private_key_from_pem(&bytes) {
            Ok(sk) => Ok(sk.public_key_to_pem().unwrap()),
            Err(e) => {
                return Err(Error::CryptoError(format!(
                    "Can't read sig secret key for {}\nErrorStack: {}",
                    key_with_rev,
                    e
                )))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use std::fs::{self, File};
    use std::io::Read;

    use tempdir::TempDir;

    use super::SigKeyPair;
    use super::super::PairType;
    use super::super::super::test_support::*;

    static VALID_KEY: &'static str = "ca.key";
    static VALID_PUB: &'static str = "ca.crt";
    static VALID_NAME_WITH_REV: &'static str = "ca.crt";

    #[test]
    fn empty_struct() {
        let pair = SigKeyPair::new("grohl".to_string(), None, None);

        assert_eq!(pair.name, "grohl");
        assert_eq!(pair.public, None);

        match pair.public() {
            Ok(_) => panic!("Empty pair should not have a public key"),
            Err(_) => assert!(true),
        }
        assert_eq!(pair.secret, None);
        match pair.secret() {
            Ok(_) => panic!("Empty pair should not have a secret key"),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn generated_origin_pair() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();

        assert_eq!(pair.name, "unicorn");
        match pair.public() {
            Ok(_) => assert!(true),
            Err(_) => panic!("Generated pair should have a public key"),
        }
        match pair.secret() {
            Ok(_) => assert!(true),
            Err(_) => panic!("Generated pair should have a secret key"),
        }
        assert!(cache.path().join(format!("{}.crt", pair.name)).exists());
        assert!(cache.path().join(format!("{}.key", pair.name)).exists());
    }

    #[test]
    fn get_pairs_for() {
        let cache = TempDir::new("key_cache").unwrap();
        let pairs = SigKeyPair::get_pairs_for("unicorn", cache.path(), None).unwrap();
        assert_eq!(pairs.len(), 0);

        let _ = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let pairs = SigKeyPair::get_pairs_for("unicorn", cache.path(), None).unwrap();
        assert_eq!(pairs.len(), 1);

        let _ = match wait_until_ok(|| {
            SigKeyPair::generate_pair_for_origin("unicorn", cache.path())
        }) {
            Some(pair) => pair,
            None => panic!("Failed to generate another keypair after waiting"),
        };
        let pairs = SigKeyPair::get_pairs_for("unicorn", cache.path(), None).unwrap();
        assert_eq!(pairs.len(), 2);

        // We should not include another named key in the count
        let _ = SigKeyPair::generate_pair_for_origin("dragon", cache.path()).unwrap();
        let pairs = SigKeyPair::get_pairs_for("unicorn", cache.path(), None).unwrap();
        assert_eq!(pairs.len(), 2);

        // We should be able to count public and private keys separately
        let pairs = SigKeyPair::get_pairs_for("unicorn", cache.path(), Some(&PairType::Secret)).unwrap();
        assert_eq!(pairs.len(), 2);

        let pairs = SigKeyPair::get_pairs_for("unicorn", cache.path(), Some(&PairType::Public)).unwrap();
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn get_pair_for() {
        let cache = TempDir::new("key_cache").unwrap();
        let p1 = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let p2 = match wait_until_ok(|| {
            SigKeyPair::generate_pair_for_origin("unicorn", cache.path())
        }) {
            Some(pair) => pair,
            None => panic!("Failed to generate another keypair after waiting"),
        };

        let p1_fetched = SigKeyPair::get_pair_for(&p1.name, cache.path()).unwrap();
        assert_eq!(p1.name, p1_fetched.name);
        let p2_fetched = SigKeyPair::get_pair_for(&p2.name, cache.path()).unwrap();
        assert_eq!(p2.name, p2_fetched.name);
    }

    #[test]
    #[should_panic(expected = "No public or secret keys found for")]
    fn get_pair_for_nonexistent() {
        let cache = TempDir::new("key_cache").unwrap();
        SigKeyPair::get_pair_for("nope-nope-20160405144901", cache.path()).unwrap();
    }

    #[test]
    fn get_latest_pair_for_single() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();

        let latest = SigKeyPair::get_latest_pair_for("unicorn", cache.path(), None).unwrap();
        assert_eq!(latest.name, pair.name);
    }

    #[test]
    fn get_latest_pair_for_multiple() {
        let cache = TempDir::new("key_cache").unwrap();
        let _ = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let p2 = match wait_until_ok(|| {
            SigKeyPair::generate_pair_for_origin("unicorn", cache.path())
        }) {
            Some(pair) => pair,
            None => panic!("Failed to generate another keypair after waiting"),
        };

        let latest = SigKeyPair::get_latest_pair_for("unicorn", cache.path(), None).unwrap();
        assert_eq!(latest.name, p2.name);
    }

    #[test]
    fn get_latest_pair_for_secret() {
        let cache = TempDir::new("key_cache").unwrap();
        let p = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let latest = SigKeyPair::get_latest_pair_for("unicorn", cache.path(), Some(&PairType::Secret)).unwrap();
        assert_eq!(latest.name, p.name);
    }

    #[test]
    fn get_latest_pair_for_public() {
        let cache = TempDir::new("key_cache").unwrap();
        let p = SigKeyPair::generate_pair_for_origin("unicorn", cache.path()).unwrap();
        let latest = SigKeyPair::get_latest_pair_for("unicorn", cache.path(), Some(&PairType::Public)).unwrap();
        assert_eq!(latest.name, p.name);
    }

    #[test]
    #[should_panic(expected = "No revisions found for")]
    fn get_latest_pair_for_nonexistent() {
        let cache = TempDir::new("key_cache").unwrap();
        SigKeyPair::get_latest_pair_for("nope-nope", cache.path(), None).unwrap();
    }

    #[test]
    fn get_public_key_path() {
        let cache = TempDir::new("key_cache").unwrap();
        fs::copy(
            fixture(&format!("keys/{}", VALID_PUB)),
            cache.path().join(VALID_PUB),
        ).unwrap();

        let result = SigKeyPair::get_public_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
        assert_eq!(result, cache.path().join(VALID_PUB));
    }

    #[test]
    #[should_panic(expected = "No public key found at")]
    fn get_public_key_path_nonexistent() {
        let cache = TempDir::new("key_cache").unwrap();
        SigKeyPair::get_public_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
    }

    #[test]
    fn get_secret_key_path() {
        let cache = TempDir::new("key_cache").unwrap();
        fs::copy(
            fixture(&format!("keys/{}", VALID_KEY)),
            cache.path().join(VALID_KEY),
        ).unwrap();

        let result = SigKeyPair::get_secret_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
        assert_eq!(result, cache.path().join(VALID_KEY));
    }

    #[test]
    #[should_panic(expected = "No secret key found at")]
    fn get_secret_key_path_nonexistent() {
        let cache = TempDir::new("key_cache").unwrap();
        SigKeyPair::get_secret_key_path(VALID_NAME_WITH_REV, cache.path()).unwrap();
    }

}
