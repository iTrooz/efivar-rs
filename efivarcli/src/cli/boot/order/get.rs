use crate::exit_code::ExitCode;

use efivar::{boot::BootVarFormat, VarManager};

pub fn run(manager: &dyn VarManager) -> ExitCode {
    let ids = match manager.get_boot_order() {
        Ok(ids) => ids,
        Err(err) => {
            log::error!("Failed to get boot order IDs: {err}");
            return ExitCode::FAILURE;
        }
    };

    println!("Boot order:");

    for id in ids {
        println!("{}", id.boot_var_format());
    }

    ExitCode::SUCCESS
}
