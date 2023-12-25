pub struct SystemManager;

use std::convert::TryInto;

use byteorder::{LittleEndian, ReadBytesExt};
use ntapi::ntexapi::NtEnumerateSystemEnvironmentValuesEx;
use std::ffi::OsStr;
use std::iter::once;
use std::mem;
use std::os::windows::ffi::OsStrExt;
use winapi::ctypes::{c_ulong, c_void};
use winapi::shared::minwindef::DWORD;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::winbase::{GetFirmwareEnvironmentVariableExW, SetFirmwareEnvironmentVariableExW};

use crate::efi::{Variable, VariableFlags};
use crate::utils::read_nt_utf16_string;
use crate::{Error, VarEnumerator, VarManager, VarReader, VarWriter};

#[cfg(target_os = "windows")]
mod security;

impl SystemManager {
    pub fn new() -> SystemManager {
        // Update current thread token with the right privileges
        security::update_privileges().unwrap();
        SystemManager {}
    }

    fn parse_name(var: &Variable) -> crate::Result<(Vec<u16>, Vec<u16>)> {
        // Split into LPCWSTR
        let guid_wide: Vec<u16> = OsStr::new(&format!("{{{}}}", var.vendor()))
            .encode_wide()
            .chain(once(0))
            .collect();
        let name_wide: Vec<u16> = OsStr::new(var.name())
            .encode_wide()
            .chain(once(0))
            .collect();

        Ok((guid_wide, name_wide))
    }
}

fn parse_efi_variable(buf: &mut &[u8]) -> crate::Result<Variable> {
    let uuid_u128 = buf
        .read_u128::<LittleEndian>()
        .map_err(crate::Error::UnknownIoError)?;

    let guid = uuid::Uuid::from_bytes_le(uuid_u128.to_le_bytes());
    let name = read_nt_utf16_string(buf).map_err(crate::Error::StringParseError)?;

    Ok(Variable::new_with_vendor(&name, guid))
}

fn parse_efi_variables(buf: &mut &[u8]) -> crate::Result<Vec<Variable>> {
    let mut vars: Vec<Variable> = vec![];
    while buf.len() > 0 {
        let struct_size = buf
            .read_u32::<LittleEndian>()
            .map_err(crate::Error::UnknownIoError)?;

        if struct_size == 0 {
            break;
        };

        let (mut efi_var_struct, new_buf) = buf.split_at(
            (struct_size - 4)
                .try_into()
                .expect("EFI variable structure size should fit into a usize"),
        );
        *buf = new_buf;

        vars.push(parse_efi_variable(&mut efi_var_struct)?);
    }

    Ok(vars)
}

impl VarEnumerator for SystemManager {
    fn get_all_vars<'a>(&'a self) -> crate::Result<Box<dyn Iterator<Item = Variable> + 'a>> {
        // get size of buffer to allocate for variables
        let mut size: u32 = 0;
        const STATUS_BUFFER_TOO_SMALL: i32 = 0xc0000023_u32 as i32;
        {
            let status: i32 = unsafe {
                NtEnumerateSystemEnvironmentValuesEx(
                    1, // 1 means system variables, so EFI variables
                    std::ptr::null_mut() as *mut c_void,
                    &mut size as *mut c_ulong,
                )
            };

            // handle error
            if status != STATUS_BUFFER_TOO_SMALL {
                return Err(crate::Error::UnknownIoError(
                    std::io::Error::from_raw_os_error(status),
                ));
            }
        }

        // retrieve EFI variables
        let buf: Vec<u8> = vec![
            0u8;
            size.try_into().expect(
                "Value returned by NtEnumerateSystemEnvironmentValuesEx() should be a valid usize"
            )
        ];
        {
            let status: i32 = unsafe {
                NtEnumerateSystemEnvironmentValuesEx(
                    1, // 1 means system variables, so EFI variables
                    buf.as_ptr() as *mut c_void,
                    &mut size,
                )
            };

            // handle error
            if status != 0 {
                return Err(crate::Error::UnknownIoError(
                    std::io::Error::from_raw_os_error(status),
                ));
            }
        }

        let vars = parse_efi_variables(&mut &buf[..])?;
        Ok(Box::new(vars.into_iter()))
    }
}

impl VarReader for SystemManager {
    fn read(&self, var: &Variable) -> crate::Result<(Vec<u8>, VariableFlags)> {
        // Parse name, and split into LPCWSTR
        let (guid_wide, name_wide) = SystemManager::parse_name(var)?;

        // Attribute return value
        let mut attributes: u32 = 0;

        let mut buf: Vec<u8> = vec![0u8; 256];

        const ERROR_BUFFER_TOO_SMALL: DWORD = 122;
        loop {
            unsafe {
                let written_bytes = GetFirmwareEnvironmentVariableExW(
                    name_wide.as_ptr(),
                    guid_wide.as_ptr(),
                    buf.as_mut_ptr() as *mut c_void,
                    buf.len() as u32,
                    &mut attributes as *mut u32,
                );

                if written_bytes == 0 {
                    if GetLastError() == ERROR_BUFFER_TOO_SMALL {
                        buf.resize(buf.len() * 2, 0u8);
                    } else {
                        return Err(Error::for_variable_os(var));
                    }
                } else {
                    buf.resize(written_bytes.try_into().expect("GetFirmwareEnvironmentVariableExW() return value should be a value usize"), 0u8);
                    return Ok((
                        buf,
                        VariableFlags::from_bits(attributes).unwrap_or(VariableFlags::empty()),
                    ));
                }
            };
        }
    }
}

impl VarWriter for SystemManager {
    fn write(
        &mut self,
        var: &Variable,
        attributes: VariableFlags,
        value: &[u8],
    ) -> crate::Result<()> {
        // Parse name, and split into LPCWSTR
        let (guid_wide, name_wide) = SystemManager::parse_name(var)?;

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
            0 => Err(Error::for_variable_os(var)),
            _ => Ok(()),
        }
    }

    fn delete(&mut self, var: &Variable) -> crate::Result<()> {
        let (guid_wide, name_wide) = SystemManager::parse_name(var)?;

        let result = unsafe {
            SetFirmwareEnvironmentVariableExW(
                name_wide.as_ptr(),
                guid_wide.as_ptr(),
                mem::transmute::<*const u8, *mut c_void>(std::ptr::null()),
                0,
                VariableFlags::NON_VOLATILE.bits(),
            )
        };

        match result {
            0 => Err(Error::for_variable_os(var)),
            _ => Ok(()),
        }
    }
}

impl VarManager for SystemManager {}
