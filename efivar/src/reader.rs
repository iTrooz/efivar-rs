use super::efi::VariableFlags;

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
    /// * `name`: full name (including vendor GUID) of the variable to read
    /// * `value`: target buffer for returning the variable value
    ///
    /// # Return value
    ///
    /// On success, number of bytes read and associated EFI variable flags.
    fn read(&self, name: &str, value: &mut [u8]) -> crate::Result<(usize, VariableFlags)>;
}

/// Represents the capability of reading EFI variables of a dynamic size
pub trait VarReaderEx {
    /// Read the EFI variable `name` and return its attributes and raw value
    ///
    /// This function will allocate a large enough buffer to hold the resulting
    /// value and return it.
    ///
    /// # Arguments
    ///
    /// * `name`: full name (including vendor GUID) of the variable to read
    ///
    /// # Return value
    ///
    /// On success, read bytes and associated EFI variable flags.
    fn read_buf(&self, name: &str) -> crate::Result<(Vec<u8>, VariableFlags)>;
}
