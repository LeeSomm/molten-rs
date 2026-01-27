//! # Molten Core
//!
//! `molten-core` provides the foundational domain models, traits, and types for the Molten
//! Document and Workflow Management system. This crate defines the contracts that all other
//! Molten crates implement and depend upon.
//!
//! This crate is under active development and is not yet stable.
//! If this crate has been abandoned, please message me and we can discuss ownership transfer.

#![warn(missing_docs)]

pub mod document;
pub mod field;
pub mod form;
pub mod workflow;

pub use document::Document;
pub use field::{FieldBuilder, FieldDefinition, FieldType};
pub use form::{FormBuilder, FormDefinition};
pub use workflow::{Phase, Transition, WorkflowDefinition};
