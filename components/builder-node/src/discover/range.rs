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
    pub fn get_ip_list(&self) -> Vec<String> {
        let ips = match IpType::find(&self.ip_type) {
            IpType::IPV4 => IpAddrRange::from(Ipv4AddrRange::new(
                self.from.parse().unwrap(),
                self.to.parse().unwrap(),
            )).collect::<Vec<IpAddr>>(),
            IpType::IPV6 => IpAddrRange::from(Ipv6AddrRange::new(
                self.from.parse().unwrap(),
                self.to.parse().unwrap(),
            )).collect::<Vec<IpAddr>>(),
        };
        let mut x = vec![];
        for d in ips {
            x.push(format!("{}", d));
        }
        x
    }
}

enum IpType {
    IPV4,
    IPV6,
}

impl IpType {
    pub fn find(range: &str) -> IpType {
        match &range[..] {
            "IPV4" => IpType::IPV4,
            "IPV6" => IpType::IPV6,
            _ => IpType::IPV4,
        }
    }
}
