use crate::efi::VariableFlags;

use base64;
use std::error::Error as std_error;

#[derive(Serialize, Deserialize)]
pub struct StoreValue {
    attributes: u32,
    data: String,
}

impl StoreValue {
    pub fn new() -> Self {
        StoreValue {
            attributes: 0u32,
            data: String::new(),
        }
    }

    pub fn set_from(&mut self, value: &(VariableFlags, &[u8])) {
        self.attributes = value.0.bits();
        self.data = base64::encode(&value.1);
    }

    pub fn to_tuple(&self) -> Result<(VariableFlags, Vec<u8>), String> {
        let attr = VariableFlags::from_bits(self.attributes).unwrap_or(VariableFlags::empty());
        let data = base64::decode(&self.data);

        match data {
            Ok(buffer) => Ok((attr, buffer)),
            Err(reason) => Err(String::from(reason.description())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let attributes = VariableFlags::NON_VOLATILE;
        let bytes = vec![1, 2, 3, 4];

        let mut value = StoreValue::new();
        value.set_from(&(attributes, &bytes));

        let tuple = value.to_tuple().unwrap();
        assert_eq!(attributes, tuple.0);
        assert_eq!(bytes, tuple.1);
    }
}
