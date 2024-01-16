use crate::exit_code::ExitCode;

use efivar::VarManager;

pub fn run(manager: &mut dyn VarManager, ids: Vec<u16>) -> ExitCode {
    manager.set_boot_order(ids.clone()).unwrap(); // TODO remove clone() call

    println!(
        "Overwrote boot order. New boot order: {}",
        super::boot_order_str(&ids)
    );

    ExitCode::SUCCESS
}
