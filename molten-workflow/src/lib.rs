//! # Molten Workflow
//!
//! `molten-workflow` provides the workflow state machine engine, transition rules, and
//! workflow management for the Molten Document and Workflow Management system.
//!
//! This crate is under active development and is not yet stable.
//! If this crate has been abandoned, please message me and we can discuss ownership transfer.

#![warn(missing_docs)]
/// Provides the core workflow engine logic, including the `transition` function.
pub mod engine;
/// Defines custom error types specific to workflow operations.
pub mod error;

/// Re-exports the main workflow transition function from the `engine` module.
pub use engine::transition;
/// Re-exports the `WorkflowError` enum from the `error` module.
pub use error::WorkflowError;
