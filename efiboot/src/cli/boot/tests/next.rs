use clap::Parser;
use efivar::{efi::Variable, store::MemoryStore, utils, VarReader};

use crate::{
    cli::{boot::tests::add_entry, Command},
    exit_code::ExitCode,
};

#[test]
fn set_next() {
    let manager = &mut MemoryStore::new();

    add_entry(manager, 0x0001, true);

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efiboot", "boot", "next", "set", "0001",]),
            manager,
        )
    );

    let (data, _) = manager.read(&Variable::new("BootNext")).unwrap();
    assert_eq!(data, utils::u16_to_u8(&[0x0001]));
}

#[test]
fn set_inexistent_next() {
    let manager = &mut MemoryStore::new();

    assert_eq!(
        ExitCode::FAILURE,
        crate::run(
            Command::parse_from(["efiboot", "boot", "next", "set", "0001",]),
            manager,
        )
    );

    assert!(!manager.exists(&Variable::new("BootNext")).unwrap());
}
