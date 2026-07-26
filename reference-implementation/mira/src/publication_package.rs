use crate::academic_output::AcademicOutput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationPackage {
    pub title: String,
    pub latex_source: String,
    pub pdf_bytes: Vec<u8>,
    pub bibtex_source: Option<String>,
}

impl PublicationPackage {
    pub fn from_academic_output(
        title: impl Into<String>,
        output: &AcademicOutput,
        bibtex_source: Option<String>,
    ) -> Self {
        Self {
            title: title.into(),
            latex_source: output.latex_source.clone(),
            pdf_bytes: output.pdf_bytes.clone(),
            bibtex_source,
        }
    }

    pub fn is_complete(&self) -> bool {
        !self.title.trim().is_empty()
            && !self.latex_source.trim().is_empty()
            && !self.pdf_bytes.is_empty()
    }

    pub fn has_bibliography(&self) -> bool {
        self.bibtex_source
            .as_ref()
            .is_some_and(|value| !value.trim().is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_publication_package_is_valid() {
        let package = PublicationPackage {
            title: "Rasterast Verification".to_string(),
            latex_source:
                "\\documentclass{article}\n\\begin{document}\n"
                    .to_string(),
            pdf_bytes: b"%PDF-1.7\n".to_vec(),
            bibtex_source: Some(
                "@article{veysi2025}".to_string(),
            ),
        };

        assert!(package.is_complete());
        assert!(package.has_bibliography());
    }

    #[test]
    fn package_without_bibliography_remains_complete() {
        let package = PublicationPackage {
            title: "Zanistarast".to_string(),
            latex_source:
                "\\documentclass{article}".to_string(),
            pdf_bytes: b"%PDF-1.7\n".to_vec(),
            bibtex_source: None,
        };

        assert!(package.is_complete());
        assert!(!package.has_bibliography());
    }

    #[test]
    fn empty_pdf_prevents_package_completeness() {
        let package = PublicationPackage {
            title: "Zanistarast".to_string(),
            latex_source:
                "\\documentclass{article}".to_string(),
            pdf_bytes: Vec::new(),
            bibtex_source: None,
        };

        assert!(!package.is_complete());
    }

    #[test]
    fn builds_package_from_academic_output() {
        use crate::academic_output::{
            generate_academic_output,
            AcademicOutputInput,
        };

        let output = generate_academic_output(
            AcademicOutputInput {
                title: "Rasterast".to_string(),
                author: "Veysi yê MALA SAF".to_string(),
                abstract_text:
                    "Deterministic verification.".to_string(),
                body:
                    "\\section{Intro}\nContent.".to_string(),
                bibliography: None,
            },
        );

        let package = PublicationPackage::from_academic_output(
            "Rasterast",
            &output,
            Some("@article{rasterast2026}".to_string()),
        );

        assert_eq!(package.title, "Rasterast");
        assert_eq!(package.latex_source, output.latex_source);
        assert_eq!(package.pdf_bytes, output.pdf_bytes);
        assert!(package.has_bibliography());
        assert!(package.is_complete());
    }
}



