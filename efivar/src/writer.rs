use super::efi::{VariableFlags, VariableName};

/// Represents the capability of writing EFI variables
pub trait VarWriter {
    /// Write the new value of the given EFI variable
    ///
    /// Note that some implementations will ignore attribute changes.
    ///
    /// # Arguments
    ///
    /// * `name`: name of the variable to write
    /// * `attributes`: EFI variable attributes
    /// * `value`: EFI variable contents
    fn write(
        &mut self,
        name: &VariableName,
        attributes: VariableFlags,
        value: &[u8],
    ) -> crate::Result<()>;

    fn delete(&mut self, name: &VariableName) -> crate::Result<()>;
}
