use crossterm_keybind::{DisplayFormat, KeyBind, KeyBindTrait};

// Each test uses a distinct enum type so the per-type BINDING_INIT static doesn't conflict
// between tests (each derived type gets its own static via the macro expansion).

/// Calling init_from_table(None) should succeed and apply the attribute-defined defaults.
#[test]
fn init_from_table_none_applies_defaults() {
    #[derive(KeyBind)]
    enum TestKeyBindings {
        #[keybindings["q", "Esc"]]
        Quit,
        #[keybindings["j", "Down"]]
        Next,
    }

    TestKeyBindings::init_from_table::<toml::Table>(None)
        .expect("init_from_table(None) should succeed");

    let example = TestKeyBindings::toml_example();
    let quit_bindings =
        TestKeyBindings::Quit.key_bindings_display_with_format(&DisplayFormat::Full);
    let next_bindings =
        TestKeyBindings::Next.key_bindings_display_with_format(&DisplayFormat::Full);

    assert!(
        example.contains("quit"),
        "toml_example should contain 'quit'"
    );
    assert!(
        example.contains("next"),
        "toml_example should contain 'next'"
    );

    assert!(quit_bindings.contains('q'), "'q' key was bound");
    assert!(quit_bindings.contains("Esc"), "\"Esc\" key was bound");
    assert!(next_bindings.contains('j'), "'j' key was bound");
    assert!(next_bindings.contains("Down"), "\"Down\" key was bound");
}

/// Calling init_from_table(Some(table)) should patch the defaults with the provided table.
#[test]
fn init_from_table_some_patches_defaults() {
    #[derive(KeyBind)]
    enum TestKeyBindings {
        #[keybindings["q", "Esc"]]
        Quit,
    }

    // Build a table that overrides the "quit" binding to only "x"
    let mut table = toml::Table::new();
    table.insert(
        "quit".to_string(),
        toml::Value::Array(vec![toml::Value::String("x".to_string())]),
    );

    TestKeyBindings::init_from_table(Some(table))
        .expect("init_from_table(Some(table)) should succeed");

    // The display for Quit should reflect the patched "x" binding
    let bindings = TestKeyBindings::Quit.key_bindings_display_with_format(&DisplayFormat::Full);
    assert!(
        !bindings.contains('q'),
        "'q' default wasn't bound in this case"
    );
    assert!(
        !bindings.contains("Esc"),
        "'Esc' default wasn't bound in this case"
    );
    assert!(
        bindings.contains('x'),
        "patched binding should show 'x', got: {bindings}"
    );
}

/// Calling init_from_table with an invalid keybind string should return LoadConfigError.
#[test]
fn init_from_table_invalid_keybind_returns_error() {
    #[derive(KeyBind)]
    enum TestKeyBindings {
        #[keybindings["q"]]
        Quit,
    }

    let mut table = toml::Table::new();
    table.insert(
        "quit".to_string(),
        toml::Value::Array(vec![toml::Value::String("NotARealKey".to_string())]),
    );

    let result = TestKeyBindings::init_from_table(Some(table));
    assert!(
        matches!(result, Err(crossterm_keybind::Error::LoadConfigError(_))),
        "expected LoadConfigError for invalid keybind, got: {result:?}"
    );
}

/// init_from_table and init_and_load both set the same init guard — calling one then the
/// other on the same type should return ConfigDoubleInitError (safety feature only).
#[test]
#[cfg(feature = "safety")]
fn init_from_table_prevents_double_init() {
    #[derive(KeyBind)]
    enum TestKeyBindings {
        #[keybindings["q"]]
        Quit,
    }

    TestKeyBindings::init_from_table::<toml::Table>(None).expect("first init should succeed");

    let second = TestKeyBindings::init_and_load(None);
    assert!(
        matches!(second, Err(crossterm_keybind::Error::ConfigDoubleInitError)),
        "expected ConfigDoubleInitError on second init, got: {second:?}"
    );
}
