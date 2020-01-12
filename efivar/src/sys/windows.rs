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
        // Buffer for BootOrder
        let mut buf = vec![0u8; 512];

        // Read BootOrder
        let (boot_order_size, _flags) =
            sm.read(&format!("BootOrder-{}", crate::efi::EFI_GUID), &mut buf[..])?;

        // Resize to actual value size
        buf.resize(boot_order_size, 0);

        Ok(BootOrderIterator {
            cursor: Cursor::new(buf),
        })
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
    fn read(&self, name: &str, value: &mut [u8]) -> crate::Result<(usize, VariableFlags)> {
        // Parse name, and split into LPCWSTR
        let (guid_wide, name_wide) = SystemManager::parse_name(name)?;

        // Attribute return value
        let mut attributes: u32 = 0;

        let result = unsafe {
            GetFirmwareEnvironmentVariableExW(
                name_wide.as_ptr(),
                guid_wide.as_ptr(),
                value.as_mut_ptr() as *mut c_void,
                value.len() as u32,
                &mut attributes as *mut u32,
            )
        };

        match result {
            0 => Err(Error::for_variable_os(name.into())),
            len => Ok((
                len as usize,
                VariableFlags::from_bits(attributes).unwrap_or(VariableFlags::empty()),
            )),
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
