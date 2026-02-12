//! Re-exports of all SeaORM entities for convenient access.
//!
//! This module provides a convenient way to import all defined entities
//! within the `molten-storage-seaorm` crate using a single `use` statement.

pub use super::document::Entity as Document;
pub use super::form::Entity as Form;
pub use super::workflow::Entity as Workflow;
