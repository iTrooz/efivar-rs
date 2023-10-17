mod cli;
pub mod id;

use cli::Command;
use std::{path::PathBuf, process::ExitCode};
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

fn main() -> ExitCode {
    let opts = Opt::from_args();
    let manager = if let Some(filename) = opts.file_store {
        efivar::file_store(filename)
    } else {
        efivar::system()
    };

    cli::run(manager, opts.cmd)
}
