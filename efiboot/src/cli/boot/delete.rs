use crate::exit_code::ExitCode;

use efivar::{boot::BootVarName, efi::Variable, VarManager};

pub fn run(manager: &mut dyn VarManager, id: u16) -> ExitCode {
    // in this function, we assume that boot entry presence and boot order id presence are not correlated,
    // so we need to remove both of them, no matter if one of these steps raises an error

    // delete the entry
    match manager.delete(&Variable::new(&id.boot_var_name())) {
        Ok(_) => println!("Deleted entry with success"),
        Err(efivar::Error::VarNotFound { var: _ }) => eprintln!("Boot entry not found"),
        Err(err) => eprintln!("Failed to delete entry: {}", err),
    }

    // remove it from boot order
    let mut ids = manager.get_boot_order().unwrap();
    let old_size = ids.len();
    ids.retain(|v| *v != id);
    if old_size == ids.len() {
        eprintln!("Failed to remove id from boot order");
    } else {
        manager.set_boot_order(ids).unwrap();
    }

    ExitCode::SUCCESS
}
