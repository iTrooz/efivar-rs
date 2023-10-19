#[test]
fn set_next() {
    let manager = &mut MemoryStore::new();

    add_entry(manager, 0x0001);

    assert_eq!(
        ExitCode::SUCCESS,
        crate::run(
            Command::parse_from(["efiboot", "boot", "next", "set", "0001",]),
            manager,
        )
    );

    let (data, _) = manager.read(&Variable::new("BootNext")).unwrap();
    assert_eq!(data, utils::u16_to_u8(&[0x0001]));
}

#[test]
fn set_inexistent_next() {
    let manager = &mut MemoryStore::new();

    assert_eq!(
        ExitCode::FAILURE,
        crate::run(
            Command::parse_from(["efiboot", "boot", "next", "set", "0001",]),
            manager,
        )
    );

    assert!(!manager.exists(&Variable::new("BootNext")).unwrap());
}
