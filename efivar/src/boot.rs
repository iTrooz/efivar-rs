mod boot_entries_reader;
mod boot_entry_parser;
mod boot_order_reader;
mod boot_var_reader;
mod parse;

pub use boot_entry_parser::{BootEntry, BootEntryAttributes};
pub use boot_var_reader::BootVarReader;

pub trait BootVarName {
    fn boot_var_name(self) -> String;
}

impl BootVarName for u16 {
    fn boot_var_name(self) -> String {
        format!("Boot{:04X}", self)
    }
}