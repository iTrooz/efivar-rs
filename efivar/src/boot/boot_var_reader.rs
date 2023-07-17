use crate::VarReader;

use super::{boot_entries_reader::BootEntriesIterator, boot_order_reader::BootOrderIterator};

pub trait BootVarReader {
    fn get_boot_order(&self) -> crate::Result<BootOrderIterator>;
    fn get_boot_entries(&self) -> crate::Result<BootEntriesIterator>;
}

impl<T: VarReader> BootVarReader for T {
    fn get_boot_order(&self) -> crate::Result<BootOrderIterator> {
        BootOrderIterator::new(self)
    }

    fn get_boot_entries(&self) -> crate::Result<BootEntriesIterator> {
        BootEntriesIterator::new(self)
    }
}
