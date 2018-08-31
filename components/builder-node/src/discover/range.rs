use super::IpType;
use error::Result;
use ipnet::{IpAddrRange, Ipv4AddrRange, Ipv6AddrRange};
use std::net::IpAddr;

pub struct Range {
    from: String,
    to: String,
    ip_type: String,
}

impl Range {
    pub fn new(from: String, to: String, ip_type: String) -> Self {
        Range {
            from: from,
            to: to,
            ip_type: ip_type,
        }
    }
    pub fn get_ip_list(&self) -> Result<Vec<String>> {
        let ips = match IpType::find(&self.ip_type) {
            IpType::IPV4 => IpAddrRange::from(Ipv4AddrRange::new(self.from.parse()?, self.to.parse()?)).collect::<Vec<IpAddr>>(),
            IpType::IPV6 => IpAddrRange::from(Ipv6AddrRange::new(self.from.parse()?, self.to.parse()?)).collect::<Vec<IpAddr>>(),
        };
        let mut ip_list = vec![];
        for ip in ips {
            ip_list.push(format!("{}", ip));
        }
        Ok(ip_list)
    }
}
