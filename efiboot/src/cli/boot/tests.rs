use crate::exit_code::ExitCode;

use super::*;

#[test]
fn list() {
    // // normal list command
    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(crate::Opt::from_iter(["efiboot", "list"]))
    );

    // list namespace
    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(crate::Opt::from_iter([
            "efiboot",
            "list",
            "-n",
            "f2aab986-4175-47bb-890a-3cba5f6d2547"
        ]))
    );
}
