use crate::efi::{Variable, VariableFlags};
use crate::{Error, VarEnumerator, VarManager, VarReader, VarWriter};

use super::VendorGroup;

pub trait VariableStore: VarManager {
    fn get_vendor_group(&self) -> &VendorGroup;
    fn get_vendor_group_mut(&mut self) -> &mut VendorGroup;
}

impl<T: VariableStore> VarEnumerator for T {
    fn get_var_names<'a>(&'a self) -> crate::Result<Box<dyn Iterator<Item = Variable> + 'a>> {
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
    fn read(&self, name: &Variable) -> crate::Result<(Vec<u8>, VariableFlags)> {
        self.get_vendor_group()
            .vendor(name.vendor())
            .and_then(|guid_group| guid_group.variable(name.variable()))
            .ok_or_else(|| Error::VarNotFound { name: name.clone() })
            .and_then(|val| val.to_tuple())
    }
}

impl<T: VariableStore> VarWriter for T {
    fn write(
        &mut self,
        name: &Variable,
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

    fn delete(&mut self, name: &Variable) -> crate::Result<()> {
        self.get_vendor_group_mut()
            .vendor_mut(name.vendor())
            .delete_variable(name.variable());
        Ok(())
    }
}

impl<T: VariableStore> VarManager for T {}
