pub trait KeyBindTrait {
    /// init a default key bind config and optionally load a config from the path then patch it
    fn init_and_load(patch_path: Option<std::path::PathBuf>) -> Result<(), crate::Error>;

    /// Key event match for the key bindings
    fn match_any(&self, key_event: &crate::event::KeyEvent) -> bool;

    /// Key config example for events
    fn toml_example() -> String;

    /// Export a file with key config example for events
    fn to_toml_example<P: AsRef<std::path::Path>>(file_name: P) -> std::io::Result<()>;

    /// Key bindings display
    fn key_bindings_display(&self) -> String;

    /// Key bindings display with format
    fn key_bindings_display_with_format(&self, f: DisplayFormat) -> String;

    /// Dispatch events from the key bindings
    fn dispatch(key_event: &crate::event::KeyEvent) -> Vec<Self>
    where
        Self: Sized;
}

#[derive(Default, PartialEq)]
pub enum DisplayFormat {
    #[default]
    Symbols,
    Verbose,
}
