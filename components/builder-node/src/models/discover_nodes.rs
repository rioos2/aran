
use oping::Ping;
use super::{cidrs, range};
use protocol::api::node::{NodeFilter, CidrItem};
use error::Result;

pub struct DiscoverNodes {
    item: NodeFilter,
}

impl DiscoverNodes {
    pub fn new(item: NodeFilter) -> Self {
        DiscoverNodes { item: item }
    }
    pub fn discovered(&self) -> Result<Vec<String>> {
        let list_ip = match DiscoverModes::convert(self.item.get_cidrs(), self.item.get_range_address_from()) {
            DiscoverModes::IN_MASTER_SUBNET => vec!["".to_string()],
            DiscoverModes::IN_PROVIDED_CIDRS => cidrs::Cidrs::new(self.item.get_cidrs()).get_ip_list(),
            DiscoverModes::IN_IP_ADDRESS_RANGE => {
                range::Range::new(
                    self.item.get_range_address_from(),
                    self.item.get_range_address_to(),
                    self.item.get_ip_type(),
                ).get_ip_list()
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
    IN_MASTER_SUBNET,
    IN_PROVIDED_CIDRS,
    IN_IP_ADDRESS_RANGE,
}

impl DiscoverModes {
    pub fn convert(cidrs: Vec<CidrItem>, range: String) -> DiscoverModes {
        if cidrs.len() > 0 {
            return DiscoverModes::IN_PROVIDED_CIDRS;
        }

        if range != "" {
            return DiscoverModes::IN_IP_ADDRESS_RANGE;
        }

        DiscoverModes::IN_MASTER_SUBNET
    }
}
