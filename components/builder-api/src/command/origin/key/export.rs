// Copyright (c) 2017 RioCorp Inc.
//

use std::io;
use std::fs::File;
use std::path::Path;

use rio_core::crypto::SigKeyPair;
use rio_core::crypto::keys::PairType;

use error::Result;

pub fn start(origin: &str, pair_type: PairType, cache: &Path) -> Result<()> {
    let latest = SigKeyPair::get_latest_pair_for(origin, cache, Some(&pair_type))?;
    let path = match pair_type {
        PairType::Public => SigKeyPair::get_public_key_path(&latest.name_with_rev(), cache)?,
        PairType::Secret => SigKeyPair::get_secret_key_path(&latest.name_with_rev(), cache)?,
    };
    let mut file = File::open(&path)?;
    debug!(
        "Streaming file contents of {} {} to standard out",
        &pair_type,
        &path.display()
    );
    io::copy(&mut file, &mut io::stdout())?;
    Ok(())
}
