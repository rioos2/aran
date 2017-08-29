// Copyright (c) 2017 RioCorp Inc.
//

use std::path::Path;

use common::ui::UI;
use rio_core::crypto::SigKeyPair;

use error::Result;

pub fn start(ui: &mut UI, content: &str, cache: &Path) -> Result<()> {
    ui.begin("Importing origin key from standard input")?;
    let (pair, pair_type) = SigKeyPair::write_file_from_str(content, cache)?;
    ui.end(format!(
        "Imported {} origin key {}.",
        &pair_type,
        &pair.name_with_rev()
    ))?;
    Ok(())
}
