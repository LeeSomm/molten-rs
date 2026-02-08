//! This module serves as a re-export module for various API handlers within the `molten-api` crate.
//!
//! It provides a consolidated place to access handlers for Document, Form, and Workflow entities,
//! simplifying imports and promoting a cleaner API surface for routing.
/// API Handler for CRUD operations on the Document entity
pub mod document;
/// API Handler for CRUD operations on the Form entity
pub mod form;
/// API Handler for CRUD operations on the Workflow entity
pub mod workflow;

pub use document::{create_document, get_document};
pub use form::{create_form, get_form};
pub use workflow::{create_workflow, get_workflow};
