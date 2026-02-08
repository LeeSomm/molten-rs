//! # Molten API
//!
//! `molten-api` is the web API interface for the Molten Document and Workflow Management system.
//! This crate provides Axum-based REST endpoints and handlers.
//!
//! This crate is under active development and is not yet stable.
//! If this crate has been abandoned, please message me and we can discuss ownership transfer.
#![warn(missing_docs)]

pub mod error;
pub mod handlers;
pub mod startup;
pub mod state;
pub mod telemetry;
