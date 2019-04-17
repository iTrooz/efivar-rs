use std::convert::From;
use std::io;

/// Describes an error returned by EFI variable operations
#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "failed to parse variable name: {}", name)]
    InvalidVarName { name: String },
    #[fail(display = "variable not found: {}", name)]
    VarNotFound { name: String },
    #[fail(display = "unknown i/o error for variable: {}", name)]
    VarUnknownError { name: String, error: io::Error },
    #[fail(display = "base64 decoding error: {}", error)]
    Base64DecodeError { error: base64::DecodeError },
    #[fail(display = "unknown i/o error")]
    UnknownIoError { error: io::Error },
    #[fail(display = "unknown EFI variable flag: '{}'", flag)]
    UnknownFlag { flag: String },
    #[fail(display = "failed to decode name as valid UTF-8")]
    InvalidUTF8,
}

#[cfg(not(target_os = "windows"))]
fn is_variable_not_found_error(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::NotFound
}

#[cfg(target_os = "windows")]
fn is_variable_not_found_error(err: &io::Error) -> bool {
    err.kind() == io::ErrorKind::Other && err.raw_os_error() == Some(203)
}

impl Error {
    pub fn for_variable(error: io::Error, name: String) -> Self {
        if is_variable_not_found_error(&error) {
            return Error::VarNotFound { name }
        }

        Error::VarUnknownError { name, error }
    }

    #[cfg(target_os = "windows")]
    pub fn for_variable_os(name: String) -> Self {
        Error::for_variable(io::Error::last_os_error(), name)
    }
}

impl From<base64::DecodeError> for Error {
    fn from(error: base64::DecodeError) -> Self {
        Error::Base64DecodeError { error }
    }
}
