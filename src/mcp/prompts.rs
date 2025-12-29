//! Prompt handlers for best practices guidance
//!
//! This module provides MCP prompts that help LLMs create effective resume content.
//! The prompts include best practices, writing guidelines, and schema references.

use crate::documents::Resume;
use crate::mcp::resources::RESUME_SCHEMA_URI;
use rmcp::model::{GetPromptResult, Prompt, PromptMessage, PromptMessageRole};

/// Prompt name for resume best practices
pub const RESUME_BEST_PRACTICES_PROMPT: &str = "resume-best-practices";

/// Raw markdown template for best practices (embedded at compile time)
const BEST_PRACTICES_TEMPLATE: &str = include_str!("../../prompts/resume-best-practices.md");

/// Returns a list of all available prompts
pub fn list_prompts() -> Vec<Prompt> {
    vec![Prompt {
        name: RESUME_BEST_PRACTICES_PROMPT.to_string(),
        title: Some("Resume Best Practices".to_string()),
        description: Some(
            "Guidelines and best practices for creating effective resume content. \
             Includes writing tips, formatting guidance, and the schema reference."
                .to_string(),
        ),
        arguments: None,
        icons: None,
        meta: None,
    }]
}

/// Gets a prompt by name and returns its content
pub fn get_prompt(name: &str) -> Option<GetPromptResult> {
    match name {
        RESUME_BEST_PRACTICES_PROMPT => Some(build_resume_best_practices_prompt()),
        _ => None,
    }
}

/// Builds the resume best practices prompt with guidelines and schema reference
fn build_resume_best_practices_prompt() -> GetPromptResult {
    // Generate the schema for reference
    let schema = schemars::schema_for!(Resume);
    let schema_json =
        serde_json::to_string_pretty(&schema).expect("Failed to serialize schema");

    // Replace placeholders in the template
    let content = BEST_PRACTICES_TEMPLATE
        .replace("{{SCHEMA_JSON}}", &schema_json)
        .replace("{{SCHEMA_URI}}", RESUME_SCHEMA_URI);

    GetPromptResult {
        description: Some(
            "Best practices and guidelines for creating effective resume content".to_string(),
        ),
        messages: vec![PromptMessage::new_text(PromptMessageRole::User, content)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_prompts() {
        let prompts = list_prompts();
        assert_eq!(prompts.len(), 1);
        assert_eq!(prompts[0].name, RESUME_BEST_PRACTICES_PROMPT);
        assert!(prompts[0].description.is_some());
    }

    #[test]
    fn test_get_prompt_resume_best_practices() {
        let result = get_prompt(RESUME_BEST_PRACTICES_PROMPT);
        assert!(result.is_some());

        let prompt_result = result.unwrap();
        assert!(prompt_result.description.is_some());
        assert_eq!(prompt_result.messages.len(), 1);

        // Verify the message contains expected content
        if let rmcp::model::PromptMessageContent::Text { text } = &prompt_result.messages[0].content
        {
            assert!(text.contains("Resume Best Practices"));
            assert!(text.contains("Contact Information"));
            assert!(text.contains("Work Experience"));
            assert!(text.contains("schema"));
        } else {
            panic!("Expected text content in prompt message");
        }
    }

    #[test]
    fn test_get_prompt_unknown() {
        let result = get_prompt("unknown-prompt");
        assert!(result.is_none());
    }

    #[test]
    fn test_prompt_includes_schema() {
        let result = get_prompt(RESUME_BEST_PRACTICES_PROMPT).unwrap();

        if let rmcp::model::PromptMessageContent::Text { text } = &result.messages[0].content {
            // Verify schema JSON is included
            assert!(text.contains("\"$schema\""));
            assert!(text.contains("\"Resume\""));
            assert!(text.contains("basics"));
            assert!(text.contains("work"));
        } else {
            panic!("Expected text content");
        }
    }

    #[test]
    fn test_prompt_includes_schema_uri_reference() {
        let result = get_prompt(RESUME_BEST_PRACTICES_PROMPT).unwrap();

        if let rmcp::model::PromptMessageContent::Text { text } = &result.messages[0].content {
            assert!(
                text.contains(RESUME_SCHEMA_URI),
                "Prompt should reference the schema URI"
            );
        } else {
            panic!("Expected text content");
        }
    }
}
