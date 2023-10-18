use efivar::{
    efi::{Variable, VariableFlags},
    store::MemoryStore,
    utils, VarReader, VarWriter,
};

use crate::cli::Command;

pub use super::*;

#[test]
fn test_boot_order_str() {
    assert_eq!(boot_order_str(&[0x0001, 0x2000, 0xBEEF]), "0001 2000 BEEF");
}

#[test]
fn get_order() {
    let manager = &mut MemoryStore::new();

    manager
        .write(
            &Variable::new("BootOrder"),
            VariableFlags::default(),
            &utils::u16_to_u8(&[0x0001, 0x0002, 0x0003, 0x0004]),
        )
        .unwrap();

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efiboot", "boot", "order", "get"]),
            manager,
        )
    );
}

#[test]
fn set_order() {
    let manager = &mut MemoryStore::new();

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efiboot", "boot", "order", "set", "0001", "2", "1000", "500"]),
            manager,
        )
    );

    let (data, _) = manager.read(&Variable::new("BootOrder")).unwrap();

    assert_eq!(data, utils::u16_to_u8(&[0x0001, 0x0002, 0x1000, 0x0500]));
}

#[test]
fn add_to_order() {
    let manager = &mut MemoryStore::new();

    manager
        .write(
            &Variable::new("BootOrder"),
            VariableFlags::default(),
            &utils::u16_to_u8(&[0x0002, 0x1000]),
        )
        .unwrap();

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efiboot", "boot", "order", "add", "1", "--position", "0"]),
            manager,
        )
    );

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efiboot", "boot", "order", "add", "500"]),
            manager,
        )
    );

    let (data, _) = manager.read(&Variable::new("BootOrder")).unwrap();

    assert_eq!(data, utils::u16_to_u8(&[0x0001, 0x0002, 0x1000, 0x0500]));
}

#[test]
fn remove_from_order() {
    let manager = &mut MemoryStore::new();

    manager
        .write(
            &Variable::new("BootOrder"),
            VariableFlags::default(),
            &utils::u16_to_u8(&[0x0001, 0x0002, 0x1000, 0x0500]),
        )
        .unwrap();

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efiboot", "boot", "order", "remove", "1000"]),
            manager,
        )
    );

    let (data, _) = manager.read(&Variable::new("BootOrder")).unwrap();

    assert_eq!(data, utils::u16_to_u8(&[0x0001, 0x0002, 0x0500]));
}

#[test]
fn remove_inexistent_from_order() {
    let manager = &mut MemoryStore::new();

    manager
        .write(
            &Variable::new("BootOrder"),
            VariableFlags::default(),
            &utils::u16_to_u8(&[0x0001, 0x0002, 0x1000, 0x0500]),
        )
        .unwrap();

    assert_eq!(
        ExitCode::FAILURE,
        crate::run(
            Command::parse_from(["efiboot", "boot", "order", "remove", "2000"]),
            manager,
        )
    );
}
