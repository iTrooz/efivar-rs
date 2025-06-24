use core::panic;
use std::{io::BufRead, path::PathBuf, process::Command};

use efivar::boot::{EFIHardDrive, EFIHardDriveType};
use itertools::Itertools;

/// Absolute name of a partition, should be enough to uniquely identify it
/// e.g. '/dev/sda1' on Linux
pub type Partition = String;

pub fn query_partition(disk: Option<String>, partition: String) -> Result<Partition, ()> {
    let abs_partition = partition.starts_with("/dev/"); // check if partition is absolute
    match (abs_partition, disk) {
        // if the partition is absolute, it should not have a disk specified
        (true, Some(_)) => Err(()),
        (true, None) => Ok(partition),
        (false, Some(disk)) => {
            if disk.starts_with("/dev/") {
                let disk = disk.trim_end_matches('/');
                // if the disk is absolute, append the partition to it
                Ok(format!("{disk}{partition}"))
            } else {
                // Invalid disk
                Err(())
            }
        }
        // if the partition is relative, it should have a disk specified
        (false, None) => Err(()),
    }
}

/// get the partition UUID from its name
/// * `name`: the name of the partition, e.g. '/dev/sda1'
fn get_partition_uuid(name: &str) -> Option<uuid::Uuid> {
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
                return Some(uuid::Uuid::parse_str(value).unwrap());
            }
        }

        break;
    }

    None
}

/// Partition names are in the form '/dev/sda1', so just take the number at the end
fn get_partition_number(name: &str) -> Option<u32> {
    name.chars()
        .rev()
        .take_while(|c| c.is_ascii_digit())
        .collect_vec()
        .into_iter()
        .rev()
        .collect::<String>()
        .parse::<u32>()
        .ok()
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
    let partition_sig = get_partition_uuid(name).unwrap();
    let partition_number = get_partition_number(name).unwrap();
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn get_real_partition() -> (String, String) {
        //! Returns an actual partition existing on the system that runs it
        //! Used for tests because we can't easily simulate partitions externally
        for line in std::fs::read_to_string("/proc/mounts").unwrap().lines() {
            let mut iter = line.splitn(3, ' ');
            let partition_name = iter.next().unwrap();
            let mount_point = iter.next().unwrap();
            if partition_name.starts_with("/dev/")
                && partition_name.ends_with(|c: char| char::is_ascii_digit(&c))
            {
                println!("found partition {partition_name} with mount point {mount_point} in system. It will be used for this test");
                return (partition_name.to_owned(), mount_point.to_owned());
            }
        }
        core::panic!("Could not find a valid partition in system. Some tests will fail");
    }

    #[test]
    fn real_mount_point() {
        let (part_name, mount_point) = get_real_partition();
        assert_eq!(
            get_mount_point(&part_name).unwrap(),
            PathBuf::from(mount_point)
        );
    }

    #[test]
    fn inexistent_mount_point() {
        assert_eq!(get_mount_point("heythere"), None);
    }

    #[test]
    fn partition_number() {
        assert_eq!(get_partition_number("/dev/sda1"), Some(1));
        assert_eq!(get_partition_number("/dev/sda13"), Some(13));
        assert_eq!(get_partition_number("/dev/sda"), None);
    }
}
