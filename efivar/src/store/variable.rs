use crate::efi::{VariableFlags, VariableName};
use crate::{Error, VarEnumerator, VarManager, VarManagerEx, VarReader, VarReaderEx, VarWriter};

use super::VendorGroup;

pub trait VariableStore: VarManagerEx {
    fn get_vendor_group(&self) -> &VendorGroup;
    fn get_vendor_group_mut(&mut self) -> &mut VendorGroup;
}

impl<T: VariableStore> VarEnumerator for T {
    fn get_var_names<'a>(&'a self) -> crate::Result<Box<dyn Iterator<Item = VariableName> + 'a>> {
        Ok(Box::new(self.get_vendor_group().vendors.iter().flat_map(
            |(guid, group)| {
                group
                    .values
                    .iter()
                    .map(move |(name, _value)| VariableName::new_with_vendor(name, *guid))
            },
        )))
    }
}

impl<T: VariableStore> VarReader for T {
    fn read(&self, name: &VariableName, value: &mut [u8]) -> crate::Result<(usize, VariableFlags)> {
        self.get_vendor_group()
            .vendor(name.vendor())
            .and_then(|guid_group| guid_group.variable(name.variable()))
            .ok_or_else(|| Error::VarNotFound { name: name.clone() })
            .and_then(|val| val.to_tuple(name, value))
    }
}

impl<T: VariableStore> VarReaderEx for T {
    fn read_buf(&self, name: &VariableName) -> crate::Result<(Vec<u8>, VariableFlags)> {
        self.get_vendor_group()
            .vendor(name.vendor())
            .and_then(|guid_group| guid_group.variable(name.variable()))
            .ok_or_else(|| Error::VarNotFound { name: name.clone() })
            .and_then(|val| val.to_tuple_buf())
    }
}

impl<T: VariableStore> VarWriter for T {
    fn write(
        &mut self,
        name: &VariableName,
        attributes: VariableFlags,
        value: &[u8],
    ) -> crate::Result<()> {
        // Set variable
        self.get_vendor_group_mut()
            .vendor_mut(name.vendor())
            .variable_mut(name.variable())
            .set_from(&(attributes, value));

        Ok(())
    }
}

impl<T: VariableStore> VarManager for T {}
impl<T: VariableStore> VarManagerEx for T {}
