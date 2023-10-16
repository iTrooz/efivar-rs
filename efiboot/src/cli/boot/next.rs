use core::panic;

use byteorder::{LittleEndian, ReadBytesExt};
use efivar::{
    boot::{BootEntry, BootEntryAttributes, BootVarName},
    efi::{Variable, VariableFlags},
    Error, VarManager,
};
use structopt::StructOpt;

use crate::id::BootEntryId;

#[derive(StructOpt)]
pub enum BootNextCommand {
    /// Unset the BootNext variable
    Unset,
    /// Get BootNext
    Get,
    /// Set BootNext
    Set {
        /// ID BootNext entry
        #[structopt()]
        id: BootEntryId,
    },
}

pub fn run(mut manager: Box<dyn VarManager>, cmd: BootNextCommand) {
    match cmd {
        BootNextCommand::Get => {
            let res = manager.read(&Variable::new("BootNext"));
            match res {
                Ok((data, _)) => {
                    println!(
                        "Next booting on: {:04X}",
                        data.as_slice().read_u16::<LittleEndian>().unwrap()
                    );
                }
                Err(Error::VarNotFound { name: _ }) => {
                    println!("BootNext is not set")
                }
                Err(err) => {
                    panic!("{}", err);
                }
            }
        }
        BootNextCommand::Set { id } => {
            let id = id.0;

            let boot_entry = match BootEntry::parse(&*manager, &Variable::new(&id.boot_var_name()))
            {
                Ok(boot_entry) => boot_entry,
                Err(Error::VarNotFound { name: _ }) => {
                    println!("No boot entry with id {id:04X} found");
                    return;
                }
                Err(err) => panic!("{}", err),
            };

            if !boot_entry
                .attributes
                .contains(BootEntryAttributes::LOAD_OPTION_ACTIVE)
            {
                eprintln!("Warning: boot entry is not active, and may not boot. Enable it with `efiboot boot enable {id:04X}`");
            }

            manager
                .write(
                    &Variable::new("BootNext"),
                    VariableFlags::NON_VOLATILE
                        | VariableFlags::BOOTSERVICE_ACCESS
                        | VariableFlags::RUNTIME_ACCESS,
                    &id.to_le_bytes(),
                )
                .unwrap();

            println!(
                "BootNext set to {id:04X} ({}) with success",
                boot_entry.description
            );
        }
        BootNextCommand::Unset => {
            match manager.delete(&Variable::new("BootNext")) {
                Ok(_) => println!("BootNext unset with success"),
                Err(Error::VarNotFound { name: _ }) => println!("BootNext not set"),
                Err(err) => panic!("{}", err),
            };
        }
    }
}
