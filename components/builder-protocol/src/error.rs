// Copyright (c) 2017 RioCorp Inc
//

use std::error;
use std::fmt;
use std::result;

pub enum Error {}


pub type Result<T> = result::Result<T, Error>;
