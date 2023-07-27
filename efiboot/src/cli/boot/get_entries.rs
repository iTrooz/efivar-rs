use efivar::VarManager;

pub fn get_entries(manager: Box<dyn VarManager>) {
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
    }
}
