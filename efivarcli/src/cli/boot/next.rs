use crate::exit_code::ExitCode;

use byteorder::{LittleEndian, ReadBytesExt};
use clap::Parser;
use efivar::{
    boot::{BootEntry, BootEntryAttributes, BootVarName},
    efi::{Variable, VariableFlags},
    Error, VarManager,
};

use crate::id::BootEntryId;

#[derive(Parser)]
pub enum BootNextCommand {
    /// Unset the BootNext variable
    Unset,
    /// Get BootNext
    Get,
    /// Set BootNext
    Set {
        /// ID BootNext entry
        #[arg()]
        id: BootEntryId,
    },
}

pub fn run(manager: &mut dyn VarManager, cmd: BootNextCommand) -> ExitCode {
    match cmd {
        BootNextCommand::Get => {
            let res = manager.read(&Variable::new("BootNext"));
            match res {
                Ok((data, _)) => {
                    log::info!(
                        "Next booting on: {:04X}",
                        data.as_slice().read_u16::<LittleEndian>().unwrap()
                    );
                    ExitCode::SUCCESS
                }
                Err(Error::VarNotFound { var: _ }) => {
                    log::warn!("BootNext is not set");
                    ExitCode::FAILURE
                }
                Err(err) => {
                    log::error!("Failed to read BootNext: {err}");
                    ExitCode::FAILURE
                }
            }
        }
        BootNextCommand::Set { id } => {
            let id = id.0;

            let boot_entry = match BootEntry::read(&*manager, &Variable::new(&id.boot_var_name())) {
                Ok(boot_entry) => boot_entry,
                Err(Error::VarNotFound { var: _ }) => {
                    log::error!("No boot entry with id {id:04X} found");
                    return ExitCode::FAILURE;
                }
                Err(err) => {
                    log::error!("Failed to read boot entry: {err}");
                    return ExitCode::FAILURE;
                }
            };

            if !boot_entry
                .attributes
                .contains(BootEntryAttributes::LOAD_OPTION_ACTIVE)
            {
                log::warn!("Boot entry is not active, and may not boot. Enable it with `efivarcli boot enable {id:04X}`");
            }

            manager
                .write(
                    &Variable::new("BootNext"),
                    VariableFlags::default(),
                    &id.to_le_bytes(),
                )
                .unwrap();

            log::info!(
                "BootNext set to {id:04X} ({}) with success",
                boot_entry.description
            );

            ExitCode::SUCCESS
        }
        BootNextCommand::Unset => match manager.delete(&Variable::new("BootNext")) {
            Ok(_) => {
                log::info!("BootNext unset with success");
                ExitCode::SUCCESS
            }
            Err(Error::VarNotFound { var: _ }) => {
                log::warn!("BootNext not set");
                ExitCode::FAILURE
            }
            Err(err) => {
                log::error!("Failed to unset BootNext: {err}");
                ExitCode::FAILURE
            }
        },
    }
}
