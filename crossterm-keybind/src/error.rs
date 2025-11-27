#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("can not init keybind config more than once")]
    ConfigDoubleInitError,
    #[error("can not init keybind config with the default keybindings")]
    DefaultConfigError(String),
    #[error("can not read keybind config")]
    ReadConfigError(#[from] std::io::Error),
    #[error("can not load keybind config")]
    LoadConfigError(#[from] toml::de::Error),
}
