// Copyright 2018 The Rio Advancement Inc

use super::sys;
use num_cpus;
use os::system::dist::{PartialTargetTriple, TargetTriple};
use systemstat;
use systemstat::data::ByteSize;
use systemstat::{Platform, System};

// Probed stat
#[derive(Clone, Debug)]
pub struct ProbedBridge {
    // The bridge name
    name: String,
    // The ip address allocated to the bridge.
    ip: String,
}

impl ProbedBridge {
    fn new(name: String, ip: String) -> Self {
        ProbedBridge { name: name, ip: ip }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    fn get_ip(&self) -> String {
        self.ip.clone()
    }
}

// Probed stat
#[derive(Clone, Debug, Default)]
pub struct ProbedDisks {
    disks: Vec<ProbedDisk>,
}

impl ProbedDisks {
    fn new(disks: Vec<ProbedDisk>) -> Self {
        ProbedDisks { disks: disks }
    }

    //Total size of the disks
    pub fn get_total(&self) -> String {
        format!("{}", self.disks.iter().map(|d| d.total).fold(ByteSize::b(0), |sum, i| sum + i))
    }

    //The available storage
    fn get_avail(&self) -> String {
        format!("{}", self.disks.iter().map(|d| d.avail).fold(ByteSize::b(0), |sum, i| sum + i))
    }
}

// Probed stat
#[derive(Clone, Debug)]
pub struct ProbedDisk {
    avail: ByteSize,
    total: ByteSize,
    fs_type: String,
    fs_mounted_from: String,
    fs_mounted_on: String,
}

impl ProbedDisk {
    fn new(fs_mounted_from: String, fs_type: String, fs_mounted_on: String, avail: ByteSize, total: ByteSize) -> Self {
        ProbedDisk {
            avail: avail,
            total: total,
            fs_type: fs_type,
            fs_mounted_from: fs_mounted_from,
            fs_mounted_on: fs_mounted_on,
        }
    }

    fn get_total(&self) -> ByteSize {
        self.total
    }

    fn get_avail(&self) -> ByteSize {
        self.avail
    }
}

// Probed stat
#[derive(Debug, Default)]
pub struct ProbedStat {
    //architecture. This must be merged with uname
    arch: String,
    //
    uptime: String,
    //
    boot_time: String,
    //  The cpu cores of the system
    cpu: String,
    //  The memory capacity of the system in KiB/MiB, format
    memory: String,
    //  The disk capacity of the system in KiB/MiB, format
    storage: ProbedDisks,
    //
    bridges: Vec<ProbedBridge>,
}

impl ProbedStat {
    fn new() -> Self {
        ::std::default::Default::default()
    }

    pub fn set_arch(&mut self, v: ::std::string::String) {
        self.arch = v;
    }

    pub fn get_arch(&self) -> ::std::string::String {
        self.arch.clone()
    }

    pub fn set_uptime(&mut self, v: ::std::string::String) {
        self.uptime = v;
    }

    pub fn get_uptime(&self) -> ::std::string::String {
        self.uptime.clone()
    }

    pub fn set_boot_time(&mut self, v: ::std::string::String) {
        self.boot_time = v;
    }

    pub fn get_boot_time(&self) -> ::std::string::String {
        self.boot_time.clone()
    }

    pub fn set_cpu(&mut self, v: ::std::string::String) {
        self.cpu = v;
    }

    pub fn get_cpu(&self) -> ::std::string::String {
        self.cpu.clone()
    }

    pub fn set_memory(&mut self, v: ::std::string::String) {
        self.memory = v;
    }

    pub fn get_memory(&self) -> ::std::string::String {
        self.memory.clone()
    }

    pub fn set_storage(&mut self, v: ProbedDisks) {
        self.storage = v;
    }

    pub fn get_storage(&self) -> ProbedDisks {
        self.storage.clone()
    }

    pub fn set_bridges(&mut self, v: Vec<ProbedBridge>) {
        self.bridges = v;
    }

    pub fn get_bridges(&self) -> Vec<ProbedBridge> {
        self.bridges.clone()
    }

    //Just randomly pick the next ProbedBridge and return the first one.
    //If nothing exists then send back as 0.0.0.0
    pub fn get_host_address(&self) -> ::std::string::String {
        self.get_bridges()
            .clone()
            .iter()
            .next()
            .unwrap_or(&ProbedBridge::new("ong".to_string(), "0.0.0.0".to_string()))
            .ip
            .clone()
    }
}

pub struct Probe {
    // System stat prober  (Grabs from /proc files in linux)
    sys: System,
}

/// System stat
impl Probe {
    pub fn new() -> Self {
        Probe { sys: System::new() }
    }

    pub fn probe(&self) -> ProbedStat {
        let mut pr = ProbedStat::new();
        pr.set_arch(
            PartialTargetTriple::from_str(&TargetTriple::from_host_or_build().to_string())
                .map_or("x86_64".to_string(), |p| p.arch.unwrap_or("x86_64".to_string())),
        );
        pr.set_uptime(self.uptime());
        pr.set_boot_time(self.boot_time());
        pr.set_cpu(self.cpu());
        pr.set_memory(self.memory());
        pr.set_storage(ProbedDisks::new(self.disks()));
        pr.set_bridges(self.bridges());

        pr
    }

    fn uptime(&self) -> String {
        match self.sys.uptime() {
            Ok(uptime) => format!("{:?}", uptime),
            Err(_) => "".to_string(),
        }
    }

    fn boot_time(&self) -> String {
        match self.sys.boot_time() {
            Ok(boot_time) => format!("{:?}", boot_time),
            Err(_) => "".to_string(),
        }
    }

    fn cpu(&self) -> String {
        num_cpus::get().to_string()
    }

    fn memory(&self) -> String {
        match self.sys.memory() {
            Ok(mem) => format!("{}", mem.total),
            Err(_) => "".to_string(),
        }
    }

    fn disks(&self) -> Vec<ProbedDisk> {
        match self.sys.mounts() {
            Ok(mounts) => mounts
                .iter()
                .map(|m| {
                    let o = m.clone();
                    ProbedDisk::new(o.fs_mounted_from, o.fs_type, o.fs_mounted_on, o.avail, o.total)
                })
                .collect(),
            Err(_) => vec![],
        }
    }

    fn bridges(&self) -> Vec<ProbedBridge> {
        match self.sys.networks() {
            Ok(netifs) => {
                let mut r = vec![];
                netifs.values().into_iter().map(|netif| {
                    netif
                        .clone()
                        .addrs
                        .into_iter()
                        .filter(|n| match n.addr {
                            /*TO-DO: The correct code is as below, but the following methods are marked unstable
                            IPV4: is_global() is marked unstable 
                            IPV6: is_unicast_link_local() is marked unstable 
                            IPV6: is_global() is marked unstable 
                            IPV6: is_private  is missing (no method available)
                            V4(ip) => return (ip.is_private() || ip.is_global()) && !(ip.is_loopback() || !ip.is_link_local()),
                            V6(ip) => return (ip.is_global()) && !(ip.is_loopback() || !ip.is_unicast_link_local()),
                            */
                            systemstat::IpAddr::V4(ip) => return (ip.is_private()) && !(ip.is_loopback() || !ip.is_link_local()),
                            systemstat::IpAddr::V6(ip) => return !(ip.is_loopback()),
                            systemstat::IpAddr::Empty | systemstat::IpAddr::Unsupported => return false,
                        })
                        .map(|n1| {
                            let ip: Option<String> = match n1.addr {
                                systemstat::IpAddr::V4(ipv4) => Some(ipv4.to_string()),
                                systemstat::IpAddr::V6(ipv6) => Some(ipv6.to_string()),
                                systemstat::IpAddr::Empty | systemstat::IpAddr::Unsupported => None,
                            };

                            r.push(ProbedBridge::new(netif.clone().name, ip.clone().unwrap_or("0.0.0.0".to_string())));

                            ip //This just temporary return, as we don't care what happens to netif.
                        })
                        .collect::<Option<String>>();
                });
                r
            }
            Err(_) => vec![],
        }
    }
}
