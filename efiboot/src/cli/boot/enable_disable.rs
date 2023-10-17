use std::process::ExitCode;

use efivar::{
    boot::{BootEntry, BootEntryAttributes, BootVarName},
    efi::Variable,
    VarManager,
};

pub fn enable(mut manager: Box<dyn VarManager>, id: u16) -> ExitCode {
    let mut boot_entry = BootEntry::parse(&*manager, &Variable::new(&id.boot_var_name())).unwrap();

    if boot_entry
        .attributes
        .contains(BootEntryAttributes::LOAD_OPTION_ACTIVE)
    {
        eprintln!("Boot entry is already enabled");
        return ExitCode::FAILURE;
    }

    boot_entry
        .attributes
        .insert(BootEntryAttributes::LOAD_OPTION_ACTIVE);

    manager.add_boot_entry(id, boot_entry).unwrap();
    println!("Enabled boot entry with success");

    ExitCode::SUCCESS
}

pub fn disable(mut manager: Box<dyn VarManager>, id: u16) -> ExitCode {
    let mut boot_entry = BootEntry::parse(&*manager, &Variable::new(&id.boot_var_name())).unwrap();

    if !boot_entry
        .attributes
        .contains(BootEntryAttributes::LOAD_OPTION_ACTIVE)
    {
        eprintln!("Boot entry is already disabled");
        return ExitCode::FAILURE;
    }

    boot_entry
        .attributes
        .remove(BootEntryAttributes::LOAD_OPTION_ACTIVE);

    manager.add_boot_entry(id, boot_entry).unwrap();
    println!("Disabled boot entry with success");

    ExitCode::SUCCESS
}
