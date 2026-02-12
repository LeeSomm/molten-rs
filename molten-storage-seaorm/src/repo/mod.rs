//! Repository implementations for interacting with Molten entities in the database.
//!
//! This module provides concrete implementations of the repository traits, using SeaORM
//! to perform CRUD operations for documents, forms, and workflows.

pub mod document;
pub mod form;
pub mod workflow;

// Re-export for easier access
pub use document::DocumentRepository;
pub use form::FormRepository;
pub use workflow::WorkflowRepository;
