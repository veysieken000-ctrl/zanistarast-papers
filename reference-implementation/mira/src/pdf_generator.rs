#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdfDocument {
    pub latex_source: String,
}

pub fn generate_pdf(document: &PdfDocument) -> Vec<u8> {
    // Şimdilik deterministik bir placeholder.
    // Gerçek PDF derleyicisi daha sonraki adımda eklenecek.
    document.latex_source.as_bytes().to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_non_empty_pdf_placeholder() {
        let document = PdfDocument {
            latex_source: "\\documentclass{article}".to_string(),
        };

        let pdf = generate_pdf(&document);

        assert!(!pdf.is_empty());
    }

    #[test]
    fn pdf_generation_is_deterministic() {
        let document = PdfDocument {
            latex_source: "same input".to_string(),
        };

        let first = generate_pdf(&document);
        let second = generate_pdf(&document);

        assert_eq!(first, second);
    }
}



