mod config_trait;
mod string_serialized_trait;

pub use config_trait::toml::TomlConfigTrait;
pub use config_trait::{ConfigTrait, ValidationType};

pub use string_serialized_trait::SerializedItemTrait;
