use std::collections::HashMap;

use super::GuidGroup;

#[derive(Serialize, Deserialize)]
pub struct VendorGroup {
    vendors: HashMap<String, GuidGroup>,
}

impl VendorGroup {
    pub fn new() -> Self {
        VendorGroup {
            vendors: HashMap::new(),
        }
    }

    pub fn vendor(&self, guid: &str) -> Option<&GuidGroup> {
        self.vendors.get(guid)
    }

    pub fn vendor_mut(&mut self, guid: &str) -> &mut GuidGroup {
        self.vendors
            .entry(String::from(guid))
            .or_insert_with(|| GuidGroup::new())
    }
}
