pub trait KeyBindTrait {
    /// Initialize a default key bind config and optionally load a config from the path then patch it
    ///
    /// Please noted, this will be the first method you need to call before using `match_any`,
    /// `dispatch`, `key_bindings_display` or `key_bindings_display_with_format`, such that all the
    /// keybind from the user can be initialized.
    fn init_and_load(patch_path: Option<std::path::PathBuf>) -> Result<(), crate::Error>;

    /// Key event match for the key bindings
    ///
    /// Please noted, this method required `init_and_load` to run ahead.
    fn match_any(&self, key_event: &crate::event::KeyEvent) -> bool;

    /// Key config example for events
    fn toml_example() -> String;

    /// Export a file with key config example for events
    fn to_toml_example<P: AsRef<std::path::Path>>(file_name: P) -> std::io::Result<()>;

    /// Key bindings display
    ///
    /// Please noted, this method required `init_and_load` to run ahead.
    fn key_bindings_display(&self) -> String;

    /// Key bindings display with format
    ///
    /// Please noted, this method required `init_and_load` to run ahead.
    fn key_bindings_display_with_format(&self, f: &crossterm_keybind_core::DisplayFormat) -> String;

    /// Dispatch events from the key bindings
    ///
    /// Please noted, this method required `init_and_load` to run ahead.
    fn dispatch(key_event: &crate::event::KeyEvent) -> Vec<Self>
    where
        Self: Sized;
}
