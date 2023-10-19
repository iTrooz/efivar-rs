use crate::{efi::Variable, Error, VarReader};

/// asserts that the variable doesn't exist. Also validates the error
pub fn assert_var_not_found(manager: &mut dyn VarReader, var: &Variable) {
    if let Error::VarNotFound { var: error_var } = manager.read(var).unwrap_err() {
        assert_eq!(&error_var, var);
    } else {
        panic!("Reading a non-existent variable should raise VarNotFound");
    }
}
