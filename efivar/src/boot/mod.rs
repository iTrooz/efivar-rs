//! This module handles everything related to boot entries

mod boot_entry_iter;
pub(crate) mod parse;
mod reader;
mod writer;

pub use parse::*;
pub use parse::{EFIHardDrive, EFIHardDriveType, FilePath, FilePathList};
pub use reader::BootVarReader;
pub use writer::BootVarWriter;

pub trait BootVarFormat {
    fn boot_id_format(self) -> String;
    fn boot_var_format(self) -> String;
}

impl BootVarFormat for u16 {
    fn boot_id_format(self) -> String {
        format!("{self:04X}")
    }

    /// Get the boot entry name associated with that ID.
    /// See [`crate::efi::Variable::boot_var_id`]
    fn boot_var_format(self) -> String {
        format!("Boot{}", self.boot_id_format())
    }
}
