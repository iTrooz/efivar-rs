use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{efi::VariableName, VarReader};

pub struct BootOrderIterator {
    cursor: Cursor<Vec<u8>>,
}

/// Loop over boot order IDs. The corresponding entries are not queried
impl BootOrderIterator {
    fn new(sm: &impl VarReader) -> crate::Result<BootOrderIterator> {
        // Buffer for BootOrder
        let mut buf = vec![0u8; 512];

        // Read BootOrder
        let (boot_order_size, _flags) = sm.read(&VariableName::new("BootOrder"), &mut buf[..])?;

        // Resize to actual value size
        buf.resize(boot_order_size, 0);

        Ok(BootOrderIterator {
            cursor: Cursor::new(buf),
        })
    }
}

impl Iterator for BootOrderIterator {
    type Item = VariableName;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor
            .read_u16::<LittleEndian>()
            .map(|id| VariableName::new(&format!("Boot{:04X}", id)))
            .ok()
    }
}

pub trait BootVarReader {
    fn get_boot_order(&self) -> crate::Result<BootOrderIterator>;
}

impl<T: VarReader> BootVarReader for T {
    fn get_boot_order(&self) -> crate::Result<BootOrderIterator> {
        BootOrderIterator::new(self)
    }
}
