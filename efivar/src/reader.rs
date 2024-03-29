use crate::Error;

use super::efi::{Variable, VariableFlags};

/// Represents the capability of reading EFI variables
pub trait VarReader {
    /// Read the EFI variable `name` and return its attributes and raw value
    ///
    /// The caller is responsible for allocating a large enough buffer to hold
    /// the value for the target variable. This interface is used since some backends
    /// (Windows) don't have an API for reporting the value size before reading it.
    ///
    /// # Arguments
    ///
    /// * `var`: variable to read
    /// * `value`: target buffer for returning the variable value
    ///
    /// # Return value
    ///
    /// On success, number of bytes read and associated EFI variable flags.
    fn read(&self, var: &Variable) -> crate::Result<(Vec<u8>, VariableFlags)>;
    fn exists(&self, var: &Variable) -> crate::Result<bool> {
        match self.read(var) {
            Ok(_) => Ok(true),
            Err(Error::VarNotFound { var: _ }) => Ok(false),
            Err(err) => Err(err),
        }
    }
}
