use crate::exit_code::ExitCode;

use efivar::{
    efi::{Variable, VariableVendor},
    VarManager,
};

pub fn run(manager: &mut dyn VarManager, name: &str, namespace: Option<uuid::Uuid>) -> ExitCode {
    let var_name = Variable::new_with_vendor(
        name,
        namespace.map_or(VariableVendor::Efi, VariableVendor::Custom),
    );

    match manager.delete(&var_name) {
        Ok(_) => {
            log::info!("Deleted variable {var_name} successfully");
            ExitCode::SUCCESS
        }
        Err(err) => {
            log::error!("Failed to delete variable {var_name}: {err}");
            ExitCode::FAILURE
        }
    }
}
