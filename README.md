# Crossterm Keybind
[![Crates.io][crates-badge]][crate-url]
[![MIT licensed][mit-badge]][mit-url]
[![Docs][doc-badge]][doc-url]


This crate help you build tui with keybings config in an easy way.

When building a tui application, we need address following topics.
- `Define a set of keybing for some events`
- `Capture one key bindings and perform one the event`
- `Display the prompt of key bindings`
- `Provide a config file and let user to change all or part of it`

These topics can be abstract these into a trait, and the key binding serialize and deserialize to a
config file are solved in this crate.

```rust
pub trait KeyBindTrait {
    fn init_and_load(patch_path: Option<PathBuf>) -> Result<(), Error>;
    fn match_any(&self, key_event: &KeyEvent) -> bool;
    fn config_example() -> String;
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
    /// Close the application
    #[keybindings["Control+c", "Q", "q"]]
    Quit,
}
```

You can easy to use `Quit.match_any(&key)` in the control flow, and `Quit.key_bindings_display()` in the ui.
The config can be initialized and customized by `KeyEvent::init_and_load(cli.key_config)`,
Besides, you can easy to provide a key bind config by `KeyEvent::config_example()`.
This is an example [termshark](https://github.com/PRO-2684/termshark/pull/1) to refactor a tui application with `crossterm-keybind`

### Dependency
If the project does not dependent on the latest `ratatui` or `crossterm`, you can specific the version as features in following way.
```toml
crossterm-keybind = { version = "*", default-features = false, features = ["ratatui_0_28_1", "derive"] } # work with ratatui 0.28.1
```
