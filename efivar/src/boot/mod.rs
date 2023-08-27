//! This module handles everything related to boot entries

mod boot_entries_reader;
mod boot_entry_parser;
mod boot_order_reader;
mod boot_var_reader;
mod parse;
pub mod writer;

pub use boot_entry_parser::{BootEntry, BootEntryAttributes};
pub use boot_var_reader::BootVarReader;
pub use parse::{EFIHardDrive, EFIHardDriveType, FilePath, FilePathList};

pub trait BootVarName {
    fn boot_var_name(self) -> String;
}

impl BootVarName for u16 {
    /// Get the boot entry name associated with that ID.
    /// See [`crate::efi::Variable::boot_var_id`]
    fn boot_var_name(self) -> String {
        format!("Boot{:04X}", self)
    }
}
