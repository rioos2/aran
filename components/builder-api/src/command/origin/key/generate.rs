// Copyright (c) 2017 RioCorp Inc.


use std::path::Path;

use common::ui::UI;
use rio_core::crypto::SigKeyPair;
use rio_core::package::ident;
use rio_core::Error::InvalidOrigin;

use error::{Error, Result};

pub fn start(ui: &mut UI, origin: &str, cache: &Path) -> Result<()> {
    match ident::is_valid_origin_name(origin) {
        false => Err(Error::from(InvalidOrigin(origin.to_string()))),
        true => {
            ui.begin(format!("Generating origin key for {}", &origin))?;
            let pair = SigKeyPair::generate_pair_for_origin(origin, cache)?;

            ui.end(format!(
                "Generated origin key pair {}.",
                &pair.name
            ))?;
            Ok(())
        }
    }
}
