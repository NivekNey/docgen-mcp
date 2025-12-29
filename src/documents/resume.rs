//! Resume document types
//!
//! Defines the structure for resume documents. These types serve as the single source
//! of truth - they are used for:
//! - JSON Schema generation (via schemars)
//! - Deserialization/validation (via serde)
//! - Transformation to Typst markup

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// A complete resume document
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[schemars(description = "A complete resume/CV document")]
pub struct Resume {
    /// Basic personal information
    pub basics: Basics,

    /// Work experience entries
    pub work: Vec<WorkExperience>,

    /// Educational background
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub education: Vec<Education>,

    /// Skills and competencies
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub skills: Vec<Skill>,

    /// Publications summary (free-form text)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Free-form text describing publications, e.g., '11 peer-reviewed publications at EMNLP, IEEE TNNLS, IEEE Big Data, ACM CIKM on text moderation, hate speech detection, ad creative optimization, and graph neural networks'")]
    pub publications: Option<String>,
}

/// Basic personal information
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[schemars(description = "Basic personal and contact information")]
pub struct Basics {
    /// Full name
    pub name: String,

    /// Email address
    #[schemars(email)]
    pub email: String,

    /// Phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Location (city, state/country)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,

    /// Professional summary or objective
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,

    /// Online profiles and links
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub profiles: Vec<Profile>,
}

/// An online profile or link
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[schemars(description = "An online profile or link (e.g., LinkedIn, GitHub)")]
pub struct Profile {
    /// Network or platform name (e.g., "LinkedIn", "GitHub")
    pub network: String,

    /// URL to the profile
    #[schemars(url)]
    pub url: String,
}

/// A work experience entry
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[schemars(description = "A work experience entry")]
pub struct WorkExperience {
    /// Company or organization name
    pub company: String,

    /// Job title or position
    pub position: String,

    /// Start date (YYYY-MM-DD or YYYY-MM format)
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Start date in YYYY-MM-DD or YYYY-MM format")]
    pub start_date: Option<String>,

    /// End date (YYYY-MM-DD, YYYY-MM format, or "Present")
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    #[schemars(description = "End date in YYYY-MM-DD or YYYY-MM format, or 'Present' for current positions")]
    pub end_date: Option<String>,

    /// Key achievements and responsibilities
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub highlights: Vec<String>,
}

/// An education entry
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[schemars(description = "An education entry")]
pub struct Education {
    /// Institution name
    pub institution: String,

    /// Degree or certificate type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub degree: Option<String>,

    /// Field of study or major
    #[serde(rename = "fieldOfStudy", skip_serializing_if = "Option::is_none")]
    pub field_of_study: Option<String>,

    /// Start date (YYYY-MM-DD or YYYY-MM format)
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Start date in YYYY-MM-DD or YYYY-MM format")]
    pub start_date: Option<String>,

    /// End date or expected graduation (YYYY-MM-DD, YYYY-MM format, or "Expected YYYY")
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    #[schemars(description = "End date in YYYY-MM-DD or YYYY-MM format, or 'Expected YYYY' for ongoing")]
    pub end_date: Option<String>,

    /// GPA or grade (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gpa: Option<String>,

    /// Notable achievements, honors, or coursework
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub highlights: Vec<String>,
}

/// A skill or competency
#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema)]
#[schemars(description = "A skill category with related keywords")]
pub struct Skill {
    /// Skill category name (e.g., "Programming Languages", "Frameworks")
    pub name: String,

    /// List of specific skills in this category
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub keywords: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resume_serialization() {
        let resume = Resume {
            basics: Basics {
                name: "John Doe".to_string(),
                email: "john@example.com".to_string(),
                phone: Some("+1-555-123-4567".to_string()),
                location: Some("San Francisco, CA".to_string()),
                summary: Some("Experienced software engineer".to_string()),
                profiles: vec![Profile {
                    network: "GitHub".to_string(),
                    url: "https://github.com/johndoe".to_string(),
                }],
            },
            work: vec![WorkExperience {
                company: "Tech Corp".to_string(),
                position: "Senior Engineer".to_string(),
                start_date: Some("2020-01".to_string()),
                end_date: Some("Present".to_string()),
                highlights: vec!["Led team of 5 engineers".to_string()],
            }],
            education: vec![Education {
                institution: "MIT".to_string(),
                degree: Some("B.S.".to_string()),
                field_of_study: Some("Computer Science".to_string()),
                start_date: Some("2012-09".to_string()),
                end_date: Some("2016-05".to_string()),
                gpa: Some("3.8".to_string()),
                highlights: vec![],
            }],
            skills: vec![Skill {
                name: "Programming Languages".to_string(),
                keywords: vec!["Rust".to_string(), "Python".to_string()],
            }],
            publications: Some("5 peer-reviewed publications at NeurIPS and ICML".to_string()),
        };

        let json = serde_json::to_string_pretty(&resume).unwrap();
        assert!(json.contains("\"name\": \"John Doe\""));
        assert!(json.contains("\"startDate\": \"2020-01\""));
        assert!(json.contains("\"publications\""));
    }

    #[test]
    fn test_resume_deserialization() {
        let json = r#"{
            "basics": {
                "name": "Jane Smith",
                "email": "jane@example.com"
            },
            "work": [
                {
                    "company": "Startup Inc",
                    "position": "CTO"
                }
            ]
        }"#;

        let resume: Resume = serde_json::from_str(json).unwrap();
        assert_eq!(resume.basics.name, "Jane Smith");
        assert_eq!(resume.work[0].company, "Startup Inc");
        assert!(resume.education.is_empty());
        assert!(resume.skills.is_empty());
    }

    #[test]
    fn test_schema_generation() {
        let schema = schemars::schema_for!(Resume);
        let schema_json = serde_json::to_string_pretty(&schema).unwrap();

        // Verify schema has expected structure
        assert!(schema_json.contains("\"$schema\""));
        assert!(schema_json.contains("\"Resume\""));
        assert!(schema_json.contains("\"basics\""));
        assert!(schema_json.contains("\"work\""));
    }

    #[test]
    fn test_sample_fixture_deserialization() {
        let fixture = include_str!("../../tests/fixtures/sample_resume.json");
        let resume: Resume = serde_json::from_str(fixture)
            .expect("Sample fixture should deserialize correctly");

        assert_eq!(resume.basics.name, "Jane Smith");
        assert_eq!(resume.basics.email, "jane.smith@example.com");
        assert_eq!(resume.work.len(), 2);
        assert_eq!(resume.work[0].company, "Tech Innovations Inc.");
        assert_eq!(resume.education.len(), 1);
        assert_eq!(resume.skills.len(), 3);
    }
}
