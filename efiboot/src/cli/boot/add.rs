use efivar::VarManager;

pub fn add(
    manager: Box<dyn VarManager>,
    partition: String,
    file_path: String,
    description: String,
    force: bool,
    id: Option<u16>,
) {
    panic!();
}
