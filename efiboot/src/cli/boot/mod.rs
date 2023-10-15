use efivar::VarManager;
use structopt::StructOpt;

use crate::id::BootEntryId;

use self::order::OrderCommand;

pub mod add;
pub mod del;
pub mod get_entries;
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
    Del {
        /// ID of the boot entry to delete
        #[structopt()]
        id: BootEntryId,
    },
    /// Manage boot order
    Order(OrderCommand),
}

pub fn run(manager: Box<dyn VarManager>, cmd: BootCommand) {
    match cmd {
        BootCommand::GetEntries { verbose } => {
            get_entries::run(manager, verbose);
        }
        BootCommand::Add {
            partition,
            file,
            description,
            force,
            id,
        } => {
            add::run(
                manager,
                partition,
                file,
                description,
                force,
                id.map(|id| id.0),
            );
        }
        BootCommand::Del { id } => {
            del::run(manager, id.0);
        }
        BootCommand::Order(arg) => {
            order::run(manager, arg);
        }
    }
}
