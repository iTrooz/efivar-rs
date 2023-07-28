use itertools::Itertools;
use std::str::FromStr;

use efivar::{efi::VariableName, VarManager};

pub fn run(reader: Box<dyn VarManager>, name: &str, as_string: bool) {
    let mut buf = vec![0u8; 512];

    let name = VariableName::from_str(name).expect("failed to parse variable name");

    match reader.read(&name, &mut buf[..]) {
        Ok((size, attr)) => {
            println!("Attributes: {}", attr.to_string());
            if as_string {
                println!("{}", String::from_utf8_lossy(&buf[..size]));
            } else {
                println!(
                    "{}",
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
