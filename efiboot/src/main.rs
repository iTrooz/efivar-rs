mod cli;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
enum BootCommand {
    /// Get current boot order IDs. See get-entries to get boot entries information
    GetOrder,
}

#[derive(StructOpt)]
enum Command {
    /// Read the value of a variable
    Read {
        /// Name of the variable to read
        #[structopt(short, long, value_name = "VARIABLE")]
        name: String,

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

    /// Manage boot-related variables
    Boot(BootCommand),
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
        efivar::file_store_std(filename)
    } else {
        efivar::system()
    };

    match opts.cmd {
        Command::Read { name, string } => {
            cli::read(manager, &name, string);
        }
        Command::List { namespace, all } => {
            cli::list(manager, namespace, all);
        }
        Command::Boot(arg) => match arg {
            BootCommand::GetOrder => {
                cli::get_boot_order(manager);
            }
        },
    }
}
