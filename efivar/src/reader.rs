use super::efi::VariableFlags;

/// Represents the capability of reading EFI variables
pub trait VarReader {
    /// Read the EFI variable `name` and return its attributes and raw value
    ///
    /// # Arguments
    ///
    /// * `name` - Full name (including vendor GUID) of the variable to read
    fn read(&self, name: &str) -> crate::Result<(VariableFlags, Vec<u8>)>;
}
