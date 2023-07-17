use std::io::Cursor;

use crate::{efi::VariableName, VarReader};

pub struct BootEntry {}

impl BootEntry {
    pub fn parse(manager: &(impl ?Sized + VarReader), variable: &VariableName) -> Self {
        let mut buf = vec![0u8; 512];

        let (written_size, _flags) = manager.read(variable, &mut buf).unwrap();

        buf.resize(written_size, 0);

        let cursor = Cursor::new(buf);

        todo!();
    }
}
