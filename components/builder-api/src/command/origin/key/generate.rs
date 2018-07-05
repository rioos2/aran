// Copyright 2018 The Rio Advancement Inc

use regex::Regex;
use std::path::Path;

use common::ui::UI;
use rio_core::crypto::keys::{PairConf, PairSaverExtn};
use rio_core::crypto::SigKeyPair;
use rio_core::Error::InvalidCertificateName;

use error::{Error, Result};

//Creates a certificate authority
pub fn start(ui: &mut UI, ca: &str, cache: &Path) -> Result<()> {
    match is_valid_cert_name(ca) {
        false => Err(Error::from(InvalidCertificateName(ca.to_string()))),
        true => {
            ui.begin(format!("Generating key for {}", &ca))?;
            let pair =
                SigKeyPair::mk_ca_cert(ca, PairConf::with_extn(PairSaverExtn::PemX509), cache)?;
            ui.end(format!("Generated key pair {}.", &pair.name))?;
            Ok(())
        }
    }
}

//Create a signed certificates (x509, rsa format) using the  certificate authority
//server-ca.
//The PairConf dictates the type of extension being generated.
//The  default is PUB_RSA
pub fn signed_with_rsa(
    ui: &mut UI,
    name: &str,
    cache: &Path,
    pair: PairConf,
) -> Result<SigKeyPair> {
    match is_valid_cert_name(name) {
        false => Err(Error::from(InvalidCertificateName(name.to_string()))),
        true => {
            ui.begin(format!("Generating key for {}", &name))?;
            let pair = SigKeyPair::mk_signed(name, pair, cache)?;
            ui.end(format!("Generated key pair {}.", &pair.name))?;
            Ok(pair)
        }
    }
}

//Create a signed certificates pfx - pkcs12 format using the  certificate authority
//server-ca.
//The PairConf dictates the type of extension being generated.
//We can send the PairSaverExtn from outside,but this just being used
//for the api-server pair generation.
pub fn signed_with_pfx(ui: &mut UI, name: &str, cache: &Path) -> Result<SigKeyPair> {
    match is_valid_cert_name(name) {
        false => Err(Error::from(InvalidCertificateName(name.to_string()))),
        true => {
            ui.begin(format!("Generating key for {}", &name))?;
            let pair =
                SigKeyPair::mk_signed(name, PairConf::with_extn(PairSaverExtn::PfxPKCS12), cache)?;
            ui.end(format!("Generated key pair {}.", &pair.name))?;
            Ok(pair)
        }
    }
}

//Create a signed certificates X509 (cert.pem) format using the  certificate authority
//server-ca.
//The PairConf dictates the type of extension being generated.
pub fn signed_with_x509(ui: &mut UI, name: &str, cache: &Path) -> Result<SigKeyPair> {
    match is_valid_cert_name(name) {
        false => Err(Error::from(InvalidCertificateName(name.to_string()))),
        true => {
            ui.begin(format!("Generating key for {}", &name))?;
            let pair =
                SigKeyPair::mk_signed(name, PairConf::with_extn(PairSaverExtn::PemX509), cache)?;
            ui.end(format!("Generated key pair {}.", &pair.name))?;
            Ok(pair)
        }
    }
}

/// Retuns true if this is a valid certificatename
/// If the string is < 255 characters and contains alphabet/numeric
fn is_valid_cert_name(name: &str) -> bool {
    let certificate_name_re: Regex =
        Regex::new(r"\A[a-z0-9][a-z0-9_-]*\z").expect("Unable to compile regex");
    name.chars().count() <= 255 && certificate_name_re.is_match(name)
}
