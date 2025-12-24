#[cfg(feature = "crossterm_0_28_1")]
use crossterm_0_28_1::event::{KeyCode, KeyEvent, KeyModifiers, MediaKeyCode};

use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;
use str_utils::*;

#[derive(Default, PartialEq)]
pub enum DisplayFormat {
    /// use symbol for each key stroke
    #[default]
    Symbols,

    /// Debug print same as config format
    Debug,

    /// Display with full name of key
    Full,

    /// Display with abbreviated key names (e.g., 'Ctrl' instead of 'Control')
    Abbreviation
}

#[derive(PartialEq)]
pub struct KeyBinding {
    // TODO: Where is Space
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
            KeyModifiers::ALT => "Alternate+".to_string(),
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
    let s = s.trim();
    if s.len() == 1 {
        KeyCode::Char(s.chars().next().unwrap())
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
                if s.starts_with_ignore_ascii_case("Shift") {
                    key_bindings.modifiers = KeyModifiers::SHIFT;
                } else if s.starts_with_ignore_ascii_case("Control") || s.starts_with_ignore_ascii_case("Ctrl") {
                    key_bindings.modifiers = KeyModifiers::CONTROL;
                } else if s.starts_with_ignore_ascii_case("Alternate") || s.starts_with_ignore_ascii_case("Alt") {
                    key_bindings.modifiers = KeyModifiers::ALT;
                } else if s.starts_with_ignore_ascii_case("Super") {
                    key_bindings.modifiers = KeyModifiers::SUPER;
                } else if s.starts_with_ignore_ascii_case("Hyper") {
                    key_bindings.modifiers = KeyModifiers::HYPER;
                } else if s.starts_with_ignore_ascii_case("Meta") {
                    key_bindings.modifiers = KeyModifiers::META;
                } else {
                    error = Some(de::Error::custom(
                        "Currently only support following KeyModifiers: Shift, Control, Alternate, Super, Hyper, Meta"
                    ));
                }
                let mut splitter = s.splitn(2, '+');
                key_bindings.code = str_to_keycode(splitter.nth(1).unwrap());
            } else {
                key_bindings.code = str_to_keycode(&s);
            }
        })?;
        if let Some(e) = error {
            Err(e)
        } else if key_bindings.code == KeyCode::Null {
            Err(de::Error::custom(
                r#"Can not load a KeyCode, please use a char or one of following KeyCodes:
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

// ref: http://xahlee.info/comp/unicode_computing_symbols.html
// TODO add FormattingOptions for different layout,
// ex: Canadian Multilingual Layout, Truly Ergonomic Keyboard
impl fmt::Display for KeyBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.modifiers {
            KeyModifiers::SHIFT => write!(f, "\u{21e7}")?, //⇧
            KeyModifiers::CONTROL => write!(f, "^")?,
            KeyModifiers::ALT => write!(f, "\u{2387}")?, //⎇
            KeyModifiers::SUPER => write!(f, "\u{2756}")?, //❖
            KeyModifiers::HYPER => write!(f, "\u{2388}")?, //⎈
            KeyModifiers::META => write!(f, "\u{2318}")?, //⌘
            KeyModifiers::NONE => write!(f, "")?,
            _ => write!(f, "?")?,
        };
        match self.code {
            KeyCode::Char(c) => write!(f, "{}", c),
            KeyCode::Backspace => write!(f, "\u{232b}"), //⌫
            KeyCode::Enter => write!(f, "\u{23ce}"),     //⏎
            KeyCode::Left => write!(f, "\u{2190}"),      //←
            KeyCode::Right => write!(f, "\u{2192}"),     //→
            KeyCode::Up => write!(f, "\u{2191}"),        //↑
            KeyCode::Down => write!(f, "\u{2193}"),      //↓
            KeyCode::Home => write!(f, "\u{2912}"),      //⤒
            KeyCode::End => write!(f, "\u{2913}"),       //⤓
            KeyCode::PageUp => write!(f, "\u{21de}"),    //⇞
            KeyCode::PageDown => write!(f, "\u{21df}"),  //⇟
            KeyCode::Tab => write!(f, "\u{21e5}"),       //⇥
            KeyCode::BackTab => write!(f, "\u{21e4}"),   //⇤
            KeyCode::Delete => write!(f, "\u{2326}"),    //⌦
            KeyCode::Insert => write!(f, "\u{2380}"),    //⎀
            KeyCode::F(n) => write!(f, "F{}", n),
            KeyCode::Esc => write!(f, "\u{238b}"),          //⎋
            KeyCode::CapsLock => write!(f, "\u{1F130}"),    //🄰
            KeyCode::ScrollLock => write!(f, "\u{1F4DC}"),  //📜
            KeyCode::NumLock => write!(f, "\u{2460}"),      //①
            KeyCode::PrintScreen => write!(f, "\u{2399}"),  //⎙
            KeyCode::Pause => write!(f, "\u{2389}"),        //⎉
            KeyCode::Menu => write!(f, "\u{1F5C7}"),        //🗇
            KeyCode::KeypadBegin => write!(f, "\u{1F5CA}"), //🗊
            KeyCode::Media(MediaKeyCode::Play) => write!(f, "\u{23F5}"), //⏵
            KeyCode::Media(MediaKeyCode::PlayPause) => write!(f, "\u{23EF}"), //⏯
            KeyCode::Media(MediaKeyCode::Reverse) => write!(f, "\u{2B6F}"), //⭯
            KeyCode::Media(MediaKeyCode::Stop) => write!(f, "\u{23F9}"), //⏹
            KeyCode::Media(MediaKeyCode::FastForward) => write!(f, "\u{23ED}"), //⏭
            KeyCode::Media(MediaKeyCode::Rewind) => write!(f, "\u{2B6E}"), //⭮
            KeyCode::Media(MediaKeyCode::TrackNext) => write!(f, "\u{29D0}"), //⧐
            KeyCode::Media(MediaKeyCode::TrackPrevious) => write!(f, "\u{29CF}"), //⧏
            KeyCode::Media(MediaKeyCode::Record) => write!(f, "\u{241E}"), //␞
            KeyCode::Media(MediaKeyCode::LowerVolume) => write!(f, "\u{1F508}"), //🔈
            KeyCode::Media(MediaKeyCode::RaiseVolume) => write!(f, "\u{1F50A}"), //🔊
            KeyCode::Media(MediaKeyCode::MuteVolume) => write!(f, "\u{1F507}"), //🔇
            _ => write!(f, "?"),
        }
    }
}

impl fmt::Debug for KeyBinding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"\"")?;
        match self.modifiers {
            KeyModifiers::SHIFT => write!(f, "Shift+")?,
            KeyModifiers::CONTROL => write!(f, "Control+")?,
            KeyModifiers::ALT => write!(f, "Alternate+")?,
            KeyModifiers::SUPER => write!(f, "Super+")?,
            KeyModifiers::HYPER => write!(f, "Hyper+")?,
            KeyModifiers::META => write!(f, "Meta+")?,
            KeyModifiers::NONE => write!(f, "")?,
            _ => write!(f, "UNKNOWN+")?,
        };
        match self.code {
            KeyCode::Char(c) => write!(f, "{}", c)?,
            KeyCode::Backspace => write!(f, "Backspace")?,
            KeyCode::Enter => write!(f, "Enter")?,
            KeyCode::Left => write!(f, "Left")?,
            KeyCode::Right => write!(f, "Right")?,
            KeyCode::Up => write!(f, "Up")?,
            KeyCode::Down => write!(f, "Down")?,
            KeyCode::Home => write!(f, "Home")?,
            KeyCode::End => write!(f, "End")?,
            KeyCode::PageUp => write!(f, "PageUp")?,
            KeyCode::PageDown => write!(f, "PageDown")?,
            KeyCode::Tab => write!(f, "Tab")?,
            KeyCode::BackTab => write!(f, "BackTab")?,
            KeyCode::Delete => write!(f, "Delete")?,
            KeyCode::Insert => write!(f, "Insert")?,
            KeyCode::F(n) => write!(f, "F{}", n)?,
            KeyCode::Esc => write!(f, "Esc")?,
            KeyCode::CapsLock => write!(f, "CapsLock")?,
            KeyCode::ScrollLock => write!(f, "ScrollLock")?,
            KeyCode::NumLock => write!(f, "NumLock")?,
            KeyCode::PrintScreen => write!(f, "PrintScreen")?,
            KeyCode::Pause => write!(f, "Pause")?,
            KeyCode::Menu => write!(f, "Menu")?,
            KeyCode::KeypadBegin => write!(f, "KeypadBegin")?,
            KeyCode::Media(MediaKeyCode::Play) => write!(f, "Play")?,
            KeyCode::Media(MediaKeyCode::PlayPause) => write!(f, "PlayPause")?,
            KeyCode::Media(MediaKeyCode::Reverse) => write!(f, "Reverse")?,
            KeyCode::Media(MediaKeyCode::Stop) => write!(f, "Stop")?,
            KeyCode::Media(MediaKeyCode::FastForward) => write!(f, "FastForward")?,
            KeyCode::Media(MediaKeyCode::Rewind) => write!(f, "Rewind")?,
            KeyCode::Media(MediaKeyCode::TrackNext) => write!(f, "TrackNext")?,
            KeyCode::Media(MediaKeyCode::TrackPrevious) => write!(f, "TrackPrevious")?,
            KeyCode::Media(MediaKeyCode::Record) => write!(f, "Record")?,
            KeyCode::Media(MediaKeyCode::LowerVolume) => write!(f, "LowerVolume")?,
            KeyCode::Media(MediaKeyCode::RaiseVolume) => write!(f, "RaiseVolume")?,
            KeyCode::Media(MediaKeyCode::MuteVolume) => write!(f, "MuteVolume")?,
            _ => write!(f, "?")?,
        }
        write!(f,"\"")
    }
}

impl KeyBinding {
    pub fn display(&self, f: &DisplayFormat) -> String {
        match f {
            DisplayFormat::Symbols => format!("{}", self),
            DisplayFormat::Debug => format!("{:?}", self),
            DisplayFormat::Full | DisplayFormat::Abbreviation =>  {
                let mut display = match (f, self.modifiers) {
                    (_, KeyModifiers::SHIFT) => "Shift+".to_string(),
                    (DisplayFormat::Full, KeyModifiers::CONTROL) => "Control+".to_string(),
                    (DisplayFormat::Abbreviation, KeyModifiers::CONTROL) => "Ctrl+".to_string(),
                    (DisplayFormat::Full, KeyModifiers::ALT) => "Alternate+".to_string(),
                    (DisplayFormat::Abbreviation, KeyModifiers::ALT) => "Alt+".to_string(),
                    (_, KeyModifiers::SUPER) => "Super+".to_string(),
                    (_, KeyModifiers::HYPER) => "Hyper+".to_string(),
                    (_, KeyModifiers::META) =>  "Meta+".to_string(),
                    (_, KeyModifiers::NONE) => String::new(),
                    (_, _) => "UNKNOWN+".to_string(),
                };
                match self.code {
                    KeyCode::Char(c) => display.push(c),
                    KeyCode::Backspace => display.push_str("Backspace"),
                    KeyCode::Enter => display.push_str("Enter"),
                    KeyCode::Left => display.push_str("Left"),
                    KeyCode::Right => display.push_str("Right"),
                    KeyCode::Up => display.push_str("Up"),
                    KeyCode::Down => display.push_str("Down"),
                    KeyCode::Home => display.push_str("Home"),
                    KeyCode::End => display.push_str("End"),
                    KeyCode::PageUp => display.push_str("PageUp"),
                    KeyCode::PageDown => display.push_str("PageDown"),
                    KeyCode::Tab => display.push_str("Tab"),
                    KeyCode::BackTab => display.push_str("BackTab"),
                    KeyCode::Delete => display.push_str("Delete"),
                    KeyCode::Insert => display.push_str("Insert"),
                    KeyCode::F(n) => display.push_str(&format!("F{}", n)),
                    KeyCode::Esc => display.push_str("Esc"),
                    KeyCode::CapsLock => display.push_str("CapsLock"),
                    KeyCode::ScrollLock => display.push_str("ScrollLock"),
                    KeyCode::NumLock => display.push_str("NumLock"),
                    KeyCode::PrintScreen => display.push_str("PrintScreen"),
                    KeyCode::Pause => display.push_str("Pause"),
                    KeyCode::Menu => display.push_str("Menu"),
                    KeyCode::KeypadBegin => display.push_str("KeypadBegin"),
                    KeyCode::Media(MediaKeyCode::Play) => display.push_str("Play"),
                    KeyCode::Media(MediaKeyCode::PlayPause) => display.push_str("PlayPause"),
                    KeyCode::Media(MediaKeyCode::Reverse) => display.push_str("Reverse"),
                    KeyCode::Media(MediaKeyCode::Stop) => display.push_str("Stop"),
                    KeyCode::Media(MediaKeyCode::FastForward) => display.push_str("FastForward"),
                    KeyCode::Media(MediaKeyCode::Rewind) => display.push_str("Rewind"),
                    KeyCode::Media(MediaKeyCode::TrackNext) => display.push_str("TrackNext"),
                    KeyCode::Media(MediaKeyCode::TrackPrevious) => display.push_str("TrackPrevious"),
                    KeyCode::Media(MediaKeyCode::Record) => display.push_str("Record"),
                    KeyCode::Media(MediaKeyCode::LowerVolume) => display.push_str("LowerVolume"),
                    KeyCode::Media(MediaKeyCode::RaiseVolume) => display.push_str("RaiseVolume"),
                    KeyCode::Media(MediaKeyCode::MuteVolume) => display.push_str("MuteVolume"),
                    _ => display.push('?'),
                }
                display
            }
        }
    }
}

/// KeyBindings struct for key bind configure
#[derive(Serialize, Deserialize, PartialEq)]
pub struct KeyBindings(Vec<KeyBinding>);

impl KeyBindings {
    /// Match one of key bindings
    pub fn match_any(&self, key_event: &KeyEvent) -> bool {
        for key_bind in self.0.iter() {
            if key_bind.code == key_event.code && key_bind.modifiers == key_event.modifiers {
                return true;
            }
        }
        false
    }
}

impl fmt::Display for KeyBindings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, kb) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, "|{}", kb)?; // Add delimiter
            } else {
                write!(f, "{}", kb)?;
            }
        }
        Ok(())
    }
}

impl fmt::Debug for KeyBindings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        for (i, kb) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", {:?}", kb)?;
            } else {
                write!(f, "{:?}", kb)?;
            }
        }
        write!(f, "]")?;
        Ok(())
    }
}
impl KeyBindings {
    pub fn display(&self, f: &DisplayFormat) -> String {
        match f {
            DisplayFormat::Symbols => format!("{}", self),
            DisplayFormat::Debug => format!("{:?}", self),
            _ => {
                let mut display = String::new();
                for (i, kb) in self.0.iter().enumerate() {
                    if i > 0 {
                        display.push_str(" | ");
                        display.push_str(&kb.display(f));
                    } else {
                        display.push_str(&kb.display(f));
                    }
                }
                display
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize, Deserialize)]
    struct T {
        kb: KeyBinding,
    }
    #[derive(Serialize, Deserialize)]
    struct U {
        kbs: KeyBindings,
    }

    #[test]
    fn ser_keybinding_config() {
        let (t_with_ctrl_modifier, t_with_alt_modifier,t, only_modifiers, t_with_esc) = keybinding_configs();

        let serialized = toml::to_string(&t_with_ctrl_modifier).unwrap();
        assert_eq!(serialized, "kb = \"Control+c\"\n");

        let serialized = toml::to_string(&t_with_alt_modifier).unwrap();
        assert_eq!(serialized, "kb = \"Alternate+c\"\n");

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
    fn de_keybinding_config() {
        let (t_with_ctrl_modifier, t_with_alt_modifier, t, _only_modifiers, t_with_esc) = keybinding_configs();

        let serialized = toml::to_string(&t_with_ctrl_modifier).unwrap();
        let desered_t: T = toml::from_str(serialized.as_str()).unwrap();
        assert_eq!(desered_t.kb.code, t_with_ctrl_modifier.kb.code);
        assert_eq!(desered_t.kb.modifiers, t_with_ctrl_modifier.kb.modifiers);

        let serialized = toml::to_string(&t_with_alt_modifier).unwrap();
        let desered_t: T = toml::from_str(serialized.as_str()).unwrap();
        assert_eq!(desered_t.kb.code, t_with_alt_modifier.kb.code);
        assert_eq!(desered_t.kb.modifiers, t_with_alt_modifier.kb.modifiers);

        let serialized = toml::to_string(&t).unwrap();
        let desered_t: T = toml::from_str(serialized.as_str()).unwrap();
        assert_eq!(desered_t.kb.code, t.kb.code);
        assert_eq!(desered_t.kb.modifiers, t.kb.modifiers);

        let serialized = toml::to_string(&t_with_esc).unwrap();
        let desered_t: T = toml::from_str(serialized.as_str()).unwrap();
        assert_eq!(desered_t.kb.code, t_with_esc.kb.code);
        assert_eq!(desered_t.kb.modifiers, t_with_esc.kb.modifiers);
    }

    #[test]
    fn fmt_keybinding_config() {
        let (t_with_modifiers, _t_with_alt, _t, _only_modifiers, t_with_esc) = keybinding_configs();

        assert_eq!(format!("{}", t_with_modifiers.kb), "^c");
        assert_eq!(format!("{}", t_with_esc.kb), "⎋");

        assert_eq!(t_with_modifiers.kb.display(&DisplayFormat::Full), "Control+c");
        assert_eq!(t_with_modifiers.kb.display(&DisplayFormat::Abbreviation), "Ctrl+c");
        assert_eq!(t_with_esc.kb.display(&DisplayFormat::Full), "Esc");
    }

    #[test]
    fn ser_keybindings_config() {
        let config = keybindings_config();

        let serialized = toml::to_string(&config).unwrap();
        assert_eq!(serialized, "kbs = [\"Control+c\", \"Q\"]\n");
    }

    #[test]
    fn fmt_keybindings_config() {
        let config = keybindings_config();
        assert_eq!(format!("{}", config.kbs), "^c|Q");
        assert_eq!(config.kbs.display(&DisplayFormat::Full), "Control+c | Q");
        assert_eq!(config.kbs.display(&DisplayFormat::Abbreviation), "Ctrl+c | Q");
    }

    /// Return keybind config with modifiers, keybind without modifiers, only modifiers
    fn keybinding_configs() -> (T, T, T, T, T) {
        (
            T {
                kb: KeyBinding {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                },
            },
            T {
                kb: KeyBinding {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::ALT,
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

    /// Return keybind config with multiple keybindings
    fn keybindings_config() -> U {
        U {
            kbs: KeyBindings(vec![
                KeyBinding {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                },
                KeyBinding {
                    code: KeyCode::Char('Q'),
                    modifiers: KeyModifiers::NONE,
                },
            ]),
        }
    }
}
