use crate::efi::Variable;

/// Represents the capability of enumerating EFI variables
pub trait VarEnumerator {
    /// Enumerates all known variables on the system. Returns a list of found variable names.
    fn get_all_vars<'a>(&'a self) -> crate::Result<Box<dyn Iterator<Item = Variable> + 'a>>;
}
