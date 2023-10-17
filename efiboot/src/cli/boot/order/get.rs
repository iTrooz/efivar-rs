use crate::exit_code::ExitCode;

use efivar::{boot::BootVarName, VarManager};

pub fn run(manager: &dyn VarManager) -> ExitCode {
    let ids = match manager.get_boot_order() {
        Ok(ids) => ids,
        Err(err) => {
            eprintln!("Failed to get boot order IDs: {}", err);
            return ExitCode::FAILURE;
        }
    };

    println!("Boot order:");

    for id in ids {
        println!("{}", id.boot_var_name());
    }

    ExitCode::SUCCESS
}
