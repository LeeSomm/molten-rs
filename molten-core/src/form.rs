//! This module defines the core structures for managing `FormDefinition`s,
//! which serve as blueprints for documents within the Molten system.
//!
//! It includes `FormDefinition` to describe the overall structure of a form,
//! containing a collection of `FieldDefinition`s and associated validation rules.
//! The `FormBuilder` is provided for programmatic construction and validation
//! of form definitions.
use crate::field::FieldDefinition;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use validator::{Validate, ValidationError};

// Only alphanumeric, hyphens, and underscores
static ID_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap());

/// Defines the structure of a Form (the "Table Schema").
///
/// A Form is a collection of fields with a unique identifier and versioning.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(try_from = "FormBuilder")]
pub struct FormDefinition {
    /// The unique identifier for this form (e.g., "incident_report").
    /// ID must be between 1 and 64 characters with only alhpanumeric, hyphens, and underscores
    #[validate(length(min = 1, max = 64), regex(path = *ID_REGEX))]
    id: String,

    /// Human-readable name (e.g., "Incident Report").
    #[validate(length(min = 1, max = 100))]
    name: String,

    /// A version number for schema evolution.
    /// Useful when you update a form but want to keep old data compatible.
    version: u32,

    /// The list of fields that make up this form.
    ///
    /// # Validation
    /// 1. Each field must be valid (nested validation).
    /// 2. Field IDs must be unique within the form (custom validation).
    #[validate(nested, custom(function = "validate_unique_field_ids"))]
    fields: Vec<FieldDefinition>,
}

/// Custom validator to ensure no two fields share the same ID.
fn validate_unique_field_ids(fields: &[FieldDefinition]) -> Result<(), ValidationError> {
    let mut seen = HashSet::new();
    for field in fields {
        // We use the getter .id() because the field 'id' is private
        if !seen.insert(field.id()) {
            let mut err = ValidationError::new("duplicate_field_id");
            err.add_param(std::borrow::Cow::from("duplicate_id"), &field.id());
            return Err(err);
        }
    }
    Ok(())
}

impl FormDefinition {
    /// ID getter
    pub fn id(&self) -> &str {
        &self.id
    }
    /// Name getter
    pub fn name(&self) -> &str {
        &self.name
    }
    /// Version getter
    pub fn version(&self) -> u32 {
        self.version
    }
    /// Fields getter
    pub fn fields(&self) -> &[FieldDefinition] {
        &self.fields
    }
}

/// Builder for constructing validated [`FormDefinition`] instances.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormBuilder {
    /// The unique identifier for this form.
    pub id: String,
    /// Human-readable name for the form.
    pub name: String,
    #[serde(default = "default_version")]
    /// The version number for the form. Defaults to 1.
    pub version: u32,
    #[serde(default)]
    /// The list of field definitions that make up this form.
    pub fields: Vec<FieldDefinition>,
}

/// Provides the default version number for a form, which is `1`.
fn default_version() -> u32 {
    1
}

impl FormBuilder {
    /// Creates a new `FormBuilder` instance with the given ID and name,
    /// defaulting the version to 1 and fields to an empty list.
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            version: 1,
            fields: Vec::new(),
        }
    }

    /// Sets the version for the form.
    pub fn version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }

    /// Adds a `FieldDefinition` to the form.
    pub fn add_field(mut self, field: FieldDefinition) -> Self {
        self.fields.push(field);
        self
    }

    /// Replaces the current list of fields with a new vector of `FieldDefinition`s.
    pub fn with_fields(mut self, fields: Vec<FieldDefinition>) -> Self {
        self.fields = fields;
        self
    }

    /// Builds a validated `FormDefinition` from the `FormBuilder` instance.
    ///
    /// # Returns
    /// A `Result` containing the `FormDefinition` if valid, or a
    /// `validator::ValidationErrors` if validation fails.
    pub fn build(self) -> Result<FormDefinition, validator::ValidationErrors> {
        FormDefinition::try_from(self)
    }
}

impl TryFrom<FormBuilder> for FormDefinition {
    type Error = validator::ValidationErrors;

    fn try_from(builder: FormBuilder) -> Result<Self, Self::Error> {
        let form = FormDefinition {
            id: builder.id,
            name: builder.name,
            version: builder.version,
            fields: builder.fields,
        };

        form.validate()?;
        Ok(form)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field::{FieldBuilder, FieldType};
    use serde_json::json;

    fn create_field(id: &str) -> FieldDefinition {
        FieldBuilder::new(id, "Label", FieldType::Text)
            .build()
            .unwrap()
    }

    #[test]
    fn test_form_builder_valid() {
        let first_name = create_field("first_name");
        let last_name = create_field("last_name");

        // Test add_field
        let form = FormBuilder::new("user_profile", "User Profile")
            .add_field(first_name.clone())
            .add_field(last_name.clone())
            .build();

        assert!(form.is_ok());
        let form = form.unwrap();
        assert_eq!(form.fields.len(), 2);

        // Test with_fields
        let form = FormBuilder::new("user_profile", "User Profile")
            .with_fields(vec![first_name, last_name])
            .build();

        assert!(form.is_ok());
        let form = form.unwrap();
        assert_eq!(form.fields.len(), 2);
    }

    #[test]
    fn test_form_duplicate_fields() {
        // Try to add two fields with the ID "email"
        let form_res = FormBuilder::new("signup", "Sign Up")
            .add_field(create_field("email"))
            .add_field(create_field("email"))
            .build();

        assert!(form_res.is_err());

        let err = form_res.unwrap_err();
        assert!(err.to_string().contains("duplicate_field_id"));
    }

    #[test]
    fn test_serde_integration() {
        let json_input = json!({
            "id": "bug_report",
            "name": "Bug Report",
            "fields": [
                {
                    "id": "title",
                    "label": "Title",
                    "field_type": {
                        "kind": "text"
                    }
                },
                {
                    "id": "severity",
                    "label": "Severity",
                    "field_type": {
                        "kind": "number",
                        "config": { "min": 1.0, "max": 5.0 }
                    }
                }
            ]
        });

        let form: FormDefinition = serde_json::from_value(json_input).expect("Should deserialize");
        assert_eq!(form.id, "bug_report");
        assert_eq!(form.fields.len(), 2);
    }
}
