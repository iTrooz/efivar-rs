use std::str::FromStr;

mod variable_flags;
pub use variable_flags::*;

mod variable_name;
pub use variable_name::*;

lazy_static! {
    /// Vendor GUID of the EFI variables according to the specification
    pub static ref EFI_GUID: uuid::Uuid =
        uuid::Uuid::from_str("8be4df61-93ca-11d2-aa0d-00e098032b8c").unwrap();
}
