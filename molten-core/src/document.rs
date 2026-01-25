use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use validator::Validate;

/// Represents a single instance of a Form.
///
/// While `FormDefinition` defines the structure, `Document` holds the actual data.
/// The `data` field is a dynamic map where keys correspond to `FieldDefinition.id`.
#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Document {
    /// Unique identifier for this specific document (usually a UUID).
    #[validate(length(min = 1, max = 64))]
    pub id: String,

    /// Links this document to a specific Form Definition.
    #[validate(length(min = 1, max = 64))]
    pub form_id: String,

    /// The generic data payload.
    /// Key: Field ID (e.g., "incident_date")
    /// Value: The user input (String, Number, Boolean, etc.)
    ///
    /// Note: We do NOT validate the *content* of these values here.
    /// That requires the `FormDefinition` and happens in the `molten-document` crate.
    pub data: HashMap<String, Value>,

    /// Metadata: When this document was created.
    pub created_at: DateTime<Utc>,

    /// Metadata: When this document was last modified.
    pub updated_at: DateTime<Utc>,
}

impl Document {
    /// Creates a new, empty document for a specific form.
    pub fn new(id: &str, form_id: &str) -> Self {
        let now = Utc::now();
        Self {
            id: id.to_string(),
            form_id: form_id.to_string(),
            data: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Helper to get a value for a specific field ID.
    pub fn get_value(&self, field_id: &str) -> Option<&Value> {
        self.data.get(field_id)
    }

    /// Helper to set a value.
    pub fn set_value(&mut self, field_id: &str, value: Value) {
        self.data.insert(field_id.to_string(), value);
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_document_creation() {
        let mut doc = Document::new("doc_123", "incident_report");

        // Simulate user input
        doc.set_value("title", json!("Server Crash"));
        doc.set_value("severity", json!(5));

        assert_eq!(doc.id, "doc_123");
        assert_eq!(doc.data.get("title"), Some(&json!("Server Crash")));
    }

    #[test]
    fn test_serialization() {
        let mut doc = Document::new("doc_1", "form_1");
        doc.set_value("active", json!(true));

        let json_output = serde_json::to_string(&doc).unwrap();
        assert!(json_output.contains("doc_1"));
        assert!(json_output.contains("active"));
        assert!(json_output.contains("true"));
    }
}
