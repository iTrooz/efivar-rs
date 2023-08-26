use core::panic;
use std::{io::BufRead, path::PathBuf, process::Command};

use efivar::boot::{EFIHardDrive, EFIHardDriveType};
use itertools::Itertools;

/// get the partition UUID from its name
/// * `name`: the name of the partition, e.g. '/dev/sda1'
fn get_partition_uuid(name: &str) -> uuid::Uuid {
    let output = Command::new("blkid").output().unwrap().stdout;

    if output.is_empty() {
        panic!("No output");
    }

    for line in output.lines() {
        let line = line.unwrap();
        let (part_name, data) = line.split_once(": ").unwrap();

        // continue to the next one if its not the right name
        if part_name != name {
            continue;
        };

        // extract the 'PARTUUID' key from the line
        for pair in data.split(' ') {
            let (key, value) = pair.split_once('=').unwrap();
            if key == "PARTUUID" {
                let value = value.trim_matches('"');
                return uuid::Uuid::parse_str(value).unwrap();
            }
        }

        break;
    }

    panic!("No partition found");
}

/// Partition names are in the form '/dev/sda1', so just take the number at the end
fn get_partition_number(name: &str) -> u32 {
    name.chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .collect_vec()
        .into_iter()
        .rev()
        .collect::<String>()
        .parse::<u32>()
        .unwrap()
}

/// get partitition start and size
fn get_partition_location(name: &str) -> (u64, u64) {
    let stripped_name = name.strip_prefix("/dev/").unwrap();
    let start = std::fs::read_to_string(format!("/sys/class/block/{stripped_name}/start"))
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap();
    let size = std::fs::read_to_string(format!("/sys/class/block/{stripped_name}/size"))
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap();

    (start, size)
}

/// retrieve data needed to generate a EFIHardDrive from the system, from a friendly name of the partition
pub fn retrieve_efi_partition_data(name: &str) -> EFIHardDrive {
    let partition_sig = get_partition_uuid(name);
    let partition_number = get_partition_number(name);
    let (partition_start, partition_size) = get_partition_location(name);

    EFIHardDrive {
        partition_number,
        partition_start,
        partition_size,
        partition_sig,
        format: 0x02, // no idea
        sig_type: EFIHardDriveType::Gpt,
    }
}

pub fn get_mount_point(name: &str) -> Option<PathBuf> {
    for line in std::fs::read_to_string("/proc/mounts").unwrap().lines() {
        let mut iter = line.splitn(3, ' ');
        let partition_name = iter.next().unwrap();
        let mount_point = iter.next().unwrap();

        if partition_name == name {
            return Some(mount_point.into());
        }
    }

    None
}
