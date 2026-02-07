//! # Molten Config
//!
//! `molten-config` provides configuration schema parsing, validation, and runtime configuration
//! management for the Molten Document and Workflow Management system. Supports YAML, TOML,
//! and JSON configuration formats.
//!
//! This crate is under active development and is not yet stable.
//! If this crate has been abandoned, please message me and we can discuss ownership transfer.

#![warn(missing_docs)]
pub mod entity_parser;
pub mod error;
pub mod settings_parser;

use molten_core::form::FormDefinition;
use molten_core::workflow::WorkflowDefinition;
use std::path::Path;

pub use entity_parser::{ConfigFormat, load_from_file, parse_content};
pub use error::ConfigError;

// -----------------------------------------------------------------------------
// Convenience Helpers
// -----------------------------------------------------------------------------

/// Helper to specifically load a Form Definition from a file.
pub fn load_form(path: impl AsRef<Path>) -> Result<FormDefinition, ConfigError> {
    load_from_file(path.as_ref())
}

/// Helper to specifically load a Workflow Definition from a file.
pub fn load_workflow(path: impl AsRef<Path>) -> Result<WorkflowDefinition, ConfigError> {
    load_from_file(path.as_ref())
}
