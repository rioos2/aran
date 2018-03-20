// Copyright 2018 The Rio Advancement Inc

use std::net::{IpAddr, UdpSocket};

use error::Result;

pub use os::system::{uname, Uname};

static GOOGLE_DNS: &'static str = "8.8.8.8:53";

pub fn ip() -> Result<IpAddr> {
    let socket = try!(UdpSocket::bind("0.0.0.0:0"));
    let _ = try!(socket.connect(GOOGLE_DNS));
    let addr = try!(socket.local_addr());
    Ok(addr.ip())
}
