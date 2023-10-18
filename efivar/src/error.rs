use std::io;

use crate::efi::Variable;

/// Describes an error returned by EFI variable operations
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to parse variable name: {}", name)]
    InvalidVarName { name: String },
    #[error("variable not found: {}", var)]
    VarNotFound { var: Variable },
    #[error("permission denied for variable: {}", var)]
    PermissionDenied { var: Variable },
    #[error("unknown i/o error for variable: {}", var)]
    VarUnknownError { var: Variable, error: io::Error },
    #[error("base64 decoding error: {}", error)]
    #[cfg(feature = "store")]
    Base64DecodeError { error: base64::DecodeError },
    #[error("base64 decoding error: {}", error)]
    #[cfg(feature = "store")]
    Base64DecodeSliceError { error: base64::DecodeSliceError },
    #[error("unknown i/o error")]
    UnknownIoError { error: io::Error },
    #[error("unknown EFI variable flag: '{}'", flag)]
    UnknownFlag { flag: String },
    #[error("failed to decode name as valid UTF-8")]
    InvalidUTF8,
    #[error("failed to decode uuid: {}", error)]
    UuidError { error: uuid::Error },
    #[error("failed to parse variable content (invalid content)")]
    VarParseError,
    #[error("failed to parse string: {}", 0)]
    StringParseError(crate::utils::StringParseError),
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
fn is_permission_denied_error(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::PermissionDenied
}

#[cfg(target_os = "windows")]
fn is_permission_denied_error(err: &io::Error) -> bool {
    err.raw_os_error() == Some(1314)
}

impl Error {
    pub fn for_variable(error: io::Error, var: &Variable) -> Self {
        let var = var.clone();

        if is_variable_not_found_error(&error) {
            return Error::VarNotFound { var };
        }

        if is_permission_denied_error(&error) {
            return Error::PermissionDenied { var };
        }

        Error::VarUnknownError { var, error }
    }

    #[cfg(target_os = "windows")]
    pub fn for_variable_os(var: &Variable) -> Self {
        Error::for_variable(io::Error::last_os_error(), var)
    }
}

#[cfg(feature = "store")]
impl From<base64::DecodeError> for Error {
    fn from(error: base64::DecodeError) -> Self {
        Error::Base64DecodeError { error }
    }
}

#[cfg(feature = "store")]
impl From<base64::DecodeSliceError> for Error {
    fn from(error: base64::DecodeSliceError) -> Self {
        Error::Base64DecodeSliceError { error }
    }
}
