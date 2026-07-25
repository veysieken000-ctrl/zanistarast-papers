#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdfDocument {
    pub latex_source: String,
}

pub fn generate_pdf(document: &PdfDocument) -> Vec<u8> {
    // Şimdilik deterministik bir placeholder.
    // Gerçek PDF derleyicisi daha sonraki adımda eklenecek.
    document.latex_source.as_bytes().to_vec()
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PdfCompiler {
    PdfLatex,
}

impl PdfCompiler {
    pub fn program(self) -> &'static str {
        match self {
            Self::PdfLatex => "pdflatex",
        }
    }

    pub fn arguments(self, input_file: &str) -> Vec<String> {
        match self {
            Self::PdfLatex => vec![
                "-interaction=nonstopmode".to_string(),
                "-halt-on-error".to_string(),
                input_file.to_string(),
            ],
        }
    }
}
pub fn build_working_directory(job_name: &str) -> String {
    format!("target/mira/pdf/{job_name}")
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

#[test]
fn prepares_deterministic_pdflatex_command() {
    let compiler = PdfCompiler::PdfLatex;

    assert_eq!(compiler.program(), "pdflatex");

    assert_eq!(
        compiler.arguments("article.tex"),
        vec![
            "-interaction=nonstopmode".to_string(),
            "-halt-on-error".to_string(),
            "article.tex".to_string(),
        ]
    );
}

#[test]
fn builds_deterministic_working_directory() {
    let path = build_working_directory("article");

    assert_eq!(path, "target/mira/pdf/article");
}

#[test]
fn different_jobs_have_different_directories() {
    assert_ne!(
        build_working_directory("paper_a"),
        build_working_directory("paper_b"),
    );
}



