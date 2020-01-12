use efivar::VarManager;

use itertools::Itertools;

pub fn run(reader: Box<dyn VarManager>, name: &str, as_string: bool) {
    let mut buf = vec![0u8; 512];

    match reader.read(&name, &mut buf[..]) {
        Ok((size, attr)) => {
            eprintln!("Attributes: {}", attr.to_string());
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
