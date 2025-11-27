use crossterm_keybind::KeyBind;
use crossterm_keybind::KeyBindTrait;

#[derive(KeyBind)]
pub enum KeyEvent {
    /// The app will be closed with following key bindings
    /// - combin key Control and c
    /// - single key Q
    /// - single key q
    #[keybindings["Control+c", "Q", "q"]]
    Quit,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Init keybind coonfig with some user's file
    KeyEvent::init_and_load(None)?;

    //  Following code will somewhere in your event handler
    //  if ratatui::crossterm::event::poll() {
    //      if let Event::Key(key) = evt {
    //          if Quit.match_any(&key) {
    //              Close the app
    //          }
    //      }
    //  }

    // Display the keybind in your ui layer
    println!("{}", KeyEvent::Quit.key_bindings_display());

    // Show the key bind config to your user
    println!("{}", KeyEvent::config_example());

    Ok(())
}
