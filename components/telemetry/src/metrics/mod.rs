// Copyright 2018 The Rio Advancement Inc

//! A module containing the metrics http clients
pub mod collector;
pub mod expression;
pub mod prometheus;
pub mod executer;
pub mod query;
pub mod hooks;

pub const NODE_MEMORY_TOTAL: &'static str = "node_memory_MemTotal";
pub const NODE_MEMORY_FREE: &'static str = "node_memory_MemFree";
pub const NODE_MEMORY_BUFFER: &'static str = "node_memory_Buffers";
pub const NODE_CPU: &'static str = "node_cpu";
pub const INSTANCE: &'static str = "instance";

pub const NODE_FILE_SYSTEM_SIZE: &'static str = "node_filesystem_size";
pub const NODE_FILE_SYSTEM_FREE: &'static str = "node_filesystem_free";

pub const NODE_NETWORK_TRANSMIT_BYTES_TOTAL: &'static str = "node_network_transmit_bytes_total";
pub const NODE_NETWORK_RECEIVE_BYTES_TOTAL: &'static str = "node_network_receive_bytes_total";
pub const NODE_NETWORK_RECEIVE_ERRS_TOTAL: &'static str = "node_network_receive_errs_total";
pub const NODE_NETWORK_TRANSMIT_ERRS_TOTAL: &'static str = "node_network_transmit_errs_total";

pub const NODE_PROCESS_CPU: &'static str = "node_process_cpu";
pub const NODE_PROCESS_MEM: &'static str = "node_process_mem";

pub const NODE_DISK_MEGA_BYTES_READ: &'static str = "node_disk_mega_bytes_read";
pub const NODE_DISK_MEGA_BYTES_WRITTEN: &'static str = "node_disk_mega_bytes_written";
pub const NODE_DISK_IO_NOW: &'static str = "node_disk_io_now";
pub const NODE_DISK_MEGA_BYTES_IO_TOTAL: &'static str = "node_disk_mega_bytes_io_total";


pub const CONTAINER_CPU_USAGE_SEC_TOTAL: &'static str = "container_cpu_usage_seconds_total";
pub const CONTAINER_MEM_USAGE_BYTES: &'static str = "container_memory_usage_bytes";
pub const CONTAINER_SPEC_MEM_LIMIT_BYTES: &'static str = "container_spec_memory_limit_bytes";


pub const CONTAINER_FS_USAGE_BYTES: &'static str = "container_fs_usage_bytes";
pub const CONTAINER_FS_LIMIT_BYTES: &'static str = "container_fs_limit_bytes";

pub const CPU_CONSUMPTION: &'static str = "cpu_consumption";
pub const MEMORY_CONSUMPTION: &'static str = "memory_consumption";
pub const STORAGE_CONSUMPTION: &'static str = "storage_consumption";

pub const SENSEIS: &'static str = "senseis";
pub const NINJAS: &'static str = "ninjas";
