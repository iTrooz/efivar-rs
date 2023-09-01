use efivar::{
    boot::{BootEntry, BootEntryAttributes},
    efi::Variable,
    VarManager,
};

/// prints a boot entry to the console, and consume it
fn print_var(var: &Variable, entry: BootEntry, verbose: bool) {
    println!();

    println!(
        "ID: {:04X}",
        var.boot_var_id()
            .expect("No entry ID for variable that should bot a boot variable")
    );
    println!("Description: {}", entry.description);
    println!(
        "Enabled: {}",
        entry
            .attributes
            .contains(BootEntryAttributes::LOAD_OPTION_ACTIVE)
    );

    println!(
        "Boot file: {}",
        entry
            .file_path_list
            .map(|fpl| fpl.to_string())
            .unwrap_or_else(|| "None/Invalid".to_owned())
    );

    if verbose {
        println!(
            "Optional data: {}",
            if entry.optional_data.is_empty() {
                "None".to_owned()
            } else {
                entry
                    .optional_data
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<String>>()
                    .join(" ")
            }
        );

        println!(
            "Attributes: {}",
            if entry.attributes.is_empty() {
                "None".to_owned()
            } else {
                entry.attributes.to_string()
            }
        );
    }
}

pub fn run(manager: Box<dyn VarManager>, verbose: bool) {
    let entries = match manager.get_boot_entries() {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Failed to get boot entries: {}", err);
            return;
        }
    };

    let mut vars: Vec<Variable> = match manager.get_all_vars() {
        Ok(vars) => vars,
        Err(err) => {
            eprintln!("Failed to list EFI variable: {:?}", err);
            return;
        }
    }
    .filter(|var| var.boot_var_id().is_some())
    .filter(|var| var.vendor().is_efi())
    .collect();

    println!("Boot entries in boot sequence (in boot order):");

    for (entry, var) in entries {
        // remove this variable from the list of variables to show
        vars.retain(|loop_var| loop_var.name() != var.name());

        match entry {
            Ok(entry) => print_var(&var, entry, verbose),
            Err(err) => eprintln!("Failed to get entry from variable {}: {}", var, err),
        }
    }

    if vars.is_empty() {
        return;
    }

    println!();
    println!("Found boot entries not in boot sequence:");
    for var in vars {
        match BootEntry::parse(&*manager, &var) {
            Ok(entry) => print_var(&var, entry, verbose),
            Err(err) => eprintln!("Failed to get entry from variable {}: {}", var, err),
        };
    }
}
