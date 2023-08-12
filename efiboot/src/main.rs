mod cli;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
enum BootCommand {
    /// Get current boot order IDs. See get-entries to get boot entries information
    GetOrder,
    GetEntries {
        /// Show more information, such as optional data
        #[structopt(short, long)]
        verbose: bool,
    },
}

#[derive(StructOpt)]
enum Command {
    /// Read the value of a variable
    Read {
        /// Name of the variable to read
        #[structopt(value_name = "VARIABLE")]
        name: String,

        /// GUID of the namespace. Default: EFI standard namespace
        #[structopt(value_name = "GUID")]
        namespace: Option<uuid::Uuid>,

        /// Print the value as an UTF-8 string
        #[structopt(short, long)]
        string: bool,
    },
    /// List known EFI variables
    List {
        /// GUID of the namespace. Default: EFI standard namespace
        #[structopt(value_name = "GUID")]
        namespace: Option<uuid::Uuid>,
        /// ignore --namespace and show all namespaces
        #[structopt(short, long)]
        all: bool,
    },
    /// Delete an EFI variabe
    Delete {
        /// Name of the variable to delete
        #[structopt(value_name = "VARIABLE")]
        name: String,

        /// GUID of the namespace. Default: EFI standard namespace
        #[structopt(value_name = "GUID")]
        namespace: Option<uuid::Uuid>,
    },

    /// Manage boot-related variables
    Boot(BootCommand),
    /// Dump a variable to file
    Dump {
        /// Name of the variable to dump
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
        #[structopt(value_name = "OUTPUT_FILE")]
        input_file: PathBuf,

        /// Name of the variable to create
        #[structopt(value_name = "VARIABLE")]
        name: String,

        /// GUID of the namespace. Default: EFI standard namespace
        #[structopt(short, long, value_name = "NAMESPACE")]
        namespace: Option<uuid::Uuid>,
    },
}

#[derive(StructOpt)]
#[structopt(name = env!("CARGO_PKG_NAME"), author, about)]
struct Opt {
    /// TOML file to use for variable storage instead of the system
    #[structopt(
        short,
        long,
        value_name = "FILE",
        parse(from_os_str),
        env = "EFIBOOT_STORE"
    )]
    file_store: Option<PathBuf>,

    #[structopt(subcommand)]
    cmd: Command,
}

#[paw::main]
fn main(opts: Opt) {
    let manager = if let Some(filename) = opts.file_store {
        efivar::file_store(filename)
    } else {
        efivar::system()
    };

    match opts.cmd {
        Command::Read {
            name,
            namespace,
            string,
        } => {
            cli::read(manager, &name, namespace, string);
        }
        Command::List { namespace, all } => {
            cli::list(manager, namespace, all);
        }
        Command::Delete { name, namespace } => {
            cli::delete(manager, &name, namespace);
        }
        Command::Boot(arg) => match arg {
            BootCommand::GetOrder => {
                cli::get_boot_order(manager);
            }
            BootCommand::GetEntries { verbose } => {
                cli::get_boot_entries(manager, verbose);
            }
        },
        Command::Dump {
            name,
            namespace,
            output_file,
        } => {
            cli::dump(manager, &name, namespace, &output_file);
        }
        Command::Import {
            input_file,
            name,
            namespace,
        } => {
            cli::import(manager, &input_file, &name, namespace);
        }
    }
}
