use crate::exit_code::ExitCode;

use clap::Parser;
use efivar::VarManager;
use itertools::Itertools;

use crate::id::BootEntryId;

pub mod add;
pub mod get;
pub mod remove;
pub mod set;
#[cfg(test)]
mod tests;

#[derive(Parser)]
pub enum OrderCommand {
    /// Get current boot order IDs. See `efivarcli boot list` to get boot entries information
    Get,
    /// Adds an id to the boot order
    Add {
        /// ID of the entry to add
        #[arg(value_name = "ID")]
        id: BootEntryId,

        /// Position to insert the ID at. 0 is the beginning of the boot order. Defaults to the end.
        #[arg(short, long, value_name = "POSITION")]
        position: Option<usize>,
    },
    /// Remove an id from the boot order
    #[command(alias = "del")]
    #[command(alias = "delete")]
    Remove {
        /// ID of the entry to remove
        #[arg(value_name = "ID")]
        id: BootEntryId,

        /// whether to override warnings
        #[arg(long)]
        force: bool,
    },
    /// Overwrite the boot order with the ids provided
    /// Warning: the old boot order will be erased !
    Set {
        /// ids that will compose the new boot order
        ids: Vec<BootEntryId>,
    },
}

pub fn run(manager: &mut dyn VarManager, cmd: OrderCommand) -> ExitCode {
    match cmd {
        OrderCommand::Get => get::run(manager),
        OrderCommand::Add { id, position } => add::run(manager, id.0, position),
        OrderCommand::Remove { id, force } => remove::run(manager, id.0, force),
        OrderCommand::Set { ids } => {
            set::run(manager, ids.into_iter().map(|id| id.0).collect_vec())
        }
    }
}

/// Generate a string version of the boot order.
fn boot_order_str(ids: &[u16]) -> String {
    ids.iter().map(|id| format!("{id:04X}")).join(" ")
}
