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
pub mod document_service;

pub use document_service::DocumentService;
pub use error::ServiceError;