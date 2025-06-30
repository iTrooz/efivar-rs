use crate::exit_code::ExitCode;

use byteorder::{LittleEndian, ReadBytesExt};
use efivar::{
    boot::{BootEntry, BootEntryAttributes, BootVariable},
    efi::Variable,
    VarManager,
};

/// prints a boot entry to the console, and consume it
pub fn print_var(boot_var: &BootVariable, verbose: bool, active_boot_id: u16) {
    println!();

    println!("ID: {:04X}", boot_var.id);
    println!("Description: {}", boot_var.entry.description);
    println!(
        "Enabled: {}",
        boot_var
            .entry
            .attributes
            .contains(BootEntryAttributes::LOAD_OPTION_ACTIVE)
    );

    println!(
        "Boot file: {}",
        boot_var
            .entry
            .file_path_list
            .as_ref()
            .map(|fpl| fpl.to_string())
            .unwrap_or_else(|| "None/Invalid".to_owned())
    );

    if verbose {
        println!(
            "Optional data: {}",
            if boot_var.entry.optional_data.is_empty() {
                "None".to_owned()
            } else {
                boot_var
                    .entry
                    .optional_data
                    .iter()
                    .map(|b| format!("{b:02x}"))
                    .collect::<Vec<String>>()
                    .join(" ")
            }
        );

        println!(
            "Attributes: {}",
            if boot_var.entry.attributes.is_empty() {
                "None".to_owned()
            } else {
                boot_var.entry.attributes.to_string()
            }
        );
    }

    if active_boot_id == boot_var.id {
        println!("Active boot entry: true")
    }
}

pub fn run(manager: &dyn VarManager, verbose: bool) -> ExitCode {
    let entries = match manager.get_boot_entries() {
        Ok(entries) => entries,
        Err(err) => {
            log::error!("Failed to get boot entries: {err}");
            return ExitCode::FAILURE;
        }
    };

    let mut vars: Vec<Variable> = match manager.get_all_vars() {
        Ok(vars) => vars,
        Err(err) => {
            log::error!("Failed to list EFI variables: {err:?}");
            return ExitCode::FAILURE;
        }
    }
    .filter(|var| var.vendor().is_efi())
    .collect();

    println!("Boot entries in boot sequence (in boot order):");

    let active_id = manager
        .read(&Variable::new("BootCurrent"))
        .unwrap()
        .0
        .as_slice()
        .read_u16::<LittleEndian>()
        .unwrap();

    for (entry, var) in entries {
        // remove this variable from the list of variables to show
        vars.retain(|loop_var| loop_var.name() != var.name());

        match entry {
            Ok(entry) => print_var(&entry, verbose, active_id),
            Err(err) => log::error!("Failed to get boot entry from variable {var}: {err}"),
        }
    }

    if vars.is_empty() {
        return ExitCode::SUCCESS;
    }

    println!();
    println!("Found boot entries not in boot sequence:");
    for var in vars {
        let boot_id = match var.boot_var_id() {
            Some(id) => id,
            None => continue,
        };
        match BootEntry::read(manager, &var) {
            Ok(entry) => print_var(&BootVariable { entry, id: boot_id }, verbose, active_id),
            Err(err) => log::error!("Failed to get boot entry from variable {var}: {err}"),
        };
    }

    ExitCode::SUCCESS
}
