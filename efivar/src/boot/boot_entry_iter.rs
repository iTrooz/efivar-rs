//! This module contains the custom iterator used to loop lazily over boot entries

use crate::{efi::Variable, VarReader};

use super::{BootEntry, BootVarName, BootVarReader};

/// Loop over boot entries. On each iteration, a variable data will be queried from the OS
pub struct BootEntriesIterator<'a> {
    ids: Vec<u16>,
    var_reader: &'a dyn VarReader,
}

impl<'a> BootEntriesIterator<'a> {
    pub(in super::super) fn new(
        var_reader: &'a impl VarReader,
    ) -> crate::Result<BootEntriesIterator<'a>> {
        Ok(BootEntriesIterator {
            ids: var_reader.get_boot_order()?,
            var_reader,
        })
    }
}

impl<'a> Iterator for BootEntriesIterator<'a> {
    type Item = (Result<BootEntry, crate::Error>, Variable);

    fn next(&mut self) -> Option<Self::Item> {
        self.ids
            .pop()
            .map(|id| Variable::new(&id.boot_var_name()))
            .map(|var| (BootEntry::read(self.var_reader, &var), var))
    }
}
