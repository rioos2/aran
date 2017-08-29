// Copyright (c) 2017 RioCorp Inc.
//

pub mod export;
pub mod generate;
pub mod import;

/*use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use error::{Error, Result};
use rio_core;

// shared between origin::key::upload and origin::key::upload_latest
fn get_name_with_rev(keyfile: &Path, expected_vsn: &str) -> Result<String> {
    let f = File::open(&keyfile)?;
    let f = BufReader::new(f);
    let mut lines = f.lines();
    let _ = match lines.next() {
        Some(val) => {
            let val = val?;
            if &val != expected_vsn {
                let msg = format!("Unsupported version: {}", &val);
                return Err(Error::RioosAranCore(rio_core::Error::CryptoError(msg)));
            }
            ()
        }
        None => {
            let msg = "Corrupt key file, can't read file version".to_string();
            return Err(Error::RioosAranCore(rio_core::Error::CryptoError(msg)));
        }
    };
    let name_with_rev = match lines.next() {
        Some(val) => val?,
        None => {
            let msg = "Corrupt key file, can't read name with rev".to_string();
            return Err(Error::RioosAranCore(rio_core::Error::CryptoError(msg)));
        }
    };
    Ok(name_with_rev)
}
*/
