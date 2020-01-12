use super::{VariableStore, VendorGroup};

/// Represents an in-memory EFI variable store
pub struct MemoryStore {
    vendor_group: VendorGroup,
}

impl MemoryStore {
    /// Create a new empty memory store
    pub fn new() -> Self {
        MemoryStore {
            vendor_group: VendorGroup::new(),
        }
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
    use crate::efi::VariableFlags;
    use crate::VarWriter;

    #[test]
    fn missing_vendor() {
        let store = MemoryStore::new();

        assert!(store
            .get_vendor_group()
            .vendor(crate::efi::EFI_GUID)
            .is_none());
    }

    #[test]
    fn missing_variable() {
        let mut store = MemoryStore::new();
        store
            .write(
                &format!("BootOrder-{}", crate::efi::EFI_GUID),
                VariableFlags::empty(),
                &vec![],
            )
            .unwrap();

        let group = store
            .get_vendor_group()
            .vendor(crate::efi::EFI_GUID)
            .unwrap();
        assert!(group.variable("Boot0001").is_none());
    }

    #[test]
    fn existing_variable() {
        let mut store = MemoryStore::new();
        store
            .write(
                &format!("BootOrder-{}", crate::efi::EFI_GUID),
                VariableFlags::empty(),
                &vec![],
            )
            .unwrap();

        let group = store
            .get_vendor_group()
            .vendor(crate::efi::EFI_GUID)
            .unwrap();
        let variable = group.variable("BootOrder").unwrap().to_tuple_buf().unwrap();

        assert_eq!(variable.0, vec![]);
        assert_eq!(variable.1, VariableFlags::empty());
    }
}
