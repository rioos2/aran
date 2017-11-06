// Copyright (c) 2017 RioCorp Inc.


pub use error::{Error, Result};

use common::ui::UI;
use rioos_core::env;

use api_client::{self, Client};
use AUTH_TOKEN_ENVVAR;
use {PRODUCT, VERSION};
use config;



pub fn start(ui: &mut UI,url: &str) -> Result<()> {
    ui.br()?;
    ui.title("Rio/OS CLI")?;
    
let config = config::load()?;
    ui.br()?;
    ui.para(&format!("Currently logged in as {} @ {}",&config.email.unwrap(),&config.api_server.unwrap()))?;
    Ok(())
}
