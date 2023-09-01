use crate::efi::{Variable, VariableFlags};
use crate::{Error, VarEnumerator, VarManager, VarReader, VarWriter};

use super::VendorGroup;

pub trait VariableStore: VarManager {
    fn get_vendor_group(&self) -> &VendorGroup;
    fn get_vendor_group_mut(&mut self) -> &mut VendorGroup;
}

impl<T: VariableStore> VarEnumerator for T {
    fn get_all_vars<'a>(&'a self) -> crate::Result<Box<dyn Iterator<Item = Variable> + 'a>> {
        Ok(Box::new(self.get_vendor_group().vendors.iter().flat_map(
            |(guid, group)| {
                group
                    .values
                    .keys()
                    .map(move |name| Variable::new_with_vendor(name, *guid))
            },
        )))
    }
}

impl<T: VariableStore> VarReader for T {
    fn read(&self, var: &Variable) -> crate::Result<(Vec<u8>, VariableFlags)> {
        self.get_vendor_group()
            .vendor(var.vendor())
            .and_then(|guid_group| guid_group.variable(var.name()))
            .ok_or_else(|| Error::VarNotFound { name: var.clone() })
            .and_then(|val| val.to_tuple())
    }
}

impl<T: VariableStore> VarWriter for T {
    fn write(
        &mut self,
        var: &Variable,
        attributes: VariableFlags,
        value: &[u8],
    ) -> crate::Result<()> {
        // Set variable
        self.get_vendor_group_mut()
            .vendor_mut(var.vendor())
            .variable_mut(var.name())
            .set_from(&(attributes, value));

        Ok(())
    }

    fn delete(&mut self, var: &Variable) -> crate::Result<()> {
        self.get_vendor_group_mut()
            .vendor_mut(var.vendor())
            .delete_variable(var.name());
        Ok(())
    }
}

impl<T: VariableStore> VarManager for T {}
