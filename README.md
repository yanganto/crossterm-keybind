# Crossterm Keybind
[![Crates.io][crates-badge]][crate-url]
[![MIT licensed][mit-badge]][mit-url]
[![Docs][doc-badge]][doc-url]

This crate help you build tui with keybindings config in an easy way.

When building a tui application, we need address following topics.
- `Define a set of keybinding for some events`
- `Capture one/some keybindings and perform one the event`
- `Display a prompt of a keybinding`
- `Provide a config file and let user to change all or part of it`

These topics can be abstract these into a trait, and the key binding serialize and deserialize to a
config file are solved in this crate.

```rust
pub trait KeyBindTrait {
    fn init_and_load(patch_path: Option<PathBuf>) -> Result<(), Error>;
    fn match_any(&self, key_event: &KeyEvent) -> bool;
    fn toml_example() -> String;
    fn to_toml_example<P: AsRef<Path>>(file_name: P) -> std::io::Result<()>;
    fn key_bindings_display(&self) -> String;
}
```

However, there still are a lot of trivial works, ahead you and your great ideal to build tui application.
So this crate also provides a macro help you to generate the the keyconfig in a supper easy way,
you can have a toml key config for your events and allow user to patch part of it.
Following code snippets help you set up.


```toml
crossterm-keybind = { version = "*", features = ["derive"] }
serde = { version = "*", features = ["derive"] }
```

```rust
use crossterm_keybind::KeyBind;

#[derive(KeyBind)]
pub enum KeyEvent {
    /// The app will be closed with following key bindings
    /// - combin key Control and c
    /// - single key Q
    /// - single key q
    #[keybindings["Control+c", "Q", "q"]]
    Quit,
}
```

You can easy to use `Quit.match_any(&key)` in the control flow, and `Quit.key_bindings_display()` in the ui.
Besides, you can easy to provide a key bind config by `KeyEvent::toml_example()` or `KeyEvent::to_toml_example(path)` as following.

```toml
# Following are the keybindings to trigger event to control application
# The app will be closed with following key bindings
# - combin key Control and c
# - single key Q
# - single key q
quit = ["Control+c", "Q", "q"]

```

Then, user can customized the key as they need and the config can be initialized and load by `KeyEvent::init_and_load(key_config)`.
Please check the [example](./example) or a working example with [termshark](https://github.com/PRO-2684/termshark/pull/1).

### Dependency
If the project does not dependent on the latest `ratatui` or `crossterm`,
you can specific the version of ratatui or the version of crossterm as features in following way.
```toml
crossterm-keybind = { version = "*", default-features = false, features = ["ratatui_0_28_1", "derive"] } # work with ratatui 0.28.1
```

[crates-badge]: https://img.shields.io/crates/v/crossterm-keybind.svg
[crate-url]: https://crates.io/crates/crossterm-keybind
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/yanganto/crossterm-keybind/blob/readme/LICENSE
[doc-badge]: https://img.shields.io/badge/docs-rs-orange.svg
[doc-url]: https://docs.rs/crossterm-keybind/latest/crossterm_keybind/
