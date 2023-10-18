use std::collections::HashMap;

use super::StoreValue;

#[derive(Default, Serialize, Deserialize)]
pub struct GuidGroup {
    pub values: HashMap<String, StoreValue>,
}

impl GuidGroup {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn variable(&self, name: &str) -> Option<&StoreValue> {
        self.values.get(name)
    }

    pub fn variable_mut(&mut self, name: &str) -> &mut StoreValue {
        self.values.entry(String::from(name)).or_default()
    }

    /// Delete a variable from this GUID group.
    pub fn delete_variable(&mut self, name: &str) {
        self.values.remove(name);
    }
}
