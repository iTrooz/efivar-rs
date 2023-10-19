use clap::Parser;
use efivar::store::MemoryStore;

use crate::{
    cli::{
        boot::tests::{add_entry, standard_setup},
        Command,
    },
    exit_code::ExitCode,
};

#[test]
fn get_entries() {
    let manager = &mut MemoryStore::new();

    standard_setup(manager, 0x0001);

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efiboot", "boot", "get-entries"]),
            manager,
        )
    );
}

#[test]
fn get_entries_verbose() {
    let manager = &mut MemoryStore::new();

    standard_setup(manager, 0x0001);

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efiboot", "boot", "get-entries", "-v"]),
            manager,
        )
    );
}

#[test]
fn get_entries_not_in_bootorder() {
    let manager = &mut MemoryStore::new();

    standard_setup(manager, 0x0001);
    add_entry(manager, 0x0002, true);

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efiboot", "boot", "get-entries"]),
            manager,
        )
    );
}
