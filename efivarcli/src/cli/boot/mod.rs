use crate::exit_code::ExitCode;

use clap::Parser;
use efivar::VarManager;

use crate::id::BootEntryId;

use self::{next::BootNextCommand, order::OrderCommand};

pub mod add;
pub mod delete;
pub mod enable_disable;
pub mod get_entries;
pub mod next;
pub mod order;
pub mod partition;

#[cfg(test)]
mod tests;

#[derive(Parser)]
pub enum BootCommand {
    /// Get all boot entries found, both in the boot order, and outside it if the name matchs
    #[command(alias = "get-entries")]
    List {
        /// Show more information, such as optional data
        #[arg(short, long)]
        verbose: bool,
    },
    Add {
        /// Disk device to use (e.g. /dev/sda)
        #[arg(long, requires = "partition")]
        disk: Option<String>,

        /// Partition that holds the file to boot from.
        /// May be a partition number (1) or a full partition name on linux (/dev/sda1)
        /// Defaults to the currently active boot partition
        #[arg(short, long)]
        partition: Option<String>,

        /// File to boot from, inside the partition
        #[arg(short, long)]
        file: String,

        /// Set entry description
        #[arg(short, long, alias = "desc")]
        description: String,

        /// Skip checks to ensure data is valid
        #[arg(long)]
        force: bool,

        /// ID to give to the boot entry
        #[arg(long)]
        id: Option<BootEntryId>,
    },
    /// Delete boot entry
    #[command(alias = "del")]
    #[command(alias = "remove")]
    Delete {
        /// ID of the boot entry to delete
        #[arg()]
        id: BootEntryId,
    },
    /// Enable boot entry
    Enable {
        /// ID of the boot entry to enable
        #[arg()]
        id: BootEntryId,
    },
    /// Disable boot entry
    Disable {
        /// ID of the boot entry to disable
        #[arg()]
        id: BootEntryId,
    },
    /// Manage boot order
    #[command(subcommand)]
    Order(OrderCommand),
    /// Manage BootNext variable
    #[command(subcommand)]
    Next(BootNextCommand),
}

pub fn run(manager: &mut dyn VarManager, cmd: BootCommand) -> ExitCode {
    match cmd {
        BootCommand::List { verbose } => get_entries::run(manager, verbose),
        BootCommand::Add {
            disk,
            partition,
            file,
            description,
            force,
            id,
        } => add::run(
            manager,
            disk,
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
