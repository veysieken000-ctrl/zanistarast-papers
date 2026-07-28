use crate::academic_output::AcademicOutput;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationPackage {
    pub title: String,
    pub latex_source: String,
    pub pdf_bytes: Vec<u8>,
    pub bibtex_source: Option<String>,
}

pub fn sanitize_base_name(value: &str) -> String {
    let sanitized: String = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric()
                || character == '-'
                || character == '_'
            {
                character
            } else {
                '_'
            }
        })
        .collect();

    let sanitized = sanitized.trim_matches('_');

    if sanitized.is_empty() {
        "publication".to_string()
    } else {
        sanitized.to_string()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationTarget {
    Zenodo,
    Arxiv,
    HuggingFace,
    PapersWithCode,
}

impl PublicationTarget {
    pub fn name(self) -> &'static str {
        match self {
            Self::Zenodo => "Zenodo",
            Self::Arxiv => "arXiv",
            Self::HuggingFace => "Hugging Face",
            Self::PapersWithCode => "Papers With Code",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationRequest {
    pub target: PublicationTarget,
    pub package: PublicationPackage,
}

impl PublicationRequest {
    pub fn new(
        target: PublicationTarget,
        package: PublicationPackage,
    ) -> Self {
        Self {
            target,
            package,
        }
    }

    pub fn is_ready(&self) -> bool {
        self.package.is_ready_for_publication()
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationResult {
    pub target: PublicationTarget,
    pub success: bool,
    pub identifier: Option<String>,
}

pub trait PublicationService {
    fn publish(
        &self,
        request: &PublicationRequest,
    ) -> PublicationResult;
}
    

pub fn build_publication_package(
    title: impl Into<String>,
    academic_output: &AcademicOutput,
    bibtex_source: Option<String>,
) -> PublicationPackage {
    PublicationPackage::from_academic_output(
        title,
        academic_output,
        bibtex_source,
    )
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

    pub fn with_bibtex(
        mut self,
        bibtex: impl Into<String>,
    ) -> Self {
        self.bibtex_source = Some(bibtex.into());
        self
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

    pub fn is_ready_for_publication(&self) -> bool {
        self.is_complete() && self.has_bibliography()
    }
}

pub fn export_publication_package(
    package: &PublicationPackage,
    output_directory: &str,
    base_name: &str,
) -> std::io::Result<Vec<std::path::PathBuf>> {
    let directory = std::path::Path::new(output_directory);
    let base_name = sanitize_base_name(base_name);

    std::fs::create_dir_all(directory)?;

    let tex_path = directory.join(format!("{base_name}.tex"));
    let pdf_path = directory.join(format!("{base_name}.pdf"));

    std::fs::write(&tex_path, &package.latex_source)?;
    std::fs::write(&pdf_path, &package.pdf_bytes)?;

    let mut written_files = vec![tex_path, pdf_path];

    if let Some(bibtex_source) = &package.bibtex_source {
        let bib_path = directory.join(format!("{base_name}.bib"));

        std::fs::write(&bib_path, bibtex_source)?;

        written_files.push(bib_path);
    }

    Ok(written_files)
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

#[test]
fn can_attach_bibtex_after_creation() {
    use crate::academic_output::{
        generate_academic_output,
        AcademicOutputInput,
    };

    let output = generate_academic_output(
        AcademicOutputInput {
            title: "Rasterast".to_string(),
            author: "Veysi yê MALA SAF".to_string(),
            abstract_text: String::new(),
            body: String::new(),
            bibliography: None,
        },
    );

    let package = PublicationPackage::from_academic_output(
        "Rasterast",
        &output,
        None,
    )
    .with_bibtex("@article{rasterast2026}");

    assert!(package.has_bibliography());

    assert_eq!(
        package.bibtex_source.as_deref(),
        Some("@article{rasterast2026}")
    );
}

#[test]
fn publication_package_requires_bibliography_for_publication() {
    use crate::academic_output::{
        generate_academic_output,
        AcademicOutputInput,
    };

    let output = generate_academic_output(
        AcademicOutputInput {
            title: "Rasterast".to_string(),
            author: "Veysi yê MALA SAF".to_string(),
            abstract_text: String::new(),
            body: String::new(),
            bibliography: None,
        },
    );

    let package = PublicationPackage::from_academic_output(
        "Rasterast",
        &output,
        None,
    );

    assert!(package.is_complete());
    assert!(!package.has_bibliography());
    assert!(!package.is_ready_for_publication());

    let package = package.with_bibtex("@article{rasterast2026}");

    assert!(package.has_bibliography());
    assert!(package.is_ready_for_publication());
}

#[test]
fn exports_publication_package_files() {
    let package = PublicationPackage {
        title: "Rasterast Verification".to_string(),
        latex_source:
            "\\documentclass{article}\n".to_string(),
        pdf_bytes: b"%PDF-1.7\n".to_vec(),
        bibtex_source: Some(
            "@article{rasterast2026}".to_string(),
        ),
    };

    let output_directory = std::env::temp_dir().join(
        format!(
            "mira-publication-package-{}",
            std::process::id()
        ),
    );

    let output_directory_text = output_directory
        .to_str()
        .expect("temporary directory should be valid UTF-8");

    let written_files = export_publication_package(
        &package,
        output_directory_text,
        "rasterast",
    )
    .expect("publication package should be exported");

    assert_eq!(written_files.len(), 3);

    assert_eq!(
        std::fs::read_to_string(
            output_directory.join("rasterast.tex")
        )
        .expect("LaTeX file should be readable"),
        package.latex_source
    );

    assert_eq!(
        std::fs::read(
            output_directory.join("rasterast.pdf")
        )
        .expect("PDF file should be readable"),
        package.pdf_bytes
    );

    assert_eq!(
        std::fs::read_to_string(
            output_directory.join("rasterast.bib")
        )
        .expect("BibTeX file should be readable"),
        "@article{rasterast2026}"
    );

    std::fs::remove_dir_all(&output_directory)
        .expect("temporary directory should be removed");
}

#[test]
fn exports_package_without_bibtex_as_two_files() {
    let package = PublicationPackage {
        title: "Zanistarast".to_string(),
        latex_source:
            "\\documentclass{article}\n".to_string(),
        pdf_bytes: b"%PDF-1.7\n".to_vec(),
        bibtex_source: None,
    };

    let output_directory = std::env::temp_dir().join(
        format!(
            "mira-publication-package-no-bib-{}",
            std::process::id()
        ),
    );

    let output_directory_text = output_directory
        .to_str()
        .expect("temporary directory should be valid UTF-8");

    let written_files = export_publication_package(
        &package,
        output_directory_text,
        "zanistarast",
    )
    .expect("publication package should be exported");

    assert_eq!(written_files.len(), 2);

    assert!(
        output_directory
            .join("zanistarast.tex")
            .exists()
    );

    assert!(
        output_directory
            .join("zanistarast.pdf")
            .exists()
    );

    assert!(
        !output_directory
            .join("zanistarast.bib")
            .exists()
    );

    std::fs::remove_dir_all(&output_directory)
        .expect("temporary directory should be removed");
}

#[test]
fn sanitizes_publication_file_name() {
    assert_eq!(
        sanitize_base_name("Rasterast Verification/2026"),
        "Rasterast_Verification_2026"
    );

    assert_eq!(
        sanitize_base_name("zanistarast-paper_v1"),
        "zanistarast-paper_v1"
    );

    assert_eq!(
        sanitize_base_name("///"),
        "publication"
    );
}

#[test]
fn exports_files_with_sanitized_base_name() {
    let package = PublicationPackage {
        title: "Rasterast Verification".to_string(),
        latex_source:
            "\\documentclass{article}\n".to_string(),
        pdf_bytes: b"%PDF-1.7\n".to_vec(),
        bibtex_source: Some(
            "@article{rasterast2026}".to_string(),
        ),
    };

    let output_directory = std::env::temp_dir().join(
        format!(
            "mira-publication-sanitized-{}",
            std::process::id()
        ),
    );

    let output_directory_text = output_directory
        .to_str()
        .expect("temporary directory should be valid UTF-8");

    let written_files = export_publication_package(
        &package,
        output_directory_text,
        "Rasterast Verification/2026",
    )
    .expect("publication package should be exported");

    assert_eq!(written_files.len(), 3);

    assert!(
        output_directory
            .join("Rasterast_Verification_2026.tex")
            .exists()
    );

    assert!(
        output_directory
            .join("Rasterast_Verification_2026.pdf")
            .exists()
    );

    assert!(
        output_directory
            .join("Rasterast_Verification_2026.bib")
            .exists()
    );

    assert!(
        !output_directory
            .join("Rasterast Verification")
            .exists()
    );

    std::fs::remove_dir_all(&output_directory)
        .expect("temporary directory should be removed");
}

#[test]
fn builds_final_publication_package() {
    use crate::academic_output::{
        generate_academic_output,
        AcademicOutputInput,
    };

    let academic_output = generate_academic_output(
        AcademicOutputInput {
            title: "Rasterast Verification".to_string(),
            author: "Veysi yê MALA SAF".to_string(),
            abstract_text:
                "Deterministic verification.".to_string(),
            body:
                "\\section{Introduction}\nZanistarast."
                    .to_string(),
            bibliography: Some("references".to_string()),
        },
    );

    let package = build_publication_package(
        "Rasterast Verification",
        &academic_output,
        Some("@article{rasterast2026}".to_string()),
    );

    assert_eq!(
        package.title,
        "Rasterast Verification"
    );

    assert_eq!(
        package.latex_source,
        academic_output.latex_source
    );

    assert_eq!(
        package.pdf_bytes,
        academic_output.pdf_bytes
    );

    assert!(package.has_bibliography());
    assert!(package.is_complete());
    assert!(package.is_ready_for_publication());
}

#[test]
fn builds_and_exports_final_publication_package() {
    use crate::academic_output::{
        generate_academic_output,
        AcademicOutputInput,
    };

    let academic_output = generate_academic_output(
        AcademicOutputInput {
            title: "Rasterast Verification".to_string(),
            author: "Veysi yê MALA SAF".to_string(),
            abstract_text:
                "Deterministic verification.".to_string(),
            body:
                "\\section{Introduction}\nZanistarast."
                    .to_string(),
            bibliography: Some("references".to_string()),
        },
    );

    let package = build_publication_package(
        "Rasterast Verification",
        &academic_output,
        Some("@article{rasterast2026}".to_string()),
    );

    assert!(package.is_ready_for_publication());

    let output_directory = std::env::temp_dir().join(
        format!(
            "mira-final-publication-package-{}",
            std::process::id()
        ),
    );

    let output_directory_text = output_directory
        .to_str()
        .expect("temporary directory should be valid UTF-8");

    let written_files = export_publication_package(
        &package,
        output_directory_text,
        "Rasterast Verification/2026",
    )
    .expect("final publication package should be exported");

    assert_eq!(written_files.len(), 3);

    assert!(
        output_directory
            .join("Rasterast_Verification_2026.tex")
            .exists()
    );

    assert!(
        output_directory
            .join("Rasterast_Verification_2026.pdf")
            .exists()
    );

    assert!(
        output_directory
            .join("Rasterast_Verification_2026.bib")
            .exists()
    );

    std::fs::remove_dir_all(&output_directory)
        .expect("temporary directory should be removed");
}

#[test]
fn publication_targets_have_stable_names() {
    assert_eq!(PublicationTarget::Zenodo.name(), "Zenodo");
    assert_eq!(PublicationTarget::Arxiv.name(), "arXiv");
    assert_eq!(
        PublicationTarget::HuggingFace.name(),
        "Hugging Face"
    );
    assert_eq!(
        PublicationTarget::PapersWithCode.name(),
        "Papers With Code"
    );
}

#[test]
fn publication_request_reports_package_readiness() {
    let ready_package = PublicationPackage {
        title: "Rasterast Verification".to_string(),
        latex_source:
            "\\documentclass{article}\n\\begin{document}\nZanistarast.\n\\end{document}"
                .to_string(),
        pdf_bytes: b"%PDF-1.7\n".to_vec(),
        bibtex_source: Some(
            "@article{rasterast2026}".to_string(),
        ),
    };

    let request = PublicationRequest::new(
        PublicationTarget::Zenodo,
        ready_package,
    );

    assert_eq!(
        request.target,
        PublicationTarget::Zenodo
    );

    assert_eq!(
        request.target.name(),
        "Zenodo"
    );

    assert!(request.is_ready());
}

#[test]
fn publication_request_rejects_incomplete_package() {
    let incomplete_package = PublicationPackage {
        title: "Rasterast Verification".to_string(),
        latex_source:
            "\\documentclass{article}\n\\begin{document}\nZanistarast.\n\\end{document}"
                .to_string(),
        pdf_bytes: b"%PDF-1.7\n".to_vec(),
        bibtex_source: None,
    };

    let request = PublicationRequest::new(
        PublicationTarget::Zenodo,
        incomplete_package,
    );

    assert!(!request.is_ready());
}

#[test]
fn publication_result_can_represent_success() {
    let result = PublicationResult {
        target: PublicationTarget::Zenodo,
        success: true,
        identifier: Some(
            "10.5281/zenodo.1234567".to_string(),
        ),
    };

    assert!(result.success);
    assert_eq!(
        result.target,
        PublicationTarget::Zenodo
    );
    assert_eq!(
        result.identifier.as_deref(),
        Some("10.5281/zenodo.1234567")
    );
}






