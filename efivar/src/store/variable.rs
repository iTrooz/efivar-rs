use ::efi::{VariableFlags, parse_name};
use ::{VarManager, VarReader, VarWriter};

use super::VendorGroup;

use std::io;
use std::io::{Error, ErrorKind};

pub trait VariableStore: VarManager {
    fn get_vendor_group(&self) -> &VendorGroup;
    fn get_vendor_group_mut(&mut self) -> &mut VendorGroup;
}

impl<T: VariableStore> VarReader for T {
    fn read(&self, name: &str) -> io::Result<(VariableFlags, Vec<u8>)> {
        let (guid, variable_name) = parse_name(name)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        self.get_vendor_group()
            .vendor(guid)
            .and_then(|guid_group| guid_group.variable(variable_name))
            .ok_or_else(|| Error::new(ErrorKind::Other, format!("Variable {} not found", name)))
            .and_then(|variable| {
                variable.to_tuple().map_err(|e| {
                    Error::new(
                        ErrorKind::Other,
                        format!("Failed to decode {}: {}", name, e),
                    )
                })
            })
    }
}

impl<T: VariableStore> VarWriter for T {
    fn write(&mut self, name: &str, attributes: VariableFlags, value: &[u8]) -> io::Result<()> {
        let (guid, variable_name) = parse_name(name)
            .map_err(|e| Error::new(ErrorKind::Other, e))?;

        // Set variable
        self.get_vendor_group_mut()
            .vendor_mut(guid)
            .variable_mut(variable_name)
            .set_from(&(attributes, value));

        Ok(())
    }
}

impl<T: VariableStore> VarManager for T {
}
