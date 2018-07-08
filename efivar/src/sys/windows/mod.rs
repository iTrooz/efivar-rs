pub struct SystemManager;

use std::ffi::OsStr;
use std::iter::once;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::c_void;
use winapi::um::winbase::{
    GetFirmwareEnvironmentVariableExW, SetFirmwareEnvironmentVariableExW,
};

use std::io;
use std::io::{Error, ErrorKind};

use efi::VariableFlags;
use {VarManager, VarReader, VarWriter};

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
