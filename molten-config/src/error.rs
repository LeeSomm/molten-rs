//! Defines the error types specific to the `molten-config` crate.
//!
//! This module encapsulates various errors that can occur during configuration
//! parsing, loading, and validation, providing a centralized and
//! `thiserror`-driven approach to error handling for configuration operations.
use thiserror::Error;

/// Represents errors that occur during configuration parsing, loading, and validation.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Error originating from the `config` crate when building configuration.
    #[error("Configuration build failed: {0}")]
    ConfigBuildError(#[from] config::ConfigError),

    /// Error indicating a failure to read a configuration file from the filesystem.
    #[error("Failed to read file '{0}': {1}")]
    FileReadError(String, std::io::Error),

    /// Error for unsupported or unknown file extensions during configuration loading.
    #[error("Unknown file extension '{0}'. Supported: .yaml, .yml, .json, .toml")]
    UnknownFormat(String),

    /// Error encountered during YAML deserialization.
    #[error("YAML parsing error: {0}")]
    YamlError(#[from] serde_yaml::Error),

    /// Error encountered during JSON deserialization.
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Error encountered during TOML deserialization.
    #[error("TOML parsing error: {0}")]
    TomlError(#[from] toml::de::Error),

    /// Error due to validation failures of configuration entities.
    #[error("Validation failed: {0}")]
    ValidationErrors(#[from] validator::ValidationErrors),
}
