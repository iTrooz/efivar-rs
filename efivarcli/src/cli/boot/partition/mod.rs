//! This module contains os-specific utility functions to get information about a partition

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "linux")]
pub use self::linux::{query_partition, get_mount_point, retrieve_efi_partition_data};

#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "windows")]
pub use self::windows::{query_partition, get_mount_point, retrieve_efi_partition_data};
