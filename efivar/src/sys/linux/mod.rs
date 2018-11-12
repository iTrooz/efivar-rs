use std::io;
use std::fs;

mod efivarfs;
mod efivars;

use ::{VarManager, VarEnumerator, VarReader, VarWriter};
use efi::VariableFlags;

pub struct SystemManager {
    sys_impl: Box<dyn VarManager>
}

impl SystemManager {
    fn is_empty(p: &str) -> bool {
        !fs::read_dir(p).map(|mut list| list.any(|_item| true)).unwrap_or(false)
    }

    pub fn new() -> SystemManager {
        if !Self::is_empty(efivars::EFIVARS_ROOT) {
            SystemManager { sys_impl: Box::new(efivars::SystemManager::new()) }
        } else if !Self::is_empty(efivarfs::EFIVARFS_ROOT) {
            SystemManager { sys_impl: Box::new(efivarfs::SystemManager::new()) }
        } else {
            panic!("Failed to determine if efivars or efivarfs should be used. Please check permissions to /sys/firmware/efi");
        }
    }
}

impl VarEnumerator for SystemManager {
    fn get_var_names(&self) -> io::Result<Vec<String>> {
        self.sys_impl.get_var_names()
    }
}

impl VarReader for SystemManager {
    fn read(&self, name: &str) -> io::Result<(VariableFlags, Vec<u8>)> {
        self.sys_impl.read(name)
    }
}

impl VarWriter for SystemManager {
    fn write(&mut self, name: &str, attributes: VariableFlags, value: &[u8]) -> io::Result<()> {
        self.sys_impl.write(name, attributes, value)
    }
}

impl VarManager for SystemManager {}
