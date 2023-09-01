use efivar::{boot::BootVarName, VarManager};

pub fn get_order(manager: Box<dyn VarManager>) {
    let ids = match manager.get_boot_order() {
        Ok(ids) => ids,
        Err(err) => {
            eprintln!("Failed to get boot order IDs: {}", err);
            return;
        }
    };

    println!("Boot order:");

    for id in ids {
        println!("{}", id.boot_var_name());
    }
}
