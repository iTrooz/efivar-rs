use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::iter::once;
use std::mem;
use std::io;
use std::io::Error;

use std::ptr::null_mut;
use winapi::shared::minwindef::{BOOL, FALSE};
use winapi::shared::ntdef::NULL;
use winapi::um::handleapi::CloseHandle;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use winapi::um::securitybaseapi::AdjustTokenPrivileges;
use winapi::um::winbase::LookupPrivilegeValueW;
use winapi::um::winnt::{
    HANDLE, PLUID, SE_PRIVILEGE_ENABLED, TOKEN_ADJUST_PRIVILEGES, TOKEN_PRIVILEGES
};

/// Represents a process token. The associated `HANDLE` is closed when
/// this object is dropped.
struct ProcessToken(HANDLE);

impl ProcessToken {
    /// Obtains the process token with the given access for the current process
    ///
    /// # Arguments
    ///
    /// * `desired_access` - Token access level
    pub fn open_current(desired_access: u32) -> io::Result<Self> {
        Self::open(unsafe { GetCurrentProcess() }, desired_access)
    }

    /// Obtains the process token for the given `process`
    ///
    /// # Arguments
    ///
    /// * `process` - Process to get the token for
    /// * `desired_access` - Token access level
    pub fn open(process: HANDLE, desired_access: u32) -> io::Result<Self> {
        let mut process_token: HANDLE = NULL;
        let result = unsafe {
            OpenProcessToken(
                process,
                desired_access,
                &mut process_token as *mut HANDLE)
        };

        match result {
            0 => Err(Error::last_os_error()),
            _ => Ok(ProcessToken(process_token))
        }
    }
}

impl Drop for ProcessToken {
    fn drop(&mut self) {
        unsafe { CloseHandle(self.0) };
    }
}

/// Updates the privileges of the current thread to include SeSystemEnvironmentPrivilege, which is
/// required to read and write NVRAM variables.
///
/// # Errors
///
/// Any errors from the underlying winapi calls will be returned as `Err()`
pub fn update_privileges() -> io::Result<()> {
    // We need SeSystemEnvironmentPrivilege to do anything NVRAM-related
    // So we configure it for the current thread here
    // This means SystemManager is not Send
    let mut tp = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        Privileges: unsafe { mem::uninitialized() },
    };

    // Lookup privilege value for SeSystemEnvironmentPrivilege
    let se_system_environment_privilege: Vec<u16> = OsStr::new("SeSystemEnvironmentPrivilege")
        .encode_wide()
        .chain(once(0))
        .collect();
    let result = unsafe {
        LookupPrivilegeValueW(
            null_mut(),
            se_system_environment_privilege.as_ptr(),
            &mut tp.Privileges[0].Luid as PLUID,
        )
    };

    if result == 0 {
        return Err(Error::last_os_error());
    }

    // Set privilege to enabled
    tp.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;

    // Get current thread token
    let process_token = ProcessToken::open_current(TOKEN_ADJUST_PRIVILEGES)?;

    // Update current security token
    let result = unsafe {
        AdjustTokenPrivileges(
            process_token.0,
            FALSE as BOOL,
            &mut tp as *mut TOKEN_PRIVILEGES,
            0,
            null_mut(),
            null_mut(),
        )
    };

    // Check that the update is successful
    match result {
        0 => Err(Error::last_os_error()),
        _ => Ok(())
    }
}
