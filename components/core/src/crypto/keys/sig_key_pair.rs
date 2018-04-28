// Copyright 2018 The Rio Advancement Inc

use std::path::{Path, PathBuf};

use openssl::asn1::Asn1Time;
use openssl::bn::{BigNum, MSB_MAYBE_ZERO};
use openssl::hash::MessageDigest;
use openssl::pkey::{PKey, PKeyRef};

use openssl::x509::{X509, X509NameBuilder, X509Req, X509ReqBuilder};
use openssl::x509::extension::{AuthorityKeyIdentifier, BasicConstraints, KeyUsage, SubjectAlternativeName, SubjectKeyIdentifier};

use openssl::rsa::Rsa;
use openssl::dsa::Dsa;
use openssl::pkcs12::Pkcs12;
use crypto::keys::{PairConf, PairSaverExtn};

use error::{Error, Result};
use error::Error::X509Error;

use super::{mk_key_filename, read_key_bytes, write_key_file, write_keypair_files, KeyPair};
use super::super::{PUBLIC_DSA_SUFFIX, PUBLIC_KEY_SUFFIX, PUBLIC_PFX_SUFFIX, PUBLIC_RSA_SUFFIX, ROOT_CA, SECRET_SIG_KEY_SUFFIX};

pub type SigKeyPair = KeyPair<Vec<u8>, Vec<u8>>;

/// X509 certificate version 3. Its zero indexed, hence 2.
const X509_VERSION3: i32 = 2;

///

const RIOOS_WITH_SIGNED: &'static str = "wi_rioos";

const RIOOS_COUNTRY: &'static str = "US";
const RIOOS_STATE: &'static str = "FL";
const RIOOS_ORGANIZATION: &'static str = "Rio Advancement Inc";
const RIOOS_CERT_DOMAINS: &'static [&'static str] = &[
    "*.rioos.svc.local",
    "*.rioos.xyz",
    "*.rioos.sh",
    "localhost",
    "rioos.default",
    "rioos.default.svc",
    "rioos.default.svc.local",
    "rioos.default.cluster.local",
];

const RIOOS_CERT_IPS: &'static [&'static str] = &["127.0.0.1"];
const RIOOS_CERT_EXPIRES_IN_DAYS: &'static u32 = &365;

const RIOOS_PFX_PASSWORD: &'static str = "TEAMRIOADVANCEMENT123";

struct PairSavingData {
    public_keyfile: PathBuf,
    public: Vec<u8>,
    multi: Option<bool>,
}

// We use the CA to sign certificates for api-server, and service-accounts
// Ruby explains the process we are trying to do here.
// http://ruby-doc.org/stdlib-2.4.1/libdoc/openssl/rdoc/OpenSSL.html
impl SigKeyPair {
    //Generates the root certificate authority ca.cert.csr, ca.key
    //in /var/lib/rioos/cache/keys (or)
    // .rioos/cache/keys
    pub fn mk_ca_cert<P: AsRef<Path> + ?Sized>(name: &str, conf: PairConf, cache_key_path: &P) -> Result<Self> {
        debug!("new root ca key name = {}", &name);

        let (public, secret) = try!(Self::gen_ca_pair(&name, conf, cache_key_path.as_ref()));

        Ok(Self::new(name.to_string(), Some(public), Some(secret)))
    }

    fn gen_ca_pair(name_with_rev: &str, conf: PairConf, cache_key_path: &Path) -> Result<(Vec<u8>, Vec<u8>)> {
        let key = gen_key_rsa(conf.bit_len())?;

        let cert = gen_ca(&key)?;

        let (public, secret) = (cert.to_pem()?, key.private_key_to_pem()?);

        if conf.save() {
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
        }

        Ok((public, secret))
    }

    //Generates signed certificates using root ca authority ca.csr, ca.pem in /var/lib/rioos/cache/keys
    //$ openssl verify -verbose -CAfile ca.cert.pem  api-server.cert.pem
    //$ openssl x509 -noout -text \
    //  -in api-server.cert.pem
    pub fn mk_signed<P: AsRef<Path> + ?Sized>(name: &str, conf: PairConf, cache_key_path: &P) -> Result<Self> {
        debug!("new signed key name = {}", &name);

        let (public, secret) = try!(Self::gen_ca_signed_pair(
            &name,
            conf,
            cache_key_path.as_ref(),
        ));

        Ok(Self::new(name.to_string(), Some(public), Some(secret)))
    }

    /// Signs certificate.
    ///
    /// CSR and PKey will be generated if it doesn't set or loaded first.
    fn gen_ca_signed_pair(name_with_rev: &str, conf: PairConf, cache_key_path: &Path) -> Result<(Vec<u8>, Vec<u8>)> {
        let privkey = {
            match conf.save_as_extn() {
                PairSaverExtn::PubRSA => gen_key_rsa(conf.bit_len())?,
                PairSaverExtn::PemX509 => gen_key_rsa(conf.bit_len())?,
                PairSaverExtn::PfxPKCS12 => gen_key_rsa(conf.bit_len())?,
                PairSaverExtn::DSA => gen_key_dsa(conf.bit_len())?,
                _ => {
                    let msg = format!(
                        "Private key not generated for this extension {}",
                        conf.save_as_extn()
                    );
                    return Err(Error::CryptoError(msg));
                }
            }
        };

        let ca = Self::get_pair_for(ROOT_CA, cache_key_path)?;

        let cert = gen_signed(
            &X509::from_pem(ca.public()?)?,
            PKey::private_key_from_pem(ca.secret()?)?.as_ref(),
            &privkey,
        )?;

        let (public, secret) = (cert.to_pem()?, privkey.private_key_to_pem()?);

        if conf.save() {
            let secret_keyfile = mk_key_filename(cache_key_path, name_with_rev, SECRET_SIG_KEY_SUFFIX);

            let p = {
                let public_pem = (cert.public_key()?.public_key_to_pem()?).clone();
                let pfx = (mk_pkcs12_pfx(name_with_rev, &cert, &privkey)?).clone();

                match conf.save_as_extn() {
                    PairSaverExtn::PubRSA => PairSavingData {
                        public_keyfile: mk_key_filename(cache_key_path, name_with_rev, PUBLIC_RSA_SUFFIX),
                        public: public_pem,
                        multi: Some(true),
                    },
                    PairSaverExtn::PemX509 => PairSavingData {
                        public_keyfile: mk_key_filename(cache_key_path, name_with_rev, PUBLIC_KEY_SUFFIX),
                        public: public.clone(),
                        multi: Some(true),
                    },
                    PairSaverExtn::PfxPKCS12 => PairSavingData {
                        public_keyfile: mk_key_filename(cache_key_path, name_with_rev, PUBLIC_PFX_SUFFIX),
                        public: pfx,
                        multi: None,
                    },
                    PairSaverExtn::DSA => PairSavingData {
                        public_keyfile: mk_key_filename(cache_key_path, name_with_rev, PUBLIC_DSA_SUFFIX),
                        public: public_pem,
                        multi: None,
                    },
                    _ => {
                        let msg = format!("File not saved for this extension {}", conf.save_as_extn());
                        return Err(Error::CryptoError(msg));
                    }
                }
            };

            debug!("public keyfile = {}", &p.public_keyfile.display());
            debug!("secret keyfile = {}", &secret_keyfile.display());

            if p.multi.is_some() {
                try!(write_keypair_files(
                    Some(&p.public_keyfile),
                    Some(&p.public[..]),
                    Some(&secret_keyfile),
                    Some(&secret[..]),
                ));
            } else {
                try!(write_key_file(Some(&p.public_keyfile), Some(&p.public[..])));
            }
        }

        Ok((public, secret))
    }

    pub fn get_pair_for<P: AsRef<Path> + ?Sized>(name_with_rev: &str, cache_key_path: &P) -> Result<Self> {
        let pk = match Self::get_public_key(name_with_rev, cache_key_path.as_ref()) {
            Ok(k) => Some(k),
            Err(e) => {
                // Not an error, just continue
                debug!(
                    "Can't find public key for name_with_rev {}: {}",
                    name_with_rev, e
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
                    name_with_rev, e
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
            return Err(Error::CryptoError(format!(
                "No public key found at {}",
                path.display()
            )));
        }
        Ok(path)
    }

    pub fn get_secret_key_path<P: AsRef<Path> + ?Sized>(key_with_rev: &str, cache_key_path: &P) -> Result<PathBuf> {
        let path = mk_key_filename(cache_key_path.as_ref(), key_with_rev, SECRET_SIG_KEY_SUFFIX);

        if !path.is_file() {
            return Err(Error::CryptoError(format!(
                "No secret key found at {}",
                path.display()
            )));
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
                    key_with_rev, e
                )))
            }
        }
    }

    // Returns the rsa public key which is public key + subject information
    pub fn get_rsa_public_key(key_with_rev: &str, cache_key_path: &Path) -> Result<Vec<u8>> {
        let public_keyfile = mk_key_filename(cache_key_path, key_with_rev, PUBLIC_RSA_SUFFIX);
        let bytes = try!(read_key_bytes(&public_keyfile));

        match PKey::public_key_from_pem(&bytes) {
            Ok(pk) => Ok(pk.public_key_to_pem().unwrap()),
            Err(e) => {
                return Err(Error::CryptoError(format!(
                    "Can't read rsa public key for {}\n: {}",
                    key_with_rev, e
                )))
            }
        }
    }

    fn get_secret_key(key_with_rev: &str, cache_key_path: &Path) -> Result<Vec<u8>> {
        let secret_keyfile = mk_key_filename(cache_key_path, key_with_rev, SECRET_SIG_KEY_SUFFIX);
        let bytes = try!(read_key_bytes(&secret_keyfile));

        match PKey::private_key_from_pem(&bytes) {
            Ok(sk) => Ok(sk.private_key_to_pem().unwrap()),
            Err(e) => {
                return Err(Error::CryptoError(format!(
                    "Can't read sig secret key for {}: {}",
                    key_with_rev, e
                )))
            }
        }
    }
}

/// Make a X509 request with the given private key
fn mk_request(privkey: &PKey) -> Result<X509Req> {
    let mut req_builder = X509ReqBuilder::new()?;
    req_builder.set_pubkey(&privkey)?;

    let mut x509_name = X509NameBuilder::new()?;
    x509_name.append_entry_by_text("C", RIOOS_COUNTRY)?;
    x509_name.append_entry_by_text("ST", RIOOS_STATE)?;
    x509_name.append_entry_by_text("O", RIOOS_ORGANIZATION)?;
    x509_name.append_entry_by_text("CN", RIOOS_WITH_SIGNED)?;
    let x509_name = x509_name.build();
    req_builder.set_subject_name(&x509_name)?;

    req_builder.sign(&privkey, MessageDigest::sha256())?;
    let req = req_builder.build();
    Ok(req)
}

/// Make a PKCS12 certificate using the pub cert + key pair
fn mk_pkcs12_pfx(name_with_rev: &str, cert: &X509, privkey: &PKey) -> Result<Vec<u8>> {
    let pkcs12_builder = Pkcs12::builder();

    let pkcs12 = pkcs12_builder
        .build(RIOOS_PFX_PASSWORD, &name_with_rev, &privkey, &cert)
        .unwrap();

    pkcs12.to_der().map_err(X509Error)
}

/// Generates a new PKey
fn gen_key_rsa(bit_len: u32) -> Result<PKey> {
    let rsa = Rsa::generate(bit_len)?;
    let key = PKey::from_rsa(rsa)?;
    Ok(key)
}

/// Generates a new PKey
fn gen_key_dsa(bit_len: u32) -> Result<PKey> {
    let rsa = Dsa::generate(bit_len)?;
    let key = PKey::from_dsa(rsa)?;
    Ok(key)
}

/// An helper to generate a selfsigned certificate authority
fn gen_ca(privkey: &PKey) -> Result<X509> {
    let mut x509_name = X509NameBuilder::new()?;
    x509_name.append_entry_by_text("C", RIOOS_COUNTRY)?;
    x509_name.append_entry_by_text("ST", RIOOS_STATE)?;
    x509_name.append_entry_by_text("O", RIOOS_ORGANIZATION)?;
    let x509_name = x509_name.build();

    let mut cert_builder = X509::builder()?;
    cert_builder.set_version(X509_VERSION3)?;
    let serial_number = {
        let mut serial = BigNum::new()?;
        serial.rand(159, MSB_MAYBE_ZERO, false)?;
        serial.to_asn1_integer()?
    };
    cert_builder.set_serial_number(&serial_number)?;
    cert_builder.set_subject_name(&x509_name)?;
    cert_builder.set_issuer_name(&x509_name)?;
    cert_builder.set_pubkey(&privkey)?;

    cert_builder.set_not_before(&Asn1Time::days_from_now(0).unwrap())?;
    cert_builder.set_not_after(&Asn1Time::days_from_now(*RIOOS_CERT_EXPIRES_IN_DAYS).unwrap())?;

    cert_builder.append_extension(BasicConstraints::new().critical().ca().build()?)?;

    cert_builder.append_extension(KeyUsage::new()
        .critical()
        .digital_signature()
        .key_encipherment()
        .key_cert_sign()
        .crl_sign()
        .build()?)?;

    let subject_key_identifier = SubjectKeyIdentifier::new().build(&cert_builder.x509v3_context(None, None))?;
    cert_builder.append_extension(subject_key_identifier)?;

    cert_builder.sign(&privkey, MessageDigest::sha256())?;
    Ok(cert_builder.build())
}

/// An helper to generate a certificate signing request
///
/// This function will generate a PKey
/// Needs the  ca_cert as X509
/// Returns X509 signed with root ca certificate
fn gen_signed(ca_cert: &X509, ca_privkey: &PKeyRef, privkey: &PKey) -> Result<X509> {
    let req = mk_request(&privkey)?;

    let mut cert_builder = X509::builder()?;
    cert_builder.set_version(X509_VERSION3)?;
    let serial_number = {
        let mut serial = BigNum::new()?;
        serial.rand(159, MSB_MAYBE_ZERO, false)?;
        serial.to_asn1_integer()?
    };
    cert_builder.set_serial_number(&serial_number)?;

    cert_builder.set_subject_name(req.subject_name())?;
    cert_builder.set_issuer_name(ca_cert.subject_name())?;
    cert_builder.set_pubkey(&privkey)?;

    cert_builder.set_not_before(&Asn1Time::days_from_now(0).unwrap())?;
    cert_builder.set_not_after(&Asn1Time::days_from_now(*RIOOS_CERT_EXPIRES_IN_DAYS).unwrap())?;

    cert_builder.append_extension(BasicConstraints::new().build()?)?;

    cert_builder.append_extension(KeyUsage::new()
        .critical()
        .non_repudiation()
        .digital_signature()
        .key_encipherment()
        .build()?)?;

    let subject_key_identifier = SubjectKeyIdentifier::new().build(&cert_builder.x509v3_context(Some(ca_cert), None))?;
    cert_builder.append_extension(subject_key_identifier)?;

    let auth_key_identifier = AuthorityKeyIdentifier::new()
        .keyid(false)
        .issuer(false)
        .build(&cert_builder.x509v3_context(Some(ca_cert), None))?;
    cert_builder.append_extension(auth_key_identifier)?;

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
            san.build(&cert_builder.x509v3_context(Some(ca_cert), None))?
        };
        cert_builder.append_extension(san_extension)?;
    }

    cert_builder.sign(ca_privkey, MessageDigest::sha256())?;

    Ok(cert_builder.build())
}

#[cfg(test)]
mod test {
    use std::fs;

    use tempdir::TempDir;

    use super::SigKeyPair;
    use crypto::keys::PairConf;
    use super::super::super::test_support::*;

    static VALID_KEY: &'static str = "ca.key";
    static VALID_PUB: &'static str = "ca.cert.pem";
    static VALID_NAME_WITH_REV: &'static str = "ca";

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
    fn generated_ca_pair() {
        let cache = TempDir::new("key_cache").unwrap();
        let pair = SigKeyPair::mk_ca_cert("unicorn", PairConf::new(), cache.path()).unwrap();

        assert_eq!(pair.name, "unicorn");
        match pair.public() {
            Ok(_) => assert!(true),
            Err(_) => panic!("Generated pair should have a public key"),
        }
        match pair.secret() {
            Ok(_) => assert!(true),
            Err(_) => panic!("Generated pair should have a secret key"),
        }
        // assert!(cache.path().join(format!("{}.crt", pair.name)).exists());
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
