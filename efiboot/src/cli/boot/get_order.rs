use efivar::VarManager;

pub fn get_order(manager: Box<dyn VarManager>) {
    let entries = match manager.get_boot_order() {
        Ok(entries) => entries,
        Err(err) => {
            eprintln!("Failed to get boot order IDs: {}", err);
            return;
        }
    };

    println!("Boot order:");

    for entry in entries {
        println!("{}", entry.variable());
    }
}
