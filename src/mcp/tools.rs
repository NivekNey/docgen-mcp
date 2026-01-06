//! Tool handlers for document validation and generation
//!
//! This module provides MCP tools for validating and generating documents.
//! Currently implements:
//! - `get_resume_schema` - Returns the JSON schema for resume structure
//! - `get_resume_best_practices` - Returns best practices for resume writing
//! - `validate_resume` - Validates JSON payload against resume schema
//! - `generate_resume` - Generates a PDF from a resume JSON payload

use rmcp::model::Tool;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::sync::Arc;

use crate::documents::Resume;
use crate::mcp::{prompts, resources};
use crate::storage::FileStorage;
use crate::typst::compiler::compile;
use crate::typst::transform::transform_resume;

/// Tool name for getting resume schema
pub const GET_RESUME_SCHEMA_TOOL: &str = "get_resume_schema";

/// Tool name for getting resume best practices
pub const GET_RESUME_BEST_PRACTICES_TOOL: &str = "get_resume_best_practices";

/// Tool name for resume validation
pub const VALIDATE_RESUME_TOOL: &str = "validate_resume";

/// Tool name for resume generation
pub const GENERATE_RESUME_TOOL: &str = "generate_resume";

/// Context for tool execution (passed from server)
pub struct ToolContext {
    /// File storage for remote PDF delivery (HTTP mode only)
    pub file_storage: Option<FileStorage>,
    /// Base URL for generating download links (HTTP mode only)
    pub base_url: Option<String>,
}

impl ToolContext {
    /// Create a new context for stdio mode (no file storage)
    pub fn stdio() -> Self {
        Self {
            file_storage: None,
            base_url: None,
        }
    }

    /// Create a new context for HTTP mode with file storage
    pub fn http(file_storage: FileStorage, base_url: String) -> Self {
        Self {
            file_storage: Some(file_storage),
            base_url: Some(base_url),
        }
    }
}

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
        /// Path to the generated PDF file (for local/stdio mode) or null (for HTTP mode)
        #[serde(skip_serializing_if = "Option::is_none")]
        file_path: Option<String>,
        /// Download URL for the PDF (for remote/HTTP mode) or null (for stdio mode)
        #[serde(skip_serializing_if = "Option::is_none")]
        download_url: Option<String>,
        /// Human-readable success message
        message: String,
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
    // Empty schema for tools that don't take parameters
    let mut empty_schema_map = serde_json::Map::new();
    empty_schema_map.insert("type".to_string(), Value::String("object".to_string()));
    empty_schema_map.insert("properties".to_string(), Value::Object(serde_json::Map::new()));
    let empty_schema = Arc::new(empty_schema_map);

    // Schema for validate_resume (resume only)
    let mut resume_prop = serde_json::Map::new();
    resume_prop.insert("type".to_string(), Value::String("object".to_string()));
    resume_prop.insert(
        "description".to_string(),
        Value::String("The resume JSON payload. Use 'get_resume_schema' tool to see the full schema structure.".to_string()),
    );

    let mut validate_properties = serde_json::Map::new();
    validate_properties.insert("resume".to_string(), Value::Object(resume_prop.clone()));

    let mut validate_schema = serde_json::Map::new();
    validate_schema.insert("type".to_string(), Value::String("object".to_string()));
    validate_schema.insert("properties".to_string(), Value::Object(validate_properties));
    validate_schema.insert(
        "required".to_string(),
        Value::Array(vec![Value::String("resume".to_string())]),
    );

    let validate_schema_arc = Arc::new(validate_schema);

    // Schema for generate_resume (resume + optional filename)
    let mut filename_prop = serde_json::Map::new();
    filename_prop.insert("type".to_string(), Value::String("string".to_string()));
    filename_prop.insert(
        "description".to_string(),
        Value::String("Optional filename for the generated PDF (e.g., 'john-doe-resume.pdf'). If not provided, a default name will be generated based on the resume name.".to_string()),
    );

    let mut generate_properties = serde_json::Map::new();
    generate_properties.insert("resume".to_string(), Value::Object(resume_prop));
    generate_properties.insert("filename".to_string(), Value::Object(filename_prop));

    let mut generate_schema = serde_json::Map::new();
    generate_schema.insert("type".to_string(), Value::String("object".to_string()));
    generate_schema.insert("properties".to_string(), Value::Object(generate_properties));
    generate_schema.insert(
        "required".to_string(),
        Value::Array(vec![Value::String("resume".to_string())]),
    );

    let generate_schema_arc = Arc::new(generate_schema);

    // Convenience tools for discovery
    let get_schema_tool = Tool::new(
        GET_RESUME_SCHEMA_TOOL,
        "Returns the complete JSON Schema for resume documents. Use this to understand the exact structure, required fields, and data types expected by validate_resume and generate_resume. This is a convenience wrapper around the 'docgen://schemas/resume' resource.",
        empty_schema.clone(),
    );

    let get_best_practices_tool = Tool::new(
        GET_RESUME_BEST_PRACTICES_TOOL,
        "Returns comprehensive best practices and guidelines for writing effective resume content. Includes writing tips, job description alignment strategies, content guidelines for each section, and the workflow for creating high-quality resumes. Call this BEFORE gathering user information to understand what makes a great resume. This is a convenience wrapper around the 'resume-best-practices' prompt.",
        empty_schema,
    );

    // Validation and generation tools
    let validate_tool = Tool::new(
        VALIDATE_RESUME_TOOL,
        "Validates a resume JSON payload against the schema without generating a document. Returns validation errors with paths if invalid, or confirms validity. TIP: Use 'get_resume_schema' first to understand the expected structure.",
        validate_schema_arc,
    );

    let generate_tool = Tool::new(
        GENERATE_RESUME_TOOL,
        "Generates a professionally formatted PDF resume from a JSON payload. Returns both the file path (for local usage) and base64-encoded PDF content (for remote MCP usage). Optionally accepts a 'filename' parameter (e.g., 'john-doe-resume.pdf'). RECOMMENDED: First use 'get_resume_best_practices' for writing guidance and 'get_resume_schema' for structure, then 'validate_resume' before generating.",
        generate_schema_arc,
    );

    vec![get_schema_tool, get_best_practices_tool, validate_tool, generate_tool]
}

/// Returns the JSON schema for resume documents
///
/// This is a convenience tool that wraps the 'docgen://schemas/resume' resource,
/// making it discoverable through tool listing.
pub fn get_resume_schema() -> Value {
    match resources::read_resource(resources::RESUME_SCHEMA_URI) {
        Some(rmcp::model::ResourceContents::TextResourceContents { text, .. }) => {
            // Parse the schema JSON and return it as a structured value
            serde_json::from_str(&text).unwrap_or_else(|_| {
                serde_json::json!({
                    "error": "Failed to parse schema"
                })
            })
        }
        _ => serde_json::json!({
            "error": "Schema resource not found"
        }),
    }
}

/// Returns best practices and guidelines for resume writing
///
/// This is a convenience tool that wraps the 'resume-best-practices' prompt,
/// making it discoverable through tool listing.
pub fn get_resume_best_practices() -> Value {
    match prompts::get_prompt(prompts::RESUME_BEST_PRACTICES_PROMPT) {
        Some(prompt_result) => {
            // Extract the text content from the prompt message
            if let Some(msg) = prompt_result.messages.first()
                && let rmcp::model::PromptMessageContent::Text { text } = &msg.content
            {
                return serde_json::json!({
                    "best_practices": text,
                    "description": prompt_result.description
                });
            }
            serde_json::json!({
                "error": "Failed to extract prompt content"
            })
        }
        None => serde_json::json!({
            "error": "Best practices prompt not found"
        }),
    }
}

/// Input for the validate_resume tool
#[derive(Debug, Deserialize)]
pub struct ValidateResumeInput {
    pub resume: Value,
}

/// Input for the generate_resume tool
#[derive(Debug, Deserialize)]
pub struct GenerateResumeInput {
    pub resume: Value,
    pub filename: Option<String>,
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
///
/// In stdio mode: saves the PDF to a local file
/// In HTTP mode: stores the PDF in temporary storage and returns a download URL
pub async fn generate_resume(input: Value, context: &ToolContext) -> GenerationResult {
    // 0. Parse input to get resume and optional filename
    let parsed_input: GenerateResumeInput = match serde_json::from_value(input.clone()) {
        Ok(v) => v,
        Err(e) => {
            return GenerationResult::Error {
                message: format!("Invalid tool input: expected object with 'resume' field. {}", e),
                validation_errors: None,
            };
        }
    };

    // 1. Validate
    let validation_input = serde_json::json!({ "resume": parsed_input.resume });
    let validation_result = validate_resume(validation_input);

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

    // 4. Generate filename (use provided or generate from name)
    let filename = parsed_input.filename.unwrap_or_else(|| {
        // Sanitize the name to create a safe filename
        let name = &resume.basics.name;
        let sanitized = name
            .to_lowercase()
            .replace(" ", "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect::<String>();
        format!("{}-resume.pdf", sanitized)
    });

    // 5. Handle output based on transport mode
    match (&context.file_storage, &context.base_url) {
        // HTTP mode: store in temporary storage and return download URL
        (Some(storage), Some(base_url)) => {
            let file_id = storage.store(pdf_bytes, filename.clone()).await;
            let download_url = format!("{}/files/{}", base_url, file_id);

            GenerationResult::Success {
                file_path: None,
                download_url: Some(download_url.clone()),
                message: format!(
                    "Resume successfully generated. Download it from: {}\n\
                     This link will expire in 1 hour.",
                    download_url
                ),
            }
        }
        // Stdio mode: save to local file
        _ => {
            match fs::write(&filename, pdf_bytes) {
                Ok(_) => GenerationResult::Success {
                    file_path: Some(filename.clone()),
                    download_url: None,
                    message: format!("Resume successfully generated and saved to '{}'", filename),
                },
                Err(e) => GenerationResult::Error {
                    message: format!("Failed to write PDF to file '{}': {}", filename, e),
                    validation_errors: None,
                },
            }
        }
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
pub async fn call_tool(name: &str, arguments: Value, context: &ToolContext) -> Result<Value, String> {
    match name {
        GET_RESUME_SCHEMA_TOOL => {
            let _ = arguments; // Schema tool takes no arguments
            Ok(get_resume_schema())
        }
        GET_RESUME_BEST_PRACTICES_TOOL => {
            let _ = arguments; // Best practices tool takes no arguments
            Ok(get_resume_best_practices())
        }
        VALIDATE_RESUME_TOOL => {
            let result = validate_resume(arguments);
            serde_json::to_value(result).map_err(|e| format!("Failed to serialize result: {}", e))
        }
        GENERATE_RESUME_TOOL => {
            let result = generate_resume(arguments, context).await;
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
        assert_eq!(tools.len(), 4);
        assert_eq!(tools[0].name, GET_RESUME_SCHEMA_TOOL);
        assert_eq!(tools[1].name, GET_RESUME_BEST_PRACTICES_TOOL);
        assert_eq!(tools[2].name, VALIDATE_RESUME_TOOL);
        assert_eq!(tools[3].name, GENERATE_RESUME_TOOL);
    }

    #[test]
    fn test_get_resume_schema() {
        let schema = get_resume_schema();

        // Should return a valid JSON object
        assert!(schema.is_object());

        // Should have the basic schema structure
        assert!(schema.get("$schema").is_some());
        assert!(schema.get("title").is_some());

        // Should contain Resume type information
        let schema_str = serde_json::to_string(&schema).unwrap();
        assert!(schema_str.contains("Resume"));
        assert!(schema_str.contains("basics"));
        assert!(schema_str.contains("work"));
    }

    #[test]
    fn test_get_resume_best_practices() {
        let result = get_resume_best_practices();

        // Should return a valid JSON object
        assert!(result.is_object());

        // Should have best_practices field with text content
        assert!(result.get("best_practices").is_some());

        let best_practices = result["best_practices"].as_str().unwrap();

        // Should contain key guidance
        assert!(best_practices.contains("Resume Best Practices"));
        assert!(best_practices.contains("Contact Information"));
        assert!(best_practices.contains("Work Experience"));
        assert!(best_practices.contains("schema"));
    }

    #[tokio::test]
    async fn test_call_tool_get_schema() {
        let context = ToolContext::stdio();
        let result = call_tool(GET_RESUME_SCHEMA_TOOL, serde_json::json!({}), &context).await;
        assert!(result.is_ok());

        let value = result.unwrap();
        assert!(value.is_object());
        assert!(value.get("$schema").is_some());
    }

    #[tokio::test]
    async fn test_call_tool_get_best_practices() {
        let context = ToolContext::stdio();
        let result = call_tool(GET_RESUME_BEST_PRACTICES_TOOL, serde_json::json!({}), &context).await;
        assert!(result.is_ok());

        let value = result.unwrap();
        assert!(value.is_object());
        assert!(value.get("best_practices").is_some());
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

    #[tokio::test]
    async fn test_call_tool_validate_resume() {
        let context = ToolContext::stdio();
        let input = serde_json::json!({
            "resume": {
                "basics": {
                    "name": "John Doe",
                    "email": "john@example.com"
                },
                "work": []
            }
        });

        let result = call_tool(VALIDATE_RESUME_TOOL, input, &context).await;
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value["status"], "valid");
    }

    #[tokio::test]
    async fn test_call_tool_unknown() {
        let context = ToolContext::stdio();
        let result = call_tool("unknown_tool", serde_json::json!({}), &context).await;
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
                publications: vec![],
                section_order: None,
                section_titles: None,
                show_header: None,
                show_page_numbers: None,
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
    #[tokio::test]
    async fn test_generate_resume_valid() {
        let context = ToolContext::stdio();
        let input = serde_json::json!({
            "resume": {
                "basics": {
                    "name": "John Doe",
                    "email": "john@example.com"
                },
                "work": []
            },
            "filename": "test-generate-resume-valid.pdf"
        });

        // This is a slow test because it compiles PDF
        let result = generate_resume(input, &context).await;

        match result {
            GenerationResult::Success { file_path, download_url, message } => {
                assert_eq!(file_path, Some("test-generate-resume-valid.pdf".to_string()));
                assert_eq!(download_url, None); // stdio mode doesn't have download URL
                assert!(message.contains("successfully"));

                // Verify file was created
                assert!(std::path::Path::new("test-generate-resume-valid.pdf").exists());

                // Clean up
                let _ = fs::remove_file("test-generate-resume-valid.pdf");
            }
            GenerationResult::Error { message, .. } => {
                panic!("Expected success, got error: {}", message);
            }
        }
    }

    #[tokio::test]
    async fn test_generate_resume_invalid() {
        let context = ToolContext::stdio();
        let input = serde_json::json!({
            "resume": {
                "basics": {
                    "name": "John Doe"
                    // missing email
                },
                "work": []
            }
        });

        let result = generate_resume(input, &context).await;

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

    #[tokio::test]
    async fn test_call_tool_generate_resume() {
        let context = ToolContext::stdio();
        let input = serde_json::json!({
            "resume": {
                "basics": {
                    "name": "John Doe",
                    "email": "john@example.com"
                },
                "work": []
            },
            "filename": "test-call-tool-generate.pdf"
        });

        let result = call_tool(GENERATE_RESUME_TOOL, input, &context).await;
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value["status"], "success");
        assert!(value.get("file_path").is_some());
        assert!(value.get("message").is_some());

        // In stdio mode, should have file_path but no download_url
        assert!(value["file_path"].is_string());
        assert!(value["download_url"].is_null());

        // Clean up generated file
        if let Some(file_path) = value["file_path"].as_str() {
            let _ = fs::remove_file(file_path);
        }
    }

    #[tokio::test]
    async fn test_generate_resume_with_custom_filename() {
        let context = ToolContext::stdio();
        let input = serde_json::json!({
            "resume": {
                "basics": {
                    "name": "Jane Smith",
                    "email": "jane@example.com"
                },
                "work": []
            },
            "filename": "custom-resume.pdf"
        });

        let result = generate_resume(input, &context).await;

        match result {
            GenerationResult::Success { file_path, download_url, message } => {
                assert_eq!(file_path, Some("custom-resume.pdf".to_string()));
                assert!(message.contains("custom-resume.pdf"));
                assert_eq!(download_url, None); // stdio mode

                // Verify file was created
                assert!(std::path::Path::new("custom-resume.pdf").exists());

                // Clean up
                let _ = fs::remove_file("custom-resume.pdf");
            }
            GenerationResult::Error { message, .. } => {
                panic!("Expected success, got error: {}", message);
            }
        }
    }

    #[tokio::test]
    async fn test_generate_resume_default_filename() {
        let context = ToolContext::stdio();
        let input = serde_json::json!({
            "resume": {
                "basics": {
                    "name": "Alice Wonder",
                    "email": "alice@example.com"
                },
                "work": []
            }
        });

        let result = generate_resume(input, &context).await;

        match result {
            GenerationResult::Success { file_path, download_url, .. } => {
                // Should generate filename from name
                assert_eq!(file_path, Some("alice-wonder-resume.pdf".to_string()));
                assert_eq!(download_url, None); // stdio mode

                // Clean up
                let _ = fs::remove_file("alice-wonder-resume.pdf");
            }
            GenerationResult::Error { message, .. } => {
                panic!("Expected success, got error: {}", message);
            }
        }
    }
}
