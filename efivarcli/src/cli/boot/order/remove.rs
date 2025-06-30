use crate::exit_code::ExitCode;

use efivar::{boot::BootVarName, efi::Variable, VarManager};

pub fn run(manager: &mut dyn VarManager, id: u16, force: bool) -> ExitCode {
    let mut ids = manager.get_boot_order().unwrap();

    if let Some(index) = ids.iter().position(|loop_id| loop_id == &id) {
        ids.remove(index);
    } else {
        log::error!("Id {id:04X} not found in boot order");
        return ExitCode::FAILURE;
    }

    if manager.read(&Variable::new(&id.boot_var_name())).is_ok() && !force {
        log::warn!("A variable with the name {} exists. Deleting its id from the boot order won't delete it.\n\
            Use `efivarcli boot del {id:04X}` instead.\n\
            Pass argument --force to skip this warning", id.boot_var_name());
        return ExitCode::FAILURE;
    }

    manager.set_boot_order(ids.clone()).unwrap(); // TODO remove clone() call

    log::info!(
        "Removed id {id:04X} from boot order. New boot order: {}",
        super::boot_order_str(&ids)
    );

    ExitCode::SUCCESS
}
