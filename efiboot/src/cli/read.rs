use itertools::Itertools;

use efivar::{
    efi::{VariableName, VariableVendor},
    VarManager,
};

pub fn run(
    reader: Box<dyn VarManager>,
    name: &str,
    namespace: Option<uuid::Uuid>,
    as_string: bool,
) {
    let mut buf = vec![0u8; 512];

    let name = VariableName::new_with_vendor(
        name,
        namespace.map_or(VariableVendor::Efi, VariableVendor::Custom),
    );

    match reader.read(&name, &mut buf[..]) {
        Ok((size, attr)) => {
            println!("Attributes: {}", attr.to_string());
            if as_string {
                println!(
                    "Value (as UTF8 string): {}",
                    String::from_utf8_lossy(&buf[..size])
                );
            } else {
                println!(
                    "Value: {}",
                    buf[..size]
                        .iter()
                        .tuples()
                        .map(|(a, b)| format!("{:02X}{:02X}", a, b))
                        .fold(String::new(), |acc, ref item| acc + " " + item)
                        .trim()
                );
            }
        }
        Err(reason) => eprintln!("Failed: {}", reason),
    }
}
