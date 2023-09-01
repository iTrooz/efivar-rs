use efivar::{efi::VariableVendor, VarManager};

fn list_all(enumerator: Box<dyn VarManager>) {
    println!("{: >36} Variable", "Namespace");
    for var in enumerator
        .get_all_vars()
        .expect("Failed to list variable names")
    {
        println!("{} {}", var.vendor(), var.variable());
    }
}

fn list_namespace(enumerator: Box<dyn VarManager>, vendor: VariableVendor) {
    println!("Variables in namespace {} :", vendor);
    for var in enumerator
        .get_all_vars()
        .expect("Failed to list variable names")
    {
        if var.vendor() == &vendor {
            println!("{}", var.variable());
        }
    }
}

pub fn run(enumerator: Box<dyn VarManager>, namespace: Option<uuid::Uuid>, all: bool) {
    if all {
        list_all(enumerator);
    } else {
        list_namespace(
            enumerator,
            namespace.map_or(VariableVendor::Efi, VariableVendor::Custom),
        );
    }
}
