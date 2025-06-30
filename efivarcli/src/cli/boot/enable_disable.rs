use crate::exit_code::ExitCode;

use efivar::{
    boot::{BootEntry, BootEntryAttributes, BootVarName},
    efi::Variable,
    VarManager,
};

pub fn enable(manager: &mut dyn VarManager, id: u16) -> ExitCode {
    let mut boot_entry = BootEntry::read(&*manager, &Variable::new(&id.boot_var_name())).unwrap();

    if boot_entry
        .attributes
        .contains(BootEntryAttributes::LOAD_OPTION_ACTIVE)
    {
        log::warn!("Boot entry is already enabled");
        return ExitCode::FAILURE;
    }

    boot_entry
        .attributes
        .insert(BootEntryAttributes::LOAD_OPTION_ACTIVE);

    manager.create_boot_entry(id, boot_entry).unwrap();
    log::info!("Enabled boot entry with success");

    ExitCode::SUCCESS
}

pub fn disable(manager: &mut dyn VarManager, id: u16) -> ExitCode {
    let mut boot_entry = BootEntry::read(&*manager, &Variable::new(&id.boot_var_name())).unwrap();

    if !boot_entry
        .attributes
        .contains(BootEntryAttributes::LOAD_OPTION_ACTIVE)
    {
        log::warn!("Boot entry is already disabled");
        return ExitCode::FAILURE;
    }

    boot_entry
        .attributes
        .remove(BootEntryAttributes::LOAD_OPTION_ACTIVE);

    manager.create_boot_entry(id, boot_entry).unwrap();
    log::info!("Disabled boot entry with success");

    ExitCode::SUCCESS
}
