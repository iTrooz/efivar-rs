//! This module handles everything related to boot entries

mod boot_entries_reader;
mod parse;
mod reader;
mod writer;

pub use parse::*;
pub use parse::{EFIHardDrive, EFIHardDriveType, FilePath, FilePathList};
pub use reader::BootVarReader;
pub use writer::BootVarWriter;

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
