use crate::{
    boot::BootVarName,
    efi::{VariableFlags, VariableName},
    VarWriter,
};

use super::BootEntry;

pub trait BootVarWriter {
    fn add_boot_entry(&mut self, id: u16, entry: BootEntry) -> crate::Result<()>;
}

impl<T: VarWriter> BootVarWriter for T {
    fn add_boot_entry(&mut self, id: u16, entry: BootEntry) -> crate::Result<()> {
        let bytes = entry.to_bytes();

        self.write(
            &VariableName::new(&id.boot_var_name()),
            VariableFlags::NON_VOLATILE
                | VariableFlags::BOOTSERVICE_ACCESS
                | VariableFlags::RUNTIME_ACCESS,
            &bytes,
        )
        .unwrap();

        Ok(())
    }
}
