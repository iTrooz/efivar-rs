use clap::Parser;
use efivar::{
    efi::{Variable, VariableFlags},
    store::MemoryStore,
    test_utils::assert_var_not_found,
    utils, VarReader, VarWriter,
};

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
            Command::parse_from(["efivarcli", "boot", "next", "set", "0001",]),
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
            Command::parse_from(["efivarcli", "boot", "next", "set", "0001",]),
            manager,
        )
    );

    assert_var_not_found(manager, &Variable::new("BootNext"));
}

#[test]
fn unset_next() {
    let manager = &mut MemoryStore::new();

    manager
        .write(
            &Variable::new("BootNext"),
            VariableFlags::default(),
            &utils::u16_to_u8(&[0x0001]),
        )
        .unwrap();

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efivarcli", "boot", "next", "unset"]),
            manager,
        )
    );

    assert_var_not_found(manager, &Variable::new("BootNext"));
}

#[test]
fn unset_inexistent_next() {
    let manager = &mut MemoryStore::new();

    assert_eq!(
        ExitCode::FAILURE,
        crate::run(
            Command::parse_from(["efivarcli", "boot", "next", "unset"]),
            manager,
        )
    );

    assert_var_not_found(manager, &Variable::new("BootNext"));
}

#[test]
fn get_next() {
    let manager = &mut MemoryStore::new();

    manager
        .write(
            &Variable::new("BootNext"),
            VariableFlags::default(),
            &utils::u16_to_u8(&[0x0001]),
        )
        .unwrap();

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efivarcli", "boot", "next", "get"]),
            manager,
        )
    );
}

#[test]
fn get_inexistent_next() {
    let manager = &mut MemoryStore::new();

    assert_eq!(
        ExitCode::FAILURE,
        crate::run(
            Command::parse_from(["efivarcli", "boot", "next", "get"]),
            manager,
        )
    );
}
