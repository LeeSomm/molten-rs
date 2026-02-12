//! This module provides the core logic for validating a `Document` against its
//! corresponding `FormDefinition`.
//!
//! It includes functions to perform comprehensive checks on document data,
//! ensuring that all required fields are present, data types match, and
//! specific constraints (like numerical ranges or selection options) are met.
use crate::error::DocumentValidationError;
use molten_core::document::Document;
use molten_core::field::{FieldDefinition, FieldType};
use molten_core::form::FormDefinition;
use serde_json::Value;

/// Validates a `Document` against its `FormDefinition`.
///
/// This function performs a comprehensive check, ensuring:
/// 1. The `Document`'s `form_id` matches the `FormDefinition`'s ID.
/// 2. All `required` fields as defined in the `FormDefinition` are present and not null in the `Document`.
/// 3. Data types for each field in the `Document` match the `FieldType` specified in the `FormDefinition`.
/// 4. Specific constraints (e.g., `min`/`max` for numbers, `options` for selects, ISO 8601 for dates) are met.
///
/// # Arguments
/// * `doc` - A reference to the `Document` to be validated.
/// * `form` - A reference to the `FormDefinition` to validate against.
///
/// # Returns
/// A `Result` which is `Ok(())` if the document is valid, or `Err(Vec<DocumentValidationError>)`
/// containing a list of all validation errors found.
pub fn validate_document(
    doc: &Document,
    form: &FormDefinition,
) -> Result<(), Vec<DocumentValidationError>> {
    let mut errors = Vec::new();

    // 1. Guard: Form ID mismatch
    if doc.form_id != form.id() {
        errors.push(DocumentValidationError::FormIdMismatch {
            doc_form: doc.form_id.clone(),
            def_id: form.id().to_string(),
        });
        return Err(errors);
    }

    // 2. Iterate over every field defined in the schema
    for field_def in form.fields() {
        let value = doc.get_value(field_def.id());

        // Check Required
        if field_def.is_required() && (value.is_none() || value.unwrap().is_null()) {
            errors.push(DocumentValidationError::MissingRequiredField(
                field_def.id().to_string(),
            ));
            continue; // Cannot validate type if missing
        }

        // If value exists, validate its content
        if let Some(val) = value
            && !val.is_null()
        {
            if let Err(e) = validate_value(val, field_def) {
                errors.push(e);
            }
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Validates a single `serde_json::Value` against a `FieldDefinition`.
///
/// This private helper function checks the value's type and applies any constraints
/// specified in the `FieldDefinition` (e.g., numerical ranges, valid selection options,
/// or date format).
///
/// # Arguments
/// * `value` - A reference to the `serde_json::Value` to validate.
/// * `field` - A reference to the `FieldDefinition` to validate against.
///
/// # Returns
/// A `Result` which is `Ok(())` if the value is valid according to the field definition,
/// or `Err(DocumentValidationError)` if any validation rule is violated.
fn validate_value(value: &Value, field: &FieldDefinition) -> Result<(), DocumentValidationError> {
    match field.field_type() {
        FieldType::Text | FieldType::TextArea => {
            if !value.is_string() {
                return Err(DocumentValidationError::InvalidType {
                    field_id: field.id().to_string(),
                    expected_type: "String".to_string(),
                    got_type: get_json_type(value),
                });
            }
            // Future: Add Regex validation here
        }
        FieldType::Number { min, max } => {
            let num = value
                .as_f64()
                .ok_or_else(|| DocumentValidationError::InvalidType {
                    field_id: field.id().to_string(),
                    expected_type: "Number".to_string(),
                    got_type: get_json_type(value),
                })?;

            if let Some(min_val) = min
                && num < *min_val
            {
                return Err(DocumentValidationError::ValueTooLow {
                    field_id: field.id().to_string(),
                    value: num,
                    min: *min_val,
                });
            }
            if let Some(max_val) = max
                && num > *max_val
            {
                return Err(DocumentValidationError::ValueTooHigh {
                    field_id: field.id().to_string(),
                    value: num,
                    max: *max_val,
                });
            }
        }
        FieldType::Boolean => {
            if !value.is_boolean() {
                return Err(DocumentValidationError::InvalidType {
                    field_id: field.id().to_string(),
                    expected_type: "Boolean".to_string(),
                    got_type: get_json_type(value),
                });
            }
        }
        FieldType::Select {
            options,
            allow_multiple,
        } => {
            if *allow_multiple {
                // Expect an array of strings
                let arr = value
                    .as_array()
                    .ok_or_else(|| DocumentValidationError::InvalidType {
                        field_id: field.id().to_string(),
                        expected_type: "Array".to_string(),
                        got_type: get_json_type(value),
                    })?;

                for item in arr {
                    let s = item
                        .as_str()
                        .ok_or_else(|| DocumentValidationError::InvalidType {
                            field_id: field.id().to_string(),
                            expected_type: "String".to_string(),
                            got_type: get_json_type(item),
                        })?;
                    if !options.contains(&s.to_string()) {
                        return Err(DocumentValidationError::InvalidSelection {
                            field_id: field.id().to_string(),
                            value: s.to_string(),
                            allowed: options.clone(),
                        });
                    }
                }
            } else {
                // Expect a single string
                let s = value
                    .as_str()
                    .ok_or_else(|| DocumentValidationError::InvalidType {
                        field_id: field.id().to_string(),
                        expected_type: "String".to_string(),
                        got_type: get_json_type(value),
                    })?;

                if !options.contains(&s.to_string()) {
                    return Err(DocumentValidationError::InvalidSelection {
                        field_id: field.id().to_string(),
                        value: s.to_string(),
                        allowed: options.clone(),
                    });
                }
            }
        }
        FieldType::DateTime => {
            let s = value
                .as_str()
                .ok_or_else(|| DocumentValidationError::InvalidType {
                    field_id: field.id().to_string(),
                    expected_type: "String (ISO 8601)".to_string(),
                    got_type: get_json_type(value),
                })?;

            // Validate it parses as an ISO string
            if chrono::DateTime::parse_from_rfc3339(s).is_err() {
                return Err(DocumentValidationError::InvalidDateFormat {
                    field_id: field.id().to_string(),
                    value: s.to_string(),
                });
            }
        }
    }
    Ok(())
}

/// Helper function to get a string representation of a `serde_json::Value`'s type.
///
/// # Arguments
/// * `v` - A reference to the `serde_json::Value`.
///
/// # Returns
/// A `String` representing the JSON type (e.g., "String", "Number", "Boolean", "Null", "Array", "Object").
fn get_json_type(v: &Value) -> String {
    match v {
        Value::Null => "Null",
        Value::Bool(_) => "Boolean",
        Value::Number(_) => "Number",
        Value::String(_) => "String",
        Value::Array(_) => "Array",
        Value::Object(_) => "Object",
    }
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use molten_core::document::Document;
    use molten_core::field::{FieldBuilder, FieldType};
    use molten_core::form::FormBuilder;
    use serde_json::json;

    fn create_test_form() -> FormDefinition {
        FormBuilder::new("ticket", "Ticket")
            .add_field(
                FieldBuilder::new("title", "Title", FieldType::Text)
                    .required(true)
                    .build()
                    .unwrap(),
            )
            .add_field(
                FieldBuilder::new(
                    "severity",
                    "Severity",
                    FieldType::Number {
                        min: Some(1.0),
                        max: Some(5.0),
                    },
                )
                .build()
                .unwrap(),
            )
            .add_field(
                FieldBuilder::new(
                    "status",
                    "Status",
                    FieldType::Select {
                        options: vec!["Open".into(), "Closed".into()],
                        allow_multiple: false,
                    },
                )
                .build()
                .unwrap(),
            )
            .build()
            .unwrap()
    }

    #[test]
    fn test_valid_document() {
        let form = create_test_form();
        let mut doc = Document::new("doc1", "ticket", "flow_ticket");
        doc.set_value("title", json!("Server Down"));
        doc.set_value("severity", json!(3));
        doc.set_value("status", json!("Open"));

        assert!(validate_document(&doc, &form).is_ok());
    }

    #[test]
    fn test_missing_required() {
        let form = create_test_form();
        let doc = Document::new("doc1", "ticket", "flow_ticket");
        // "title" is missing!

        let res = validate_document(&doc, &form);
        assert!(res.is_err());
        let errs = res.unwrap_err();
        assert!(matches!(
            errs[0],
            DocumentValidationError::MissingRequiredField(_)
        ));
    }

    #[test]
    fn test_type_mismatch() {
        let form = create_test_form();
        let mut doc = Document::new("doc1", "ticket", "flow_ticket");
        doc.set_value("title", json!("Valid"));
        doc.set_value("severity", json!("Five")); // String instead of Number

        let res = validate_document(&doc, &form);
        assert!(res.is_err());
        assert!(format!("{:?}", res.unwrap_err()).contains("InvalidType"));
    }

    #[test]
    fn test_number_range() {
        let form = create_test_form();
        let mut doc = Document::new("doc1", "ticket", "flow_ticket");
        doc.set_value("title", json!("Valid"));
        doc.set_value("severity", json!(10)); // Max is 5!

        let res = validate_document(&doc, &form);
        assert!(res.is_err());
        assert!(matches!(
            res.unwrap_err()[0],
            DocumentValidationError::ValueTooHigh { .. }
        ));
    }

    #[test]
    fn test_select_options() {
        let form = create_test_form();
        let mut doc = Document::new("doc1", "ticket", "flow_ticket");
        doc.set_value("title", json!("Valid"));
        doc.set_value("status", json!("In Progress")); // Not in ["Open", "Closed"]

        let res = validate_document(&doc, &form);
        assert!(res.is_err());
        assert!(matches!(
            res.unwrap_err()[0],
            DocumentValidationError::InvalidSelection { .. }
        ));
    }
}
