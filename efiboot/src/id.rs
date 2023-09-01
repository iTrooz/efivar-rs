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
