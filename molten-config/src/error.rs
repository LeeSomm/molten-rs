use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Configuration build failed: {0}")]
    ConfigBuildError(#[from] config::ConfigError),

    #[error("Failed to read file '{0}': {1}")]
    FileReadError(String, std::io::Error),

    #[error("Unknown file extension '{0}'. Supported: .yaml, .yml, .json, .toml")]
    UnknownFormat(String),

    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("Validation failed: {0}")]
    ValidationErrors(#[from] validator::ValidationErrors),
}
