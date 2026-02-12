//! This module serves as a re-export module for various services within the `molten-service` crate.
//!
//! It provides a consolidated place to access services for Document, Form, and Workflow entities.

pub mod document;
pub mod form;
pub mod workflow;

pub use document::DocumentService;
pub use form::FormService;
pub use workflow::WorkflowService;
