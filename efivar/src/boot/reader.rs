//! This module contains functions to read boot entries. Actual boot entry parsing is done in [`crate::boot::parse`]

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{boot::BootVarFormat, efi::Variable, Error, VarReader};

use super::boot_entry_iter::BootEntriesIterator;

pub trait BootVarReader {
    fn get_boot_order(&self) -> crate::Result<Vec<u16>>;
    fn get_boot_entries(&self) -> crate::Result<BootEntriesIterator>;
}

impl<T: VarReader> BootVarReader for T {
    fn get_boot_order(&self) -> crate::Result<Vec<u16>> {
        let (data, _) = self.read(&Variable::new("BootOrder"))?;

        assert!(data.len() % 2 == 0);

        let mut ids = vec![0u16; data.len() / 2];
        data.as_slice()
            .read_u16_into::<LittleEndian>(&mut ids)
            .map_err(Error::UnknownIoError)?;

        let ids_formatted: Vec<String> = ids.iter().map(|id| id.boot_id_format()).collect();
        log::debug!("Queried BootOrder: [{}]", ids_formatted.join(", "));
        Ok(ids)
    }

    fn get_boot_entries(&self) -> crate::Result<BootEntriesIterator> {
        BootEntriesIterator::new(self)
    }
}
