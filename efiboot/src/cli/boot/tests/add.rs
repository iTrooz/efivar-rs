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
