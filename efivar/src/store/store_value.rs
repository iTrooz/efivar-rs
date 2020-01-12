use crate::efi::VariableFlags;

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

    pub fn to_tuple(&self, value: &mut [u8]) -> crate::Result<(usize, VariableFlags)> {
        let attr = VariableFlags::from_bits(self.attributes).unwrap_or(VariableFlags::empty());

        // base64::decode_config_slice panics if the target buffer is too small
        if value.len() < (self.data.len() + 3) / 4 * 3 {
            return Err(crate::Error::BufferTooSmall);
        }

        Ok((
            base64::decode_config_slice(&self.data, base64::STANDARD, value)?,
            attr,
        ))
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
