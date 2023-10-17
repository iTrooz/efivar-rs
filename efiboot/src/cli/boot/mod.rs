use std::process::ExitCode;

use efivar::VarManager;
use structopt::StructOpt;

use crate::id::BootEntryId;

use self::{next::BootNextCommand, order::OrderCommand};

pub mod add;
pub mod delete;
pub mod enable_disable;
pub mod get_entries;
pub mod next;
pub mod order;

#[derive(StructOpt)]
pub enum BootCommand {
    /// Get all boot entries found, both in the boot order, and outside it if the name matchs
    GetEntries {
        /// Show more information, such as optional data
        #[structopt(short, long)]
        verbose: bool,
    },
    Add {
        /// Partition that holds the file to boot from. Defaults to the currently active boot partition
        #[structopt(short, long)]
        partition: Option<String>,

        /// File to boot from, inside the partition
        #[structopt(short, long)]
        file: String,

        /// Set entry description
        #[structopt(short, long)]
        description: String,

        /// Skip checks to ensure data is valid
        #[structopt(long)]
        force: bool,

        /// ID to give to the boot entry
        #[structopt(long)]
        id: Option<BootEntryId>,
    },
    /// Delete boot entry
    #[structopt(visible_alias = "del")]
    #[structopt(visible_alias = "remove")]
    Delete {
        /// ID of the boot entry to delete
        #[structopt()]
        id: BootEntryId,
    },
    /// Enable boot entry
    Enable {
        /// ID of the boot entry to enable
        #[structopt()]
        id: BootEntryId,
    },
    /// Disable boot entry
    Disable {
        /// ID of the boot entry to disable
        #[structopt()]
        id: BootEntryId,
    },
    /// Manage boot order
    Order(OrderCommand),
    /// Manage BootNext variable
    Next(BootNextCommand),
}

pub fn run(manager: Box<dyn VarManager>, cmd: BootCommand) -> ExitCode {
    match cmd {
        BootCommand::GetEntries { verbose } => get_entries::run(manager, verbose),
        BootCommand::Add {
            partition,
            file,
            description,
            force,
            id,
        } => add::run(
            manager,
            partition,
            file,
            description,
            force,
            id.map(|id| id.0),
        ),
        BootCommand::Delete { id } => delete::run(manager, id.0),
        BootCommand::Enable { id } => enable_disable::enable(manager, id.0),
        BootCommand::Disable { id } => enable_disable::disable(manager, id.0),
        BootCommand::Order(arg) => order::run(manager, arg),
        BootCommand::Next(arg) => next::run(manager, arg),
    }
}
