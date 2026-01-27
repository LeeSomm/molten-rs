use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum DocumentValidationError {
    #[error("Field '{0}' is required but was missing or null")]
    MissingRequiredField(String),

    #[error("Field '{field_id}' expected type '{expected_type}', but got '{got_type}'")]
    InvalidType {
        field_id: String,
        expected_type: String,
        got_type: String,
    },

    #[error("Field '{field_id}' value {value} is less than minimum {min}")]
    ValueTooLow {
        field_id: String,
        value: f64,
        min: f64,
    },

    #[error("Field '{field_id}' value {value} is greater than maximum {max}")]
    ValueTooHigh {
        field_id: String,
        value: f64,
        max: f64,
    },

    #[error("Field '{field_id}' value '{value}' is not a valid option. Allowed: {allowed:?}")]
    InvalidSelection {
        field_id: String,
        value: String,
        allowed: Vec<String>,
    },

    #[error("Field '{field_id}' expected a date string (ISO 8601), but got '{value}'")]
    InvalidDateFormat { field_id: String, value: String },

    #[error("Document form_id '{doc_form}' does not match definition id '{def_id}'")]
    FormIdMismatch { doc_form: String, def_id: String },
}
