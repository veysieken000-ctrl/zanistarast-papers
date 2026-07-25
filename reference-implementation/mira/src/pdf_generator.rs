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
pub fn write_latex_source(
    working_directory: &str,
    job_name: &str,
    latex_source: &str,
) -> std::io::Result<std::path::PathBuf> {
    let directory = std::path::Path::new(working_directory);

    std::fs::create_dir_all(directory)?;

    let tex_file = directory.join(format!("{job_name}.tex"));

    std::fs::write(&tex_file, latex_source)?;

    Ok(tex_file)
}
#[derive(Debug)]
pub struct PdfCompileOutput {
    pub success: bool,
    pub stdout: String,
    pub stderr: String,
}

pub fn run_pdf_compiler(
    compiler: PdfCompiler,
    working_directory: &str,
    input_file: &str,
) -> std::io::Result<PdfCompileOutput> {
    let output = std::process::Command::new(compiler.program())
        .args(compiler.arguments(input_file))
        .current_dir(working_directory)
        .output()?;

    Ok(PdfCompileOutput {
        success: output.status.success(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PdfCompileErrorReport {
    pub message: String,
    pub stdout: String,
    pub stderr: String,
}

pub fn build_compile_error_report(
    output: &PdfCompileOutput,
) -> Option<PdfCompileErrorReport> {
    if output.success {
        return None;
    }

    Some(PdfCompileErrorReport {
        message: "PDF compilation failed".to_string(),
        stdout: output.stdout.clone(),
        stderr: output.stderr.clone(),
    })
}
pub fn read_generated_pdf(
    working_directory: &str,
    job_name: &str,
) -> std::io::Result<Vec<u8>> {
    let pdf_path = std::path::Path::new(working_directory)
        .join(format!("{job_name}.pdf"));

    std::fs::read(pdf_path)
}

pub fn has_valid_pdf_signature(pdf: &[u8]) -> bool {
    pdf.starts_with(b"%PDF-")
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
#[test]
fn writes_latex_source_to_tex_file() {
    let working_directory = std::env::temp_dir()
        .join(format!("mira-pdf-test-{}", std::process::id()));

    let working_directory_text = working_directory
        .to_str()
        .expect("temporary directory should be valid UTF-8");

    let latex_source = "\\documentclass{article}\n";

    let tex_file = write_latex_source(
        working_directory_text,
        "article",
        latex_source,
    )
    .expect("LaTeX source should be written");

    assert_eq!(
        tex_file.file_name().and_then(|name| name.to_str()),
        Some("article.tex")
    );

    let written_content = std::fs::read_to_string(&tex_file)
        .expect("written LaTeX source should be readable");

    assert_eq!(written_content, latex_source);

    std::fs::remove_dir_all(&working_directory)
        .expect("temporary directory should be removed");
}

#[test]
fn reports_error_when_pdf_compiler_is_unavailable() {
    let result = std::process::Command::new(
        "mira-nonexistent-pdf-compiler"
    )
    .output();

    assert!(result.is_err());
}

#[test]
fn creates_error_report_for_failed_compilation() {
    let output = PdfCompileOutput {
        success: false,
        stdout: "compiler output".to_string(),
        stderr: "fatal LaTeX error".to_string(),
    };

    let report = build_compile_error_report(&output)
        .expect("failed compilation should create a report");

    assert_eq!(report.message, "PDF compilation failed");
    assert_eq!(report.stdout, "compiler output");
    assert_eq!(report.stderr, "fatal LaTeX error");
}

#[test]
fn does_not_create_error_report_for_successful_compilation() {
    let output = PdfCompileOutput {
        success: true,
        stdout: "success".to_string(),
        stderr: String::new(),
    };

    assert!(build_compile_error_report(&output).is_none());
}

#[test]
fn reads_generated_pdf_file() {
    let working_directory = std::env::temp_dir()
        .join(format!("mira-pdf-read-test-{}", std::process::id()));

    std::fs::create_dir_all(&working_directory)
        .expect("temporary directory should be created");

    let pdf_path = working_directory.join("article.pdf");
    let expected = b"%PDF-1.4\nMira\n";

    std::fs::write(&pdf_path, expected)
        .expect("test PDF should be written");

    let working_directory_text = working_directory
        .to_str()
        .expect("temporary directory should be valid UTF-8");

    let generated = read_generated_pdf(
        working_directory_text,
        "article",
    )
    .expect("generated PDF should be readable");

    assert_eq!(generated, expected);

    std::fs::remove_dir_all(&working_directory)
        .expect("temporary directory should be removed");
}

#[test]
fn reports_error_when_generated_pdf_is_missing() {
    let working_directory = std::env::temp_dir()
        .join(format!("mira-pdf-missing-test-{}", std::process::id()));

    std::fs::create_dir_all(&working_directory)
        .expect("temporary directory should be created");

    let working_directory_text = working_directory
        .to_str()
        .expect("temporary directory should be valid UTF-8");

    let result = read_generated_pdf(
        working_directory_text,
        "missing",
    );

    assert!(result.is_err());

    std::fs::remove_dir_all(&working_directory)
        .expect("temporary directory should be removed");
}

#[test]
fn accepts_valid_pdf_signature() {
    let pdf = b"%PDF-1.7\n";

    assert!(has_valid_pdf_signature(pdf));
}

#[test]
fn rejects_invalid_pdf_signature() {
    let invalid = b"not a pdf";

    assert!(!has_valid_pdf_signature(invalid));
}

#[test]
fn rejects_empty_pdf_content() {
    assert!(!has_valid_pdf_signature(&[]));
}




