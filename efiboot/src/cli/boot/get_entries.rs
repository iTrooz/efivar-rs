use efivar::VarManager;

pub fn get_entries(manager: Box<dyn VarManager>, verbose: bool) {
    let entries = match manager.get_boot_entries() {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Failed to get boot entries: {}", err);
            return;
        }
    };

    println!("Boot entries (in boot order):");

    for entry in entries {
        println!("--");
        println!("Attributes: {:?}", entry.attributes);
        println!("Description: {:?}", entry.description);
        if let Some(file_path_list) = entry.file_path_list {
            println!("Boot file: {}", file_path_list);
        } else {
            println!("No valid boot file location");
        }
        if verbose && !entry.optional_data.is_empty() {
            println!(
                "Optional data: {}",
                entry
                    .optional_data
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<String>>()
                    .join(" ")
            );
        }
    }
}
