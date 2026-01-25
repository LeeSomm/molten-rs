//! # Molten
//!
//! `molten` is a configurable document and workflow management system. This crate provides
//! a convenient entry point with re-exports from the core Molten crates.
//!
//! For most users, this is the crate you want to add to your dependencies.
//!
//! ## Architecture
//!
//! Molten is composed of several focused crates:
//! - `molten-core` - Domain models and traits
//! - `molten-document` - Document management
//! - `molten-workflow` - Workflow state machine
//! - `molten-config` - Configuration parsing
//! - `molten-service` - Application orchestration
//! - `molten-storage-seaorm` - SeaORM storage implementation
//! - `molten-api` - Web API (Axum)
//!
//! This crate is under active development and is not yet stable.
//! If this crate has been abandoned, please message me and we can discuss ownership transfer.

#![warn(missing_docs)]
// Re-export the most commonly used types
// pub use molten_core::*;
// pub use molten_document::*;
// pub use molten_workflow::*;
// pub use molten_service::*;

// // Re-export entire modules for advanced users
// pub mod config {
//     pub use molten_config::*;
// }

// pub mod storage {
//     pub use molten_storage_seaorm::*;
// }
