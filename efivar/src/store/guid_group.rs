use std::collections::HashMap;

use super::StoreValue;

#[derive(Serialize, Deserialize)]
pub struct GuidGroup {
    pub values: HashMap<String, StoreValue>,
}

impl GuidGroup {
    pub fn new() -> Self {
        GuidGroup {
            values: HashMap::new(),
        }
    }

    pub fn variable(&self, name: &str) -> Option<&StoreValue> {
        self.values.get(name)
    }

    pub fn variable_mut(&mut self, name: &str) -> &mut StoreValue {
        self.values
            .entry(String::from(name))
            .or_insert_with(|| StoreValue::new())
    }
}
