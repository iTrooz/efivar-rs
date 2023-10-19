use std::str::FromStr;

use efivar::{
    boot::{
        BootEntry, BootEntryAttributes, EFIHardDrive, EFIHardDriveType, FilePath, FilePathList,
    },
    efi::{Variable, VariableFlags},
    store::MemoryStore,
    utils, VarManager, VarWriter,
};
use uuid::Uuid;

use crate::cli::boot::add::get_used_ids;

mod add;
mod delete;
mod enable_disable;
mod next;

fn add_entry(manager: &mut dyn VarManager, id: u16, enabled: bool) -> BootEntry {
    // define partition
    let hard_drive = EFIHardDrive {
        partition_number: 1,
        partition_start: 2,
        partition_size: 3,
        partition_sig: Uuid::from_str("62ca22b7-b071-4bc5-be1d-136a745e7c50").unwrap(),
        format: 5,
        sig_type: EFIHardDriveType::Gpt,
    };

    let entry = BootEntry {
        attributes: if enabled {
            BootEntryAttributes::LOAD_OPTION_ACTIVE
        } else {
            BootEntryAttributes::empty()
        },
        description: "".to_owned(),
        file_path_list: Some(FilePathList {
            file_path: FilePath {
                path: "somefile".into(),
            },
            hard_drive: hard_drive.clone(),
        }),
        optional_data: vec![],
    };

    manager.add_boot_entry(id, entry.clone()).unwrap();

    entry
}

fn standard_setup(manager: &mut dyn VarManager, id: u16) -> BootEntry {
    // add entry
    let entry = add_entry(manager, id, true);

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

    entry
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
