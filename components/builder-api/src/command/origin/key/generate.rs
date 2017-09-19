// Copyright (c) 2017 RioCorp Inc.


use std::path::Path;

use common::ui::UI;
use rio_core::crypto::SigKeyPair;
use rio_core::package::ident;
use rio_core::Error::InvalidOrigin;

use error::{Error, Result};

//creates the certificate authority or self signed.
pub fn start(ui: &mut UI, origin: &str, cache: &Path) -> Result<()> {
    match ident::is_valid_ca_name(origin) {
        false => Err(Error::from(InvalidOrigin(origin.to_string()))),
        true => {
            ui.begin(format!("Generating key for {}", &origin))?;
            let pair = SigKeyPair::root_ca(origin, cache)?;

            ui.end(format!(
                "Generated key pair {}.",
                &pair.name
            ))?;
            Ok(())
        }
    }
}

//creates the signed certificate by certificate authority.
pub fn signed(ui: &mut UI, origin: &str, cache: &Path) -> Result<()> {
    match ident::is_valid_ca_name(origin) {
        false => Err(Error::from(InvalidOrigin(origin.to_string()))),
        true => {
            ui.begin(format!("Generating key for {}", &origin))?;
            let pair = SigKeyPair::signed_with(origin, cache)?;

            ui.end(format!(
                "Generated key pair {}.",
                &pair.name
            ))?;
            Ok(())
        }
    }
}
