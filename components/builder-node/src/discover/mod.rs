pub mod cidrs;
pub mod range;
pub mod search;

pub enum IpType {
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
