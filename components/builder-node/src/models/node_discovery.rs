use cidr::Cidr;
use oping::Ping;
use super::cidrs::Cidrs;
use protocol::api::node::NodeFilter;
use error::Result;


pub struct NodeDiscovery {
    item: NodeFilter,
}

impl NodeDiscovery {
    pub fn new(item: NodeFilter) -> Self {
        NodeDiscovery { item: item }
    }
    pub fn ping_ips(&self) -> Result<Vec<String>> {
        let list_ip = match FilterType::conver_filter_type(self.item.cidrs, self.item.range_address_from) {
            FilterType::DEFAULT => vec!["".to_string()],
            FilterType::CIDRS => Cidrs::new(self.item.cidrs, self.item.ip_address_type).get_ip_list(),
            FilterType::RANGE => vec!["".to_string()],
            _ => vec!["".to_string()],
        };
        let mut ips = vec![];
        for d in list_ip {
            let mut ping = Ping::new();
            try!(ping.set_timeout(5.0));
            try!(ping.add_host(&d));
            let responses = try!(ping.send()); // waits for responses from all, or timeout
            for resp in responses {
                if resp.dropped < 0 {
                    ips.push(resp.hostname);
                }
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
    pub fn conver_filter_type(cidrs: Vec<String>, range: String) -> FilterType {
        if cidrs.len() > 0 {
            return FilterType::CIDRS;
        }

        if range != "" {
            return FilterType::RANGE;
        }

        FilterType::DEFAULT
    }
}
