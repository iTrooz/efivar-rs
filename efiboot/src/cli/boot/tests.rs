use std::str::FromStr;

use clap::Parser;
use efivar::{
    boot::{
        BootEntry, BootEntryAttributes, EFIHardDrive, EFIHardDriveType, FilePath, FilePathList,
    },
    efi::{Variable, VariableFlags},
    store::MemoryStore,
    utils, VarManager, VarReader, VarWriter,
};
use uuid::Uuid;

use crate::{cli::Command, exit_code::ExitCode};

use super::add::get_used_ids;

fn add_entry(manager: &mut dyn VarManager, id: u16) -> EFIHardDrive {
    // define partition
    let hard_drive = EFIHardDrive {
        partition_number: 1,
        partition_start: 2,
        partition_size: 3,
        partition_sig: Uuid::from_str("62ca22b7-b071-4bc5-be1d-136a745e7c50").unwrap(),
        format: 5,
        sig_type: EFIHardDriveType::Gpt,
    };

    manager
        .add_boot_entry(
            id,
            BootEntry {
                attributes: BootEntryAttributes::LOAD_OPTION_ACTIVE,
                description: "".to_owned(),
                file_path_list: Some(FilePathList {
                    file_path: FilePath {
                        path: "somefile".into(),
                    },
                    hard_drive: hard_drive.clone(),
                }),
                optional_data: vec![],
            },
        )
        .unwrap();

    hard_drive
}

fn standard_setup(manager: &mut dyn VarManager, id: u16) -> EFIHardDrive {
    // add entry
    let hard_drive = add_entry(manager, id);

    // set it as BootCurrent
    manager
        .write(
            &Variable::new("BootCurrent"),
            VariableFlags::default(),
            &utils::u16_to_u8(&[id]),
        )
        .unwrap();

    // define BootOrder
    manager
        .write(
            &Variable::new("BootOrder"),
            VariableFlags::default(),
            &utils::u16_to_u8(&[id]),
        )
        .unwrap();

    hard_drive
}

#[test]
fn add_on_current_partition() {
    //! Test that the basic `efiboot boot add` subcommand works.

    let manager = &mut MemoryStore::new();

    let hard_drive = standard_setup(manager, 0x0001);

    let current_exe = std::env::current_exe()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    // execute `efiboot boot add`
    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from([
                "efiboot",
                "boot",
                "add",
                "--file",
                &current_exe,
                "--description",
                "Some entry"
            ]),
            manager,
        )
    );

    // verify inserted entry is right
    let (data, _) = manager.read(&Variable::new("Boot0000")).unwrap();
    let entry = BootEntry::parse(data).unwrap();
    assert_eq!(
        entry,
        BootEntry {
            attributes: BootEntryAttributes::LOAD_OPTION_ACTIVE,
            description: "Some entry".to_owned(),
            file_path_list: Some(FilePathList {
                file_path: FilePath {
                    path: current_exe.into()
                },
                hard_drive // use partition defined earlier
            }),
            optional_data: vec![]
        }
    );

    // verify new boot order is right
    let (data, _) = manager.read(&Variable::new("BootOrder")).unwrap();
    assert_eq!(data, utils::u16_to_u8(&[0x0000, 0x0001]));
}

#[test]
fn get_used_boot_ids() {
    let manager = &mut MemoryStore::new();

    manager
        .write(&Variable::new("Boot0001"), VariableFlags::default(), &[])
        .unwrap();
    manager
        .write(&Variable::new("Boot1000"), VariableFlags::default(), &[])
        .unwrap();
    manager
        .write(&Variable::new("Boot0500"), VariableFlags::default(), &[])
        .unwrap();
    manager
        .write(&Variable::new("BootFFFF"), VariableFlags::default(), &[])
        .unwrap();

    let mut used_ids = get_used_ids(manager);
    used_ids.sort();

    assert_eq!(used_ids, vec![0x0001, 0x0500, 0x1000, 0xFFFF]);
}

#[test]
fn set_next() {
    let manager = &mut MemoryStore::new();

    add_entry(manager, 0x0001);

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

#[test]
fn delete() {
    let manager = &mut MemoryStore::new();

    standard_setup(manager, 0x0001);

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efiboot", "boot", "delete", "1"]),
            manager,
        )
    );

    assert!(!manager.exists(&Variable::new("Boot0001")).unwrap());
}
