use crate::article_templates::AcademicTemplate;

pub fn required_sections(
    template: AcademicTemplate,
) -> Vec<&'static str> {
    match template {
        AcademicTemplate::Theoretical => vec![
            "Abstract",
            "Introduction",
            "Theory",
            "Discussion",
            "Conclusion",
            "References",
        ],

        AcademicTemplate::Ontological => vec![
            "Abstract",
            "Ontology",
            "Definitions",
            "Discussion",
            "Conclusion",
            "References",
        ],

        AcademicTemplate::Mathematical => vec![
            "Abstract",
            "Definitions",
            "Axioms",
            "Theorems",
            "Proofs",
            "Conclusion",
            "References",
        ],

        AcademicTemplate::Methodology => vec![
            "Abstract",
            "Method",
            "Validation",
            "Results",
            "Conclusion",
            "References",
        ],

        AcademicTemplate::SystemArchitecture => vec![
            "Abstract",
            "Architecture",
            "Components",
            "Implementation",
            "Conclusion",
            "References",
        ],

        AcademicTemplate::Experimental => vec![
            "Abstract",
            "Method",
            "Experiment",
            "Results",
            "Discussion",
            "References",
        ],

        AcademicTemplate::Survey => vec![
            "Abstract",
            "Introduction",
            "Literature Review",
            "Discussion",
            "Conclusion",
            "References",
        ],
    }
}



