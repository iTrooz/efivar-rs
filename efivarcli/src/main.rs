mod cli;
pub mod exit_code;
pub mod id;

use clap::Parser;
use cli::Command;
use efivar::VarManager;
use exit_code::ExitCode;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = env!("CARGO_PKG_NAME"), author, about, version, long_about = None)]
struct Opt {
    /// TOML file to use for variable storage instead of the system
    #[arg(short, long, value_name = "FILE", env = "EFIBOOT_STORE")]
    file_store: Option<PathBuf>,

    #[command(subcommand)]
    cmd: Command,
}

fn main() -> std::process::ExitCode {
    let opts = Opt::parse();

    let manager = &mut *if let Some(filename) = opts.file_store {
        efivar::file_store(filename)
    } else {
        efivar::system().expect("Failed to instanciate variable manager")
    };

    run(opts.cmd, manager).into()
}

fn run(cmd: Command, manager: &mut dyn VarManager) -> ExitCode {
    cli::run(manager, cmd)
}
