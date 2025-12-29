use crate::documents::resume::Resume;
use serde_json;

/// The raw Typst template content
const RESUME_TEMPLATE: &str = include_str!("../../templates/resume.typ");

/// Transforms a Resume struct into a Typst source string
pub fn transform_resume(resume: &Resume) -> Result<String, serde_json::Error> {
    // Serialize the resume data to JSON
    let json_data = serde_json::to_string(resume)?;

    // Construct the full Typst source
    // We treat the template as a library and import it or just append the call.
    // Since we embedded the content, we prepend it.

    // We use a raw string block for the JSON data to avoid escaping issues.
    // However, if the JSON data itself contains the delimiter "```", it would break.
    // Standard JSON does not contain "```" unless it's in a string value.
    // To be perfectly safe, we could use a variable number of backticks, but standard JSON is usually safe.
    //
    // A more robust way:
    // Typst raw strings can use more backticks.
    // We'll use 5 backticks to be safe.

    let source = format!(
        r#"{template}

#let json-string = `````
{json}
`````.text

#let json-data = json.decode(json-string)

#resume(json-data)
"#,
        template = RESUME_TEMPLATE,
        json = json_data
    );

    Ok(source)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::documents::resume::{Basics, Resume};

    #[test]
    fn test_transform_resume() {
        let resume = Resume {
            basics: Basics {
                name: "Test User".to_string(),
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
            section_order: None,
        };

        let result = transform_resume(&resume);
        assert!(result.is_ok());
        let source = result.unwrap();

        assert!(source.contains("#let resume(data) = {"));
        assert!(source.contains("Test User"));
        assert!(source.contains("test@example.com"));
        assert!(source.contains("#resume(json-data)"));
    }

    #[test]
    fn test_transform_and_compile() {
        let resume = Resume {
            basics: Basics {
                name: "Test User".to_string(),
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
            section_order: None,
        };

        let source = transform_resume(&resume).unwrap();
        // println!("{}", source); // Uncomment to debug
        let result = crate::typst::compiler::compile(source);
        if let Err(e) = &result {
            for diag in e {
                println!("Diag: {:?} {}", diag.severity, diag.message);
            }
        }
        assert!(result.is_ok());
    }

    #[test]
    fn test_transform_with_section_order() {
        let resume = Resume {
            basics: Basics {
                name: "Test User".to_string(),
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
            section_order: Some(vec![
                "experience".to_string(),
                "education".to_string(),
                "skills".to_string(),
            ]),
        };

        let source = transform_resume(&resume).unwrap();
        // Verify section order is included in the JSON
        assert!(source.contains("sectionOrder"));
        assert!(source.contains("experience"));

        // Verify it compiles successfully
        let result = crate::typst::compiler::compile(source);
        assert!(result.is_ok());
    }
}
