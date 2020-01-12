use crate::efi::{parse_name, VariableFlags};
use crate::{Error, VarEnumerator, VarManager, VarReader, VarWriter};

use super::VendorGroup;

pub trait VariableStore: VarManager {
    fn get_vendor_group(&self) -> &VendorGroup;
    fn get_vendor_group_mut(&mut self) -> &mut VendorGroup;
}

impl<T: VariableStore> VarEnumerator for T {
    fn get_var_names<'a>(&'a self) -> crate::Result<Box<dyn Iterator<Item = String> + 'a>> {
        Ok(Box::new(self.get_vendor_group().vendors.iter().flat_map(
            |(guid, group)| {
                group
                    .values
                    .iter()
                    .map(move |(name, _value)| format!("{}-{}", name, guid))
            },
        )))
    }
}

impl<T: VariableStore> VarReader for T {
    fn read(&self, name: &str, value: &mut [u8]) -> crate::Result<(usize, VariableFlags)> {
        let (guid, variable_name) = parse_name(name)?;

        self.get_vendor_group()
            .vendor(guid)
            .and_then(|guid_group| guid_group.variable(variable_name))
            .ok_or_else(|| Error::VarNotFound {
                name: variable_name.into(),
            })
            .and_then(|val| val.to_tuple(name, value))
    }
}

impl<T: VariableStore> VarWriter for T {
    fn write(&mut self, name: &str, attributes: VariableFlags, value: &[u8]) -> crate::Result<()> {
        let (guid, variable_name) = parse_name(name)?;

        // Set variable
        self.get_vendor_group_mut()
            .vendor_mut(guid)
            .variable_mut(variable_name)
            .set_from(&(attributes, value));

        Ok(())
    }
}

impl<T: VariableStore> VarManager for T {}
