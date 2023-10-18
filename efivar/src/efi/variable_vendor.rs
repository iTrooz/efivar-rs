use std::fmt;

use super::EFI_GUID;

#[derive(Copy, Clone, Eq)]
/// An EFI variable vendor identifier
pub enum VariableVendor {
    /// Standard EFI variables
    Efi,
    /// Other EFI vendors
    Custom(uuid::Uuid),
}

impl VariableVendor {
    /// Return true if this vendor is the EFI vendor
    pub fn is_efi(&self) -> bool {
        matches!(self, VariableVendor::Efi)
    }
}

impl PartialEq for VariableVendor {
    fn eq(&self, other: &Self) -> bool {
        match self {
            VariableVendor::Efi => match other {
                VariableVendor::Efi => true,
                VariableVendor::Custom(uuid) => *uuid == *EFI_GUID,
            },
            VariableVendor::Custom(uuid) => match other {
                VariableVendor::Efi => *uuid == *EFI_GUID,
                VariableVendor::Custom(other_uuid) => *other_uuid == *uuid,
            },
        }
    }
}

impl From<uuid::Uuid> for VariableVendor {
    fn from(other: uuid::Uuid) -> Self {
        if other == *EFI_GUID {
            VariableVendor::Efi
        } else {
            VariableVendor::Custom(other)
        }
    }
}

impl AsRef<uuid::Uuid> for VariableVendor {
    fn as_ref(&self) -> &uuid::Uuid {
        match self {
            VariableVendor::Efi => &EFI_GUID,
            VariableVendor::Custom(uuid) => uuid,
        }
    }
}

impl fmt::Debug for VariableVendor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.as_ref(), f)
    }
}

impl fmt::Display for VariableVendor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self.as_ref(), f)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use uuid::Uuid;

    use crate::efi::{variable_vendor::VariableVendor, EFI_GUID};

    #[test]
    fn variable_vendor_eq() {
        assert_eq!(VariableVendor::Efi, VariableVendor::Efi);

        // idk what the right behaviour would be here
        assert_eq!(VariableVendor::Efi, VariableVendor::Custom(*EFI_GUID));
        assert_eq!(VariableVendor::Custom(*EFI_GUID), VariableVendor::Efi);

        assert_eq!(
            VariableVendor::Custom(Uuid::from_str("9acae909-5f29-43c8-b925-30040693bdff").unwrap()),
            VariableVendor::Custom(Uuid::from_str("9acae909-5f29-43c8-b925-30040693bdff").unwrap())
        );

        assert_ne!(
            VariableVendor::Custom(Uuid::from_str("9acae909-5f29-43c8-b925-30040693bdff").unwrap()),
            VariableVendor::Custom(Uuid::from_str("d3464728-c118-4d88-a450-7ac21a85a099").unwrap())
        );

        assert_ne!(
            VariableVendor::Efi,
            VariableVendor::Custom(Uuid::from_str("d3464728-c118-4d88-a450-7ac21a85a099").unwrap())
        );
    }
}
