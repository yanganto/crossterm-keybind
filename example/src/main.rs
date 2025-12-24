use crossterm_keybind::KeyBind;
use crossterm_keybind::{KeyBindTrait, DisplayFormat};

/// Following are the keybindings to trigger event to control application.
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init keybind config from a file path,
    // The file can be customized from `"{}", KeyEvent::config_example()`
    KeyEvent::init_and_load(None)?;

    //  Following code will somewhere in your event handler
    //  ```rust
    //  let evt = ratatui::crossterm::event::read()?;
    //  if let Event::Key(key) = evt {
    //      if Quit.match_any(&key) {
    //          // Close the app
    //      }
    //
    //      for event in Event::dispatch(&key) {
    //          match event {
    //              Quit => {
    //                  // Close the app
    //              }
    //          }
    //      }
    //  }
    //  ```

    println!("--- Following are keybinds displays ---");
    // Display the keybind in your ui layer
    println!(
        "You can trigger Quit by {}",
        KeyEvent::Quit.key_bindings_display()
        // The same as KeyEvent::Quit.key_bindings_display_with_format(DisplayFormat::Symbols)
    );
    println!();

    println!("--- Following are keybinds displays with a verbose formator ---");
    // Display the keybind in your ui layer
    println!(
        "You can trigger Quit by {}",
        KeyEvent::Quit.key_bindings_display_with_format(DisplayFormat::Verbose)
    );
    println!();

    // Show the key bind config to your user
    println!("--- Following is the example config for keybinds ---");
    println!("{}", KeyEvent::toml_example());

    Ok(())
}
