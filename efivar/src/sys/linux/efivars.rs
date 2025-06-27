//! efivars sysfs is the old, disabled-by-default, filesystem to access EFI variables

use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::str::FromStr;

use super::LinuxSystemManager;
use crate::efi::{Variable, VariableFlags};
use crate::{Error, VarEnumerator, VarManager, VarReader, VarWriter};

pub const EFIVARS_ROOT: &str = "/sys/firmware/efi/vars";

pub struct SystemManager;

impl SystemManager {
    pub fn new() -> SystemManager {
        SystemManager {}
    }
}

impl LinuxSystemManager for SystemManager {
    #[cfg(test)]
    fn supported(&self) -> bool {
        fs::metadata(EFIVARS_ROOT).is_ok()
    }
}

impl VarEnumerator for SystemManager {
    fn get_all_vars<'a>(&'a self) -> crate::Result<Box<dyn Iterator<Item = Variable> + 'a>> {
        fs::read_dir(EFIVARS_ROOT)
            .map(|list| {
                list.filter_map(Result::ok)
                    .filter(|entry| match entry.file_type() {
                        Ok(file_type) => file_type.is_dir(),
                        _ => false,
                    })
                    .filter_map(|entry| {
                        entry
                            .file_name()
                            .into_string()
                            .map_err(|_str| Error::InvalidUTF8)
                            .and_then(|s| Variable::from_str(&s))
                            .ok()
                    })
            })
            .map(|it| -> Box<dyn Iterator<Item = Variable>> { Box::new(it) })
            .map_err(Error::UnknownIoError) // TODO: check for specific error types
    }
}

impl VarReader for SystemManager {
    fn read(&self, var: &Variable) -> crate::Result<(Vec<u8>, VariableFlags)> {
        // Path to the attributes file
        let attributes_filename = format!("{EFIVARS_ROOT}/{var}/attributes");

        // Open attributes file
        let f = File::open(attributes_filename).map_err(|error| Error::for_variable(error, var))?;
        let reader = BufReader::new(&f);

        let mut flags = VariableFlags::empty();
        for line in reader.lines() {
            let line = line.map_err(|error| Error::for_variable(error, var))?;
            let parsed = VariableFlags::from_str(&line)?;
            flags |= parsed;
        }

        // Filename to the matching data for this variable
        let filename = format!("{EFIVARS_ROOT}/{var}/data");

        let mut f = File::open(filename).map_err(|error| Error::for_variable(error, var))?;

        // Read variable contents
        let mut value: Vec<u8> = vec![];
        f.read_to_end(&mut value)
            .map_err(|error| Error::for_variable(error, var))?;

        Ok((value, flags))
    }
}

impl VarWriter for SystemManager {
    fn write(
        &mut self,
        var: &Variable,
        attributes: VariableFlags,
        value: &[u8],
    ) -> crate::Result<()> {
        // Path to the attributes file
        let attributes_filename = format!("{EFIVARS_ROOT}/{var}/attributes");
        // Open attributes file
        let mut f =
            File::open(attributes_filename).map_err(|error| Error::for_variable(error, var))?;
        let mut writer = BufWriter::new(&mut f);

        // Write attributes
        writer
            .write_all(attributes.to_string().as_bytes())
            .map_err(|error| Error::for_variable(error, var))?;

        // Filename to the matching file for this variable
        let filename = format!("{EFIVARS_ROOT}/{var}/data");

        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(filename)
            .map_err(|error| Error::for_variable(error, var))?;

        // Write variable contents
        f.write(value)
            .map_err(|error| Error::for_variable(error, var))?;

        Ok(())
    }

    fn delete(&mut self, _var: &Variable) -> crate::Result<()> {
        // Unimplemented because I wasn't able to enable efivars sysfs on my system
        unimplemented!("Variable deletion not supported on efivars sysfs. See https://github.com/iTrooz/efivar-rs/issues/55");
    }
}

impl VarManager for SystemManager {}
