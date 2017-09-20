// Copyright (c) 2017 RioCorp Inc.

use std::path::{Path, PathBuf};

use openssl::asn1::Asn1Time;
use openssl::bn::{BigNum, MSB_MAYBE_ZERO};

use openssl::hash::MessageDigest;
use openssl::pkey::PKey;
use openssl::rsa::Rsa;
use openssl::x509::{X509, X509Name};
use openssl::x509::extension::{BasicConstraints, KeyUsage, ExtendedKeyUsage, SubjectAlternativeName};

use error::{Error, Result};

use super::{mk_key_filename, read_key_bytes, write_keypair_files, KeyPair};
use super::super::{ROOT_CA, PUBLIC_KEY_SUFFIX, SECRET_SIG_KEY_SUFFIX};

pub type SigKeyPair = KeyPair<Vec<u8>, Vec<u8>>;

/// X509 certificate version 3. Its zero indexed, hence 2.
const X509_VERSION3: i32 = 2;
/// Default bit lenght for RSA keys and `X509_REQ`
const BIT_LENGTH: u32 = 2048;
///
const RIOOS_SIGNER: &'static str = "rioos-signer";
///
const RIOOS_CERT_DOMAINS: &'static [&'static str] = &[
    "localhost",
    "rioos.default",
    "rioos.default.svc",
    "rioos.default.svc.local",
    "rioos.default.cluster.local",
];
///
const RIOOS_CERT_IPS: &'static [&'static str] = &["127.0.0.1"];

// We create a root CA.
// We use the CA to sign certificates for api-server, and service-accounts
// Ruby explains the process we are trying to do here.
// http://ruby-doc.org/stdlib-2.4.1/libdoc/openssl/rdoc/OpenSSL.html
impl SigKeyPair {
    //Generates the root certificate authority ca.cert.csr, ca.key
    //in /var/lib/rioos/cache/keys (or)
    // .rioos/cache/keys
    pub fn root_ca<P: AsRef<Path> + ?Sized>(name: &str, cache_key_path: &P) -> Result<Self> {
        debug!("new root ca key name = {}", &name);

        let (public, secret) = try!(Self::generate_pair_ca_files(&name, cache_key_path.as_ref()));

        Ok(Self::new(name.to_string(), Some(public), Some(secret)))
    }

    //Generates signed certificates using root ca authority ca.csr, ca.pem in /var/lib/rioos/cache/keys
    //$ openssl verify -verbose -CAfile ca.cert.pem  api-server.cert.pem
    //$ openssl x509 -noout -text \
    //  -in api-server.cert.pem
    pub fn signed_with<P: AsRef<Path> + ?Sized>(name: &str, cache_key_path: &P) -> Result<Self> {
        debug!("new signed key name = {}", &name);

        let (public, secret) = try!(Self::generate_pair_signed_files(
            &name,
            cache_key_path.as_ref(),
        ));

        Ok(Self::new(name.to_string(), Some(public), Some(secret)))
    }

    fn generate_pair_ca_files(name_with_rev: &str, cache_key_path: &Path) -> Result<(Vec<u8>, Vec<u8>)> {

        let key = gen_key()?;

        let cert = gen_ca(&key)?; //self-signed certificate named ca.csr, ca.key

        let (public, secret) = (cert.to_pem()?, key.private_key_to_pem()?);

        let public_keyfile = mk_key_filename(cache_key_path, name_with_rev, PUBLIC_KEY_SUFFIX);
        let secret_keyfile = mk_key_filename(cache_key_path, name_with_rev, SECRET_SIG_KEY_SUFFIX);

        debug!("public keyfile = {}", public_keyfile.display());
        debug!("secret keyfile = {}", secret_keyfile.display());

        try!(write_keypair_files(
            Some(&public_keyfile),
            Some(&public[..]),
            Some(&secret_keyfile),
            Some(&secret[..]),
        ));
        Ok((public, secret))
    }

    /// Signs certificate.
    ///
    /// CSR and PKey will be generated if it doesn't set or loaded first.
    fn generate_pair_signed_files(name_with_rev: &str, cache_key_path: &Path) -> Result<(Vec<u8>, Vec<u8>)> {
        let key = gen_key()?;

        let ca = Self::get_pair_for(ROOT_CA, cache_key_path)?;
        let cert = gen_signed(&X509::from_pem(ca.public()?)?, &key)?;

        let (public, secret) = (cert.to_pem()?, key.private_key_to_pem()?);

        let public_keyfile = mk_key_filename(cache_key_path, name_with_rev, PUBLIC_KEY_SUFFIX);
        let secret_keyfile = mk_key_filename(cache_key_path, name_with_rev, SECRET_SIG_KEY_SUFFIX);

        debug!("public keyfile = {}", public_keyfile.display());
        debug!("secret keyfile = {}", secret_keyfile.display());

        try!(write_keypair_files(
            Some(&public_keyfile),
            Some(&public[..]),
            Some(&secret_keyfile),
            Some(&secret[..]),
        ));
        Ok((public, secret))

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

    //Returns the X509 certificate which is public key + subject information
    fn get_public_key(key_with_rev: &str, cache_key_path: &Path) -> Result<Vec<u8>> {
        let public_keyfile = mk_key_filename(cache_key_path, key_with_rev, PUBLIC_KEY_SUFFIX);
        let bytes = try!(read_key_bytes(&public_keyfile));

        match X509::from_pem(&bytes) {
            Ok(sk) => Ok(sk.to_pem().unwrap()),
            Err(e) => {
                return Err(Error::CryptoError(format!(
                    "Can't read sig public key for {}\n: {}",
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
                    "Can't read sig secret key for {}: {}",
                    key_with_rev,
                    e
                )))
            }
        }
    }
}



/// Generates a new PKey
fn gen_key() -> Result<PKey> {
    let rsa = Rsa::generate(BIT_LENGTH)?;
    let key = PKey::from_rsa(rsa)?;
    Ok(key)
}

/// An helper to generate a selfsigned certifate authority
fn gen_ca(pkey: &PKey) -> Result<X509> {

    let mut builder = X509::builder()?;

    let name = {
        let mut name = X509Name::builder()?;
        name.append_entry_by_text("CN", RIOOS_SIGNER)?;
        name.build()
    };

    builder.set_version(X509_VERSION3)?;
    builder.set_subject_name(&name)?;
    builder.set_issuer_name(&name)?;
    builder.set_not_before(&Asn1Time::days_from_now(0).unwrap())?;
    builder.set_not_after(
        &Asn1Time::days_from_now(365).unwrap(),
    )?;

    let mut serial = BigNum::new().unwrap();
    serial.rand(128, MSB_MAYBE_ZERO, false)?;
    builder.set_serial_number(
        &serial.to_asn1_integer().unwrap(),
    )?;

    let basic_constraints = BasicConstraints::new().critical().ca().build()?;
    builder.append_extension(basic_constraints)?;

    let key_usage = KeyUsage::new()
        .critical()
        .digital_signature()
        .key_encipherment()
        .key_cert_sign()
        .crl_sign()
        .build()?;
    builder.append_extension(key_usage)?;

    builder.set_pubkey(&pkey)?;
    builder.sign(&pkey, MessageDigest::sha256())?;

    Ok(builder.build())
}


/// An helper to generate a certifate signing request
///
/// This function will generate a PKey
/// Needs the  ca_cert as X509
/// Returns X509 signed with root ca certificate
fn gen_signed(ca_cert: &X509, pkey: &PKey) -> Result<X509> {
    let mut builder = X509::builder()?;

    // I think, this isn't needed as it can be pulled from ca_cert
    let name = {
        let mut name = X509Name::builder()?;
        name.append_entry_by_text("CN", RIOOS_SIGNER)?;
        name.build()
    };

    builder.set_version(X509_VERSION3)?;
    builder.set_subject_name(&name)?;
    builder.set_issuer_name(&name)?;
    builder.set_not_before(&Asn1Time::days_from_now(0).unwrap())?;
    builder.set_not_after(
        &Asn1Time::days_from_now(365).unwrap(),
    )?;

    let mut serial = BigNum::new().unwrap();
    serial.rand(128, MSB_MAYBE_ZERO, false)?;
    builder.set_serial_number(
        &serial.to_asn1_integer().unwrap(),
    )?;

    let basic_constraints = BasicConstraints::new().critical().build()?;
    builder.append_extension(basic_constraints)?;
    let key_usage = KeyUsage::new()
        .critical()
        .digital_signature()
        .key_encipherment()
        .build()?;
    builder.append_extension(key_usage)?;

    let ext_key_usage = ExtendedKeyUsage::new().server_auth().build()?;
    builder.append_extension(ext_key_usage)?;

    let domains = RIOOS_CERT_DOMAINS;
    if domains.len() > 1 {
        let san_extension = {
            let mut san = SubjectAlternativeName::new();
            for domain in domains.iter() {
                san.dns(domain);
            }

            let ips = RIOOS_CERT_IPS;
            for ip in ips.iter() {
                san.ip(ip);
            }
            san.build(&builder.x509v3_context(Some(ca_cert), None))?
        };
        builder.append_extension(san_extension)?;
    }

    builder.set_pubkey(&pkey)?;

    builder.sign(&pkey, MessageDigest::sha256())?;

    Ok(builder.build())
}




#[cfg(test)]
mod test {
    use std::fs::{self, File};
    use std::io::Read;

    use tempdir::TempDir;

    use super::SigKeyPair;
    use super::super::super::test_support::*;

    static VALID_KEY: &'static str = "ca.key";
    static VALID_PUB: &'static str = "ca.cert.csr";
    static VALID_NAME_WITH_REV: &'static str = "ca.cert.csr";

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
    #[should_panic(expected = "No public or secret keys found for")]
    fn get_pair_for_nonexistent() {
        let cache = TempDir::new("key_cache").unwrap();
        SigKeyPair::get_pair_for("nope-nope-20160405144901", cache.path()).unwrap();
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
