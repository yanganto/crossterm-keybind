pub trait KeyBindTrait {
    /// init a default key bind config and optionally load a config from the path then patch it
    fn init_and_load(patch_path: Option<std::path::PathBuf>) -> Result<(), crate::Error>;

    /// Key event match for the key bindings
    fn match_any(&self, key_event: &crate::event::KeyEvent) -> bool;

    /// Key config example for events
    fn config_example() -> String;

    /// Key bindings display
    fn key_bindings_display(&self) -> String;
}
