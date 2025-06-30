use crate::exit_code::ExitCode;

use efivar::{boot::BootVarFormat, efi::Variable, VarManager};

pub fn run(manager: &mut dyn VarManager, id: u16, force: bool) -> ExitCode {
    let mut ids = manager.get_boot_order().unwrap();

    if let Some(index) = ids.iter().position(|loop_id| loop_id == &id) {
        ids.remove(index);
    } else {
        log::error!("Id {} not found in boot order", id.boot_id_format());
        return ExitCode::FAILURE;
    }

    if manager.read(&Variable::new(&id.boot_var_format())).is_ok() && !force {
        log::warn!(
            "A variable with ID {} exists. Deleting its id from the boot order won't delete it.\n\
            Use `efivarcli boot del {}` instead.\n\
            Pass argument --force to skip this warning",
            id.boot_id_format(),
            id.boot_id_format()
        );
        return ExitCode::FAILURE;
    }

    manager.set_boot_order(ids.clone()).unwrap(); // TODO remove clone() call

    log::info!(
        "Removed id {} from boot order. New boot order: {}",
        id.boot_id_format(),
        super::boot_order_str(&ids)
    );

    ExitCode::SUCCESS
}
