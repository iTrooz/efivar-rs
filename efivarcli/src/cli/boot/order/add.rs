use crate::exit_code::ExitCode;

use efivar::{boot::BootVarFormat, VarManager};

pub fn run(manager: &mut dyn VarManager, id: u16, position: Option<usize>) -> ExitCode {
    let mut ids = manager.get_boot_order().unwrap();
    if let Some(position) = position {
        ids.insert(position, id);
    } else {
        ids.push(id);
    }

    manager.set_boot_order(ids.clone()).unwrap(); // TODO remove clone() call

    log::info!(
        "Added new id {} to boot order. New boot order: {}",
        id.boot_id_format(),
        super::boot_order_str(&ids)
    );

    ExitCode::SUCCESS
}
