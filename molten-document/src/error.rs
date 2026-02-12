//! Defines error types specific to document validation within the `molten-document` crate.
//!
//! This module provides a comprehensive set of error variants encapsulated by
//! `DocumentValidationError`, each detailing specific reasons why a document
//! might fail validation against its `FormDefinition`.
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents errors encountered during the document validation process.
#[derive(Error, Debug, PartialEq, Serialize, Deserialize)]
pub enum DocumentValidationError {
    /// Indicates that a required field was missing from the document or its value was null.
    #[error("Field '{0}' is required but was missing or null")]
    MissingRequiredField(String),

    /// Indicates a type mismatch for a field's value.
    #[error("Field '{field_id}' expected type '{expected_type}', but got '{got_type}'")]
    InvalidType {
        /// The ID of the field that had an invalid type.
        field_id: String,
        /// The expected data type for the field.
        expected_type: String,
        /// The actual data type received for the field.
        got_type: String,
    },

    /// Indicates that a numerical field's value is below its specified minimum.
    #[error("Field '{field_id}' value {value} is less than minimum {min}")]
    ValueTooLow {
        /// The ID of the numerical field.
        field_id: String,
        /// The value that was too low.
        value: f64,
        /// The minimum allowed value for the field.
        min: f64,
    },

    /// Indicates that a numerical field's value is above its specified maximum.
    #[error("Field '{field_id}' value {value} is greater than maximum {max}")]
    ValueTooHigh {
        /// The ID of the numerical field.
        field_id: String,
        /// The value that was too high.
        value: f64,
        /// The maximum allowed value for the field.
        max: f64,
    },

    /// Indicates that a selection field's value is not among the allowed options.
    #[error("Field '{field_id}' value '{value}' is not a valid option. Allowed: {allowed:?}")]
    InvalidSelection {
        /// The ID of the selection field.
        field_id: String,
        /// The invalid value received.
        value: String,
        /// The list of allowed options for the field.
        allowed: Vec<String>,
    },

    /// Indicates that a date/time field's value is not in a valid ISO 8601 format.
    #[error("Field '{field_id}' expected a date string (ISO 8601), but got '{value}'")]
    InvalidDateFormat {
        /// The ID of the date field.
        field_id: String,
        /// The invalid date string received.
        value: String,
    },

    /// Indicates that the document's `form_id` does not match the `FormDefinition`'s ID.
    #[error("Document form_id '{doc_form}' does not match definition id '{def_id}'")]
    FormIdMismatch {
        /// The `form_id` specified in the document.
        doc_form: String,
        /// The `id` specified in the `FormDefinition`.
        def_id: String,
    },
}
