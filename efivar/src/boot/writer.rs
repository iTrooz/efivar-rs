//! This module contains functions to write boot entries

use crate::{
    boot::BootVarName,
    efi::{Variable, VariableFlags},
    VarWriter,
};

use super::BootEntry;

pub trait BootVarWriter {
    fn add_boot_entry(&mut self, id: u16, entry: BootEntry) -> crate::Result<()>;
    fn set_boot_order(&mut self, ids: Vec<u16>) -> crate::Result<()>;
}

impl<T: VarWriter> BootVarWriter for T {
    fn set_boot_order(&mut self, ids: Vec<u16>) -> crate::Result<()> {
        let bytes: Vec<u8> = ids.into_iter().flat_map(|id| id.to_le_bytes()).collect();

        self.write(
            &Variable::new("BootOrder"),
            VariableFlags::NON_VOLATILE
                | VariableFlags::BOOTSERVICE_ACCESS
                | VariableFlags::RUNTIME_ACCESS,
            &bytes,
        )
        .unwrap();

        Ok(())
    }

    fn add_boot_entry(&mut self, id: u16, entry: BootEntry) -> crate::Result<()> {
        let bytes = entry.to_bytes();

        self.write(
            &Variable::new(&id.boot_var_name()),
            VariableFlags::NON_VOLATILE
                | VariableFlags::BOOTSERVICE_ACCESS
                | VariableFlags::RUNTIME_ACCESS,
            &bytes,
        )
        .unwrap();

        Ok(())
    }
}
