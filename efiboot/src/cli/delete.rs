use efivar::{
    efi::{VariableName, VariableVendor},
    VarManager,
};

pub fn run(mut manager: Box<dyn VarManager>, name: &str, namespace: Option<uuid::Uuid>) {
    let var_name = VariableName::new_with_vendor(
        name,
        namespace.map_or(VariableVendor::Efi, VariableVendor::Custom),
    );

    match manager.delete(&var_name) {
        Ok(_) => println!("Deleted variable {var_name} successfully"),
        Err(err) => eprintln!("Failed to delete variable {var_name}: {err}"),
    }
}
