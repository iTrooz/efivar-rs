use efivar::boot::EFIHardDrive;
use std::path::PathBuf;

pub type Partition = String;

pub fn query_partition(_disk: Option<String>, _partition: String) -> anyhow::Result<Partition> {
    todo!();
}

pub fn retrieve_efi_partition_data(_name: &Partition) -> EFIHardDrive {
    todo!();
}

pub fn get_mount_point(_name: &Partition) -> Option<PathBuf> {
    None
}
