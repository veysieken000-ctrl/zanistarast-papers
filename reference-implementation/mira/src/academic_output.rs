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

    AcademicOutput {
        latex_source,
        pdf_bytes,
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


