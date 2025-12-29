use crate::typst::world::DocgenWorld;
use typst::diag::SourceDiagnostic;

pub fn compile(source: String) -> Result<Vec<u8>, Vec<SourceDiagnostic>> {
    let world = DocgenWorld::new(source);

    let warned_document = typst::compile(&world);

    // Convert EcoVec to Vec
    let document = warned_document
        .output
        .map_err(|e| e.into_iter().collect::<Vec<_>>())?;

    // Use default options (timestamp: None)
    let options = typst_pdf::PdfOptions::default();

    match typst_pdf::pdf(&document, &options) {
        Ok(bytes) => Ok(bytes),
        Err(_) => panic!("Failed to export PDF"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_minimal() {
        let source = "#set page(width: auto, height: auto)\nHello World".to_string();
        let result = compile(source);

        if let Err(ref e) = result {
            for diag in e {
                println!("Diagnostic: {:?}", diag);
            }
        }

        let pdf = result.expect("Compilation failed");
        assert!(pdf.starts_with(b"%PDF"));
    }

    #[test]
    fn test_pdf_content_extraction() {
        let source = "#set page(width: auto, height: auto)\nHello World Verification".to_string();
        let pdf_bytes = compile(source).expect("Compilation failed");

        // Extract text from the PDF bytes
        let text = pdf_extract::extract_text_from_mem(&pdf_bytes).expect("Failed to extract text");

        assert!(
            text.contains("Hello World Verification"),
            "PDF content did not contain expected text. Got: '{}'",
            text
        );
    }
}
