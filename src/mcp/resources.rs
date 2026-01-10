//! Resource handlers for exposing JSON schemas
//!
//! This module provides functions for MCP resource discovery and retrieval.
//! Resources expose JSON schemas generated from Rust types.

use crate::documents::{CoverLetter, Resume};
use rmcp::model::{AnnotateAble, RawResource, Resource, ResourceContents};

/// URI for the resume schema resource
pub const RESUME_SCHEMA_URI: &str = "docgen://schemas/resume";

/// URI for the cover letter schema resource
pub const COVER_LETTER_SCHEMA_URI: &str = "docgen://schemas/cover-letter";

/// Returns a list of all available resources
pub fn list_resources() -> Vec<Resource> {
    let mut resume_resource = RawResource::new(RESUME_SCHEMA_URI, "Resume Schema");
    resume_resource.description = Some("JSON Schema for resume documents".to_string());
    resume_resource.mime_type = Some("application/schema+json".to_string());

    let mut cover_letter_resource = RawResource::new(COVER_LETTER_SCHEMA_URI, "Cover Letter Schema");
    cover_letter_resource.description = Some("JSON Schema for cover letter documents".to_string());
    cover_letter_resource.mime_type = Some("application/schema+json".to_string());

    vec![
        resume_resource.no_annotation(),
        cover_letter_resource.no_annotation(),
    ]
}

/// Reads a resource by URI and returns its contents
pub fn read_resource(uri: &str) -> Option<ResourceContents> {
    match uri {
        RESUME_SCHEMA_URI => {
            let schema = schemars::schema_for!(Resume);
            let schema_json =
                serde_json::to_string_pretty(&schema).expect("Failed to serialize schema");

            Some(ResourceContents::TextResourceContents {
                uri: uri.to_string(),
                mime_type: Some("application/schema+json".to_string()),
                text: schema_json,
                meta: None,
            })
        }
        COVER_LETTER_SCHEMA_URI => {
            let schema = schemars::schema_for!(CoverLetter);
            let schema_json =
                serde_json::to_string_pretty(&schema).expect("Failed to serialize schema");

            Some(ResourceContents::TextResourceContents {
                uri: uri.to_string(),
                mime_type: Some("application/schema+json".to_string()),
                text: schema_json,
                meta: None,
            })
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_resources() {
        let resources = list_resources();
        assert_eq!(resources.len(), 1);
        assert_eq!(resources[0].raw.uri, RESUME_SCHEMA_URI);
        assert_eq!(resources[0].raw.name, "Resume Schema");
    }

    #[test]
    fn test_read_resume_schema() {
        let contents = read_resource(RESUME_SCHEMA_URI);
        assert!(contents.is_some());

        if let Some(ResourceContents::TextResourceContents { text, .. }) = contents {
            // Verify it's valid JSON
            let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();

            // Verify schema structure
            assert!(parsed.get("$schema").is_some());
            assert!(parsed.get("title").is_some());
        } else {
            panic!("Expected TextResourceContents");
        }
    }

    #[test]
    fn test_read_unknown_resource() {
        let contents = read_resource("docgen://unknown");
        assert!(contents.is_none());
    }
}
