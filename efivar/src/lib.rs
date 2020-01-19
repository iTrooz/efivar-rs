//! efivar is a crate for manipulating EFI variables using the OS interface. This crate is mainly
//! used by `efiboot` to implement its functionality.
//!
//! On Linux, it is assumed that efivarfs is mounted and available at /sys/firmware/efi/efivars,
//! which should be the default nowadays on all major distros.
//!
//! On Windows, it uses the Get/SetFirmwareEnvironmentVariable family of functions, which require
//! administrative rights. This also requires adjusting the security token for the current thread
//! to include SeSystemEnvironmentPrivilege. This is done during the initialization of
//! SystemManager (see  SystemManager::new() ).
//!
//! In-memory and filesystem storage are also provided for testing purposes, or as a way to dump
//! system variables to an external file.

#![doc(html_root_url = "https://docs.rs/efivar/0.2.0")]

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate lazy_static;
#[cfg(feature = "store")]
#[macro_use]
extern crate serde_derive;

#[cfg(windows)]
extern crate winapi;

/// EFI constants based on the [UEFI specification](http://www.uefi.org/sites/default/files/resources/UEFI_Spec_2_7.pdf)
pub mod efi;
#[cfg(feature = "store")]
pub mod store;

mod enumerator;
mod error;
mod reader;
mod sys;
mod writer;

pub use crate::enumerator::VarEnumerator;
pub use crate::reader::*;
pub use crate::writer::VarWriter;

pub use crate::error::Error;

/// Result type for this crate's API functions
pub type Result<T> = std::result::Result<T, Error>;

/// Represents an EFI variable manager that can read, write and list variables
pub trait VarManager: VarEnumerator + VarReader + VarWriter {}

/// Represents an EFI variable manager that can read (both static and dynamic sized variables),
/// write and list variables
pub trait VarManagerEx: VarManager + VarReaderEx {}

use crate::sys::SystemManager;
/// Returns a `VarManager` that represents the firmware variables of the running system
///
/// Reading variables should not require extra permissions, but writing variables will.
///
/// ***The returned object will change the values stored in the system's NVRAM. Please be cautious
/// when using its methods.***
pub fn system() -> Box<dyn VarManager> {
    Box::new(SystemManager::new())
}

/// Returns a `VarManager` which loads and stores variables to a TOML file. The variable file will
/// be read when calling this method, and written to when the returned object is dropped.
///
/// # Arguments
///
/// * `filename`: Path to the TOML file for this variable storage. If the file doesn't exist, it
/// will be created.
///
/// # Examples
///
/// ```
/// # use efivar::file_store;
/// use efivar::efi::{VariableFlags, VariableName};
/// # {
/// // Name of the BootOrder variable
/// let boot_order = VariableName::new("BootOrder");
///
/// // Create a store from the file doc-test.toml
/// let mut store = file_store("doc-test.toml");
/// let value = vec![1, 2, 3, 4];
/// // Write the value of a variable
/// store.write(&boot_order, VariableFlags::NON_VOLATILE, &value);
///
/// // Check the value of the written variable
/// let (data, attributes) = store.read_buf(&boot_order).unwrap();
/// assert_eq!(data, value);
/// assert_eq!(attributes, VariableFlags::NON_VOLATILE);
/// // At this point, store is dropped and doc-test.toml will be updated
/// # }
/// # std::fs::remove_file("doc-test.toml");
/// ```
#[cfg(feature = "store")]
pub fn file_store<P: Into<std::path::PathBuf>>(filename: P) -> Box<dyn VarManagerEx> {
    Box::new(store::FileStore::new(filename.into()))
}

#[cfg(feature = "store")]
#[doc(hidden)]
pub fn file_store_std<P: Into<std::path::PathBuf>>(filename: P) -> Box<dyn VarManager> {
    // TODO: Fix this knowing that VarManagerEx: VarManager
    Box::new(store::FileStore::new(filename.into()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::efi::{VariableFlags, VariableName};

    #[test]
    fn file_store_roundtrip() {
        {
            // Create a store from the file doc-test.toml
            let mut store = file_store("doc-test.toml");
            let value = vec![1, 2, 3, 4];
            // Write the value of a variable
            store
                .write(
                    &VariableName::new("BootOrder"),
                    VariableFlags::NON_VOLATILE,
                    &value,
                )
                .expect("Failed to write value in store");

            // Check the value of the written variable
            let (data, attributes) = store.read_buf(&VariableName::new("BootOrder")).unwrap();
            assert_eq!(attributes, VariableFlags::NON_VOLATILE);
            assert_eq!(data, value);
            // At this point, store is dropped and doc-test.toml will be updated
        }
        std::fs::remove_file("doc-test.toml")
            .expect("Failed to remove temporary file doc-test.toml");
    }

    #[test]
    fn system_instantiate() {
        system();
    }
}
