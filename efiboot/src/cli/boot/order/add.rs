use std::process::ExitCode;

use efivar::VarManager;

pub fn run(mut manager: Box<dyn VarManager>, id: u16, position: Option<usize>) -> ExitCode {
    let mut ids = manager.get_boot_order().unwrap();
    if let Some(position) = position {
        ids.insert(position, id);
    } else {
        ids.push(id);
    }

    manager.set_boot_order(ids.clone()).unwrap(); // TODO remove clone() call

    println!(
        "Added new id {id:04X} to boot order. New boot order: {}",
        super::boot_order_str(&ids)
    );

    ExitCode::SUCCESS
}
