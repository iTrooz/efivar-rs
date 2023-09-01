//! This module hands everything related to the 'boot add' subcommand

mod disk;

use efivar::{
    boot::{BootEntry, BootEntryAttributes, FilePath, FilePathList},
    VarManager,
};
use itertools::Itertools;

/// get a boot entry ID that isnt used
fn get_used_ids(manager: &dyn VarManager) -> Vec<u16> {
    manager
        .get_all_vars()
        .unwrap()
        .filter(|var| var.vendor().is_efi())
        .filter_map(|var| var.boot_var_id())
        .collect_vec()
}

/// check if a partition+file is valid (exists), if the partition is mounted
fn check(partition: &str, file: &str) -> bool {
    if let Some(mount_point) = disk::get_mount_point(partition) {
        eprintln!(
            "Partition {} is mounted on {}. Verifying file location {file} is valid",
            partition,
            mount_point.display()
        );
        let full_path = mount_point.join(file);
        if let Ok(md) = std::fs::metadata(&full_path) {
            if md.is_file() {
                eprintln!("File location is valid");
                return true;
            }
        }
        eprintln!("{} is not a valid file", full_path.display());
        false
    } else {
        true
    }
}

pub fn add(
    mut manager: Box<dyn VarManager>,
    partition: String,
    file_path: String,
    description: String,
    force: bool,
    id: Option<u16>,
) {
    // do not continue is the file has been identified as non-existant
    if !force && !check(&partition, &file_path) {
        return;
    }

    // get necessary information from the partition to create an entry
    let efi_partition = disk::retrieve_efi_partition_data(&partition);

    let file_path_list = FilePathList {
        hard_drive: efi_partition,
        file_path: FilePath {
            path: file_path.into(),
        },
    };

    // create boot entry
    let entry = BootEntry {
        attributes: BootEntryAttributes::LOAD_OPTION_ACTIVE,
        description,
        file_path_list: Some(file_path_list),
        optional_data: vec![],
    };

    // assign the boot entry an id
    let id: u16 = {
        let used_boot_ids = get_used_ids(&*manager);
        if let Some(id) = id {
            if used_boot_ids.contains(&id) {
                eprintln!("Boot entry with id {id:04X} already exists. Delete it first");
                return;
            }
            id
        } else {
            let id = (0x0000..0xFFFF)
                .find(|&i| !used_boot_ids.contains(&i))
                .unwrap();
            println!("Chose id {id:04X} for boot entry");
            id
        }
    };

    // create the entry
    manager.add_boot_entry(id, entry).unwrap();

    // add it to boot order
    let mut ids = manager.get_boot_order().unwrap();
    ids.insert(0, id);
    manager.set_boot_order(ids).unwrap();

    println!("Added entry with success");
}
