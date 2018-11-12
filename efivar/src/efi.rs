use std::str::FromStr;
use std::io::{Error, ErrorKind};

/// Vendor GUID of the EFI variables according to the specification
pub const EFI_GUID: &'static str = "8be4df61-93ca-11d2-aa0d-00e098032b8c";

bitflags! {
    /// Possible attributes of EFI variables as a bitfield
    pub struct VariableFlags : u32 {
        const NON_VOLATILE = 0x1;
        const BOOTSERVICE_ACCESS = 0x2;
        const RUNTIME_ACCESS = 0x4;
        const HARDWARE_ERROR_RECORD = 0x8;
        const AUTHENTICATED_WRITE_ACCESS = 0x10;
        const TIME_BASED_AUTHENTICATED_WRITE_ACCESS = 0x20;
        const APPEND_WRITE = 0x40;
        const ENHANCED_AUTHENTICATED_ACCESS = 0x80;
    }
}

#[derive(Debug, Fail)]
pub enum ParseVariableFlagsError {
    #[fail(display = "Unknown EFI variable flag: '{}'", flag)]
    UnknownFlag { flag: String },
}

impl From<ParseVariableFlagsError> for Error {
    fn from(e: ParseVariableFlagsError) -> Error {
        Error::new(ErrorKind::InvalidInput, e.to_string())
    }
}

impl FromStr for VariableFlags {
    type Err = ParseVariableFlagsError;

    fn from_str(s: &str) -> Result<VariableFlags, Self::Err> {
        match s {
            "EFI_VARIABLE_NON_VOLATILE" => Ok(VariableFlags::NON_VOLATILE),
            "EFI_VARIABLE_BOOTSERVICE_ACCESS" => Ok(VariableFlags::BOOTSERVICE_ACCESS),
            "EFI_VARIABLE_RUNTIME_ACCESS" => Ok(VariableFlags::RUNTIME_ACCESS),
            "EFI_VARIABLE_HARDWARE_ERROR_RECORD" => Ok(VariableFlags::HARDWARE_ERROR_RECORD),
            "EFI_VARIABLE_AUTHENTICATED_WRITE_ACCESS" => {
                Ok(VariableFlags::AUTHENTICATED_WRITE_ACCESS)
            }
            "EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS" => {
                Ok(VariableFlags::TIME_BASED_AUTHENTICATED_WRITE_ACCESS)
            }
            "EFI_VARIABLE_APPEND_WRITE" => Ok(VariableFlags::APPEND_WRITE),
            "EFI_VARIABLE_ENHANCED_AUTHENTICATED_ACCESS" => {
                Ok(VariableFlags::ENHANCED_AUTHENTICATED_ACCESS)
            }
            _ => Err(ParseVariableFlagsError::UnknownFlag { flag: s.to_owned() }),
        }
    }
}

impl ToString for VariableFlags {
    fn to_string(&self) -> String {
        let mut flag_strings = Vec::new();

        if self.contains(VariableFlags::NON_VOLATILE) {
            flag_strings.push("EFI_VARIABLE_NON_VOLATILE");
        }
        if self.contains(VariableFlags::BOOTSERVICE_ACCESS) {
            flag_strings.push("EFI_VARIABLE_BOOTSERVICE_ACCESS");
        }
        if self.contains(VariableFlags::RUNTIME_ACCESS) {
            flag_strings.push("EFI_VARIABLE_RUNTIME_ACCESS");
        }
        if self.contains(VariableFlags::HARDWARE_ERROR_RECORD) {
            flag_strings.push("EFI_VARIABLE_HARDWARE_ERROR_RECORD");
        }
        if self.contains(VariableFlags::AUTHENTICATED_WRITE_ACCESS) {
            flag_strings.push("EFI_VARIABLE_AUTHENTICATED_WRITE_ACCESS");
        }
        if self.contains(VariableFlags::TIME_BASED_AUTHENTICATED_WRITE_ACCESS) {
            flag_strings.push("EFI_VARIABLE_TIME_BASED_AUTHENTICATED_WRITE_ACCESS");
        }
        if self.contains(VariableFlags::APPEND_WRITE) {
            flag_strings.push("EFI_VARIABLE_APPEND_WRITE");
        }
        if self.contains(VariableFlags::ENHANCED_AUTHENTICATED_ACCESS) {
            flag_strings.push("EFI_VARIABLE_ENHANCED_AUTHENTICATED_ACCESS");
        }

        return flag_strings.join("\n");
    }
}

/// Parses an EFI variable name into a (`vendor_guid`, `variable_name`) tuple
///
/// # Arguments
///
/// * `name` - Name of an EFI variable in the standard `name`-`guid` format
///
/// # Examples
///
/// Parsing a valid name into two parts succeeds:
///
/// ```
/// # use efivar::efi::parse_name;
/// let (guid, name) = parse_name("BootOrder-8be4df61-93ca-11d2-aa0d-00e098032b8c").unwrap();
/// assert_eq!(guid, "8be4df61-93ca-11d2-aa0d-00e098032b8c");
/// assert_eq!(name, "BootOrder");
/// ```
///
/// Parsing an invalid name into two parts fails:
///
/// ```
/// # use efivar::efi::parse_name;
/// let result = efivar::efi::parse_name("invalid name");
/// assert!(result.is_err());
/// ```
pub fn parse_name<'b>(name: &'b str) -> Result<(&'b str, &'b str), String> {
    let name_parts = name.splitn(2, '-').collect::<Vec<_>>();
    if name_parts.len() != 2 {
        return Err(String::from("Name must be in name-vendor_guid format"));
    }

    Ok((name_parts[1], name_parts[0]))
}

/// Returns the full form of an EFI variable name
///
/// If the given name is already in the `name`-`guid` format, the same name is returned. However,
/// if the name is missing the `guid` part, the default EFI vendor GUID is appended to `name`.
///
/// # Arguments
///
/// * `name` - Partial or full EFI variable name
///
/// # Examples
///
/// ```
/// # use efivar::efi::to_fullname;
/// assert_eq!(to_fullname("BootOrder"), "BootOrder-8be4df61-93ca-11d2-aa0d-00e098032b8c");
/// assert_eq!(to_fullname("BootOrder-xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"),
///     "BootOrder-xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx");
/// ```
pub fn to_fullname(name: &str) -> String {
    if name.find('-').is_some() {
        String::from(name)
    } else {
        format!("{}-{}", String::from(name), EFI_GUID)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variable_flags_to_string_empty() {
        assert_eq!(VariableFlags::empty().to_string(), "");
    }

    #[test]
    fn variable_flags_to_string_all() {
        let s = (VariableFlags::NON_VOLATILE |
                 VariableFlags::BOOTSERVICE_ACCESS |
                 VariableFlags::RUNTIME_ACCESS |
                 VariableFlags::HARDWARE_ERROR_RECORD |
                 VariableFlags::AUTHENTICATED_WRITE_ACCESS |
                 VariableFlags::TIME_BASED_AUTHENTICATED_WRITE_ACCESS |
                 VariableFlags::APPEND_WRITE |
                 VariableFlags::ENHANCED_AUTHENTICATED_ACCESS).to_string();

        assert!(s.contains("NON_VOLATILE"));
        assert!(s.contains("BOOTSERVICE_ACCESS"));
        assert!(s.contains("RUNTIME_ACCESS"));
        assert!(s.contains("HARDWARE_ERROR_RECORD"));
        assert!(s.contains("AUTHENTICATED_WRITE_ACCESS"));
        assert!(s.contains("TIME_BASED_AUTHENTICATED_WRITE_ACCESS"));
        assert!(s.contains("APPEND_WRITE"));
        assert!(s.contains("ENHANCED_AUTHENTICATED_ACCESS"));
    }

    #[test]
    fn parse_name_valid() {
        let (guid, name) = parse_name("BootOrder-8be4df61-93ca-11d2-aa0d-00e098032b8c").unwrap();

        assert_eq!(guid, "8be4df61-93ca-11d2-aa0d-00e098032b8c");
        assert_eq!(name, "BootOrder");
    }

    #[test]
    fn parse_name_invalid() {
        assert!(parse_name("BootOrder_Invalid").is_err());
    }

    #[test]
    fn to_fullname_partial() {
        assert_eq!(to_fullname("BootOrder"), "BootOrder-8be4df61-93ca-11d2-aa0d-00e098032b8c");
    }

    #[test]
    fn to_fullname_full() {
        assert_eq!(to_fullname("BootOrder-xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx"),
                   "BootOrder-xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx");
    }
}
