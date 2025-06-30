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

fn setup_logging() {
    let mut builder =
        &mut env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"));
    if std::env::var("RUST_LOG").is_err() && std::env::var("VERBOSE").is_err() {
        // Simplify log format
        builder = builder.format_timestamp(None).format_target(false);
    }
    builder.init();

    log::debug!("Debug logging enabled");
}

fn main() -> std::process::ExitCode {
    setup_logging();

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
