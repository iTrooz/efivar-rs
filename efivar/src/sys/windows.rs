pub struct SystemManager;

use std::io::Cursor;
use std::iter;

use std::ffi::OsStr;
use std::iter::once;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::c_void;
use winapi::um::winbase::{GetFirmwareEnvironmentVariableExW, SetFirmwareEnvironmentVariableExW};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::efi::VariableFlags;
use crate::{Error, VarEnumerator, VarManager, VarReader, VarWriter};

#[cfg(target_os = "windows")]
mod security;

impl SystemManager {
    pub fn new() -> SystemManager {
        // Update current thread token with the right privileges
        security::update_privileges().unwrap();
        SystemManager {}
    }

    fn parse_name(name: &str) -> crate::Result<(Vec<u16>, Vec<u16>)> {
        // Parse name, and split into LPCWSTR
        let (guid, name) = crate::efi::parse_name(name)?;

        let guid_wide: Vec<u16> = OsStr::new(&format!("{{{}}}", guid))
            .encode_wide()
            .chain(once(0))
            .collect();
        let name_wide: Vec<u16> = OsStr::new(name).encode_wide().chain(once(0)).collect();

        Ok((guid_wide, name_wide))
    }
}

struct BootOrderIterator {
    cursor: Cursor<Vec<u8>>,
}

impl BootOrderIterator {
    fn new(sm: &SystemManager) -> crate::Result<BootOrderIterator> {
        let (_flags, boot_order) = sm.read(&format!("BootOrder-{}", crate::efi::EFI_GUID))?;
        let cursor = Cursor::new(boot_order);

        Ok(BootOrderIterator { cursor })
    }
}

impl Iterator for BootOrderIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(id) = self.cursor.read_u16::<LittleEndian>() {
            return Some(format!("Boot{:04X}-{}", id, crate::efi::EFI_GUID).to_owned());
        }

        None
    }
}

impl VarEnumerator for SystemManager {
    fn get_var_names<'a>(&'a self) -> crate::Result<Box<dyn Iterator<Item = String> + 'a>> {
        // Windows doesn't provide access to the variable enumeration service
        // We default here to a static list of variables required by the spec
        // as well as those we can discover by reading the BootOrder variable
        Ok(Box::new(
            iter::once(format!("BootCurrent-{}", crate::efi::EFI_GUID).to_owned())
                .chain(iter::once(
                    format!("BootNext-{}", crate::efi::EFI_GUID).to_owned(),
                ))
                .chain(iter::once(
                    format!("BootOrder-{}", crate::efi::EFI_GUID).to_owned(),
                ))
                .chain(iter::once(
                    format!("Timeout-{}", crate::efi::EFI_GUID).to_owned(),
                ))
                .chain(BootOrderIterator::new(self)?),
        ))
    }
}

impl VarReader for SystemManager {
    fn read(&self, name: &str) -> crate::Result<(VariableFlags, Vec<u8>)> {
        // Parse name, and split into LPCWSTR
        let (guid_wide, name_wide) = SystemManager::parse_name(name)?;

        // Allocate buffer
        let mut buffer = mem::MaybeUninit::<[u8; 1024]>::uninit();
        let size = 1024;

        // Attribute return value
        let mut attributes: u32 = 0;

        unsafe {
            let result = GetFirmwareEnvironmentVariableExW(
                name_wide.as_ptr(),
                guid_wide.as_ptr(),
                buffer.as_mut_ptr() as *mut c_void,
                size,
                &mut attributes as *mut u32,
            );

            match result {
                0 => Err(Error::for_variable_os(name.into())),
                len => Ok((
                    VariableFlags::from_bits(attributes).unwrap_or(VariableFlags::empty()),
                    // TODO: Only part of the vector is initialized
                    Vec::from(&buffer.assume_init()[0..len as usize]),
                )),
            }
        }
    }
}

impl VarWriter for SystemManager {
    fn write(&mut self, name: &str, attributes: VariableFlags, value: &[u8]) -> crate::Result<()> {
        // Parse name, and split into LPCWSTR
        let (guid_wide, name_wide) = SystemManager::parse_name(name)?;

        let result = unsafe {
            SetFirmwareEnvironmentVariableExW(
                name_wide.as_ptr(),
                guid_wide.as_ptr(),
                // SetFirmwareEnvironmentVariableExW is not supposed to modify the contents
                // of the buffer for the value.
                mem::transmute::<*const u8, *mut c_void>(value.as_ptr()),
                value.len() as u32,
                attributes.bits(),
            )
        };

        match result {
            0 => Err(Error::for_variable_os(name.into())),
            _ => Ok(()),
        }
    }
}

impl VarManager for SystemManager {}
