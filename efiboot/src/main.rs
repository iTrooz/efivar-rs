extern crate efiboot;
extern crate efivar;
use efiboot::cli;

extern crate clap;
use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("file_store")
                .long("file-store")
                .short("f")
                .takes_value(true)
                .value_name("FILE")
                .help("TOML file to use for variable storage instead of the system"),
        )
        .subcommand(
            SubCommand::with_name("read")
                .about("Read the value of a variable")
                .arg(
                    Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .required(true)
                        .takes_value(true)
                        .value_name("VARIABLE")
                        .help("Name of the variable to read"),
                )
                .arg(
                    Arg::with_name("string")
                        .long("string")
                        .short("s")
                        .help("Print the value as a UTF-8 string"),
                ),
        )
        .subcommand(SubCommand::with_name("list").about("List known EFI variables"))
        .get_matches();

    let manager = if let Some(filename) = matches.value_of("file_store") {
        efivar::file_store(filename)
    } else {
        efivar::system()
    };

    if let Some(matches) = matches.subcommand_matches("read") {
        let name = efivar::efi::to_fullname(matches.value_of("name").unwrap());
        let as_string = matches.is_present("string");

        cli::read(manager, &name, as_string);
    } else if let Some(_matches) = matches.subcommand_matches("list") {
        cli::list(manager);
    }
}
