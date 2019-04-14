use efivar::efi::parse_name;
use efivar::efi::EFI_GUID;
use efivar::VarManager;

pub fn run(enumerator: Box<dyn VarManager>) {
    for var in enumerator
        .get_var_names()
        .expect("Failed to list variable names")
    {
        // Parse vendor GUID and variable name
        let (guid, name) = parse_name(&var).unwrap();
        if guid == EFI_GUID {
            // EFI variable, print short name
            println!("{}", name);
        } else {
            // Vendor variable, print full name
            println!("{}", var);
        }
    }
}
