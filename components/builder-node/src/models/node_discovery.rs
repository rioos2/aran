use oping::Ping;
use super::{cidrs, range};
use protocol::api::node::{NodeFilter, CidrItem};
use error::Result;


pub struct NodeDiscovery {
    item: NodeFilter,
}

impl NodeDiscovery {
    pub fn new(item: NodeFilter) -> Self {
        NodeDiscovery { item: item }
    }
    pub fn ping_ips(&self) -> Result<Vec<String>> {
        let list_ip = match FilterType::conver_filter_type(self.item.get_cidrs(), self.item.get_range_address_from()) {
            FilterType::DEFAULT => vec!["".to_string()],
            FilterType::CIDRS => cidrs::Cidrs::new(self.item.get_cidrs()).get_ip_list(),
            FilterType::RANGE => {
                range::Range::new(
                    self.item.get_range_address_from(),
                    self.item.get_range_address_to(),
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

enum FilterType {
    DEFAULT,
    CIDRS,
    RANGE,
}

impl FilterType {
    pub fn conver_filter_type(cidrs: Vec<CidrItem>, range: String) -> FilterType {
        if cidrs.len() > 0 {
            return FilterType::CIDRS;
        }

        if range != "" {
            return FilterType::RANGE;
        }

        FilterType::DEFAULT
    }
}
