//! # Molten Service
//!
//! `molten-service` provides the application orchestration layer for the Molten Document and
//! Workflow Management system. This crate coordinates between documents, workflows, and storage
//! to enforce business rules and manage the document lifecycle.
//!
//! This crate is under active development and is not yet stable.
//! If this crate has been abandoned, please message me and we can discuss ownership transfer.

#![warn(missing_docs)]
pub mod error;
pub mod services;

/// Re-exports of the service error types.
pub use error::ServiceError;
/// Re-exports of the Document service.
pub use services::DocumentService;
/// Re-exports of the Form service.
pub use services::FormService;
/// Re-exports of the Workflow service.
pub use services::WorkflowService;
