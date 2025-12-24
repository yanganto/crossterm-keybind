//! This crate help you build tui with keybings config in an easy way.
//! When building a tui application, we need address following topics.
//! - `Define a set of keybing for some events`
//! - `Capture one/some key bindings and perform one the event`
//! - `Display a prompt of a key bindings`
//! - `Provide a config file and let user to change all or part of it`
//!
//! These topics can be abstract these into [`KeyBindTrait`], and the key binding serialize and deserialize
//! to a config file are solved in this crate.
//!
//! There still are a lot of trivial works, ahead you and your great ideal to build tui application.
//! This crate also provides derive macro [KeyBind](https://docs.rs/crossterm-keybind-derive/latest/crossterm_keybind_derive/derive.KeyBind.html)
//! to generate the the keyconfig in a supper easy way, you can have a toml key config for your
//! events and allow user to patch part of it.

mod error;
mod traits;

#[cfg(feature = "crossterm_0_28_1")]
pub use crossterm_0_28_1::event;
#[cfg(feature = "derive")]
pub use crossterm_keybind_derive::KeyBind;
pub use error::Error;
#[cfg(feature = "derive")]
pub use struct_patch;
#[cfg(feature = "derive")]
pub use toml;
#[cfg(feature = "derive")]
pub use toml_example;
pub use traits::{KeyBindTrait, DisplayFormat};

pub use crossterm_keybind_core::{KeyBinding, KeyBindings};
