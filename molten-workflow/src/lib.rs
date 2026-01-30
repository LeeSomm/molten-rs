//! # Molten Workflow
//!
//! `molten-workflow` provides the workflow state machine engine, transition rules, and
//! workflow management for the Molten Document and Workflow Management system.
//!
//! This crate is under active development and is not yet stable.
//! If this crate has been abandoned, please message me and we can discuss ownership transfer.

#![warn(missing_docs)]
pub mod engine;
pub mod error;

pub use engine::transition;
pub use error::WorkflowError;
