use super::IpType;
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
    pub fn get_ip_list(&self) -> Vec<String> {
        let mut ip_list = vec![];
        match IpType::find(&self.ip_type) {
            IpType::IPV4 => {
                self.name
                    .iter()
                    .map(|x| {
                        let net: Ipv4Net = format!("{}/{}", x.ip, x.range).parse().expect(
                            "Unable to parse
                             socket address",
                        );
                        net.hosts()
                            .collect::<Vec<Ipv4Addr>>()
                            .iter()
                            .map(|ip| { ip_list.push(format!("{}", ip)); })
                            .collect::<Vec<_>>();
                    })
                    .collect::<Vec<_>>();
            }
            IpType::IPV6 => {
                self.name
                    .iter()
                    .map(|x| {
                        let net: Ipv6Net = format!("{}/{}", x.ip, x.range).parse().expect(
                            "Unable to parse
                             socket address",
                        );
                        net.hosts()
                            .collect::<Vec<Ipv6Addr>>()
                            .iter()
                            .map(|ip| { ip_list.push(format!("{}", ip)); })
                            .collect::<Vec<_>>();
                    })
                    .collect::<Vec<_>>();
            }
        };
        ip_list
    }
}
