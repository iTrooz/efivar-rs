use efivar::VarManager;
use itertools::Itertools;
use structopt::StructOpt;

use crate::id::BootEntryId;

pub mod add;
pub mod remove;
pub mod set;

#[derive(StructOpt)]
pub enum OrderCommand {
    /// Adds an id from the boot order
    Add {
        /// ID of the entry to add
        #[structopt(value_name = "ID")]
        id: BootEntryId,

        /// Position to insert the ID at. 0 is the beginning of the boot order. Defaults to the end.
        #[structopt(value_name = "POSITION")]
        position: Option<usize>,
    },
    /// Remove an id from the boot order
    Remove {
        /// ID of the entry to remove
        #[structopt(value_name = "ID")]
        id: BootEntryId,
    },
    /// Overwrite the boot order with the ids provided
    /// Warning: the old boot order will be erased !
    Set {
        /// ids that will compose the new boot order
        ids: Vec<BootEntryId>,
    },
}

pub fn run(manager: Box<dyn VarManager>, cmd: OrderCommand) {
    match cmd {
        OrderCommand::Add { id, position } => add::run(manager, id.0, position),
        OrderCommand::Remove { id } => remove::run(manager, id.0),
        OrderCommand::Set { ids } => {
            set::run(manager, ids.into_iter().map(|id| id.0).collect_vec())
        }
    }
}

/// Generate a string version of the boot order.
fn boot_order_str(ids: &[u16]) -> String {
    ids.iter().map(|id| format!("{id:04X}")).join(" ")
}

#[cfg(test)]
mod tests {

    pub use super::*;

    #[test]
    fn test_boot_order_str() {
        assert_eq!(boot_order_str(&[0x0001, 0x2000, 0xBEEF]), "0001 2000 BEEF");
    }
}