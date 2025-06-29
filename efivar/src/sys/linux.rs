use std::fs;

mod efivarfs;
mod efivars;

use crate::efi::{Variable, VariableFlags};
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

    pub fn new() -> Result<SystemManager, crate::VarManagerInitError> {
        debug!("Initializing Linux EFI variable system manager");

        if !Self::is_empty(efivarfs::EFIVARFS_ROOT) {
            debug!("Using efivarfs interface at {}", efivarfs::EFIVARFS_ROOT);
            Ok(Self::efivars())
        } else if !Self::is_empty(efivars::EFIVARS_ROOT) {
            debug!("Using efivars interface at {}", efivars::EFIVARS_ROOT);
            Ok(Self::efivarfs())
        } else if cfg!(test) {
            debug!("Running in test mode, using efivars interface");
            Ok(Self::efivars())
        } else {
            debug!("EFI variables not available - no accessible interface found");
            Err(crate::VarManagerInitError::EFIVariablesNotAvailable)
        }
    }

    pub fn efivars() -> SystemManager {
        SystemManager {
            sys_impl: Box::new(efivarfs::SystemManager::new()),
        }
    }

    pub fn efivarfs() -> SystemManager {
        SystemManager {
            sys_impl: Box::new(efivars::SystemManager::new()),
        }
    }

    #[cfg(test)]
    fn supported(&self) -> bool {
        self.sys_impl.supported()
    }
}

impl VarEnumerator for SystemManager {
    fn get_all_vars<'a>(&'a self) -> crate::Result<Box<dyn Iterator<Item = Variable> + 'a>> {
        self.sys_impl.get_all_vars()
    }
}

impl VarReader for SystemManager {
    fn read(&self, var: &Variable) -> crate::Result<(Vec<u8>, VariableFlags)> {
        self.sys_impl.read(var)
    }
}

impl VarWriter for SystemManager {
    fn write(
        &mut self,
        var: &Variable,
        attributes: VariableFlags,
        value: &[u8],
    ) -> crate::Result<()> {
        self.sys_impl.write(var, attributes, value)
    }

    fn delete(&mut self, var: &Variable) -> crate::Result<()> {
        self.sys_impl.delete(var)
    }
}

impl VarManager for SystemManager {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::efi::Variable;

    fn linux_get_var_names(manager: &SystemManager) {
        if !manager.supported() {
            return;
        }

        let mut var_names = manager.get_all_vars().unwrap();
        let name = Variable::new("BootOrder");
        assert!(var_names.any(|n| n == name));
    }

    fn linux_read_var(manager: &SystemManager) {
        if !manager.supported() {
            return;
        }

        let (data, _flags) = manager
            .read(&Variable::new("BootOrder"))
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
