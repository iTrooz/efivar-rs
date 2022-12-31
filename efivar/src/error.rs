use std::io;

use crate::efi::VariableName;

/// Describes an error returned by EFI variable operations
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "failed to parse variable name: {}", name)]
    InvalidVarName { name: String },
    #[fail(display = "variable not found: {}", name)]
    VarNotFound { name: VariableName },
    #[fail(display = "permission denied for variable: {}", name)]
    PermissionDenied { name: VariableName },
    #[fail(display = "unknown i/o error for variable: {}", name)]
    VarUnknownError {
        name: VariableName,
        error: io::Error,
    },
    #[fail(display = "base64 decoding error: {}", error)]
    #[cfg(feature = "store")]
    Base64DecodeError { error: base64::DecodeError },
    #[fail(display = "unknown i/o error")]
    UnknownIoError { error: io::Error },
    #[fail(display = "unknown EFI variable flag: '{}'", flag)]
    UnknownFlag { flag: String },
    #[fail(display = "failed to decode name as valid UTF-8")]
    InvalidUTF8,
    #[fail(display = "buffer too small for variable: {}", name)]
    BufferTooSmall { name: VariableName },
    #[fail(display = "failed to decode uuid: {}", error)]
    UuidError { error: uuid::Error },
}

#[cfg(not(target_os = "windows"))]
fn is_variable_not_found_error(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::NotFound
}

#[cfg(target_os = "windows")]
fn is_variable_not_found_error(err: &io::Error) -> bool {
    err.raw_os_error() == Some(203)
}

#[cfg(not(target_os = "windows"))]
fn is_buffer_too_small_error(_err: &io::Error) -> bool {
    // TODO: Can this error actually be raised on Linux?
    false
}

#[cfg(target_os = "windows")]
fn is_buffer_too_small_error(err: &io::Error) -> bool {
    err.raw_os_error() == Some(122)
}

#[cfg(not(target_os = "windows"))]
fn is_permission_denied_error(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::PermissionDenied
}

#[cfg(target_os = "windows")]
fn is_permission_denied_error(err: &io::Error) -> bool {
    err.raw_os_error() == Some(1314)
}

impl Error {
    pub fn for_variable(error: io::Error, name: &VariableName) -> Self {
        let name = name.clone();

        if is_variable_not_found_error(&error) {
            return Error::VarNotFound { name };
        }

        if is_buffer_too_small_error(&error) {
            return Error::BufferTooSmall { name };
        }

        if is_permission_denied_error(&error) {
            return Error::PermissionDenied { name };
        }

        Error::VarUnknownError { name, error }
    }

    #[cfg(target_os = "windows")]
    pub fn for_variable_os(name: &VariableName) -> Self {
        Error::for_variable(io::Error::last_os_error(), name)
    }
}

#[cfg(feature = "store")]
impl From<base64::DecodeError> for Error {
    fn from(error: base64::DecodeError) -> Self {
        Error::Base64DecodeError { error }
    }
}
