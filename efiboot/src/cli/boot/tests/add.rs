use clap::Parser;
use efivar::{
    boot::{BootEntry, BootEntryAttributes, FilePath, FilePathList},
    efi::Variable,
    store::MemoryStore,
    utils, VarReader,
};

use crate::{
    cli::{boot::tests::standard_setup, Command},
    exit_code::ExitCode,
};

#[test]
fn add() {
    //! Test that the basic `efiboot boot add` subcommand works.
    //! Note: we are using the current partition, not specifying one

    let manager = &mut MemoryStore::new();

    let setup_entry = standard_setup(manager, 0x0001);

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
                hard_drive: setup_entry.file_path_list.unwrap().hard_drive // use partition defined earlier
            }),
            optional_data: vec![]
        }
    );

    // verify new boot order is right
    let (data, _) = manager.read(&Variable::new("BootOrder")).unwrap();
    assert_eq!(data, utils::u16_to_u8(&[0x0000, 0x0001]));
}

#[test]
fn add_set_id() {
    //! Use `efiboot boot add` with a given entry ID.

    let manager = &mut MemoryStore::new();

    let setup_entry = standard_setup(manager, 0x0001);

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
                "Some entry",
                "--id",
                "1000"
            ]),
            manager,
        )
    );

    // verify Boot0000 did not get inserted
    assert!(!manager.exists(&Variable::new("Boot0000")).unwrap());

    // verify inserted entry is right
    let (data, _) = manager.read(&Variable::new("Boot1000")).unwrap();
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
                hard_drive: setup_entry.file_path_list.unwrap().hard_drive // use partition defined earlier
            }),
            optional_data: vec![]
        }
    );

    // verify new boot order is right
    let (data, _) = manager.read(&Variable::new("BootOrder")).unwrap();
    assert_eq!(data, utils::u16_to_u8(&[0x1000, 0x0001]));
}

#[test]
fn add_on_existing() {
    //! Try to add a boot entry with an already-existing id

    let manager = &mut MemoryStore::new();

    let setup_entry = standard_setup(manager, 0x0001);

    let current_exe = std::env::current_exe()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    // execute `efiboot boot add`
    assert_eq!(
        ExitCode::FAILURE,
        crate::run(
            Command::parse_from([
                "efiboot",
                "boot",
                "add",
                "--file",
                &current_exe,
                "--description",
                "Some entry",
                "--id",
                "0001"
            ]),
            manager,
        )
    );

    // verify Boot0001 was not changed
    assert_eq!(
        BootEntry::read(manager, &Variable::new("Boot0001")).unwrap(),
        setup_entry
    );
}
