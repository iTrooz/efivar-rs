pub struct SystemManager;

use std::iter;

use std::ffi::OsStr;
use std::iter::once;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::c_void;
use winapi::um::winbase::{GetFirmwareEnvironmentVariableExW, SetFirmwareEnvironmentVariableExW};

use crate::boot::BootVarReader;
use crate::efi::{VariableFlags, VariableName};
use crate::{Error, VarEnumerator, VarManager, VarReader, VarWriter};

#[cfg(target_os = "windows")]
mod security;

impl SystemManager {
    pub fn new() -> SystemManager {
        // Update current thread token with the right privileges
        security::update_privileges().unwrap();
        SystemManager {}
    }

    fn parse_name(name: &VariableName) -> crate::Result<(Vec<u16>, Vec<u16>)> {
        // Split into LPCWSTR
        let guid_wide: Vec<u16> = OsStr::new(&format!("{{{}}}", name.vendor()))
            .encode_wide()
            .chain(once(0))
            .collect();
        let name_wide: Vec<u16> = OsStr::new(name.variable())
            .encode_wide()
            .chain(once(0))
            .collect();

        Ok((guid_wide, name_wide))
    }
}

impl VarEnumerator for SystemManager {
    fn get_var_names<'a>(&'a self) -> crate::Result<Box<dyn Iterator<Item = VariableName> + 'a>> {
        // Windows doesn't provide access to the variable enumeration service
        // We default here to a static list of variables required by the spec
        // as well as those we can discover by reading the BootOrder variable
        Ok(Box::new(
            iter::once(VariableName::new("BootCurrent"))
                .chain(iter::once(VariableName::new("BootNext")))
                .chain(iter::once(VariableName::new("BootOrder")))
                .chain(iter::once(VariableName::new("Timeout")))
                .chain(self.get_boot_order()?),
        ))
    }
}

impl VarReader for SystemManager {
    fn read(&self, name: &VariableName, value: &mut [u8]) -> crate::Result<(usize, VariableFlags)> {
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
            0 => Err(Error::for_variable_os(name)),
            len => Ok((
                len as usize,
                VariableFlags::from_bits(attributes).unwrap_or(VariableFlags::empty()),
            )),
        }
    }
}

impl VarWriter for SystemManager {
    fn write(
        &mut self,
        name: &VariableName,
        attributes: VariableFlags,
        value: &[u8],
    ) -> crate::Result<()> {
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
            0 => Err(Error::for_variable_os(name)),
            _ => Ok(()),
        }
    }
}

impl VarManager for SystemManager {}
