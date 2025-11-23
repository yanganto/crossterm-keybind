#[cfg(feature = "crossterm_0_27")]
use crossterm_0_27::event::{KeyCode, KeyModifiers, MediaKeyCode};
#[cfg(feature = "crossterm_0_28_1")]
use crossterm_0_28_1::event::{KeyCode, KeyModifiers, MediaKeyCode};

use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};

struct KeyBinding {
    pub code: KeyCode,
    pub modifiers: KeyModifiers,
}

impl Serialize for KeyBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = match self.modifiers {
            KeyModifiers::SHIFT => "Shift+".to_string(),
            KeyModifiers::CONTROL => "Control+".to_string(),
            KeyModifiers::ALT => "Alt+".to_string(),
            KeyModifiers::SUPER => "Super+".to_string(),
            KeyModifiers::HYPER => "Hyper+".to_string(),
            KeyModifiers::META => "Meta+".to_string(),
            // TODO support more KeyModifiers
            _ => String::new(),
        };
        match self.code {
            KeyCode::Char(c) => s.push(c),
            KeyCode::Backspace => s.push_str("Backspace"),
            KeyCode::Enter => s.push_str("Enter"),
            KeyCode::Left => s.push_str("Left"),
            KeyCode::Right => s.push_str("Right"),
            KeyCode::Up => s.push_str("Up"),
            KeyCode::Down => s.push_str("Down"),
            KeyCode::Home => s.push_str("Home"),
            KeyCode::End => s.push_str("End"),
            KeyCode::PageUp => s.push_str("PageUp"),
            KeyCode::PageDown => s.push_str("PageDown"),
            KeyCode::Tab => s.push_str("Tab"),
            KeyCode::BackTab => s.push_str("BackTab"),
            KeyCode::Delete => s.push_str("Delete"),
            KeyCode::Insert => s.push_str("Insert"),
            KeyCode::F(n) => s.push_str(&format!("F{n}")),
            KeyCode::Esc => s.push_str("Esc"),
            KeyCode::CapsLock => s.push_str("CapsLock"),
            KeyCode::ScrollLock => s.push_str("ScrollLock"),
            KeyCode::NumLock => s.push_str("NumLock"),
            KeyCode::PrintScreen => s.push_str("PrintScreen"),
            KeyCode::Pause => s.push_str("Pause"),
            KeyCode::Menu => s.push_str("Menu"),
            KeyCode::KeypadBegin => s.push_str("KeypadBegin"),
            KeyCode::Media(media_keycode) => s.push_str(&media_keycode.to_string()),
            _ => {
                return Err(ser::Error::custom(format!(
                    "Unsupported KeyCode: {:}",
                    self.code
                )))
            }
        }
        serializer.serialize_str(&s)
    }
}

fn str_to_keycode(s: &str) -> KeyCode {
    // TODO: handle space
    if s.len() == 1 {
        KeyCode::Char(s.chars().nth(0).unwrap())
    } else if s == "Backspace" {
        KeyCode::Backspace
    } else if s == "Enter" {
        KeyCode::Enter
    } else if s == "Left" {
        KeyCode::Left
    } else if s == "Right" {
        KeyCode::Right
    } else if s == "Up" {
        KeyCode::Up
    } else if s == "Down" {
        KeyCode::Down
    } else if s == "Home" {
        KeyCode::Home
    } else if s == "End" {
        KeyCode::End
    } else if s == "PageUp" {
        KeyCode::PageUp
    } else if s == "PageDown" {
        KeyCode::PageDown
    } else if s == "Tab" {
        KeyCode::Tab
    } else if s == "BackTab" {
        KeyCode::BackTab
    } else if s == "Delete" {
        KeyCode::Delete
    } else if s == "Insert" {
        KeyCode::Insert
    } else if s == "F1" {
        KeyCode::F(1)
    } else if s == "F2" {
        KeyCode::F(2)
    } else if s == "F3" {
        KeyCode::F(3)
    } else if s == "F4" {
        KeyCode::F(4)
    } else if s == "F5" {
        KeyCode::F(5)
    } else if s == "F6" {
        KeyCode::F(6)
    } else if s == "F7" {
        KeyCode::F(7)
    } else if s == "F8" {
        KeyCode::F(8)
    } else if s == "F9" {
        KeyCode::F(9)
    } else if s == "F10" {
        KeyCode::F(10)
    } else if s == "F11" {
        KeyCode::F(11)
    } else if s == "F12" {
        KeyCode::F(12)
    } else if s == "Esc" {
        KeyCode::Esc
    } else if s == "CapsLock" {
        KeyCode::CapsLock
    } else if s == "ScrollLock" {
        KeyCode::ScrollLock
    } else if s == "NumLock" {
        KeyCode::NumLock
    } else if s == "PrintScreen" {
        KeyCode::PrintScreen
    } else if s == "Pause" {
        KeyCode::Pause
    } else if s == "Menu" {
        KeyCode::Menu
    } else if s == "KeypadBegin" {
        KeyCode::KeypadBegin
    } else if s == "Play" {
        KeyCode::Media(MediaKeyCode::Play)
    } else if s == "PlayPause" {
        KeyCode::Media(MediaKeyCode::PlayPause)
    } else if s == "Reverse" {
        KeyCode::Media(MediaKeyCode::Reverse)
    } else if s == "Stop" {
        KeyCode::Media(MediaKeyCode::Stop)
    } else if s == "FastForward" {
        KeyCode::Media(MediaKeyCode::FastForward)
    } else if s == "Rewind" {
        KeyCode::Media(MediaKeyCode::Rewind)
    } else if s == "TrackNext" {
        KeyCode::Media(MediaKeyCode::TrackNext)
    } else if s == "TrackPrevious" {
        KeyCode::Media(MediaKeyCode::TrackPrevious)
    } else if s == "Record" {
        KeyCode::Media(MediaKeyCode::Record)
    } else if s == "LowerVolume" {
        KeyCode::Media(MediaKeyCode::LowerVolume)
    } else if s == "RaiseVolume" {
        KeyCode::Media(MediaKeyCode::RaiseVolume)
    } else if s == "MuteVolume" {
        KeyCode::Media(MediaKeyCode::MuteVolume)
    } else {
        KeyCode::Null
    }
}

impl<'de> Deserialize<'de> for KeyBinding {
    fn deserialize<D>(deserializer: D) -> Result<KeyBinding, D::Error>
    where
        D: Deserializer<'de>,
    {
        let mut key_bindings = KeyBinding {
            code: KeyCode::Null,
            modifiers: KeyModifiers::NONE,
        };
        let mut error = None;

        <String as Deserialize>::deserialize(deserializer).map(|s| {
            if s.contains('+') {
                if s.starts_with("Shift") {
                    key_bindings.modifiers = KeyModifiers::SHIFT;
                } else if s.starts_with("Control") {
                    key_bindings.modifiers = KeyModifiers::CONTROL;
                } else if s.starts_with("Alt") {
                    key_bindings.modifiers = KeyModifiers::ALT;
                } else if s.starts_with("Super") {
                    key_bindings.modifiers = KeyModifiers::SUPER;
                } else if s.starts_with("Hyper") {
                    key_bindings.modifiers = KeyModifiers::HYPER;
                } else if s.starts_with("Meta") {
                    key_bindings.modifiers = KeyModifiers::META;
                } else {
                    error = Some(de::Error::custom(
                        "Currently only support following KeyModifiers: Shift, Control, Alt, Super, Hyper, Meta"
                    ));
                }
                let mut splitter = s.splitn(2, '+');
                let key_code_str = splitter.nth(1).unwrap();
                key_bindings.code = str_to_keycode(&key_code_str);
            } else {
                key_bindings.code = str_to_keycode(&s);
            }
        })?;
        if let Some(e) = error {
            Err(e)
        } else if key_bindings.code == KeyCode::Null {
            Err(de::Error::custom(
                r#"Can not load KeyCode, please use chars and following key
"Backspace", "Enter", "Left", "Right", "Up", "Down", "Home", "End", "PageUp", "PageDown", "Tab", "BackTab", "Delete",
"Insert", "F1" ~ "F12", "Esc", "CapsLock", "ScrollLock", "NumLock", "PrintScreen", "Pause", "Menu", "KeypadBegin",
"Play", "PlayPause", "Reverse", "Stop", "FastForward", "Rewind", "TrackNext", "TrackPrevious", "Record", "LowerVolume", 
"RaiseVolume", "MuteVolume""#,
            ))
        } else {
            Ok(key_bindings)
        }
    }
}

/// KeyBindings struct for key bind configure
#[derive(Serialize, Deserialize)]
pub struct KeyBindings {
    /// More than one key binding for an event
    key_bindings: Vec<KeyBinding>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Serialize, Deserialize)]
    struct T {
        kb: KeyBinding,
    }

    #[test]
    fn ser_keybind_config() {
        let (t_with_modifiers, t, only_modifiers, t_with_esc) = keybind_configs();

        let serialized = toml::to_string(&t_with_modifiers).unwrap();
        assert_eq!(serialized, "kb = \"Control+c\"\n");

        let serialized = toml::to_string(&t).unwrap();
        assert_eq!(serialized, "kb = \"Q\"\n");

        let serialized = toml::to_string(&only_modifiers);
        assert_eq!(
            serialized,
            Err(ser::Error::custom("Unsupported KeyCode: Null"))
        );

        let serialized = toml::to_string(&t_with_esc).unwrap();
        assert_eq!(serialized, "kb = \"Esc\"\n");
    }

    #[test]
    fn de_keybind_config() {
        let (t_with_modifiers, t, _only_modifiers, t_with_esc) = keybind_configs();

        let serialized = toml::to_string(&t_with_modifiers).unwrap();
        let desered_t: T = toml::from_str(serialized.as_str()).unwrap();
        assert_eq!(desered_t.kb.code, t_with_modifiers.kb.code);
        assert_eq!(desered_t.kb.modifiers, t_with_modifiers.kb.modifiers);

        let serialized = toml::to_string(&t).unwrap();
        let desered_t: T = toml::from_str(serialized.as_str()).unwrap();
        assert_eq!(desered_t.kb.code, t.kb.code);
        assert_eq!(desered_t.kb.modifiers, t.kb.modifiers);

        let serialized = toml::to_string(&t_with_esc).unwrap();
        let desered_t: T = toml::from_str(serialized.as_str()).unwrap();
        assert_eq!(desered_t.kb.code, t_with_esc.kb.code);
        assert_eq!(desered_t.kb.modifiers, t_with_esc.kb.modifiers);
    }

    /// Return keybind config with modifiers, keybind without modifiers, only modifiers
    fn keybind_configs() -> (T, T, T, T) {
        (
            T {
                kb: KeyBinding {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                },
            },
            T {
                kb: KeyBinding {
                    code: KeyCode::Char('Q'),
                    modifiers: KeyModifiers::NONE,
                },
            },
            T {
                kb: KeyBinding {
                    code: KeyCode::Null,
                    modifiers: KeyModifiers::ALT,
                },
            },
            T {
                kb: KeyBinding {
                    code: KeyCode::Esc,
                    modifiers: KeyModifiers::NONE,
                },
            },
        )
    }
}
