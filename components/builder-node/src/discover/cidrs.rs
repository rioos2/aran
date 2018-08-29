use super::IpType;
use error::Result;
use ipnet::{Ipv4Net, Ipv6Net};
use protocol::api::node::CidrItem;
use std::net::{Ipv4Addr, Ipv6Addr};

pub struct Cidrs {
    name: Vec<CidrItem>,
    ip_type: String,
}

impl Cidrs {
    pub fn new(name: Vec<CidrItem>, ip_type: String) -> Self {
        Cidrs {
            name: name,
            ip_type: ip_type,
        }
    }
    pub fn get_ip_list(&self) -> Result<Vec<String>> {
        let mut ip_list = vec![];
        match IpType::find(&self.ip_type) {
            IpType::IPV4 => {
                let ipv4_net: Ipv4Net = self.name
                    .iter()
                    .map(|x| format!("{}/{}", x.ip, x.range))
                    .collect::<String>()
                    .parse()?;
                ipv4_net
                    .hosts()
                    .collect::<Vec<Ipv4Addr>>()
                    .iter()
                    .map(|ip| { ip_list.push(format!("{}", ip)); })
                    .collect::<Vec<_>>();
            }
            IpType::IPV6 => {
                let ipv6_net: Ipv6Net = self.name
                    .iter()
                    .map(|x| format!("{}/{}", x.ip, x.range))
                    .collect::<String>()
                    .parse()?;
                ipv6_net
                    .hosts()
                    .collect::<Vec<Ipv6Addr>>()
                    .iter()
                    .map(|ip| { ip_list.push(format!("{}", ip)); })
                    .collect::<Vec<_>>();
            }
        };
        Ok(ip_list)
    }
}
