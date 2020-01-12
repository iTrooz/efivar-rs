use efivar::VarManager;

pub fn run(enumerator: Box<dyn VarManager>) {
    for var in enumerator
        .get_var_names()
        .expect("Failed to list variable names")
    {
        println!("{}", var.short_name());
    }
}
