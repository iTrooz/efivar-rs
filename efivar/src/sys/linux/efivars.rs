use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;

use crate::efi::VariableFlags;
use crate::{Error, VarEnumerator, VarManager, VarReader, VarWriter};
use super::LinuxSystemManager;

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub const EFIVARS_ROOT: &'static str = "/sys/firmware/efi/efivars";

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
    fn get_var_names<'a>(&'a self) -> crate::Result<Box<dyn Iterator<Item = String> + 'a>> {
        fs::read_dir(EFIVARS_ROOT).map(|list| {
            list.filter_map(|result| {
                result
                    .map_err(|error| Error::UnknownIoError { error })
                    .and_then(|entry| {
                        entry
                            .file_name()
                            .into_string()
                            .map_err(|_str| Error::InvalidUTF8)
                    })
                    .ok()
            })
        })
        .map(|it| -> Box<dyn Iterator<Item = String>> { Box::new(it) })
        .map_err(|error| {
            // TODO: check for specific error types
            Error::UnknownIoError { error }
        })
    }
}

impl VarReader for SystemManager {
    fn read(&self, name: &str) -> crate::Result<(VariableFlags, Vec<u8>)> {
        // Filename to the matching efivarfs file for this variable
        let filename = format!("{}/{}", EFIVARS_ROOT, name);

        let mut f = File::open(filename)
            .map_err(|error| Error::for_variable(error, name.into()))?;

        // Read attributes
        let attr = f.read_u32::<LittleEndian>()
            .map_err(|error| Error::for_variable(error, name.into()))?;
        let attr = VariableFlags::from_bits(attr).unwrap_or(VariableFlags::empty());

        // Read variable contents
        let mut buf = Vec::new();
        f.read_to_end(&mut buf)
            .map_err(|error| Error::for_variable(error, name.into()))?;

        Ok((attr, buf))
    }
}

impl VarWriter for SystemManager {
    fn write(&mut self, name: &str, attributes: VariableFlags, value: &[u8]) -> crate::Result<()> {
        // Prepare attributes
        let attribute_bits = attributes.bits();

        // Prepare buffer (we must only write once)
        let mut buf = Vec::with_capacity(std::mem::size_of_val(&attribute_bits));

        // Write attributes
        buf.write_u32::<LittleEndian>(attribute_bits)
            .map_err(|error| Error::for_variable(error, name.into()))?;

        // Write variable contents
        buf.write(value)
            .map_err(|error| Error::for_variable(error, name.into()))?;

        // Filename to the matching efivarfs file for this variable
        let filename = format!("{}/{}", EFIVARS_ROOT, name);

        // Open file.
        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(filename)
            .map_err(|error| Error::for_variable(error, name.into()))?;

        // Write the value using a single write.
        f.write(&buf)
            .map_err(|error| Error::for_variable(error, name.into()))?;

        Ok(())
    }
}

impl VarManager for SystemManager {}
