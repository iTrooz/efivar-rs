use crate::efi::VariableFlags;

use base64;

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

    pub fn to_tuple(&self) -> crate::Result<(VariableFlags, Vec<u8>)> {
        let attr = VariableFlags::from_bits(self.attributes).unwrap_or(VariableFlags::empty());
        let data = base64::decode(&self.data)?;

        Ok((attr, data))
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
