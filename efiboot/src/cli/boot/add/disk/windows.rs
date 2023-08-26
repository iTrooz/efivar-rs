use efivar::boot::EFIHardDrive;
use std::path::PathBuf;

pub fn retrieve_efi_partition_data(_name: &str) -> EFIHardDrive {
    todo!();
}

pub fn get_mount_point(_name: &str) -> Option<PathBuf> {
    todo!();
}
