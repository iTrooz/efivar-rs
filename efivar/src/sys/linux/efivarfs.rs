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
        log::debug!("efivarfs: Writing EFI variable {var}");
        // Filename to the matching efivarfs file for this variable
        let filename = format!("{EFIVARFS_ROOT}/{var}");

        // handle immutable file attribute. file_flags is some if the flag was removed and need to be set again
        let file_flags: Option<IFlags> = 'outer: {
            // Open file read only to get FD for flags operations.
            let f_res = File::open(&filename);
            let f = match f_res {
                Ok(f) => {
                    log::debug!("Opened variable file {filename} for flag operations");
                    f
                }
                Err(err) => {
                    // If the file does not exist, we cannot get flags (so skip the flag reading part),
                    // but should still proceed with writing the variable
                    if err.kind() == std::io::ErrorKind::NotFound {
                        log::debug!("File {filename} does not exist, will create new variable");
                        break 'outer None;
                    } else {
                        // Otherwise, return an error.
                        log::debug!("Failed to open {filename} for flag operations: {err}");
                        return Err(Error::for_variable(err, var));
                    }
                }
            };

            // Read original flags.
            let orig_flags = rustix::fs::ioctl_getflags(&f)
                .map_err(|error| Error::for_variable(error.into(), var))?;

            // If Immutable flag is present, remove it.
            if orig_flags.contains(rustix::fs::IFlags::IMMUTABLE) {
                log::debug!("Removing IMMUTABLE flag from {filename} for writing");
                // IFlags doesn't implement Clone, so cycle through bits.
                let mut modif_flags = rustix::fs::IFlags::from_bits(orig_flags.bits()).unwrap();

                modif_flags.remove(rustix::fs::IFlags::IMMUTABLE);
                rustix::fs::ioctl_setflags(&f, modif_flags)
                    .map_err(|error| Error::for_variable(error.into(), var))?;

                Some(orig_flags)
            } else {
                None
            }
        };

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
            log::debug!("Restoring original flags for variable {var}");
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
        let filename = format!("{EFIVARFS_ROOT}/{var}");

        std::fs::remove_file(&filename).map_err(|error| Error::for_variable(error, var))?;

        log::debug!("efivarfs: Deleted variable {var}");
        Ok(())
    }
}

impl VarManager for SystemManager {}
