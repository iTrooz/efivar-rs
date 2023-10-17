use std::path::PathBuf;

use efivar::VarManager;
use structopt::StructOpt;

use crate::exit_code::ExitCode;

use self::boot::BootCommand;

pub mod boot;
pub mod delete;
pub mod export;
pub mod import;
pub mod list;
pub mod read;

#[derive(StructOpt)]
pub enum Command {
    /// Read the value of a variable
    Read {
        /// Name of the variable to read
        #[structopt(value_name = "VARIABLE")]
        name: String,

        /// GUID of the namespace. Default: EFI standard namespace
        #[structopt(short, long, value_name = "NAMESPACE")]
        namespace: Option<uuid::Uuid>,

        /// Print the value as an UTF-8 string
        #[structopt(short, long)]
        string: bool,
    },
    /// List known EFI variables
    List {
        /// GUID of the namespace. Default: EFI standard namespace
        #[structopt(short, long, value_name = "NAMESPACE")]
        namespace: Option<uuid::Uuid>,
        /// ignore --namespace and show all namespaces
        #[structopt(short, long)]
        all: bool,
    },
    /// Delete an EFI variabe
    #[structopt(visible_alias = "del")]
    #[structopt(visible_alias = "remove")]
    Delete {
        /// Name of the variable to delete
        #[structopt(value_name = "VARIABLE")]
        name: String,

        /// GUID of the namespace. Default: EFI standard namespace
        #[structopt(short, long, value_name = "NAMESPACE")]
        namespace: Option<uuid::Uuid>,
    },

    /// Manage boot-related variables
    Boot(BootCommand),
    /// Export a variable to file
    Export {
        /// Name of the variable to export
        #[structopt(value_name = "VARIABLE")]
        name: String,

        /// GUID of the namespace. Default: EFI standard namespace
        #[structopt(short, long, value_name = "NAMESPACE")]
        namespace: Option<uuid::Uuid>,

        /// Output file
        #[structopt(value_name = "OUTPUT_FILE")]
        output_file: PathBuf,
    },
    /// Import a variable from a file
    Import {
        /// Input file
        #[structopt(value_name = "INPUT_FILE")]
        input_file: PathBuf,

        /// Name of the variable to create
        #[structopt(value_name = "VARIABLE")]
        name: String,

        /// GUID of the namespace. Default: EFI standard namespace
        #[structopt(short, long, value_name = "NAMESPACE")]
        namespace: Option<uuid::Uuid>,
    },
}

pub fn run(manager: Box<dyn VarManager>, cmd: Command) -> ExitCode {
    match cmd {
        Command::Read {
            name,
            namespace,
            string,
        } => read::run(manager, &name, namespace, string),
        Command::List { namespace, all } => list::run(manager, namespace, all),
        Command::Delete { name, namespace } => delete::run(manager, &name, namespace),
        Command::Boot(arg) => boot::run(manager, arg),
        Command::Export {
            name,
            namespace,
            output_file,
        } => export::run(manager, &name, namespace, &output_file),
        Command::Import {
            input_file,
            name,
            namespace,
        } => import::run(manager, &input_file, &name, namespace),
    }
}
