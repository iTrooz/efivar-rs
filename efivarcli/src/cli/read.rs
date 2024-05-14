use crate::exit_code::ExitCode;

use itertools::Itertools;

use efivar::{
    efi::{Variable, VariableVendor},
    VarManager,
};

pub fn run(
    reader: &dyn VarManager,
    name: &str,
    namespace: Option<uuid::Uuid>,
    as_string: bool,
    raw: bool,
) -> ExitCode {
    let name = Variable::new_with_vendor(
        name,
        namespace.map_or(VariableVendor::Efi, VariableVendor::Custom),
    );

    match reader.read(&name) {
        Ok((buf, attr)) => {
            if !raw {
                println!("Attributes: {}", attr);
            }
            if as_string {
                if raw {
                    println!("{}", String::from_utf8_lossy(&buf));
                } else {
                    println!("Value (as UTF8 string): {}", String::from_utf8_lossy(&buf));
                }
            } else {
                let value = buf
                    .iter()
                    .tuples()
                    .map(|(a, b)| format!("{:02X}{:02X}", a, b))
                    .fold(String::new(), |acc, ref item| acc + " " + item)
                    .trim()
                    .to_owned();
                if raw {
                    println!("{}", value);
                } else {
                    println!("Value: {}", value);
                }
            };
            ExitCode::SUCCESS
        }
        Err(reason) => {
            eprintln!("Failed: {}", reason);
            ExitCode::FAILURE
        }
    }
}
