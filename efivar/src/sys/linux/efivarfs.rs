//! efivarfs is the new interface to access EFI variables

use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

use super::LinuxSystemManager;
use crate::efi::{Variable, VariableFlags};
use crate::push::PushVecU8;
use crate::{Error, VarEnumerator, VarManager, VarReader, VarWriter};

use byteorder::{LittleEndian, ReadBytesExt};

pub const EFIVARFS_ROOT: &str = "/sys/firmware/efi/efivars";

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
    fn get_all_vars<'a>(&'a self) -> crate::Result<Box<dyn Iterator<Item = Variable> + 'a>> {
        fs::read_dir(EFIVARFS_ROOT)
            .map(|list| {
                list.filter_map(|result| {
                    result
                        .map_err(Error::UnknownIoError)
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
            .map_err(Error::UnknownIoError) // TODO: check for specific error types
    }
}

impl VarReader for SystemManager {
    fn read(&self, var: &Variable) -> crate::Result<(Vec<u8>, VariableFlags)> {
        // Filename to the matching efivarfs file for this variable
        let filename = format!("{}/{}", EFIVARFS_ROOT, var);

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
        buf.push_u32(attribute_bits);

        // Write variable contents
        buf.append(&mut value.to_vec());

        // Filename to the matching efivarfs file for this variable
        let filename = format!("{}/{}", EFIVARFS_ROOT, var);

        // Open file read only to get FD for flags operations.
        let f = File::open(&filename).map_err(|error| Error::for_variable(error, var))?;

        // Read original flags.
        let orig_flags = rustix::fs::ioctl_getflags(&f)
            .map_err(|error| Error::for_variable(error.into(), var))?;

        // If Immutable flag is present, remove it.
        let immut_flag_removed = if orig_flags.contains(rustix::fs::IFlags::IMMUTABLE) {
            // IFlags doesn't implement Clone, so cycle through bits.
            let mut flags = rustix::fs::IFlags::from_bits(orig_flags.bits()).unwrap();

            flags.remove(rustix::fs::IFlags::IMMUTABLE);
            rustix::fs::ioctl_setflags(&f, flags)
                .map_err(|error| Error::for_variable(error.into(), var))?;

            true
        } else {
            false
        };

        // Close file before re-opening for write.
        drop(f);

        // Open file.
        let mut f = File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(filename)
            .map_err(|error| Error::for_variable(error, var))?;

        // Write the value using a single write.
        f.write(&buf)
            .map_err(|error| Error::for_variable(error, var))?;

        if immut_flag_removed {
            // Add back the Immutable flag.
            rustix::fs::ioctl_setflags(&f, orig_flags)
                .map_err(|error| Error::for_variable(error.into(), var))?;
        }

        Ok(())
    }

    fn delete(&mut self, var: &Variable) -> crate::Result<()> {
        std::fs::remove_file(format!("{}/{}", EFIVARFS_ROOT, var))
            .map_err(|error| Error::for_variable(error, var))
    }
}

impl VarManager for SystemManager {}
