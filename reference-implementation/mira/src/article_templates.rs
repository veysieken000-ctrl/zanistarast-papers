use crate::article_classifier::AcademicArticleType;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcademicTemplate {
    Theoretical,
    Ontological,
    Mathematical,
    Methodology,
    SystemArchitecture,
    Experimental,
    Survey,
}

pub fn template_for(
    article_type: AcademicArticleType,
) -> AcademicTemplate {
    match article_type {
        AcademicArticleType::Theoretical =>
            AcademicTemplate::Theoretical,

        AcademicArticleType::Ontological =>
            AcademicTemplate::Ontological,

        AcademicArticleType::Mathematical =>
            AcademicTemplate::Mathematical,

        AcademicArticleType::Methodology =>
            AcademicTemplate::Methodology,

        AcademicArticleType::SystemArchitecture =>
            AcademicTemplate::SystemArchitecture,

        AcademicArticleType::Experimental =>
            AcademicTemplate::Experimental,

        AcademicArticleType::Survey
        | AcademicArticleType::Unknown =>
            AcademicTemplate::Survey,
    }
}


