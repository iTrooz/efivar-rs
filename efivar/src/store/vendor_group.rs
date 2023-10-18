use std::collections::HashMap;

use super::GuidGroup;
use crate::efi::VariableVendor;

#[derive(Default, Serialize, Deserialize)]
pub struct VendorGroup {
    pub vendors: HashMap<uuid::Uuid, GuidGroup>,
}

impl VendorGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn vendor(&self, vendor: &VariableVendor) -> Option<&GuidGroup> {
        self.vendors.get(vendor.as_ref())
    }

    pub fn vendor_mut(&mut self, vendor: &VariableVendor) -> &mut GuidGroup {
        self.vendors.entry(*vendor.as_ref()).or_default()
    }
}
