//! This module contains os-specific utility functions to get information about a partition

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use self::linux::{get_mount_point, query_partition, retrieve_efi_partition_data};

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use self::windows::{get_mount_point, query_partition, retrieve_efi_partition_data};
