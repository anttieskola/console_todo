#[derive(Debug,PartialEq)]
pub enum AppCmd {
    Quit,
    None,
    AddMissingText,
    ToggleMissingIndex,
    Add(String),
    Toggle(usize),
}
