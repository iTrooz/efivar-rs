mod cli;
pub mod exit_code;
pub mod id;

use cli::Command;
use efivar::VarManager;
use exit_code::ExitCode;
use std::path::PathBuf;
use structopt::StructOpt;

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

fn main() -> std::process::ExitCode {
    let opts = Opt::from_args();

    let manager = &mut *if let Some(filename) = opts.file_store {
        efivar::file_store(filename)
    } else {
        efivar::system()
    };

    run(opts.cmd, manager).into()
}

fn run(cmd: Command, manager: &mut dyn VarManager) -> ExitCode {
    cli::run(manager, cmd)
}
