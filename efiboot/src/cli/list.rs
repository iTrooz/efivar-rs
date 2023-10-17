use crate::exit_code::ExitCode;

use efivar::{efi::VariableVendor, VarManager};

fn list_all(enumerator: &dyn VarManager) {
    println!("{: >36} Variable", "Namespace");
    for var in enumerator
        .get_all_vars()
        .expect("Failed to list variable names")
    {
        println!("{} {}", var.vendor(), var.name());
    }
}

fn list_namespace(enumerator: &dyn VarManager, vendor: VariableVendor) {
    println!("Variables in namespace {} :", vendor);
    for var in enumerator
        .get_all_vars()
        .expect("Failed to list variable names")
    {
        if var.vendor() == &vendor {
            println!("{}", var.name());
        }
    }
}

pub fn run(enumerator: &dyn VarManager, namespace: Option<uuid::Uuid>, all: bool) -> ExitCode {
    if all {
        list_all(enumerator);
    } else {
        list_namespace(
            enumerator,
            namespace.map_or(VariableVendor::Efi, VariableVendor::Custom),
        );
    }
    ExitCode::SUCCESS
}
