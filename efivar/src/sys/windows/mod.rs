pub struct SystemManager;

use std::ffi::OsStr;
use std::iter::once;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::c_void;
use winapi::um::winbase::{GetFirmwareEnvironmentVariableExW, SetFirmwareEnvironmentVariableExW};

use std::io;
use std::io::{Error, ErrorKind};

use byteorder::{LittleEndian, ReadBytesExt};

use efi::VariableFlags;
use {VarEnumerator, VarManager, VarReader, VarWriter};

#[cfg(target_os = "windows")]
mod security;

impl SystemManager {
    pub fn new() -> SystemManager {
        // Update current thread token with the right privileges
        security::update_privileges().unwrap();
        SystemManager {}
    }

    fn parse_name(name: &str) -> io::Result<(Vec<u16>, Vec<u16>)> {
        // Parse name, and split into LPCWSTR
        let (guid, name) = ::efi::parse_name(name).map_err(|e| {
            Error::new(
                ErrorKind::Other,
                format!("Failed to parse variable name: {}", e),
            )
        })?;
        let guid_wide: Vec<u16> = OsStr::new(&format!("{{{}}}", guid))
            .encode_wide()
            .chain(once(0))
            .collect();
        let name_wide: Vec<u16> = OsStr::new(name).encode_wide().chain(once(0)).collect();

        Ok((guid_wide, name_wide))
    }
}

impl VarEnumerator for SystemManager {
    fn get_var_names(&self) -> io::Result<Vec<String>> {
        // Windows doesn't provide access to the variable enumeration service
        // We default here to a static list of variables required by the spec
        // as well as those we can discover by reading the BootOrder variable
        let mut known_vars: Vec<_> = vec!["BootCurrent", "BootNext", "BootOrder", "Timeout"]
            .iter()
            .map(|&s| s.into())
            .collect();

        // Read BootOrder
        match self.read(&format!("BootOrder-{}", ::efi::EFI_GUID)) {
            Ok((_flags, boot_order)) => {
                let mut bytes = &boot_order[..];
                while let Ok(id) = bytes.read_u16::<LittleEndian>() {
                    known_vars.push(format!("Boot{}", id));
                }
            }
            Err(e) => return Err(e),
        }

        Ok(known_vars
            .iter()
            .map(|name| format!("{}-{}", name, ::efi::EFI_GUID))
            .collect())
    }
}

impl VarReader for SystemManager {
    fn read(&self, name: &str) -> io::Result<(VariableFlags, Vec<u8>)> {
        // Parse name, and split into LPCWSTR
        let (guid_wide, name_wide) = SystemManager::parse_name(name)?;

        // Allocate buffer
        let mut buffer: [u8; 1024] = unsafe { mem::uninitialized() };
        let size = 1024;

        // Attribute return value
        let mut attributes: u32 = 0;

        let result = unsafe {
            GetFirmwareEnvironmentVariableExW(
                name_wide.as_ptr(),
                guid_wide.as_ptr(),
                buffer.as_mut_ptr() as *mut c_void,
                size,
                &mut attributes as *mut u32,
            )
        };

        match result {
            0 => Err(Error::last_os_error()),
            len => Ok((
                VariableFlags::from_bits(attributes).unwrap_or(VariableFlags::empty()),
                Vec::from(&buffer[0..len as usize]),
            )),
        }
    }
}

impl VarWriter for SystemManager {
    fn write(&mut self, name: &str, attributes: VariableFlags, value: &[u8]) -> io::Result<()> {
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
            0 => Err(Error::last_os_error()),
            _ => Ok(()),
        }
    }
}

impl VarManager for SystemManager {}
