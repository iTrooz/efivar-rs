use clap::Parser;
use efivar::store::MemoryStore;

use crate::{
    cli::{boot::tests::standard_setup, Command},
    exit_code::ExitCode,
};

#[test]
fn get_entries() {
    let manager = &mut MemoryStore::new();

    standard_setup(manager, 0x0001);

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efiboot", "boot", "get-entries"]),
            manager,
        )
    );
}
