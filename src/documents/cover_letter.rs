//! Cover letter document types
//!
//! Defines the structure for cover letter documents. These types serve as the single source
//! of truth - they are used for:
//! - JSON Schema generation (via schemars)
//! - Deserialization/validation (via serde)
//! - Transformation to Typst markup

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A complete cover letter document
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[schemars(description = "A professional cover letter document")]
pub struct CoverLetter {
    /// Sender's contact information
    pub sender: ContactInfo,

    /// Recipient's information
    pub recipient: Recipient,

    /// Date of the letter
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Date in YYYY-MM-DD format. If not provided, current date will be used.")]
    pub date: Option<String>,

    /// Opening paragraph
    #[schemars(description = "Opening paragraph expressing interest in the position and company. Should be 2-4 sentences.")]
    pub opening: String,

    /// Body paragraphs
    #[schemars(description = "Body paragraphs (typically 2-3) that demonstrate qualifications, relevant experience, and cultural fit. Each paragraph should be 3-5 sentences.")]
    pub body: Vec<String>,

    /// Closing paragraph
    #[schemars(description = "Closing paragraph expressing enthusiasm and call to action. Should be 2-3 sentences.")]
    pub closing: String,

    /// Signature line (e.g., 'Sincerely', 'Best regards')
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Signature line such as 'Sincerely', 'Best regards', etc. Defaults to 'Sincerely' if not provided.")]
    pub signature: Option<String>,
}

/// Contact information for the sender
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[schemars(description = "Sender's contact information")]
pub struct ContactInfo {
    /// Full name
    pub name: String,

    /// Email address
    #[schemars(email)]
    pub email: String,

    /// Phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Full address (street, city, state, zip)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,

    /// LinkedIn profile URL
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(url)]
    pub linkedin: Option<String>,
}

/// Recipient information
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[schemars(description = "Recipient's information (hiring manager or company)")]
pub struct Recipient {
    /// Hiring manager's name (if known)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Hiring manager's name. If unknown, use 'Hiring Manager' or omit.")]
    pub name: Option<String>,

    /// Job title (if known)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Hiring manager's title (e.g., 'Senior Engineering Manager')")]
    pub title: Option<String>,

    /// Company name
    #[schemars(description = "Company name (required)")]
    pub company: String,

    /// Company address
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Company address (street, city, state, zip)")]
    pub address: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cover_letter_serialization() {
        let cover_letter = CoverLetter {
            sender: ContactInfo {
                name: "Jane Doe".to_string(),
                email: "jane@example.com".to_string(),
                phone: Some("+1-555-123-4567".to_string()),
                address: Some("123 Main St, San Francisco, CA 94102".to_string()),
                linkedin: Some("https://linkedin.com/in/janedoe".to_string()),
            },
            recipient: Recipient {
                name: Some("John Smith".to_string()),
                title: Some("Engineering Manager".to_string()),
                company: "Tech Corp".to_string(),
                address: Some("456 Corporate Blvd, San Francisco, CA 94105".to_string()),
            },
            date: Some("2024-01-15".to_string()),
            opening: "I am writing to express my strong interest in the Senior Software Engineer position at Tech Corp.".to_string(),
            body: vec![
                "With over 5 years of experience in full-stack development, I have consistently delivered high-quality solutions.".to_string(),
                "My experience aligns perfectly with your requirements for this role.".to_string(),
            ],
            closing: "I would welcome the opportunity to discuss how my skills and experience can contribute to Tech Corp's success.".to_string(),
            signature: Some("Sincerely".to_string()),
        };

        let json = serde_json::to_string_pretty(&cover_letter).unwrap();
        assert!(json.contains("\"name\": \"Jane Doe\""));
        assert!(json.contains("\"company\": \"Tech Corp\""));
    }

    #[test]
    fn test_cover_letter_deserialization() {
        let json = r#"{
            "sender": {
                "name": "Jane Doe",
                "email": "jane@example.com"
            },
            "recipient": {
                "company": "Tech Corp"
            },
            "opening": "I am writing to apply for the position.",
            "body": ["First paragraph.", "Second paragraph."],
            "closing": "Thank you for your consideration."
        }"#;

        let cover_letter: CoverLetter = serde_json::from_str(json).unwrap();
        assert_eq!(cover_letter.sender.name, "Jane Doe");
        assert_eq!(cover_letter.recipient.company, "Tech Corp");
        assert_eq!(cover_letter.body.len(), 2);
    }

    #[test]
    fn test_schema_generation() {
        let schema = schemars::schema_for!(CoverLetter);
        let schema_json = serde_json::to_string_pretty(&schema).unwrap();

        // Verify schema has expected structure
        assert!(schema_json.contains("\"$schema\""));
        assert!(schema_json.contains("\"CoverLetter\""));
        assert!(schema_json.contains("\"sender\""));
        assert!(schema_json.contains("\"recipient\""));
    }

    #[test]
    fn test_minimal_cover_letter() {
        let json = r#"{
            "sender": {
                "name": "Jane Doe",
                "email": "jane@example.com"
            },
            "recipient": {
                "company": "Tech Corp"
            },
            "opening": "Opening paragraph.",
            "body": ["Body paragraph."],
            "closing": "Closing paragraph."
        }"#;

        let result: Result<CoverLetter, _> = serde_json::from_str(json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_missing_required_fields() {
        let json = r#"{
            "sender": {
                "name": "Jane Doe"
            },
            "recipient": {
                "company": "Tech Corp"
            },
            "opening": "Opening.",
            "body": [],
            "closing": "Closing."
        }"#;

        let result: Result<CoverLetter, _> = serde_json::from_str(json);
        assert!(result.is_err()); // Missing sender.email
    }
}
