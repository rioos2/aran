use std::net::{IpAddr, Ipv4Addr};
use cidr::{Cidr, IpCidr};

pub struct Cidrs {
    name: Vec<String>,
    ip_type: String,
}

impl Cidrs {
    pub fn new(name: Vec<String>, ip_type: String) -> Self {
        Cidrs {
            name: name,
            ip_type: ip_type,
        }
    }

    pub fn get_ip_list(&self) -> Vec<String> {
        let ip_type = self.ip_type;
        let data = self.name
            .iter()
            .map(|x| {
                let i = x.split("/").collect::<Vec<_>>();
                let j = i[0].split(".").collect::<Vec<_>>();
                match IpType::convert_str_to_iptype(&ip_type) {
                    IpType::IPV4 => IpCidr::new(IpAddr::V4(Ipv4Addr::new(j[0], j[1], j[2], j[3])), i[1]).unwrap(),
                    IpType::IPV6 => IpCidr::new(IpAddr::V4(Ipv4Addr::new(j[0], j[1], j[2], j[3])), i[1]).unwrap(),
                }
            })
            .collect::<Vec<_>>();
        let mut x = vec![];
        for d in data.iter().skip(1) {
            let i = format!("{}", d);
            x.push(i);
        }
        x
    }
}

enum IpType {
    IPV4,
    IPV6,
}

impl IpType {
    fn convert_str_to_iptype(value: &str) -> IpType {
        match &value[..] {
            "IPV4" => IpType::IPV4,
            "IPV6" => IpType::IPV6,
            _ => IpType::IPV4,
        }
    }
}
