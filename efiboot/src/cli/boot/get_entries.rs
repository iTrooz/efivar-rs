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

    for _entry in entries {
        println!("Some entry");
    }
}
