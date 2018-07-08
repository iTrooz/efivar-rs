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

extern crate base64;
#[macro_use]
extern crate bitflags;
extern crate byteorder;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate serde;

#[cfg(windows)]
extern crate winapi;

/// EFI constants based on the [UEFI specification](http://www.uefi.org/sites/default/files/resources/UEFI_Spec_2_7.pdf)
pub mod efi;
pub mod store;

mod enumerator;
mod reader;
mod sys;
mod writer;

pub use enumerator::VarEnumerator;
pub use reader::VarReader;
pub use writer::VarWriter;

/// Represents an EFI variable manager that can read, write and list variables
pub trait VarManager: VarEnumerator + VarReader + VarWriter {}

use sys::SystemManager;
/// Returns a `VarManager` that represents the firmware variables of the running system
///
/// Reading variables should not require extra permissions, but writing variables will.
///
/// ***The returned object will change the values stored in the system's NVRAM. Please be cautious
/// when using its methods.***
pub fn system() -> Box<VarManager> {
    Box::new(SystemManager::new())
}

use store::FileStore;
/// Returns a `VarManager` which loads and stores variables to a TOML file. The variable file will
/// be read when calling this method, and written to when the returned object is dropped.
///
/// # Arguments
///
/// * `filename` - Path to the TOML file for this variable storage. If the file doesn't exist, it
/// will be created.
///
/// # Examples
///
/// ```
/// # use efivar::file_store;
/// use efivar::efi::VariableFlags;
/// # {
/// // Create a store from the file doc-test.toml
/// let mut store = file_store("doc-test.toml");
/// let value = vec![1, 2, 3, 4];
/// // Write the value of a variable
/// store.write("BootOrder-8be4df61-93ca-11d2-aa0d-00e098032b8c", VariableFlags::NON_VOLATILE, &value);
///
/// // Check the value of the written variable
/// let (attributes, data) = store.read("BootOrder-8be4df61-93ca-11d2-aa0d-00e098032b8c").unwrap();
/// assert_eq!(attributes, VariableFlags::NON_VOLATILE);
/// assert_eq!(data, value);
/// // At this point, store is dropped and doc-test.toml will be updated
/// # }
/// # std::fs::remove_file("doc-test.toml");
/// ```
pub fn file_store(filename: &str) -> Box<VarManager> {
    Box::new(FileStore::new(filename))
}
