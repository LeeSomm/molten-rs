use crate::error::ConfigError;
use serde::de::DeserializeOwned;
use std::fs;
use std::path::Path;
use validator::Validate;

/// Parser for entities defined in molten-core

/// Supported configuration formats.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigFormat {
    Yaml,
    Json,
    Toml,
}

impl ConfigFormat {
    /// Infers format from file extension.
    pub fn from_extension(ext: &str) -> Option<Self> {
        match ext.to_lowercase().as_str() {
            "yaml" | "yml" => Some(Self::Yaml),
            "json" => Some(Self::Json),
            "toml" => Some(Self::Toml),
            _ => None,
        }
    }
}

/// Generic parser that loads, deserializes, and validates any config struct.
///
/// # Type Parameters
/// * `T`: The struct to parse (e.g., `FormDefinition`). Must implement `Deserialize` and `Validate`.
pub fn load_from_file<T>(path: &Path) -> Result<T, ConfigError>
where
    T: DeserializeOwned + Validate,
{
    let content = fs::read_to_string(path)
        .map_err(|e| ConfigError::FileReadError(path.display().to_string(), e))?;

    let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    let format = ConfigFormat::from_extension(ext)
        .ok_or_else(|| ConfigError::UnknownFormat(ext.to_string()))?;

    parse_content(&content, format)
}

/// Parses string content directly. Useful for API payloads or tests.
///
/// # Type Parameters
/// * `T`: The struct to parse (e.g., `FormDefinition`). Must implement `Deserialize` and `Validate`.
pub fn parse_content<T>(content: &str, format: ConfigFormat) -> Result<T, ConfigError>
where
    T: DeserializeOwned + Validate,
{
    // 1. Deserialize based on format
    let entity: T = match format {
        ConfigFormat::Yaml => serde_yaml::from_str(content)?,
        ConfigFormat::Json => serde_json::from_str(content)?,
        ConfigFormat::Toml => toml::from_str(content)?,
    };

    // 2. Validate (This triggers the logic in molten-core)
    // Note: If you used `#[serde(try_from)]` in molten-core, basic validation
    // already happened during step 1. But explicit calling here covers structs
    // that might use simple `derive(Validate)`.
    entity.validate().map_err(ConfigError::ValidationErrors)?;

    Ok(entity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use molten_core::field::{FieldDefinition, FieldType};
    use molten_core::form::FormDefinition; // Just to ensure types exist

    // A sample valid YAML form
    const VALID_YAML_FORM: &str = r#"
id: incident_report
name: Incident Report
version: 1
fields:
  - id: title
    label: Incident Title
    field_type: 
        kind: text
    required: true
  - id: severity
    label: Severity Level
    field_type: 
        kind: number
        config:
            min: 1
            max: 5
"#;

    #[test]
    fn test_parse_yaml_form() {
        let form: FormDefinition =
            parse_content(VALID_YAML_FORM, ConfigFormat::Yaml).expect("Should parse valid YAML");

        assert_eq!(form.id(), "incident_report");
        assert_eq!(form.fields().len(), 2);
    }

    #[test]
    fn test_parse_json_form() {
        // Equivalent JSON
        let json_form = r#"{
            "id": "incident_report",
            "name": "Incident Report",
            "version": 1,
            "fields": [
                { "id": "title", "label": "Title", "field_type": {"kind": "text"}, "required": true }
            ]
        }"#;

        let form: FormDefinition =
            parse_content(json_form, ConfigFormat::Json).expect("Should parse valid JSON");

        assert_eq!(form.id(), "incident_report");
    }

    #[test]
    fn test_validation_trigger() {
        // Invalid Form (ID has space, which violates regex/validation rules in Core)
        let invalid_yaml = r#"
id: incident report 
name: Report
fields: []
"#;
        let res: Result<FormDefinition, ConfigError> =
            parse_content(invalid_yaml, ConfigFormat::Yaml);

        assert!(res.is_err());
        // The error should come from the underlying validation logic
        let err_msg = res.unwrap_err().to_string();
        assert!(err_msg.contains("Validation failed") || err_msg.contains("id"));
    }
}
