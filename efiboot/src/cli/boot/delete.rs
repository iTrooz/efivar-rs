use std::process::ExitCode;

use efivar::{boot::BootVarName, efi::Variable, VarManager};

pub fn run(mut manager: Box<dyn VarManager>, id: u16) -> ExitCode {
    // in this function, we assume that boot entry presence and boot order id presence are not correlated,
    // so we need to remove both of them, no matter if one of these steps raises an error

    // delete the entry (note we do not raise the error, see above)
    #[allow(unused_must_use)]
    {
        manager.delete(&Variable::new(&id.boot_var_name()));
    }

    // remove it from boot order
    let mut ids = manager.get_boot_order().unwrap();
    ids.retain(|v| *v != id);
    manager.set_boot_order(ids).unwrap();

    println!("Deleted entry with success");

    ExitCode::SUCCESS
}
