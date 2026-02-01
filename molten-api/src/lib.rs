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
pub mod state;

use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::DatabaseConnection;
use state::AppState;

/// Creates the Axum router with all routes and state attached.
/// This function is now testable without starting a real TCP listener.
pub fn create_app(db: DatabaseConnection) -> Router {
    let state = AppState::new(db);

    Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/documents", post(handlers::create_document))
        .route("/documents/{id}", get(handlers::get_document))
        .route("/forms", post(handlers::create_form))
        .with_state(state)
}
