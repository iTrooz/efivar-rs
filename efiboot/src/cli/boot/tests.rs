use core::panic;
use std::{fs::File, io::Write};

use efivar::{
    efi::{Variable, VariableFlags},
    store::MemoryStore,
    Error, VarReader, VarWriter,
};

use crate::{cli::Command, exit_code::ExitCode};

use super::*;

#[test]
fn list() {
    // normal list command
    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::from_iter(["efiboot", "list"]),
            &mut MemoryStore::new()
        )
    );

    // list namespace
    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::from_iter([
                "efiboot",
                "list",
                "-n",
                "f2aab986-4175-47bb-890a-3cba5f6d2547"
            ]),
            &mut MemoryStore::new()
        )
    );
}

#[test]
fn import() {
    let mut manager = MemoryStore::new();

    let tmpdir = tempfile::tempdir().unwrap();
    let file_path = tmpdir.path().join("in.bin");
    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(&[0x07, 0x00, 0x00, 0x00]).unwrap(); // write header
        file.write_all(&[0x01, 0x02, 0x03, 0x04]).unwrap(); // write content
    }

    // import variable
    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::from_iter([
                "efiboot",
                "import",
                file_path.to_str().unwrap(),
                "MyVariable",
            ]),
            &mut manager
        )
    );

    // Verify variable content
    let (output_data, flags) = manager.read(&Variable::new("MyVariable")).unwrap();
    assert_eq!(vec![0x01, 0x02, 0x03, 0x04], output_data);
    assert_eq!(
        flags,
        VariableFlags::NON_VOLATILE
            | VariableFlags::BOOTSERVICE_ACCESS
            | VariableFlags::RUNTIME_ACCESS
    );
}

#[test]
fn export() {
    let mut manager = MemoryStore::new();

    manager
        .write(
            &Variable::new("MyVariable"),
            VariableFlags::NON_VOLATILE
                | VariableFlags::BOOTSERVICE_ACCESS
                | VariableFlags::RUNTIME_ACCESS,
            &[0x01, 0x02, 0x03, 0x04],
        )
        .unwrap();

    let tmpdir = tempfile::tempdir().unwrap();
    let file_path = tmpdir.path().join("in.bin");

    // export variable
    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::from_iter([
                "efiboot",
                "export",
                "MyVariable",
                file_path.to_str().unwrap(),
            ]),
            &mut manager
        )
    );

    // Verify file content
    let output_data = std::fs::read(file_path).unwrap();
    assert_eq!(
        vec![0x07, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04],
        output_data
    );
}

#[test]
fn export_no_var() {
    //! Try `efiboot export` with a variable that doesn't exist

    let mut manager = MemoryStore::new();

    let tmpdir = tempfile::tempdir().unwrap();
    let file_path = tmpdir.path().join("in.bin");

    assert_eq!(
        ExitCode::FAILURE,
        crate::run(
            Command::from_iter([
                "efiboot",
                "export",
                "MyVariable",
                file_path.to_str().unwrap(),
            ]),
            &mut manager
        )
    );

    if let Error::VarNotFound { name } = manager.read(&Variable::new("MyVariable")).unwrap_err() {
        assert_eq!(name, Variable::new("MyVariable"));
    } else {
        panic!("Reading a non-existent variable should raise VarNotFound");
    }
}
