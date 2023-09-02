use std::str::FromStr;

/// Structure used to contain the right structure for a boot id: 4 (or less, will be zero-filled) hex characters
/// Given to the CLI library
pub struct BootEntryId(pub u16);

impl FromStr for BootEntryId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 4 {
            return Err("Boot entry ID can be no longer than 4 characters".to_owned());
        }

        Ok(BootEntryId(
            u16::from_str_radix(s, 16).map_err(|err| err.to_string())?,
        ))
    }
}

#[cfg(test)]
mod tests {
    pub use super::*;

    #[test]
    fn valid_ids() -> Result<(), String> {
        assert_eq!(BootEntryId::from_str("0")?.0, 0);
        assert_eq!(BootEntryId::from_str("5")?.0, 5);
        assert_eq!(BootEntryId::from_str("0005")?.0, 5);
        assert_eq!(BootEntryId::from_str("1000")?.0, 4096);
        assert_eq!(BootEntryId::from_str("AAAA")?.0, 43690);
        Ok(())
    }

    #[test]
    fn id_too_long() -> Result<(), String> {
        assert!(BootEntryId::from_str("00000").is_err());
        Ok(())
    }

    #[test]
    fn id_invalid_hex() -> Result<(), String> {
        assert!(BootEntryId::from_str("000G").is_err());
        Ok(())
    }
}
