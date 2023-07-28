use std::{fs::File, io::Write, path::Path};

use uuid::Uuid;

use efivar::{
    efi::{VariableFlags, VariableName, VariableVendor},
    VarManager,
};

fn dump(output_path: &Path, flags: VariableFlags, data: &[u8]) -> Result<(), std::io::Error> {
    let mut file = File::create(output_path)?;
    file.write_all(&flags.bits().to_le_bytes())?;
    file.write_all(data)?;

    Ok(())
}

pub fn run(reader: Box<dyn VarManager>, name: &str, namespace: Option<Uuid>, output_path: &Path) {
    let var = VariableName::new_with_vendor(
        name,
        namespace.map_or(VariableVendor::Efi, VariableVendor::Custom),
    );

    let mut buf = vec![0u8; 512];
    match reader.read(&var, &mut buf) {
        Ok((size, flags)) => {
            buf.resize(size, 0);
            match dump(output_path, flags, &buf) {
                Ok(_) => println!(
                    "Dumped variable {} to file {}",
                    var,
                    output_path.canonicalize().unwrap().display()
                ),
                Err(err) => eprintln!("Failed to write to file: {}", err),
            }
        }
        Err(err) => eprintln!("Failed to read variable: {}", err),
    }
}
