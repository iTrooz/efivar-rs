//! Definition of the VariableFlags type

use std::str::FromStr;

use crate::Error;

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

impl FromStr for VariableFlags {
    type Err = Error;

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
            _ => Err(Error::UnknownFlag { flag: s.to_owned() }),
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

        flag_strings.join("\n")
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
        let s = (VariableFlags::NON_VOLATILE
            | VariableFlags::BOOTSERVICE_ACCESS
            | VariableFlags::RUNTIME_ACCESS
            | VariableFlags::HARDWARE_ERROR_RECORD
            | VariableFlags::AUTHENTICATED_WRITE_ACCESS
            | VariableFlags::TIME_BASED_AUTHENTICATED_WRITE_ACCESS
            | VariableFlags::APPEND_WRITE
            | VariableFlags::ENHANCED_AUTHENTICATED_ACCESS)
            .to_string();

        assert!(s.contains("NON_VOLATILE"));
        assert!(s.contains("BOOTSERVICE_ACCESS"));
        assert!(s.contains("RUNTIME_ACCESS"));
        assert!(s.contains("HARDWARE_ERROR_RECORD"));
        assert!(s.contains("AUTHENTICATED_WRITE_ACCESS"));
        assert!(s.contains("TIME_BASED_AUTHENTICATED_WRITE_ACCESS"));
        assert!(s.contains("APPEND_WRITE"));
        assert!(s.contains("ENHANCED_AUTHENTICATED_ACCESS"));
    }
}
