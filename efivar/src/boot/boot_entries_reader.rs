use crate::VarReader;

use super::{boot_entry_parser::BootEntry, boot_order_reader::BootOrderIterator};

pub struct BootEntriesIterator<'a> {
    order_iter: BootOrderIterator,
    var_reader: &'a dyn VarReader,
}

/// Loop over boot order IDs. The corresponding entries are not queried
impl<'a> BootEntriesIterator<'a> {
    pub(in super::super) fn new(
        var_reader: &'a dyn VarReader,
    ) -> crate::Result<BootEntriesIterator<'a>> {
        Ok(BootEntriesIterator {
            order_iter: BootOrderIterator::new(var_reader)?,
            var_reader,
        })
    }
}

impl<'a> Iterator for BootEntriesIterator<'a> {
    type Item = BootEntry;

    fn next(&mut self) -> Option<Self::Item> {
        self.order_iter
            .next()
            .map(|var| BootEntry::parse(self.var_reader, &var))
    }
}
