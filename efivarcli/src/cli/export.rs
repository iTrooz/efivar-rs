use std::{fs::File, io::Write, path::Path};

use uuid::Uuid;

use efivar::{
    efi::{Variable, VariableFlags, VariableVendor},
    VarManager,
};

use crate::exit_code::ExitCode;

fn export(output_path: &Path, flags: VariableFlags, data: &[u8]) -> Result<(), std::io::Error> {
    let mut file = File::create(output_path)?;
    file.write_all(&flags.bits().to_le_bytes())?;
    file.write_all(data)?;

    Ok(())
}

pub fn run(
    reader: &dyn VarManager,
    name: &str,
    namespace: Option<Uuid>,
    output_path: &Path,
) -> ExitCode {
    let var = Variable::new_with_vendor(
        name,
        namespace.map_or(VariableVendor::Efi, VariableVendor::Custom),
    );

    match reader.read(&var) {
        Ok((buf, flags)) => match export(output_path, flags, &buf) {
            Ok(_) => {
                log::info!(
                    "Exported variable {} to file {}",
                    var,
                    output_path.canonicalize().unwrap().display()
                );
                return ExitCode::SUCCESS;
            }
            Err(err) => log::error!("Failed to write to file: {err}"),
        },
        Err(err) => log::error!("Failed to read variable: {err}"),
    };

    ExitCode::FAILURE
}
