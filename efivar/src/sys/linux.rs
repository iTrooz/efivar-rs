use std::fs;

mod efivarfs;
mod efivars;

use crate::efi::{VariableFlags, VariableName};
use crate::{VarEnumerator, VarManager, VarReader, VarWriter};

trait LinuxSystemManager: VarManager {
    #[cfg(test)]
    fn supported(&self) -> bool;
}

pub struct SystemManager {
    sys_impl: Box<dyn LinuxSystemManager>,
}

impl SystemManager {
    fn is_empty(p: &str) -> bool {
        !fs::read_dir(p)
            .map(|mut list| list.any(|_item| true))
            .unwrap_or(false)
    }

    pub fn new() -> SystemManager {
        if !Self::is_empty(efivars::EFIVARS_ROOT) {
            Self::efivars()
        } else if !Self::is_empty(efivarfs::EFIVARFS_ROOT) {
            Self::efivarfs()
        } else if cfg!(test) {
            // CI environments do not have efivarfs mounted,
            // so the resulting object will have no variables
            // defined. This allows testing however.
            Self::efivars()
        } else {
            panic!("Failed to determine if efivars or efivarfs should be used. Please check permissions to /sys/firmware/efi");
        }
    }

    pub fn efivars() -> SystemManager {
        SystemManager {
            sys_impl: Box::new(efivars::SystemManager::new()),
        }
    }

    pub fn efivarfs() -> SystemManager {
        SystemManager {
            sys_impl: Box::new(efivarfs::SystemManager::new()),
        }
    }

    #[cfg(test)]
    fn supported(&self) -> bool {
        self.sys_impl.supported()
    }
}

impl VarEnumerator for SystemManager {
    fn get_var_names<'a>(&'a self) -> crate::Result<Box<dyn Iterator<Item = VariableName> + 'a>> {
        self.sys_impl.get_var_names()
    }
}

impl VarReader for SystemManager {
    fn read(&self, name: &VariableName) -> crate::Result<(Vec<u8>, VariableFlags)> {
        self.sys_impl.read(name)
    }
}

impl VarWriter for SystemManager {
    fn write(
        &mut self,
        name: &VariableName,
        attributes: VariableFlags,
        value: &[u8],
    ) -> crate::Result<()> {
        self.sys_impl.write(name, attributes, value)
    }

    fn delete(&mut self, name: &VariableName) -> crate::Result<()> {
        self.sys_impl.delete(name)
    }
}

impl VarManager for SystemManager {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::efi::VariableName;

    fn linux_get_var_names(manager: &SystemManager) {
        if !manager.supported() {
            return;
        }

        let mut var_names = manager.get_var_names().unwrap();
        let name = VariableName::new("BootOrder");
        assert!(var_names.any(|n| n == name));
    }

    fn linux_read_var(manager: &SystemManager) {
        if !manager.supported() {
            return;
        }

        let (data, _flags) = manager
            .read(&VariableName::new("BootOrder"))
            .expect("Failed to read variable");

        assert!(!data.is_empty());
    }

    #[test]
    fn efivars_linux_get_var_names() {
        linux_get_var_names(&SystemManager::efivars());
    }

    #[test]
    fn efivars_linux_read_var() {
        linux_read_var(&SystemManager::efivars());
    }

    #[test]
    fn efivarfs_linux_get_var_names() {
        linux_get_var_names(&SystemManager::efivarfs());
    }

    #[test]
    fn efivarfs_linux_read_var() {
        linux_read_var(&SystemManager::efivarfs());
    }
}
