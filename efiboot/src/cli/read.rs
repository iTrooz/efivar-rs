use efivar::VarManager;


use itertools::Itertools;

pub fn run(reader: Box<dyn VarManager>, name: &str, as_string: bool) {
    match reader.read(&name) {
        Ok((attr, buffer)) => {
            eprintln!("Attributes: {}", attr.to_string());
            if as_string {
                println!("{}", String::from_utf8_lossy(&buffer));
            } else {
                println!(
                    "{}",
                    buffer
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
