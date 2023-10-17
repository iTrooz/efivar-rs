use efivar::store::MemoryStore;

use crate::{cli::Command, exit_code::ExitCode};

use super::*;

#[test]
fn list() {
    // let manager = Box::new(MemoryStore::new());

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
