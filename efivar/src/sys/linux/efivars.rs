use std::fs;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::str::FromStr;

use super::LinuxSystemManager;
use crate::efi::{Variable, VariableFlags};
use crate::{Error, VarEnumerator, VarManager, VarReader, VarWriter};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};

pub const EFIVARS_ROOT: &str = "/sys/firmware/efi/efivars";

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
                list.filter_map(|result| {
                    result
                        .map_err(|error| Error::UnknownIoError { error })
                        .and_then(|entry| {
                            entry
                                .file_name()
                                .into_string()
                                .map_err(|_str| Error::InvalidUTF8)
                                .and_then(|s| Variable::from_str(&s))
                        })
                        .ok()
                })
            })
            .map(|it| -> Box<dyn Iterator<Item = Variable>> { Box::new(it) })
            .map_err(|error| {
                // TODO: check for specific error types
                Error::UnknownIoError { error }
            })
    }
}

impl VarReader for SystemManager {
    fn read(&self, var: &Variable) -> crate::Result<(Vec<u8>, VariableFlags)> {
        // Filename to the matching efivarfs file for this variable
        let filename = format!("{}/{}", EFIVARS_ROOT, var);

        let mut f = File::open(filename).map_err(|error| Error::for_variable(error, var))?;

        // Read attributes
        let attr = f
            .read_u32::<LittleEndian>()
            .map_err(|error| Error::for_variable(error, var))?;
        let attr = VariableFlags::from_bits(attr).unwrap_or(VariableFlags::empty());

        // Read variable contents
        let mut value: Vec<u8> = vec![];
        f.read_to_end(&mut value)
            .map_err(|error| Error::for_variable(error, var))?;

        Ok((value, attr))
    }
}

impl VarWriter for SystemManager {
    fn write(
        &mut self,
        var: &Variable,
        attributes: VariableFlags,
        value: &[u8],
    ) -> crate::Result<()> {
        // Prepare attributes
        let attribute_bits = attributes.bits();

        // Prepare buffer (we must only write once)
        let mut buf = Vec::with_capacity(std::mem::size_of_val(&attribute_bits));

        // Write attributes
        buf.write_u32::<LittleEndian>(attribute_bits)
            .map_err(|error| Error::for_variable(error, var))?;

        // Write variable contents
        buf.write(value)
            .map_err(|error| Error::for_variable(error, var))?;

        // Filename to the matching efivarfs file for this variable
        let filename = format!("{}/{}", EFIVARS_ROOT, var);

        // Open file.
        let mut f = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(filename)
            .map_err(|error| Error::for_variable(error, var))?;

        // Write the value using a single write.
        f.write(&buf)
            .map_err(|error| Error::for_variable(error, var))?;

        Ok(())
    }

    fn delete(&mut self, var: &Variable) -> crate::Result<()> {
        std::fs::remove_file(format!("{}/{}", EFIVARS_ROOT, var))
            .map_err(|error| Error::for_variable(error, var))
    }
}

impl VarManager for SystemManager {}
