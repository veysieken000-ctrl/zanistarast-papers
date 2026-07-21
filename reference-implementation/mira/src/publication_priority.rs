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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mathematical_article_with_references_and_math_is_critical() {
        let priority = calculate_priority(
            AcademicArticleType::Mathematical,
            true,
            true,
            false,
        );

        assert_eq!(priority, PublicationPriority::Critical);
    }

    #[test]
    fn theoretical_article_without_references_is_medium() {
        let priority = calculate_priority(
            AcademicArticleType::Theoretical,
            false,
            false,
            false,
        );

        assert_eq!(priority, PublicationPriority::Medium);
    }

    #[test]
    fn experimental_article_without_experiments_is_medium() {
        let priority = calculate_priority(
            AcademicArticleType::Experimental,
            true,
            false,
            false,
        );

        assert_eq!(priority, PublicationPriority::Medium);
    }

    #[test]
    fn unknown_article_is_low_priority() {
        let priority = calculate_priority(
            AcademicArticleType::Unknown,
            false,
            false,
            false,
        );

        assert_eq!(priority, PublicationPriority::Low);
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mathematical_article_with_references_and_math_is_critical() {
        let priority = calculate_priority(
            AcademicArticleType::Mathematical,
            true,
            true,
            false,
        );

        assert_eq!(priority, PublicationPriority::Critical);
    }

    #[test]
    fn theoretical_article_without_references_is_medium() {
        let priority = calculate_priority(
            AcademicArticleType::Theoretical,
            false,
            false,
            false,
        );

        assert_eq!(priority, PublicationPriority::Medium);
    }

    #[test]
    fn experimental_article_without_experiments_is_medium() {
        let priority = calculate_priority(
            AcademicArticleType::Experimental,
            true,
            false,
            false,
        );

        assert_eq!(priority, PublicationPriority::Medium);
    }

    #[test]
    fn unknown_article_is_low_priority() {
        let priority = calculate_priority(
            AcademicArticleType::Unknown,
            false,
            false,
            false,
        );

        assert_eq!(priority, PublicationPriority::Low);
    }
}



