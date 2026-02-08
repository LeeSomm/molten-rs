//! This module defines the core structures for managing the definition and
//! behavior of individual fields within a form.
//!
//! It includes `FieldType` to enumerate the various data types a field can hold,
//! `FieldDefinition` to describe the metadata and validation rules for a field,
//! and `FieldBuilder` for constructing `FieldDefinition` instances programmatically.
use serde::{Deserialize, Serialize};
use validator::Validate;

/// The specific data type of a field.
///
/// This enum determines:
/// 1. How the data is validated.
/// 2. How the data is stored in the document JSON.
/// 3. How the UI renders the input (e.g., Checkbox vs Text Input).
///
/// # Serde Serialization
/// This enum uses "Adjacently Tagged" serialization.
///
/// **Example JSON:**
/// ```json
/// {
///   "kind": "select",
///   "config": {
///     "options": ["A", "B"],
///     "allow_multiple": false
///   }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "config", rename_all = "snake_case")]
pub enum FieldType {
    /// Standard single-line text input.
    Text,

    /// Multi-line text area, suitable for descriptions or comments.
    TextArea,

    /// Numerical input (integer or floating point).
    ///
    /// Optional validation constraints can be applied via `min` and `max`.
    Number {
        /// The minimum allowed value (inclusive).
        #[serde(default)]
        min: Option<f64>,
        /// The maximum allowed value (inclusive).
        #[serde(default)]
        max: Option<f64>,
    },

    /// A boolean flag (True/False).
    Boolean,

    /// A specific date and time.
    DateTime,

    /// A selection from a predefined list of options.
    Select {
        /// The list of valid strings a user can choose from.
        options: Vec<String>,
        /// If true, the user can select more than one option.
        #[serde(default)]
        allow_multiple: bool,
    },
}

/// Defines the validated schema and metadata for a single field in a Form.
///
/// A `FieldDefinition` does not hold the data itself; rather, it describes
/// what the data *should* look like.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
#[serde(try_from = "FieldBuilder")]
pub struct FieldDefinition {
    /// The unique key used to store this field's data in the Document.
    ///
    /// *Best Practice:* Use `snake_case` (e.g., `incident_date`, `employee_id`).
    /// validation: Must be between 1-64 characters
    #[validate(length(min = 1, max = 64))]
    id: String,

    /// The human-readable label displayed in the UI.
    /// validation: Must be between 1-100 characters
    #[validate(length(min = 1, max = 100))]
    label: String,

    /// The data type configuration.
    field_type: FieldType,

    /// If `true`, the document validation will fail if this field is missing or null.
    /// Applied globally to this field. For field requirements conditional on phase
    /// transitions, see documentation for [`crate::workflow::Transition`]
    required: bool,

    /// An optional tooltip or help text to guide the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
}

impl FieldDefinition {
    /// Getter method to obtain Field ID
    pub fn id(&self) -> &str {
        &self.id
    }
    /// Getter method to obtain Field Label
    pub fn label(&self) -> &str {
        &self.label
    }
    /// Getter method to obtain Field Type
    pub fn field_type(&self) -> &FieldType {
        &self.field_type
    }
    /// Getter method to return whether a field is required
    pub fn is_required(&self) -> bool {
        self.required
    }
    /// Getter method to obtain Field Description
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl TryFrom<FieldBuilder> for FieldDefinition {
    type Error = validator::ValidationErrors;

    fn try_from(builder: FieldBuilder) -> Result<Self, Self::Error> {
        let def = FieldDefinition {
            id: builder.id,
            label: builder.label,
            field_type: builder.field_type,
            required: builder.required,
            description: builder.description,
        };

        def.validate()?;

        Ok(def)
    }
}

/// Builder for constructing validated [`FieldDefinition`] instances.
///
/// `FieldBuilder` defines form fields and validates upon construction.
/// This allows callers to incrementally configure a field and receive a validated
/// [`FieldDefinition`] only once all required metadata has been supplied.
///
/// # Examples
/// ```
/// use molten_core::field::{FieldBuilder, FieldType};
///
/// let field = FieldBuilder::new(
///     "age",
///     "User Age",
///     FieldType::Number { min: Some(0.0), max: Some(120.0) },
/// )
/// .required(true)
/// .with_description("Please enter your age in years.")
/// .build()
/// .expect("Field definition should be valid");
/// ```
///
/// # Errors
/// Returns [`validator::ValidationErrors`] if the constructed
/// [`FieldDefinition`] violates any declared validation constraints.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldBuilder {
    id: String,
    label: String,
    field_type: FieldType,
    #[serde(default)]
    required: bool,
    description: Option<String>,
}

impl FieldBuilder {
    /// Creates a new FieldDefinition with default settings (optional, no description).
    pub fn new(id: &str, label: &str, field_type: FieldType) -> Self {
        Self {
            id: id.to_string(),
            label: label.to_string(),
            field_type,
            required: false,
            description: None,
        }
    }

    /// Sets the required flag.
    pub fn required(mut self, is_required: bool) -> Self {
        self.required = is_required;
        self
    }

    /// Adds a description/tooltip.
    pub fn with_description(mut self, description: &str) -> Self {
        self.description = Some(description.to_string());
        self
    }

    /// Creates a validated FieldDefinition entity using the builder pattern
    pub fn build(self) -> Result<FieldDefinition, validator::ValidationErrors> {
        FieldDefinition::try_from(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_field_builder() {
        let field = FieldBuilder::new("test_id", "Test Label", FieldType::Text)
            .required(true)
            .with_description("A test field")
            .build()
            .expect("Field builder should produce a valid FieldDefinition here.");

        assert_eq!(field.id, "test_id");
        assert_eq!(field.required, true);
        assert_eq!(field.description, Some("A test field".to_string()));
        assert!(matches!(field.field_type, FieldType::Text));
    }

    #[test]
    fn test_serialization_text() {
        // Test simple unit variant (Text)
        let field_type = FieldType::Text;
        let json = serde_json::to_value(&field_type).unwrap();

        // Adjacently tagged unit variants usually serialize to just the tag object
        // or tag with null config depending on specific serde behavior.
        // Let's verify exact output.
        assert_eq!(json, json!({ "kind": "text" }));
    }

    #[test]
    fn test_serialization_number_config() {
        // Test variant with config (Number)
        let field_type = FieldType::Number {
            min: Some(0.0),
            max: Some(100.0),
        };
        let json = serde_json::to_value(&field_type).unwrap();

        assert_eq!(
            json,
            json!({
                "kind": "number",
                "config": {
                    "min": 0.0,
                    "max": 100.0
                }
            })
        );
    }

    #[test]
    fn test_deserialization_select() {
        // Test parsing JSON back into Rust structs
        let json_input = json!({
            "id": "status",
            "label": "Status",
            "required": true,
            "field_type": {
                "kind": "select",
                "config": {
                    "options": ["Open", "Closed"],
                    "allow_multiple": true
                }
            }
        });

        let field: FieldDefinition = serde_json::from_value(json_input).unwrap();

        assert_eq!(field.id, "status");
        match field.field_type {
            FieldType::Select {
                options,
                allow_multiple,
            } => {
                assert_eq!(options, vec!["Open", "Closed"]);
                assert_eq!(allow_multiple, true);
            }
            _ => panic!("Wrong field type deserialized"),
        }
    }

    #[test]
    fn test_deserialization_defaults() {
        // Test that optional fields (min/max/required) use defaults if missing in JSON
        let json_input = json!({
            "id": "score",
            "label": "Score",
            "field_type": {
                "kind": "number",
                "config": {}
            }
        });

        let field: FieldDefinition =
            serde_json::from_value(json_input).expect("Field definition should be valid");

        assert_eq!(field.required, false); // Default is false

        match field.field_type {
            FieldType::Number { min, max } => {
                assert_eq!(min, None);
                assert_eq!(max, None);
            }
            _ => panic!("Wrong field type"),
        }
    }

    #[test]
    fn test_serde_validation_integration() {
        // 1. Valid JSON
        let valid_json = json!({
            "id": "short_id",
            "label": "Valid Label",
            "field_type": { "kind": "text" }
        });
        let result: Result<FieldDefinition, _> = serde_json::from_value(valid_json);
        assert!(result.is_ok());

        // 2. Invalid JSON (ID exceeds length constraint)
        let long_id =
            "this_id_is_way_too_long_to_pass_validation_rules_that_we_set_at_sixty_four_characters";
        let invalid_json = json!({
            "id": long_id,
            "label": "Invalid ID",
            "field_type": { "kind": "text" }
        });
        let result: Result<FieldDefinition, _> = serde_json::from_value(invalid_json);

        assert!(result.is_err());
        let err = result.unwrap_err();
        // Is it an input data-related error?
        assert!(err.is_data());
        // Is the error caused by invalid length?
        let err_string = err.to_string();
        assert!(err_string.contains("length"));

        // 3. Invalid JSON (Label exceeds length constraint )
        let long_label = "a".repeat(101);
        let invalid_json = json!({
            "id": "this_id_is_an_ok_length",
            "label": long_label,
            "field_type": { "kind": "text" }
        });
        let result: Result<FieldDefinition, _> = serde_json::from_value(invalid_json);

        assert!(result.is_err());
        let err = result.unwrap_err();
        // Is it an input data-related error?
        assert!(err.is_data());
        // Is the error caused by invalid length?
        let err_string = err.to_string();
        assert!(err_string.contains("length"));
    }
}
