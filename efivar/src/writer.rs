use std::io;

use super::efi::VariableFlags;

/// Represents the capability of writing EFI variables
pub trait VarWriter {
    /// Write the new value of the given EFI variable
    ///
    /// Note that some implementations will ignore attribute changes.
    ///
    /// # Arguments
    ///
    /// * `name` - Full name (including vendor GUID) of the variable to read
    /// * `attributes` - EFI variable attributes
    /// * `value` - EFI variable contents
    fn write(&mut self, name: &str, attributes: VariableFlags, value: &[u8]) -> io::Result<()>;
}

