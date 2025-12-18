# Crossterm Keybind
[![Crates.io][crates-badge]][crate-url]
[![MIT licensed][mit-badge]][mit-url]
[![Docs][doc-badge]][doc-url]

With growing userbases, developers of Terminal UI (TUI) apps often get requests for alternative
keybinding schemes (like vim-style bindings or personalized shortcuts). Manually supporting such
requests quickly becomes a maintenance burden, and as your app evolves, users expect their custom
keybinds to remain compatible across updates.  This crate help you build tui with keybindingsconfig in an easy way.

When building a tui application, we need address following topics.
- `Define a set of keybinding for some events`
- `Capture one/some keybindings and perform an event`
- `Display a keybinding prompt of an event`
- `Provide a config file and let user to change all or part of it`

These topics can be abstracted into a trait, and the key binding serialize and deserialize to a
config file are solved in this crate.

```rust
pub trait KeyBindTrait {
    fn init_and_load(patch_path: Option<PathBuf>) -> Result<(), Error>;
    fn match_any(&self, key_event: &KeyEvent) -> bool;
    fn toml_example() -> String;
    fn to_toml_example<P: AsRef<Path>>(file_name: P) -> std::io::Result<()>;
    fn key_bindings_display(&self) -> String;
    fn dispatch(key_event: &crate::event::KeyEvent) -> Vec<Self>;
}
```

However, there still are a lot of trivial works, ahead you and your great ideal to build tui application.
So this crate also provides a macro help you to generate the the keyconfig in a supper easy way,
you can have a toml file for your events and allow users to patch part of it.
Because users can patch part of config, your application will be backward compatible, if there are
only additions in the enum with KeyBind derive.

## Dive In
### Core Pattern
We use an approach that defines all keybindings in _a single enum_.

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

    /// A toggle to open/close a widget show all the commands
    #[keybindings["F1", "?"]]
    ToggleHelpWidget,
}
```

#### How to capture user input

You can easy to use `Quit.match_any(&key)` in the control flow, and `Quit.key_bindings_display()` in the ui.

In a less comparing way
```rust
if KeyBindEvent::Quit.match_any(&key) {
  // Close the app
} else if KeyBindEvent::ToggleHelpWidget.match_any(&key){
  // Show documents
}
```

or use dispatch in a full comparing way to get all possilbe enum variants

```rust
for event in KeyBindEvent::dispatch(&key) {
  match event {
    KeyBindEvent::Quit => {
      // Close the app
    },
    KeyBindEvent::ToggleHelpWidget => {
      // Show documents
    },
  }
}
```

#### How to provide the default config

You can easy to provide a key bind config by `KeyEvent::toml_example()` or `KeyEvent::to_toml_example(path)` as following.
We also take care the config file documentation

```toml
# The app will be closed with following key bindings
# - combin key Control and c
# - single key Q
# - single key q
quit = ["Control+c", "Q", "q"]

# A toggle to open/close a widget show all the commands
toggle_help_widget = ["F1", "?"]
 
```
Then, users can customize the key as they need and the config can be initialized and load by `KeyEvent::init_and_load(key_config)`.

#### How can users customize their keybinds

We additionally takes care of override issues using the struct-patch feature.

If the user only customized part of the key config, the system will patch the user's customized
settings onto the default ones. You can learn this in detail with the following use case.

```toml
quit = ["Control+q"]
```

The config can be loaded successfully. After loading, only `Control+q` can quit the application, and
the default keys `Control+c`, `Q`, `q` will not work anymore. The keybinds to open a widget will
remain the same as the default, because the user did not customize them, so the user can still use
`F1` or `?` to open the widget. You also get the benefit of backward compatibility for key configs,
if you only make additions to the key binding enum.

If the user only customized part of the key config, the system will patch the user's customized
settings onto the default ones. You can learn this in detail with the following use case.

### Dependency
If the project does not dependent on the latest `ratatui` or `crossterm`,
you can specific the version of ratatui or the version of crossterm as features in following way.
```toml
crossterm-keybind = { version = "*", default-features = false, features = ["ratatui_0_28_1", "derive"] } # work with ratatui 0.28.1
```

### Summary

With these approaches, the following features are supported:

Both crates support:

- **User Customization:** Let users adapt the app to their muscle memory and workflows.
- **Multiple Shortcuts:** Map several key combos to a single action.
- **Better User Experience:** Power users and international users can adjust keyboard layouts.
- **Backward Compatibility:** It can always be compatible with legacy configs, if we only make
  additions to the Enum.
- **Maintainability:** It is easy to keep a keybind config updated with the code.
- **Better Developer Experience:** Easy to setup default keybindings.
- **Flexible Keybindings:** It is possible to trigger multiple enum variants from one keybinding.

Please check the [Github Template](https://github.com/yanganto/ratatui-keybind-template), [example](./example), [ratatui-template](https://github.com/ratatui/templates/pull/124)  or a working PR with [termshark](https://github.com/PRO-2684/termshark/pull/1) to learn how to use it with ratatui.


[crates-badge]: https://img.shields.io/crates/v/crossterm-keybind.svg
[crate-url]: https://crates.io/crates/crossterm-keybind
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/yanganto/crossterm-keybind/blob/readme/LICENSE
[doc-badge]: https://img.shields.io/badge/docs-rs-orange.svg
[doc-url]: https://docs.rs/crossterm-keybind/latest/crossterm_keybind/
