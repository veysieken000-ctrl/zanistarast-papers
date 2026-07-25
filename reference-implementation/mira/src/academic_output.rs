use crate::latex_generator::{
    generate_latex_article,
    LatexArticle,
};
use crate::pdf_generator::{
    generate_pdf,
    PdfDocument,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcademicOutputInput {
    pub title: String,
    pub author: String,
    pub abstract_text: String,
    pub body: String,
    pub bibliography: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcademicOutput {
    pub latex_source: String,
    pub pdf_bytes: Vec<u8>,
    pub is_valid: bool,
}

pub fn generate_academic_output(
    input: AcademicOutputInput,
) -> AcademicOutput {
    let latex_article = LatexArticle {
        title: input.title,
        author: input.author,
        abstract_text: input.abstract_text,
        body: input.body,
        bibliography: input.bibliography,
    };

    let latex_source = generate_latex_article(&latex_article);

    let pdf_document = PdfDocument {
        latex_source: latex_source.clone(),
    };

    let pdf_bytes = generate_pdf(&pdf_document);

    let is_valid =
    !latex_source.is_empty()
        && !pdf_bytes.is_empty()
        && pdf_bytes == latex_source.as_bytes();

AcademicOutput {
    latex_source,
    pdf_bytes,
    is_valid,
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_combined_academic_output() {
        let output = generate_academic_output(
            AcademicOutputInput {
                title: "Rasterast Verification".to_string(),
                author: "Veysi yê MALA SAF".to_string(),
                abstract_text:
                    "Deterministic verification.".to_string(),
                body:
                    "\\section{Introduction}\nZanistarast."
                        .to_string(),
                bibliography: Some(
                    "references".to_string(),
                ),
            },
        );
        assert!(output.is_valid);
        
        assert!(output.latex_source.contains(
            "\\title{Rasterast Verification}"
        ));

        assert!(output.latex_source.contains(
            "\\author{Veysi yê MALA SAF}"
        ));

        assert!(output.latex_source.contains(
            "\\bibliography{references}"
        ));

        assert_eq!(
            output.pdf_bytes,
            output.latex_source.as_bytes()
        );
    }
}

#[test]
fn generated_academic_output_is_not_empty() {
    let output = generate_academic_output(
        AcademicOutputInput {
            title: "Zanistarast".to_string(),
            author: "Veysi yê MALA SAF".to_string(),
            abstract_text: "Scientific synthesis.".to_string(),
            body: "\\section{Introduction}\nContent."
                .to_string(),
            bibliography: None,
        },
    );

    assert!(!output.latex_source.is_empty());
    assert!(!output.pdf_bytes.is_empty());
    assert!(output.is_valid);
}




