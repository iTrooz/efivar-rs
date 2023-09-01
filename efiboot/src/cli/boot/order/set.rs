use efivar::VarManager;

pub fn run(mut manager: Box<dyn VarManager>, ids: Vec<u16>) {
    manager.set_boot_order(ids.clone()).unwrap(); // TODO remove clone() call

    println!(
        "Overwrote boot order. New boot order: {}",
        super::boot_order_str(&ids)
    );
}
