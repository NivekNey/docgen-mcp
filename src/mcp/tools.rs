//! Tool handlers for document validation and generation
//!
//! This module provides MCP tools for validating and generating documents.
//! Currently implements:
//! - `validate_resume` - Validates JSON payload against resume schema
//! - `generate_resume` - Generates a PDF from a resume JSON payload

use base64::{Engine as _, engine::general_purpose};
use rmcp::model::Tool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::documents::Resume;
use crate::typst::compiler::compile;
use crate::typst::transform::transform_resume;

/// Tool name for resume validation
pub const VALIDATE_RESUME_TOOL: &str = "validate_resume";

/// Tool name for resume generation
pub const GENERATE_RESUME_TOOL: &str = "generate_resume";

/// Result of a validation operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum ValidationResult {
    /// Validation succeeded
    #[serde(rename = "valid")]
    Valid {
        /// The validated resume (echoed back for confirmation)
        resume: Box<Resume>,
    },
    /// Validation failed with errors
    #[serde(rename = "invalid")]
    Invalid {
        /// List of validation errors
        errors: Vec<ValidationError>,
    },
}

/// Result of a generation operation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "status")]
pub enum GenerationResult {
    /// Generation succeeded
    #[serde(rename = "success")]
    Success {
        /// Base64-encoded PDF data
        pdf_base64: String,
    },
    /// Generation failed (validation or compilation error)
    #[serde(rename = "error")]
    Error {
        /// Error message
        message: String,
        /// Validation errors if applicable
        #[serde(skip_serializing_if = "Option::is_none")]
        validation_errors: Option<Vec<ValidationError>>,
    },
}

/// A single validation error with location information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    /// JSON path to the error location (e.g., "basics.email", "work[0].company")
    pub path: String,
    /// Human-readable error message
    pub message: String,
}

impl ValidationError {
    /// Create a new validation error
    pub fn new(path: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            path: path.into(),
            message: message.into(),
        }
    }
}

/// Returns a list of all available tools
pub fn list_tools() -> Vec<Tool> {
    // Shared schema for resume input
    let mut resume_prop = serde_json::Map::new();
    resume_prop.insert("type".to_string(), Value::String("object".to_string()));
    resume_prop.insert(
        "description".to_string(),
        Value::String("The resume JSON payload. Use resources/read on 'docgen://schemas/resume' to get the full schema.".to_string()),
    );

    let mut properties = serde_json::Map::new();
    properties.insert("resume".to_string(), Value::Object(resume_prop.clone()));

    let mut schema = serde_json::Map::new();
    schema.insert("type".to_string(), Value::String("object".to_string()));
    schema.insert("properties".to_string(), Value::Object(properties));
    schema.insert(
        "required".to_string(),
        Value::Array(vec![Value::String("resume".to_string())]),
    );

    let schema_arc = Arc::new(schema);

    let validate_tool = Tool::new(
        VALIDATE_RESUME_TOOL,
        "Validates a resume JSON payload against the schema without generating a document. Returns validation errors with paths if invalid, or confirms validity.",
        schema_arc.clone(),
    );

    let generate_tool = Tool::new(
        GENERATE_RESUME_TOOL,
        "Generates a PDF resume from a JSON payload. Returns base64-encoded PDF data.",
        schema_arc,
    );

    vec![validate_tool, generate_tool]
}

/// Input for the validate_resume tool
#[derive(Debug, Deserialize)]
pub struct ValidateResumeInput {
    pub resume: Value,
}

/// Validates a resume JSON payload
///
/// Uses serde deserialization to validate the payload against the Resume type.
/// Returns structured validation errors if the payload is invalid.
pub fn validate_resume(input: Value) -> ValidationResult {
    // First, parse the tool input wrapper
    let parsed_input: ValidateResumeInput = match serde_json::from_value(input.clone()) {
        Ok(v) => v,
        Err(e) => {
            return ValidationResult::Invalid {
                errors: vec![ValidationError::new(
                    "",
                    format!(
                        "Invalid tool input: expected object with 'resume' field. {}",
                        e
                    ),
                )],
            };
        }
    };

    // Then validate the resume payload itself
    match serde_json::from_value::<Resume>(parsed_input.resume) {
        Ok(resume) => ValidationResult::Valid {
            resume: Box::new(resume),
        },
        Err(e) => ValidationResult::Invalid {
            errors: parse_serde_error(&e),
        },
    }
}

/// Generates a PDF resume from a JSON payload
pub fn generate_resume(input: Value) -> GenerationResult {
    // 1. Validate
    let validation_result = validate_resume(input);

    let resume = match validation_result {
        ValidationResult::Valid { resume } => resume,
        ValidationResult::Invalid { errors } => {
            return GenerationResult::Error {
                message: "Validation failed".to_string(),
                validation_errors: Some(errors),
            };
        }
    };

    // 2. Transform
    let source = match transform_resume(&resume) {
        Ok(s) => s,
        Err(e) => {
            return GenerationResult::Error {
                message: format!("Failed to transform resume to Typst: {}", e),
                validation_errors: None,
            };
        }
    };

    // 3. Compile
    let pdf_bytes = match compile(source) {
        Ok(bytes) => bytes,
        Err(diags) => {
            // Convert diagnostics to string
            let msg = diags
                .iter()
                .map(|d| format!("{:?}: {}", d.severity, d.message))
                .collect::<Vec<_>>()
                .join("\n");
            return GenerationResult::Error {
                message: format!("Typst compilation failed:\n{}", msg),
                validation_errors: None,
            };
        }
    };

    // 4. Encode
    let base64_pdf = general_purpose::STANDARD.encode(pdf_bytes);

    GenerationResult::Success {
        pdf_base64: base64_pdf,
    }
}

/// Parse a serde JSON error into structured validation errors
///
/// Extracts path information from serde error messages to provide
/// actionable feedback about where validation failed.
fn parse_serde_error(error: &serde_json::Error) -> Vec<ValidationError> {
    let message = error.to_string();

    // Serde errors often contain path information like "at line X column Y"
    // and messages like "missing field `name`" or "invalid type: expected X, found Y"
    // We extract what we can to provide structured errors

    // Check for "missing field" errors
    if let Some(field) = extract_missing_field(&message) {
        return vec![ValidationError::new(
            infer_path_from_context(&message, &field),
            format!("Missing required field: {}", field),
        )];
    }

    // Check for type errors
    if message.contains("invalid type") {
        let path = extract_path_hint(&message);
        return vec![ValidationError::new(path, message.clone())];
    }

    // Check for unknown field errors
    if message.contains("unknown field") {
        let path = extract_path_hint(&message);
        return vec![ValidationError::new(path, message.clone())];
    }

    // Default: return the full error message
    vec![ValidationError::new("", message)]
}

/// Extract field name from "missing field `fieldname`" error messages
fn extract_missing_field(message: &str) -> Option<String> {
    let prefix = "missing field `";
    if let Some(start) = message.find(prefix) {
        let rest = &message[start + prefix.len()..];
        if let Some(end) = rest.find('`') {
            return Some(rest[..end].to_string());
        }
    }
    None
}

/// Extract path hint from error message context
fn extract_path_hint(message: &str) -> String {
    // Serde errors for nested structures often mention the parent type
    // We try to extract useful context

    // Look for patterns like "Basics" or "WorkExperience" in the error
    let type_hints = [
        "Basics",
        "WorkExperience",
        "Education",
        "Skill",
        "Profile",
        "Resume",
    ];

    for hint in type_hints {
        if message.contains(hint) {
            return hint.to_lowercase();
        }
    }

    String::new()
}

/// Infer the likely path based on which type is being deserialized
fn infer_path_from_context(message: &str, field: &str) -> String {
    // Try to determine the parent object from the error context
    if message.contains("Basics") {
        return format!("basics.{}", field);
    }
    if message.contains("WorkExperience") {
        return format!("work[].{}", field);
    }
    if message.contains("Education") {
        return format!("education[].{}", field);
    }
    if message.contains("Skill") {
        return format!("skills[].{}", field);
    }
    if message.contains("Profile") {
        return format!("basics.profiles[].{}", field);
    }

    // Default to just the field name
    field.to_string()
}

/// Execute a tool by name with the given arguments
pub fn call_tool(name: &str, arguments: Value) -> Result<Value, String> {
    match name {
        VALIDATE_RESUME_TOOL => {
            let result = validate_resume(arguments);
            serde_json::to_value(result).map_err(|e| format!("Failed to serialize result: {}", e))
        }
        GENERATE_RESUME_TOOL => {
            let result = generate_resume(arguments);
            serde_json::to_value(result).map_err(|e| format!("Failed to serialize result: {}", e))
        }
        _ => Err(format!("Unknown tool: {}", name)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_tools() {
        let tools = list_tools();
        assert_eq!(tools.len(), 2);
        assert_eq!(tools[0].name, VALIDATE_RESUME_TOOL);
        assert_eq!(tools[1].name, GENERATE_RESUME_TOOL);
    }

    // ... existing validate tests ...
    #[test]
    fn test_validate_valid_resume() {
        let input = serde_json::json!({
            "resume": {
                "basics": {
                    "name": "John Doe",
                    "email": "john@example.com"
                },
                "work": [
                    {
                        "company": "Tech Corp",
                        "position": "Engineer"
                    }
                ]
            }
        });

        let result = validate_resume(input);

        match result {
            ValidationResult::Valid { resume } => {
                assert_eq!(resume.basics.name, "John Doe");
                assert_eq!(resume.basics.email, "john@example.com");
            }
            ValidationResult::Invalid { errors } => {
                panic!("Expected valid result, got errors: {:?}", errors);
            }
        }
    }

    // Ensure all previous tests are kept
    #[test]
    fn test_validate_full_resume_fixture() {
        let fixture = include_str!("../../tests/fixtures/sample_resume.json");
        let resume_value: Value = serde_json::from_str(fixture).unwrap();

        let input = serde_json::json!({
            "resume": resume_value
        });

        let result = validate_resume(input);

        match result {
            ValidationResult::Valid { resume } => {
                assert_eq!(resume.basics.name, "Jane Smith");
                assert_eq!(resume.work.len(), 2);
            }
            ValidationResult::Invalid { errors } => {
                panic!("Expected valid result, got errors: {:?}", errors);
            }
        }
    }

    #[test]
    fn test_validate_missing_basics() {
        let input = serde_json::json!({
            "resume": {
                "work": []
            }
        });

        let result = validate_resume(input);

        match result {
            ValidationResult::Invalid { errors } => {
                assert!(!errors.is_empty());
                assert!(errors[0].message.contains("basics") || errors[0].path.contains("basics"));
            }
            ValidationResult::Valid { .. } => {
                panic!("Expected invalid result");
            }
        }
    }

    #[test]
    fn test_validate_missing_required_fields_in_basics() {
        let input = serde_json::json!({
            "resume": {
                "basics": {
                    "name": "John Doe"
                    // missing email
                },
                "work": []
            }
        });

        let result = validate_resume(input);

        match result {
            ValidationResult::Invalid { errors } => {
                assert!(!errors.is_empty());
                let error_text = format!("{:?}", errors);
                assert!(
                    error_text.contains("email"),
                    "Expected error about missing email: {}",
                    error_text
                );
            }
            ValidationResult::Valid { .. } => {
                panic!("Expected invalid result for missing email");
            }
        }
    }

    #[test]
    fn test_validate_missing_work() {
        let input = serde_json::json!({
            "resume": {
                "basics": {
                    "name": "John Doe",
                    "email": "john@example.com"
                }
                // missing work
            }
        });

        let result = validate_resume(input);

        match result {
            ValidationResult::Invalid { errors } => {
                assert!(!errors.is_empty());
                assert!(errors[0].message.contains("work") || errors[0].path.contains("work"));
            }
            ValidationResult::Valid { .. } => {
                panic!("Expected invalid result for missing work");
            }
        }
    }

    #[test]
    fn test_validate_wrong_type_for_work() {
        let input = serde_json::json!({
            "resume": {
                "basics": {
                    "name": "John Doe",
                    "email": "john@example.com"
                },
                "work": "not an array"
            }
        });

        let result = validate_resume(input);

        match result {
            ValidationResult::Invalid { errors } => {
                assert!(!errors.is_empty());
                assert!(
                    errors[0].message.contains("invalid type")
                        || errors[0].message.contains("expected")
                );
            }
            ValidationResult::Valid { .. } => {
                panic!("Expected invalid result for wrong type");
            }
        }
    }

    #[test]
    fn test_validate_missing_required_in_work_entry() {
        let input = serde_json::json!({
            "resume": {
                "basics": {
                    "name": "John Doe",
                    "email": "john@example.com"
                },
                "work": [
                    {
                        "company": "Tech Corp"
                        // missing position
                    }
                ]
            }
        });

        let result = validate_resume(input);

        match result {
            ValidationResult::Invalid { errors } => {
                assert!(!errors.is_empty());
                let error_text = format!("{:?}", errors);
                assert!(
                    error_text.contains("position"),
                    "Expected error about missing position: {}",
                    error_text
                );
            }
            ValidationResult::Valid { .. } => {
                panic!("Expected invalid result for missing position");
            }
        }
    }

    #[test]
    fn test_validate_empty_work_array_is_valid() {
        // Empty work array should be valid (it's a Vec, not requiring entries)
        let input = serde_json::json!({
            "resume": {
                "basics": {
                    "name": "John Doe",
                    "email": "john@example.com"
                },
                "work": []
            }
        });

        let result = validate_resume(input);

        match result {
            ValidationResult::Valid { resume } => {
                assert!(resume.work.is_empty());
            }
            ValidationResult::Invalid { errors } => {
                panic!("Expected valid result, got errors: {:?}", errors);
            }
        }
    }

    #[test]
    fn test_validate_invalid_tool_input() {
        // Test when the input doesn't have the expected "resume" wrapper
        let input = serde_json::json!({
            "basics": {
                "name": "John Doe",
                "email": "john@example.com"
            },
            "work": []
        });

        let result = validate_resume(input);

        match result {
            ValidationResult::Invalid { errors } => {
                assert!(!errors.is_empty());
                assert!(errors[0].message.contains("resume"));
            }
            ValidationResult::Valid { .. } => {
                panic!("Expected invalid result for missing resume wrapper");
            }
        }
    }

    #[test]
    fn test_call_tool_validate_resume() {
        let input = serde_json::json!({
            "resume": {
                "basics": {
                    "name": "John Doe",
                    "email": "john@example.com"
                },
                "work": []
            }
        });

        let result = call_tool(VALIDATE_RESUME_TOOL, input);
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value["status"], "valid");
    }

    #[test]
    fn test_call_tool_unknown() {
        let result = call_tool("unknown_tool", serde_json::json!({}));
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown tool"));
    }

    #[test]
    fn test_validation_result_serialization() {
        let valid_result = ValidationResult::Valid {
            resume: Box::new(Resume {
                basics: crate::documents::resume::Basics {
                    name: "Test".to_string(),
                    email: "test@example.com".to_string(),
                    phone: None,
                    location: None,
                    summary: None,
                    profiles: vec![],
                },
                work: vec![],
                education: vec![],
                skills: vec![],
                projects: vec![],
                certifications: vec![],
                awards: vec![],
                languages: vec![],
                publications: None,
            }),
        };

        let json = serde_json::to_string(&valid_result).unwrap();
        assert!(json.contains("\"status\":\"valid\""));
        assert!(json.contains("\"resume\""));
    }

    #[test]
    fn test_validation_error_serialization() {
        let invalid_result = ValidationResult::Invalid {
            errors: vec![ValidationError::new(
                "basics.email",
                "Missing required field: email",
            )],
        };

        let json = serde_json::to_string(&invalid_result).unwrap();
        assert!(json.contains("\"status\":\"invalid\""));
        assert!(json.contains("\"path\":\"basics.email\""));
        assert!(json.contains("Missing required field"));
    }

    // Fixture-based tests

    #[test]
    fn test_validate_fixture_missing_basics() {
        let fixture = include_str!("../../tests/fixtures/invalid_missing_basics.json");
        let resume_value: Value = serde_json::from_str(fixture).unwrap();

        let input = serde_json::json!({
            "resume": resume_value
        });

        let result = validate_resume(input);

        match result {
            ValidationResult::Invalid { errors } => {
                assert!(!errors.is_empty());
                let error_text = format!("{:?}", errors);
                assert!(
                    error_text.contains("basics"),
                    "Expected error about missing basics: {}",
                    error_text
                );
            }
            ValidationResult::Valid { .. } => {
                panic!("Expected invalid result for fixture missing basics");
            }
        }
    }

    #[test]
    fn test_validate_fixture_missing_email() {
        let fixture = include_str!("../../tests/fixtures/invalid_missing_email.json");
        let resume_value: Value = serde_json::from_str(fixture).unwrap();

        let input = serde_json::json!({
            "resume": resume_value
        });

        let result = validate_resume(input);

        match result {
            ValidationResult::Invalid { errors } => {
                assert!(!errors.is_empty());
                let error_text = format!("{:?}", errors);
                assert!(
                    error_text.contains("email"),
                    "Expected error about missing email: {}",
                    error_text
                );
            }
            ValidationResult::Valid { .. } => {
                panic!("Expected invalid result for fixture missing email");
            }
        }
    }

    #[test]
    fn test_validate_fixture_wrong_type_work() {
        let fixture = include_str!("../../tests/fixtures/invalid_wrong_type_work.json");
        let resume_value: Value = serde_json::from_str(fixture).unwrap();

        let input = serde_json::json!({
            "resume": resume_value
        });

        let result = validate_resume(input);

        match result {
            ValidationResult::Invalid { errors } => {
                assert!(!errors.is_empty());
                let error_text = format!("{:?}", errors);
                assert!(
                    error_text.contains("invalid type") || error_text.contains("expected"),
                    "Expected error about type mismatch: {}",
                    error_text
                );
            }
            ValidationResult::Valid { .. } => {
                panic!("Expected invalid result for fixture with wrong type for work");
            }
        }
    }

    #[test]
    fn test_validate_fixture_work_missing_position() {
        let fixture = include_str!("../../tests/fixtures/invalid_work_missing_position.json");
        let resume_value: Value = serde_json::from_str(fixture).unwrap();

        let input = serde_json::json!({
            "resume": resume_value
        });

        let result = validate_resume(input);

        match result {
            ValidationResult::Invalid { errors } => {
                assert!(!errors.is_empty());
                let error_text = format!("{:?}", errors);
                assert!(
                    error_text.contains("position"),
                    "Expected error about missing position: {}",
                    error_text
                );
            }
            ValidationResult::Valid { .. } => {
                panic!("Expected invalid result for fixture with work entry missing position");
            }
        }
    }

    // New tests for generation
    #[test]
    fn test_generate_resume_valid() {
        let input = serde_json::json!({
            "resume": {
                "basics": {
                    "name": "John Doe",
                    "email": "john@example.com"
                },
                "work": []
            }
        });

        // This is a slow test because it compiles PDF
        let result = generate_resume(input);

        match result {
            GenerationResult::Success { pdf_base64 } => {
                assert!(!pdf_base64.is_empty());
                assert!(pdf_base64.len() > 100); // Should be a reasonable size
            }
            GenerationResult::Error { message, .. } => {
                panic!("Expected success, got error: {}", message);
            }
        }
    }

    #[test]
    fn test_generate_resume_invalid() {
        let input = serde_json::json!({
            "resume": {
                "basics": {
                    "name": "John Doe"
                    // missing email
                },
                "work": []
            }
        });

        let result = generate_resume(input);

        match result {
            GenerationResult::Error {
                message,
                validation_errors,
            } => {
                assert!(message.contains("Validation failed"));
                assert!(validation_errors.is_some());
            }
            GenerationResult::Success { .. } => {
                panic!("Expected error for invalid input");
            }
        }
    }

    #[test]
    fn test_call_tool_generate_resume() {
        let input = serde_json::json!({
            "resume": {
                "basics": {
                    "name": "John Doe",
                    "email": "john@example.com"
                },
                "work": []
            }
        });

        let result = call_tool(GENERATE_RESUME_TOOL, input);
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value["status"], "success");
        assert!(value.get("pdf_base64").is_some());
    }
}
