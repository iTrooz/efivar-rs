use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{efi::Variable, VarReader};

use super::BootVarName;

/// Loop over boot order IDs. The corresponding entries are not queried
pub struct BootOrderIterator {
    cursor: Cursor<Vec<u8>>,
}

impl BootOrderIterator {
    pub(in super::super) fn new(sm: &dyn VarReader) -> crate::Result<BootOrderIterator> {
        // Read BootOrder variable
        let (value, _flags) = sm.read(&Variable::new("BootOrder"))?;

        Ok(BootOrderIterator {
            cursor: Cursor::new(value),
        })
    }
}

impl Iterator for BootOrderIterator {
    type Item = Variable;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor
            .read_u16::<LittleEndian>()
            .map(|id| Variable::new(&id.boot_var_name()))
            .ok()
    }
}
