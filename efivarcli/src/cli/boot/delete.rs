use crate::exit_code::ExitCode;

use efivar::{boot::BootVarName, efi::Variable, VarManager};

pub fn run(manager: &mut dyn VarManager, id: u16) -> ExitCode {
    // in this function, we assume that boot entry presence and boot order id presence are not correlated,
    // so we need to remove both of them, no matter if one of these steps raises an error

    let mut result = ExitCode::FAILURE;

    // delete the entry
    match manager.delete(&Variable::new(&id.boot_var_name())) {
        Ok(_) => {
            log::info!("Deleted boot entry variable with id {id} successfully");
            result = ExitCode::SUCCESS;
        }
        Err(efivar::Error::VarNotFound { var: _ }) => log::error!("Boot entry variable not found"),
        Err(err) => log::warn!("Failed to delete boot entry variable: {err}"),
    }

    // remove it from boot order
    let mut ids = manager.get_boot_order().unwrap();
    let old_size = ids.len();
    ids.retain(|v| *v != id);

    let apply = match (old_size, ids.len()) {
        (old, new) if old == new => {
            log::warn!("ID {id} was not found in boot order");
            false
        }
        (old, new) if old - new == 1 => {
            log::info!("Removed id {id} from boot order");
            true
        }
        (old, new) if old - new > 1 => {
            log::warn!("Removed id {id} multiple times from boot order");
            true
        }
        _ => {
            log::warn!(
                "Unexpected change in boot order size: was {old_size}, now {}. Not applying",
                ids.len()
            );
            false
        }
    };

    if apply {
        manager.set_boot_order(ids).unwrap();
        result = ExitCode::SUCCESS;
    }

    result
}
