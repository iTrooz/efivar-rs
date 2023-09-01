use efivar::VarManager;

pub fn run(mut manager: Box<dyn VarManager>, id: u16) {
    let mut ids = manager.get_boot_order().unwrap();

    if let Some(index) = ids.iter().position(|loop_id| loop_id == &id) {
        ids.remove(index);
    } else {
        eprintln!("Id {id:04X} not found in boot order");
        return;
    }

    manager.set_boot_order(ids.clone()).unwrap(); // TODO remove clone() call

    println!(
        "Removed id {id:04X} from boot order. New boot order: {}",
        super::boot_order_str(&ids)
    );
}
