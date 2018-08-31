use super::{cidrs, range};
use error::Result;
use oping::Ping;
use protocol::api::node::{CidrItem, NodeFilter};

pub struct Nodes {
    item: NodeFilter,
}

impl Nodes {
    pub fn new(item: NodeFilter) -> Self {
        Nodes { item: item }
    }
    pub fn discovered(&self) -> Result<Vec<String>> {
        let list_ip = match DiscoverModes::convert(self.item.get_cidrs(), self.item.get_range_address_from()) {
            DiscoverModes::InMasterSubnet => vec!["".to_string()],
            DiscoverModes::InProvidedCidrs => {
                cidrs::Cidrs::new(self.item.get_cidrs(), self.item.get_ip_type())
                    .get_ip_list()?
            }
            DiscoverModes::InIpAddressRange => {
                range::Range::new(
                    self.item.get_range_address_from(),
                    self.item.get_range_address_to(),
                    self.item.get_ip_type(),
                ).get_ip_list()?
            }
        };
        let mut ips = vec![];
        let mut ping = Ping::new();
        for d in list_ip {
            try!(ping.add_host(&d));
        }
        let responses = try!(ping.send()); // waits for responses from all, or timeout
        for resp in responses {
            if !(resp.dropped > 0) {
                ips.push(resp.hostname);
            }
        }
        Ok(ips)
    }
}

enum DiscoverModes {
    InMasterSubnet,
    InProvidedCidrs,
    InIpAddressRange,
}

impl DiscoverModes {
    pub fn convert(cidrs: Vec<CidrItem>, range: String) -> DiscoverModes {
        if cidrs.len() > 0 {
            return DiscoverModes::InProvidedCidrs;
        }

        if range != "" {
            return DiscoverModes::InIpAddressRange;
        }

        DiscoverModes::InMasterSubnet
    }
}
