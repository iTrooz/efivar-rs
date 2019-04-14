use std::fs;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::io::{Error, ErrorKind, BufReader, BufWriter};
use std::str::FromStr;

use crate::efi::VariableFlags;
use crate::{VarEnumerator, VarManager, VarReader, VarWriter};
use super::LinuxSystemManager;

pub const EFIVARFS_ROOT: &'static str = "/sys/firmware/efi/vars";

pub struct SystemManager;

impl SystemManager {
    pub fn new() -> SystemManager {
        SystemManager {}
    }
}

impl LinuxSystemManager for SystemManager {
    #[cfg(test)]
    fn supported(&self) -> bool {
        fs::metadata(EFIVARFS_ROOT).is_ok()
    }
}

impl VarEnumerator for SystemManager {
    fn get_var_names(&self) -> io::Result<Vec<String>> {
        fs::read_dir(EFIVARFS_ROOT).map(|list| {
            list.filter_map(Result::ok)
                .filter(|ref entry| match entry.file_type() {
                    Ok(file_type) => file_type.is_dir(),
                    _ => false,
                })
                .filter_map(|entry| {
                    entry
                        .file_name()
                        .into_string()
                        .map_err(|_str| {
                            Error::new(ErrorKind::Other, "Failed to decode filename as valid UTF-8")
                        })
                        .ok()
                })
                .collect()
        })
    }
}

impl VarReader for SystemManager {
    fn read(&self, name: &str) -> io::Result<(VariableFlags, Vec<u8>)> {
        // Path to the attributes file
        let attributes_filename = format!("{}/{}/attributes", EFIVARFS_ROOT, name);

        // Open attributes file
        let f = File::open(attributes_filename)?;
        let reader = BufReader::new(&f);

        let mut flags = VariableFlags::empty();
        for line in reader.lines() {
            let line = line?;
            let parsed = VariableFlags::from_str(&line)?;
            flags = flags | parsed;
        }

        // Filename to the matching efivarfs data for this variable
        let filename = format!("{}/{}/data", EFIVARFS_ROOT, name);

        let mut f = File::open(filename)?;

        // Read variable contents
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)?;

        Ok((flags, buf))
    }
}

impl VarWriter for SystemManager {
    fn write(&mut self, name: &str, attributes: VariableFlags, value: &[u8]) -> io::Result<()> {
        // Path to the attributes file
        let attributes_filename = format!("{}/{}/attributes", EFIVARFS_ROOT, name);
        // Open attributes file
        let mut f = File::open(attributes_filename)?;
        let mut writer = BufWriter::new(&mut f);

        // Write attributes
        writer.write_all(attributes.to_string().as_bytes())?;

        // Filename to the matching efivarfs file for this variable
        let filename = format!("{}/{}/data", EFIVARFS_ROOT, name);

        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(filename)?;

        // Write variable contents
        f.write(value)?;

        Ok(())
    }
}

impl VarManager for SystemManager {}
