use crate::field::FieldDefinition;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use validator::{Validate, ValidationError};

/// Defines the structure of a Form (the "Table Schema").
///
/// A Form is a collection of fields with a unique identifier and versioning.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(try_from = "FormBuilder")]
pub struct FormDefinition {
    /// The unique identifier for this form (e.g., "incident_report").
    #[validate(length(min = 1, max = 64))]
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
    // Getters
    pub fn id(&self) -> &str {
        &self.id
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn version(&self) -> u32 {
        self.version
    }
    pub fn fields(&self) -> &[FieldDefinition] {
        &self.fields
    }
}

/// Builder for constructing validated [`FormDefinition`] instances.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormBuilder {
    pub id: String,
    pub name: String,
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub fields: Vec<FieldDefinition>,
}

fn default_version() -> u32 {
    1
}

impl FormBuilder {
    pub fn new(id: &str, name: &str) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            version: 1,
            fields: Vec::new(),
        }
    }

    pub fn version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }

    pub fn add_field(mut self, field: FieldDefinition) -> Self {
        self.fields.push(field);
        self
    }

    pub fn with_fields(mut self, fields: Vec<FieldDefinition>) -> Self {
        self.fields = fields;
        self
    }

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
                        "type": "text"
                    }
                },
                {
                    "id": "severity",
                    "label": "Severity",
                    "field_type": {
                        "type": "number",
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
