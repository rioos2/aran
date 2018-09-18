// Copyright 2018 The Rio Advancement Inc
//
// The import super::sys is needed. But rust complains, it isn't needed. 
// We need to remove super::sys and test it. If all is good.
// remove the allow unused_imports
#[allow(unused_imports)]
use super::sys;

use get_if_addrs;
use humansize::{file_size_opts, FileSize};
use num_cpus;
use os::system::dist::{PartialTargetTriple, TargetTriple};
use sys_info; //Import the trait and the options module

const TO_BYTES: u64 = 1024;

// System memory information.
#[derive(Clone, Default, Debug)]
pub struct MemInfo {
    /// Total physical memory.
    total: u64,
    free: u64,
    avail: u64,
}

/// Disk information.
#[derive(Clone, Default, Debug)]
pub struct ProbedDisk {
    total: u64,
    avail: u64,
}

// Probed stat
#[derive(Clone, Default, Debug)]
pub struct ProbedBridge {
    // The bridge name
    name: String,
    // The ip address allocated to the bridge.
    ip: String,
}

impl MemInfo {
    // This must be a trait named HumanReadableSize that every persom who wishes a
    // Human readable format shall use.
    fn opts_gi_b(&self) -> file_size_opts::FileSizeOpts {
        file_size_opts::FileSizeOpts {
            fixed_at: file_size_opts::FixedAt::Giga,
            decimal_places: 1,
            ..file_size_opts::BINARY
        }
    }

    pub fn get_total(&self) -> String {
        match (self.total * TO_BYTES).file_size(self.opts_gi_b()) {
            Ok(res) => res,
            Err(_err) => "".to_string(),
        }
    }

    fn _get_avail(&self) -> String {
        match (self.avail * TO_BYTES).file_size(self.opts_gi_b()) {
            Ok(res) => res,
            Err(_err) => "".to_string(),
        }
    }
}

impl ProbedDisk {
    //
    fn _new(avail: u64, total: u64) -> Self {
        ProbedDisk { avail: avail, total: total }
    }
    //
    fn opts_gi_b(&self) -> file_size_opts::FileSizeOpts {
        file_size_opts::FileSizeOpts {
            fixed_at: file_size_opts::FixedAt::Giga,
            decimal_places: 1,
            ..file_size_opts::BINARY
        }
    }

    pub fn get_total(&self) -> String {
        match (self.total * TO_BYTES).file_size(self.opts_gi_b()) {
            Ok(res) => res,
            Err(_err) => "".to_string(),
        }
    }

    fn _get_avail(&self) -> String {
        match (self.avail * TO_BYTES).file_size(self.opts_gi_b()) {
            Ok(res) => res,
            Err(_err) => "".to_string(),
        }
    }
}

impl ProbedBridge {
    fn new(name: String, ip: String) -> Self {
        ProbedBridge { name: name, ip: ip }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    fn _get_ip(&self) -> String {
        self.ip.clone()
    }
}

// Probed stat
#[derive(Debug, Default)]
pub struct ProbedStat {
    //architecture. This must be merged with uname
    arch: String,
    //  The cpu cores of the system
    cpu: String,
    //  The memory capacity of the system in KiB/MiB, format
    memory: MemInfo,
    //  The disk capacity of the system in KiB/MiB, format
    storage: ProbedDisk,
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

    pub fn set_cpu(&mut self, v: ::std::string::String) {
        self.cpu = v;
    }

    pub fn get_cpu(&self) -> ::std::string::String {
        self.cpu.clone()
    }

    pub fn set_memory(&mut self, v: MemInfo) {
        self.memory = v;
    }

    pub fn get_memory(&self) -> MemInfo {
        self.memory.clone()
    }

    pub fn set_storage(&mut self, v: ProbedDisk) {
        self.storage = v;
    }

    pub fn get_storage(&self) -> ProbedDisk {
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

pub struct Probe {}

/// System stat
impl Probe {
    pub fn new() -> Self {
        Probe {}
    }

    pub fn probe(&self) -> ProbedStat {
        let mut pr = ProbedStat::new();
        pr.set_arch(
            PartialTargetTriple::from_str(&TargetTriple::from_host_or_build().to_string())
                .map_or("x86_64".to_string(), |p| p.arch.unwrap_or("x86_64".to_string())),
        );
        pr.set_cpu(self.cpu());
        pr.set_memory(self.memory());
        pr.set_storage(self.disk());
        pr.set_bridges(self.bridges());

        pr
    }

    fn cpu(&self) -> String {
        num_cpus::get().to_string()
    }

    fn memory(&self) -> MemInfo {
        match sys_info::mem_info() {
            Ok(mem) => MemInfo {
                avail: mem.avail,
                total: mem.total,
                free: mem.free,
            },
            Err(_) => MemInfo { total: 0, avail: 0, free: 0 },
        }
    }

    fn disk(&self) -> ProbedDisk {
        match sys_info::disk_info() {
            Ok(disk) => ProbedDisk {
                total: disk.total,
                avail: disk.free,
            },
            Err(_) => ProbedDisk { total: 0, avail: 0 },
        }
    }

    fn bridges(&self) -> Vec<ProbedBridge> {
        match get_if_addrs::get_if_addrs() {
            Ok(netifs) => netifs
                .into_iter()
                .filter(|n| !n.is_loopback())
                .map(|netif| ProbedBridge::new(netif.clone().name, netif.clone().ip().to_string().clone()))
                .collect(),
            Err(_) => vec![],
        }
    }
}
