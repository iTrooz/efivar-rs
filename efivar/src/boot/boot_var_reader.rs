use byteorder::{LittleEndian, ReadBytesExt};

use crate::{efi::Variable, VarReader};

use super::boot_entries_reader::BootEntriesIterator;

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
            .unwrap();
        Ok(ids)
    }

    fn get_boot_entries(&self) -> crate::Result<BootEntriesIterator> {
        BootEntriesIterator::new(self)
    }
}
