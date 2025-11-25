mod key_bind;

#[cfg(all(feature = "crossterm_0_28_1", feature = "derive"))]
pub use crossterm_0_28_1::event;
#[cfg(feature = "derive")]
pub use crossterm_keybind_derive::KeyBind;
#[cfg(feature = "derive")]
pub use struct_patch;
#[cfg(feature = "derive")]
pub use toml;
#[cfg(feature = "derive")]
pub use toml_example;

pub use key_bind::{KeyBinding, KeyBindings};
