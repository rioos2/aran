use cidr::{Cidr, IpCidr};
use protocol::api::node::CidrItem;
use std::net::IpAddr;

pub struct Cidrs {
    name: Vec<CidrItem>,
}

impl Cidrs {
    pub fn new(name: Vec<CidrItem>) -> Self {
        Cidrs { name: name }
    }
    pub fn get_ip_list(&self) -> Vec<String> {
        let data = self.name
            .iter()
            .map(|x| {
                let server: IpAddr = x.ip.parse().expect("Unable to parse socket address");
                IpCidr::new(server, x.range).unwrap()
            })
            .collect::<Vec<_>>();

        let mut x = vec![];
        for d in data[0].iter().skip(1) {
            x.push(format!("{}", d));
        }
        x
    }
}
