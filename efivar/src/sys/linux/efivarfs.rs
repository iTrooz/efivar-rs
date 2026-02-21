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
use rustix::fs::IFlags;

pub const EFIVARFS_ROOT: &str = "/sys/firmware/efi/efivars";

pub struct SystemManager;

impl SystemManager {
    pub fn new() -> SystemManager {
        SystemManager {}
    }
}

fn remove_immutable(filename: &str, var: &Variable) -> crate::Result<Option<IFlags>> {
    let f = match File::open(filename) {
        Ok(f) => f,
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(err) => return Err(Error::for_variable(err, var)),
    };

    let orig_flags =
        rustix::fs::ioctl_getflags(&f).map_err(|e| Error::for_variable(e.into(), var))?;

    if orig_flags.contains(IFlags::IMMUTABLE) {
        log::trace!("Removing IMMUTABLE flag from {filename}");
        let mut new_flags = IFlags::from_bits(orig_flags.bits()).unwrap();
        new_flags.remove(IFlags::IMMUTABLE);
        rustix::fs::ioctl_setflags(&f, new_flags)
            .map_err(|e| Error::for_variable(e.into(), var))?;
        Ok(Some(orig_flags))
    } else {
        Ok(None)
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
        log::trace!("efivarfs: Reading EFI variable {var}");
        // Filename to the matching efivarfs file for this variable
        let filename = format!("{EFIVARFS_ROOT}/{var}");

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

        log::debug!(
            "efivarfs: Read variable {var} with attributes {attr:?} (value length: {})",
            value.len()
        );

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
        log::trace!("efivarfs: Writing EFI variable {var} with attributes {attributes:?} and value length {}", value.len());
        // Filename to the matching efivarfs file for this variable
        let filename = format!("{EFIVARFS_ROOT}/{var}");

        let file_flags = remove_immutable(&filename, var)?;

        // Open file for write
        let mut f = File::options()
            .write(true)
            .truncate(true)
            .create(true)
            .open(filename)
            .map_err(|error| Error::for_variable(error, var))?;

        // Prepare data (attributes + variable data)
        let attribute_bits = attributes.bits();
        let mut buf = Vec::with_capacity(std::mem::size_of_val(&attribute_bits));
        buf.push_u32(attribute_bits);
        buf.append(&mut value.to_vec());

        // Write the value using a single write.
        f.write(&buf)
            .map_err(|error| Error::for_variable(error, var))?;

        // Potentially add back the Immutable flag.
        if let Some(orig_flags) = file_flags {
            log::trace!("Restoring original flags for variable {var}");
            rustix::fs::ioctl_setflags(&f, orig_flags)
                .map_err(|error| Error::for_variable(error.into(), var))?;
        }

        log::debug!(
            "efivarfs: Wrote variable {var} with attributes {attributes:?} (value length: {})",
            value.len()
        );
        Ok(())
    }

    fn delete(&mut self, var: &Variable) -> crate::Result<()> {
        log::trace!("efivarfs: Deleting EFI variable {var}");
        let filename = format!("{EFIVARFS_ROOT}/{var}");

        remove_immutable(&filename, var)?;

        std::fs::remove_file(&filename).map_err(|error| Error::for_variable(error, var))?;

        log::debug!("efivarfs: Deleted variable {var}");
        Ok(())
    }
}

impl VarManager for SystemManager {}
