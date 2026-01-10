//! Prompt handlers for best practices guidance
//!
//! This module provides MCP prompts that help LLMs create effective document content.
//! The prompts include best practices, writing guidelines, and schema references.

use crate::documents::{CoverLetter, Resume};
use crate::mcp::resources::{COVER_LETTER_SCHEMA_URI, RESUME_SCHEMA_URI};
use rmcp::model::{GetPromptResult, Prompt, PromptMessage, PromptMessageRole};

/// Prompt name for resume best practices
pub const RESUME_BEST_PRACTICES_PROMPT: &str = "resume-best-practices";

/// Prompt name for cover letter best practices
pub const COVER_LETTER_BEST_PRACTICES_PROMPT: &str = "cover-letter-best-practices";

/// Prompt name for document type guidance
pub const DOCUMENT_TYPE_GUIDE_PROMPT: &str = "document-type-guide";

/// Raw markdown template for resume best practices (embedded at compile time)
const RESUME_BEST_PRACTICES_TEMPLATE: &str = include_str!("../../prompts/resume-best-practices.md");

/// Raw markdown template for cover letter best practices (embedded at compile time)
const COVER_LETTER_BEST_PRACTICES_TEMPLATE: &str = include_str!("../../prompts/cover-letter-best-practices.md");

/// Raw markdown for document type guide (embedded at compile time)
const DOCUMENT_TYPE_GUIDE_TEMPLATE: &str = include_str!("../../prompts/document-type-guide.md");

/// Returns a list of all available prompts
pub fn list_prompts() -> Vec<Prompt> {
    vec![
        Prompt {
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
        },
        Prompt {
            name: COVER_LETTER_BEST_PRACTICES_PROMPT.to_string(),
            title: Some("Cover Letter Best Practices".to_string()),
            description: Some(
                "Guidelines and best practices for creating compelling cover letters. \
                 Includes structure advice, writing tips, and the schema reference."
                    .to_string(),
            ),
            arguments: None,
            icons: None,
            meta: None,
        },
        Prompt {
            name: DOCUMENT_TYPE_GUIDE_PROMPT.to_string(),
            title: Some("Document Type Guide".to_string()),
            description: Some(
                "Guide to choosing between resume, CV, and cover letter. \
                 Explains differences, use cases, and when to use each document type."
                    .to_string(),
            ),
            arguments: None,
            icons: None,
            meta: None,
        },
    ]
}

/// Gets a prompt by name and returns its content
pub fn get_prompt(name: &str) -> Option<GetPromptResult> {
    match name {
        RESUME_BEST_PRACTICES_PROMPT => Some(build_resume_best_practices_prompt()),
        COVER_LETTER_BEST_PRACTICES_PROMPT => Some(build_cover_letter_best_practices_prompt()),
        DOCUMENT_TYPE_GUIDE_PROMPT => Some(build_document_type_guide_prompt()),
        _ => None,
    }
}

/// Builds the resume best practices prompt with guidelines and schema reference
fn build_resume_best_practices_prompt() -> GetPromptResult {
    // Generate the schema for reference
    let schema = schemars::schema_for!(Resume);
    let schema_json = serde_json::to_string_pretty(&schema).expect("Failed to serialize schema");

    // Replace placeholders in the template
    let content = RESUME_BEST_PRACTICES_TEMPLATE
        .replace("{{SCHEMA_JSON}}", &schema_json)
        .replace("{{SCHEMA_URI}}", RESUME_SCHEMA_URI);

    GetPromptResult {
        description: Some(
            "Best practices and guidelines for creating effective resume content".to_string(),
        ),
        messages: vec![PromptMessage::new_text(PromptMessageRole::User, content)],
    }
}

/// Builds the cover letter best practices prompt with guidelines and schema reference
fn build_cover_letter_best_practices_prompt() -> GetPromptResult {
    // Generate the schema for reference
    let schema = schemars::schema_for!(CoverLetter);
    let schema_json = serde_json::to_string_pretty(&schema).expect("Failed to serialize schema");

    // Replace placeholders in the template
    let content = COVER_LETTER_BEST_PRACTICES_TEMPLATE
        .replace("{{SCHEMA_JSON}}", &schema_json)
        .replace("{{SCHEMA_URI}}", COVER_LETTER_SCHEMA_URI);

    GetPromptResult {
        description: Some(
            "Best practices and guidelines for creating compelling cover letters".to_string(),
        ),
        messages: vec![PromptMessage::new_text(PromptMessageRole::User, content)],
    }
}

/// Builds the document type guide prompt
fn build_document_type_guide_prompt() -> GetPromptResult {
    GetPromptResult {
        description: Some(
            "Guide to choosing the right document type for different situations".to_string(),
        ),
        messages: vec![PromptMessage::new_text(
            PromptMessageRole::User,
            DOCUMENT_TYPE_GUIDE_TEMPLATE.to_string(),
        )],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_prompts() {
        let prompts = list_prompts();
        assert_eq!(prompts.len(), 3);
        assert_eq!(prompts[0].name, RESUME_BEST_PRACTICES_PROMPT);
        assert_eq!(prompts[1].name, COVER_LETTER_BEST_PRACTICES_PROMPT);
        assert_eq!(prompts[2].name, DOCUMENT_TYPE_GUIDE_PROMPT);
        assert!(prompts[0].description.is_some());
        assert!(prompts[1].description.is_some());
        assert!(prompts[2].description.is_some());
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

    #[test]
    fn test_prompt_placeholders_are_replaced() {
        let result = get_prompt(RESUME_BEST_PRACTICES_PROMPT).unwrap();

        if let rmcp::model::PromptMessageContent::Text { text } = &result.messages[0].content {
            assert!(
                !text.contains("{{"),
                "Unreplaced placeholder found in prompt"
            );
        } else {
            panic!("Expected text content");
        }
    }
}
