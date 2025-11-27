mod error;
mod key_bind;
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
pub use traits::KeyBindTrait;

pub use key_bind::{KeyBinding, KeyBindings};
