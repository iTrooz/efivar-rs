use std::process::ExitCode;

use efivar::{
    efi::{Variable, VariableVendor},
    VarManager,
};

pub fn run(
    mut manager: Box<dyn VarManager>,
    name: &str,
    namespace: Option<uuid::Uuid>,
) -> ExitCode {
    let var_name = Variable::new_with_vendor(
        name,
        namespace.map_or(VariableVendor::Efi, VariableVendor::Custom),
    );

    match manager.delete(&var_name) {
        Ok(_) => {
            println!("Deleted variable {var_name} successfully");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("Failed to delete variable {var_name}: {err}");
            ExitCode::FAILURE
        }
    }
}
