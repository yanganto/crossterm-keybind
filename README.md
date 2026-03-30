# Crossterm Keybind
[![Crates.io][crates-badge]][crate-url]
[![MIT licensed][mit-badge]][mit-url]
[![Docs][doc-badge]][doc-url]

With growing userbases, developers of Terminal UI (TUI)/ Graphic UI (GUI) apps often get requests for alternative
keybinding schemes (like vim-style bindings or personalized shortcuts). Manually supporting such
requests quickly becomes a maintenance burden, and as your app evolves, users expect their custom
keybinds to remain compatible across updates. This crate helps you build tui with keybindings config in an easy way.
One [recipe](https://www.ant-lab.tw/blog/2025-12-24/) with a migration guide for ratatui users is provided,
which is also under review in working [PR](https://github.com/ratatui/ratatui-website/pull/1008).

## Core Pattern
We use an approach that defines all keybindings in _a single enum_.

```rust
use crossterm_keybind::KeyBind;

#[derive(KeyBind)]
pub enum KeyEvent {
    /// The app will be closed with following key bindings
    /// - combined keys Control and c
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

In a less verbose way
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
# - combined keys Control and c
# - single key Q
# - single key q
quit = ["Control+c", "Q", "q"]

# A toggle to open/close a widget show all the commands
toggle_help_widget = ["F1", "?"]
```

### Initialization

Before dispatching key events, you must initialize the keybindings once at startup.
This loads the default bindings defined in your enum's `#[keybindings[...]]` attributes,
and optionally patches them with a user-supplied config.

**`init_and_load`** — use this when keybindings live in a dedicated config file managed
by this crate:

```rust
// No user config — use built-in defaults only
KeyEvent::init_and_load(None)?;

// Load defaults, then patch from a user-supplied keybind config file
KeyEvent::init_and_load(Some(PathBuf::from("~/.config/myapp/keybinds.toml")))?;
```

**`init_from_table`** — use this when your application already manages its own config
file (e.g. using an alternative storage format) and you just want to take advantage
of the macros and configuration merging.

```rust
// Parse your own config file
let config: toml::Table = toml::from_str(&std::fs::read_to_string("config.toml")?)?;

// Extract the [keybinds] section (if present) and pass it directly
let keybinds_table = config.get("keybinds").and_then(|v| v.as_table()).cloned();
KeyEvent::init_from_table(keybinds_table)?;
```

Both methods apply the same patching logic: only the keys present in the user config
override defaults; everything else falls back to the values declared in the enum.

### How users can customize their keybinds

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

### How to hint keybinds to user

To help users know what keys to use, you may want to display back to them the keybindings that they may
have also customized. Let's assume your enum is `KeyEvent` and it contains a custom `Quit` action. You
can leverage `key_bindings_display()` or `key_bindings_display_with_format()`, for example, like so:

```rust
#[derive(KeyBind)]
enum KeyEvent {
    #[keybindings["Control+c", "Q", "q"]]
    Quit,
}

// This is short-hand for key_bindings_display_with_format(DisplayFormat::Symbols)
let quit_str = KeyEvent::Quit::key_bindings_display();
println!("You can trigger Quit by {}", quit_str);
```

```text
--- key_bindings_display_with_format(DisplayFormat::Symbols) ---
You can trigger Quit by ^c|Q|q

--- key_bindings_display_with_format(DisplayFormat::Debug) ---
You can trigger Quit by ["Control+c", "Q", "q"]

--- key_bindings_display_with_format(DisplayFormat::Full) ---
You can trigger Quit by Control+c | Q | q

--- key_bindings_display_with_format(DisplayFormat::Abbreviation) ---
You can trigger Quit by Ctrl+c | Q | q
```

## Dependency

We need additional serde dependency at the same time.
```toml
# Cargo.toml
crossterm-keybind = "*"
serde = { version = "*", features = ["derive"] }
```

If the project does not depend on the latest `ratatui` or `crossterm`,
you can specify the version of ratatui or the version of crossterm as features in the following way.
```toml
crossterm-keybind = { 
  version = "*", 
  default-features = false, 
  features = ["ratatui_0_30_0",  "check", "case_ignore", "safety", "derive"]  # work with ratatui 0.30.0
}
```
Now supporting from `0.28.0` to `0.30.0` of ratatui, if you need another specific version, please open an issue.
Please check the [doc](https://github.com/yanganto/crossterm-keybind/blob/main/crossterm-keybind/Cargo.toml#L23-L31) of features, if you want to tailor the implementation from macro.

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
