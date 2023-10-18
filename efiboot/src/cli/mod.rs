use std::path::PathBuf;

use clap::Parser;
use efivar::VarManager;

use crate::exit_code::ExitCode;

use self::boot::BootCommand;

pub mod boot;
pub mod delete;
pub mod export;
pub mod import;
pub mod list;
pub mod read;
#[cfg(test)]
pub mod tests;

#[derive(Parser)]
pub enum Command {
    /// Read the value of a variable
    #[command(alias = "info")]
    Read {
        /// Name of the variable to read
        #[arg(value_name = "VARIABLE")]
        name: String,

        /// GUID of the namespace. Default: EFI standard namespace
        #[arg(short, long, value_name = "NAMESPACE")]
        namespace: Option<uuid::Uuid>,

        /// Print the value as an UTF-8 string
        #[arg(short, long)]
        string: bool,

        /// Skip the header and print the raw variable value
        #[arg(short, long)]
        raw: bool,
    },
    /// List known EFI variables
    List {
        /// GUID of the namespace. Default: EFI standard namespace
        #[arg(short, long, value_name = "NAMESPACE")]
        namespace: Option<uuid::Uuid>,
        /// ignore --namespace and show all namespaces
        #[arg(short, long)]
        all: bool,
    },
    /// Delete an EFI variabe
    // #[arg(visible_alias = "del")]
    // #[arg(visible_alias = "remove")]
    #[command(alias = "del")]
    #[command(alias = "remove")]
    Delete {
        /// Name of the variable to delete
        #[arg(value_name = "VARIABLE")]
        name: String,

        /// GUID of the namespace. Default: EFI standard namespace
        #[arg(short, long, value_name = "NAMESPACE")]
        namespace: Option<uuid::Uuid>,
    },

    /// Manage boot-related variables
    #[command(subcommand)]
    Boot(BootCommand),
    /// Export a variable to file
    Export {
        /// Name of the variable to export
        #[arg(value_name = "VARIABLE")]
        name: String,

        /// GUID of the namespace. Default: EFI standard namespace
        #[arg(short, long, value_name = "NAMESPACE")]
        namespace: Option<uuid::Uuid>,

        /// Output file
        #[arg(value_name = "OUTPUT_FILE")]
        output_file: PathBuf,
    },
    /// Import a variable from a file.
    /// Putting `-` as a file will read from stdin instead
    Import {
        /// Input file
        #[arg(value_name = "INPUT_FILE")]
        input_file: PathBuf,

        /// Name of the variable to create
        #[arg(value_name = "VARIABLE")]
        name: String,

        /// GUID of the namespace. Default: EFI standard namespace
        #[arg(short, long, value_name = "NAMESPACE")]
        namespace: Option<uuid::Uuid>,
    },
}

pub fn run(manager: &mut dyn VarManager, cmd: Command) -> ExitCode {
    match cmd {
        Command::Read {
            name,
            namespace,
            string,
            raw,
        } => read::run(manager, &name, namespace, string, raw),
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
