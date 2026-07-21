use crate::article_classifier::AcademicArticleType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PublicationPriority {
    Critical,
    High,
    Medium,
    Low,
}

pub fn calculate_priority(
    article_type: AcademicArticleType,
    has_references: bool,
    has_math: bool,
    has_experiments: bool,
) -> PublicationPriority {
    match article_type {
        AcademicArticleType::Mathematical
        | AcademicArticleType::Methodology
        | AcademicArticleType::SystemArchitecture => {
            if has_references && (has_math || has_experiments) {
                PublicationPriority::Critical
            } else {
                PublicationPriority::High
            }
        }

        AcademicArticleType::Theoretical
        | AcademicArticleType::Ontological => {
            if has_references {
                PublicationPriority::High
            } else {
                PublicationPriority::Medium
            }
        }

        AcademicArticleType::Experimental => {
            if has_experiments {
                PublicationPriority::Critical
            } else {
                PublicationPriority::Medium
            }
        }

        AcademicArticleType::Survey => PublicationPriority::Low,

        AcademicArticleType::Unknown => PublicationPriority::Low,
    }
}


