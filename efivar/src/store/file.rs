use super::{VariableStore, VendorGroup};

use std::io;

use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, ErrorKind};
use std::path::{Path, PathBuf};

/// Implements support for storing and loading EFI variables from a TOML file
///
/// Implements `Drop` in order to save the updated variables once the object is no longer in use.
pub struct FileStore {
    filename: PathBuf,
    vendor_group: VendorGroup,
}

fn load_vendors(filename: &Path) -> io::Result<VendorGroup> {
    // Read file contents
    let mut file = File::open(filename)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Deserialize document
    let doc = toml::from_str(&String::from_utf8(buffer).map_err(|_| io::ErrorKind::Other)?);

    match doc {
        Ok(vendor_group) => Ok(vendor_group),
        Err(reason) => Err(Error::new(ErrorKind::Other, reason)),
    }
}

fn save_vendors(filename: &Path, vendor_group: &VendorGroup) -> io::Result<()> {
    let mut file = File::create(filename)?;
    let data = toml::to_string(vendor_group).map_err(|e| Error::new(ErrorKind::Other, e))?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

impl FileStore {
    /// Create a new file store
    ///
    /// # Arguments
    ///
    /// * `filename`: Path to the file to use for storing the variables
    pub fn new(filename: PathBuf) -> Self {
        let vendor_group = load_vendors(&filename).unwrap_or_else(|_| VendorGroup::new());

        Self {
            filename,
            vendor_group,
        }
    }
}

impl Drop for FileStore {
    fn drop(&mut self) {
        save_vendors(&self.filename, &self.vendor_group)
            .unwrap_or_else(|_| panic!("Failed to write store to {}", self.filename.display()));
    }
}

impl VariableStore for FileStore {
    fn get_vendor_group(&self) -> &VendorGroup {
        &self.vendor_group
    }
    fn get_vendor_group_mut(&mut self) -> &mut VendorGroup {
        &mut self.vendor_group
    }
}
