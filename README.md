# Crossterm Keybind
[![Crates.io][crates-badge]][crate-url]
[![MIT licensed][mit-badge]][mit-url]
[![Docs][doc-badge]][doc-url]

With growing userbases, developers of Terminal UI (TUI)/ Graphic UI (GUI) apps often get requests for alternative
keybinding schemes (like vim-style bindings or personalized shortcuts). Manually supporting such
requests quickly becomes a maintenance burden, and as your app evolves, users expect their custom
keybinds to remain compatible across updates. This crate helps you build tui with keybindings config in an easy way.
One [recipe](https://www.ant-lab.tw/blog/2025-12-24/) with migration guilde for ratatui user are provided,
which is also under review in working [PR](https://github.com/ratatui/ratatui-website/pull/1008).

## Core Pattern
We use an approach that defines all keybindings in _a single enum_.

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

### How to capture user input

You can easily use `Quit.match_any(&key)` in the control flow,

In a less comparing way
```rust
if KeyBindEvent::Quit.match_any(&key) {
  // Close the app
} else if KeyBindEvent::ToggleHelpWidget.match_any(&key){
  // Show documents
}
```

or use dispatch in a full comparing way to get all possible enum variants

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

### How to provide the default config

You can easily provide a key bind config **with documentation** by `KeyEvent::toml_example()` or
`KeyEvent::to_toml_example(path)` as following.  We also take care the config file documentation

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

### How can users customize their keybinds

We additionally take care of override issues using the struct-patch feature.

If the user only customized part of the key config, the system will patch the user's customized
settings onto the default ones. You can learn this in detail with the following use case.

```toml
quit = ["Control+q"]
```

The config can be loaded successfully. After loading, only `Control+q` can quit the application, and
the default keys `Control+c`, `Q`, `q` will not work anymore. The keybinds to open a widget will
remain the same as the default, because the user did not customize them, so the user can still use
`F1` or `?` to open the widget. You also get the benefit of **backward compatibility** for key configs,
if you only make additions to the key binding enum.

### How to hint user the keybinds

An app with customized keybinding, user may be confuse to use the app when the keybind is changed, 
it will be nice to hint user current keybind for Quit by `Quit.key_bindings_display()`(same as symbols format),
`Quit.key_bindings_display_with_format(DisplayFormat::Symbols)` or
`Quit.key_bindings_display_with_format(DisplayFormat::Verbose)` in the ui.

```text
--- Following are keybinds displays with symbols---
You can trigger Quit by ^c|Q|q

--- Following are keybinds displays with verbose format ---
You can trigger Quit by Control+c | Q | q

```

## Dependency
We need additional serde dependency at the same time.
```toml
# Cargo.toml
crossterm-keybind = { version = "*", features = ["derive"] }
serde = { version = "*", features = ["derive"] }
```

If the project does not dependent on the latest `ratatui` or `crossterm`,
you can specific the version of ratatui or the version of crossterm as features in following way.
```toml
crossterm-keybind = { version = "*", default-features = false, features = ["ratatui_0_28_1", "derive"] } # work with ratatui 0.28.1
```

## Summary

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
