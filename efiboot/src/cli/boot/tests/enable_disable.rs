use clap::Parser;
use efivar::{
    boot::{BootEntry, BootEntryAttributes},
    efi::Variable,
    store::MemoryStore,
};

use crate::{
    cli::{boot::tests::add_entry, Command},
    exit_code::ExitCode,
};

#[test]
fn enable() {
    let manager = &mut MemoryStore::new();

    let mut orig_entry = add_entry(manager, 0x0001, false);

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efiboot", "boot", "enable", "0001",]),
            manager,
        )
    );

    orig_entry.attributes = BootEntryAttributes::LOAD_OPTION_ACTIVE;

    let new_entry = BootEntry::read(manager, &Variable::new("Boot0001")).unwrap();

    assert_eq!(orig_entry, new_entry);
}

#[test]
fn enable_enabled() {
    let manager = &mut MemoryStore::new();

    let orig_entry = add_entry(manager, 0x0001, true);

    assert_eq!(
        ExitCode::FAILURE,
        crate::run(
            Command::parse_from(["efiboot", "boot", "enable", "0001",]),
            manager,
        )
    );

    // verify variable did not change
    assert_eq!(
        orig_entry,
        BootEntry::read(manager, &Variable::new("Boot0001")).unwrap()
    );
}

#[test]
fn disable() {
    let manager = &mut MemoryStore::new();

    let mut orig_entry = add_entry(manager, 0x0001, true);

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efiboot", "boot", "disable", "0001",]),
            manager,
        )
    );

    orig_entry.attributes = BootEntryAttributes::empty();

    let new_entry = BootEntry::read(manager, &Variable::new("Boot0001")).unwrap();

    assert_eq!(orig_entry, new_entry);
}

#[test]
fn disable_disabled() {
    let manager = &mut MemoryStore::new();

    let orig_entry = add_entry(manager, 0x0001, false);

    assert_eq!(
        ExitCode::FAILURE,
        crate::run(
            Command::parse_from(["efiboot", "boot", "disable", "0001",]),
            manager,
        )
    );

    assert_eq!(
        orig_entry,
        BootEntry::read(manager, &Variable::new("Boot0001")).unwrap()
    );
}
