use efivar::boot::EFIHardDrive;
use std::path::PathBuf;

pub type Partition = String;

pub fn query_partition(disk: Option<String>, partition: String) -> anyhow::Result<Partition> {
    todo!();
}

pub fn retrieve_efi_partition_data(_name: &str) -> EFIHardDrive {
    todo!();
}

pub fn get_mount_point(_name: &str) -> Option<PathBuf> {
    todo!();
}
