// Copyright 2018 The Rio Advancement Inc

use std::path::Path;
use regex::Regex;

use common::ui::UI;
use rio_core::crypto::SigKeyPair;
use rio_core::crypto::keys::PairConf;
use rio_core::Error::InvalidCertificateName;

use error::{Error, Result};

//creates the certificate authority or self signed.
pub fn start(ui: &mut UI, origin: &str, cache: &Path) -> Result<()> {
    match is_valid_ca_name(origin) {
        false => Err(Error::from(InvalidCertificateName(origin.to_string()))),
        true => {
            ui.begin(format!("Generating key for {}", &origin))?;
            let pair = SigKeyPair::mk_ca_cert(origin, PairConf::new(), cache)?;

            ui.end(format!("Generated key pair {}.", &pair.name))?;
            Ok(())
        }
    }
}

//creates the signed certificate by certificate authority.
pub fn signed(ui: &mut UI, origin: &str, cache: &Path) -> Result<()> {
    match is_valid_ca_name(origin) {
        false => Err(Error::from(InvalidCertificateName(origin.to_string()))),
        true => {
            ui.begin(format!("Generating key for {}", &origin))?;
            let pair = SigKeyPair::mk_signed(origin, PairConf::new(), cache)?;

            ui.end(format!("Generated key pair {}.", &pair.name))?;
            Ok(())
        }
    }
}

/// Is the string a valid ca name?
fn is_valid_ca_name(name: &str) -> bool {
    let origin_name_re: Regex = Regex::new(r"\A[a-z0-9][a-z0-9_-]*\z").expect("Unable to compile regex");
    name.chars().count() <= 255 && origin_name_re.is_match(name)
}
