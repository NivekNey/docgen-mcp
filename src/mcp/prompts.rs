//! Prompt handlers for best practices guidance
//!
//! This module provides MCP prompts that help LLMs create effective resume content.
//! The prompts include best practices, writing guidelines, and schema references.

use crate::documents::Resume;
use crate::mcp::resources::RESUME_SCHEMA_URI;
use rmcp::model::{GetPromptResult, Prompt, PromptMessage, PromptMessageRole};

/// Prompt name for resume best practices
pub const RESUME_BEST_PRACTICES_PROMPT: &str = "resume-best-practices";

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

    let best_practices_content = format!(
        r#"# Resume Best Practices

You are helping create a professional resume. Follow these guidelines to produce effective, ATS-friendly content.

## Content Guidelines

### Contact Information (basics)
- Include full name, professional email, and phone number
- Add location (city, state) — full address is not necessary
- Include relevant professional profiles (LinkedIn, GitHub, portfolio)
- Ensure email sounds professional (avoid nicknames or numbers)

### Professional Summary
- Write 2-3 concise sentences highlighting your value proposition
- Focus on years of experience, key skills, and notable achievements
- Tailor to the target role — avoid generic statements
- Use strong action-oriented language

### Work Experience
- List positions in reverse chronological order (most recent first)
- Use action verbs to start each bullet point (Led, Developed, Implemented, Achieved)
- Quantify achievements with metrics when possible:
  - "Increased sales by 25%" instead of "Improved sales"
  - "Managed team of 8 engineers" instead of "Led engineering team"
  - "Reduced deployment time from 2 hours to 15 minutes"
- Focus on accomplishments, not just responsibilities
- Include 3-5 bullet points per position
- Use the STAR format: Situation, Task, Action, Result

### Education
- Include degree, institution, and graduation date
- Add GPA if 3.5+ and within 5 years of graduation
- List relevant coursework only if entry-level
- Include honors, scholarships, or relevant academic achievements

### Skills
- Group skills by category (Programming Languages, Frameworks, Tools, etc.)
- List skills in order of proficiency or relevance
- Include only skills you can confidently discuss in an interview
- Match skills to job requirements when possible
- Avoid soft skills in the skills section — demonstrate them in experience

### Projects (optional but recommended for tech roles)
- Include 2-4 significant projects
- Describe the problem solved and technologies used
- Link to live demos or repositories when available
- Highlight your specific contributions in team projects

## Writing Style

### Do:
- Be concise — aim for one page unless 10+ years of experience
- Use consistent formatting and tense
- Proofread for spelling and grammar errors
- Use industry-standard terminology
- Keep bullet points to 1-2 lines each

### Don't:
- Use first person pronouns (I, me, my)
- Include irrelevant personal information
- Use clichés ("team player", "hard worker", "detail-oriented")
- Exaggerate or misrepresent experience
- Include references or "References available upon request"

## Schema Reference

When generating the resume JSON, follow this schema exactly:

```json
{schema_json}
```

### Required Fields
- `basics.name` — Full legal name
- `basics.email` — Professional email address
- `work` — Array of work experiences (can be empty if no experience)

### Date Formats
- Use ISO 8601 format: "YYYY-MM-DD" or "YYYY-MM"
- For current positions, omit `endDate` or set to null

### Tips for Each Section
- **profiles**: Include network name and URL (e.g., LinkedIn, GitHub)
- **highlights**: Use action verbs and quantify when possible
- **skills**: Group related skills together with a descriptive name
- **publications**: Free-form text for academic or professional publications

## Example Usage

After reading the schema from `{RESUME_SCHEMA_URI}`, construct a JSON object matching the structure, then call the `generate_resume` tool to create the PDF.

Remember: A great resume is tailored, concise, and accomplishment-focused. Help the user highlight their unique value proposition for their target role."#
    );

    GetPromptResult {
        description: Some(
            "Best practices and guidelines for creating effective resume content".to_string(),
        ),
        messages: vec![PromptMessage::new_text(
            PromptMessageRole::User,
            best_practices_content,
        )],
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
