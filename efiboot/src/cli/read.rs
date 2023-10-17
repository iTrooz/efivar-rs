use std::process::ExitCode;

use itertools::Itertools;

use efivar::{
    efi::{Variable, VariableVendor},
    VarManager,
};

pub fn run(
    reader: Box<dyn VarManager>,
    name: &str,
    namespace: Option<uuid::Uuid>,
    as_string: bool,
) -> ExitCode {
    let name = Variable::new_with_vendor(
        name,
        namespace.map_or(VariableVendor::Efi, VariableVendor::Custom),
    );

    match reader.read(&name) {
        Ok((buf, attr)) => {
            println!("Attributes: {}", attr.to_string());
            if as_string {
                println!("Value (as UTF8 string): {}", String::from_utf8_lossy(&buf));
            } else {
                println!(
                    "Value: {}",
                    buf.iter()
                        .tuples()
                        .map(|(a, b)| format!("{:02X}{:02X}", a, b))
                        .fold(String::new(), |acc, ref item| acc + " " + item)
                        .trim()
                );
            };
            ExitCode::SUCCESS
        }
        Err(reason) => {
            eprintln!("Failed: {}", reason);
            ExitCode::FAILURE
        }
    }
}
