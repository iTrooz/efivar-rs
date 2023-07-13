use super::{VariableStore, VendorGroup};

/// Represents an in-memory EFI variable store
#[derive(Default)]
pub struct MemoryStore {
    vendor_group: VendorGroup,
}

impl MemoryStore {
    /// Create a new empty memory store
    pub fn new() -> Self {
        Self::default()
    }
}

impl VariableStore for MemoryStore {
    fn get_vendor_group(&self) -> &VendorGroup {
        &self.vendor_group
    }
    fn get_vendor_group_mut(&mut self) -> &mut VendorGroup {
        &mut self.vendor_group
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::efi::{VariableFlags, VariableName};
    use crate::VarWriter;

    #[test]
    fn missing_vendor() {
        let store = MemoryStore::new();

        assert!(store
            .get_vendor_group()
            .vendor(&crate::efi::VariableVendor::Efi)
            .is_none());
    }

    #[test]
    fn missing_variable() {
        let mut store = MemoryStore::new();
        store
            .write(&VariableName::new("BootOrder"), VariableFlags::empty(), &[])
            .unwrap();

        let group = store
            .get_vendor_group()
            .vendor(&crate::efi::VariableVendor::Efi)
            .unwrap();
        assert!(group.variable("Boot0001").is_none());
    }

    #[test]
    fn existing_variable() {
        let mut store = MemoryStore::new();
        store
            .write(&VariableName::new("BootOrder"), VariableFlags::empty(), &[])
            .unwrap();

        let group = store
            .get_vendor_group()
            .vendor(&crate::efi::VariableVendor::Efi)
            .unwrap();
        let variable = group.variable("BootOrder").unwrap().to_tuple_buf().unwrap();

        assert_eq!(variable.0, vec![]);
        assert_eq!(variable.1, VariableFlags::empty());
    }
}
