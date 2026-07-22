use crate::academic_runner::AcademicRunnerInput;
use crate::article_classifier::AcademicArticleType;
use crate::article_inventory::ArticleCandidate;

/// Bir makale adayının içeriğinden çıkarılan doğrulanabilir işaretler.
///
/// Bu yapı yalnızca tespit edilen özellikleri taşır.
/// Eksik veya belirsiz özellikler otomatik olarak doğru kabul edilmez.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct AcademicContentSignals {
    pub has_abstract: bool,
    pub has_references: bool,
    pub has_conclusion: bool,
    pub has_math: bool,
    pub has_experiments: bool,
}

/// Envanterdeki bir makale adayı ile akademik analiz girdisini birleştirir.
#[derive(Debug, Clone)]
pub struct AdaptedArticleAnalysis {
    pub relative_path: String,
    pub title: Option<String>,
    pub runner_input: AcademicRunnerInput,
}

/// Bir envanter adayını Academic Runner girdisine dönüştürür.
///
/// Makale türü ve içerik işaretleri ayrı analiz katmanlarından gelir.
/// Adaptör herhangi bir akademik özelliği kendiliğinden varsaymaz.
pub fn adapt_article_candidate(
    candidate: &ArticleCandidate,
    article_type: AcademicArticleType,
    signals: AcademicContentSignals,
) -> AdaptedArticleAnalysis {
    AdaptedArticleAnalysis {
        relative_path: candidate
            .relative_path
            .to_string_lossy()
            .into_owned(),

        title: candidate.title.clone(),

        runner_input: AcademicRunnerInput {
            article_type,
            has_abstract: signals.has_abstract,
            has_references: signals.has_references,
            has_conclusion: signals.has_conclusion,
            has_math: signals.has_math,
            has_experiments: signals.has_experiments,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::article_inventory::{
        ArticleCandidate,
        ArticleSourceType,
    };
    use std::path::PathBuf;

    #[test]
    fn adapter_preserves_path_and_title() {
        let candidate = ArticleCandidate {
            relative_path: PathBuf::from("papers/hebun.md"),
            title: Some("Hebûn".to_string()),
            source_type: ArticleSourceType::Markdown,
            domains: Vec::new(),
            size_bytes: 128,
        };

        let adapted = adapt_article_candidate(
            &candidate,
            AcademicArticleType::Theoretical,
            AcademicContentSignals::default(),
        );

        assert_eq!(adapted.relative_path, "papers/hebun.md");
        assert_eq!(adapted.title, Some("Hebûn".to_string()));
    }

    #[test]
    fn adapter_transfers_content_signals_without_changes() {
        let candidate = ArticleCandidate {
            relative_path: PathBuf::from("papers/rasterast.md"),
            title: Some("Rasterast".to_string()),
            source_type: ArticleSourceType::Markdown,
            domains: Vec::new(),
            size_bytes: 256,
        };

        let signals = AcademicContentSignals {
            has_abstract: true,
            has_references: true,
            has_conclusion: false,
            has_math: true,
            has_experiments: false,
        };

        let adapted = adapt_article_candidate(
            &candidate,
            AcademicArticleType::Methodology,
            signals,
        );

        assert_eq!(
            adapted.runner_input.article_type,
            AcademicArticleType::Methodology
        );
        assert!(adapted.runner_input.has_abstract);
        assert!(adapted.runner_input.has_references);
        assert!(!adapted.runner_input.has_conclusion);
        assert!(adapted.runner_input.has_math);
        assert!(!adapted.runner_input.has_experiments);
    }
}


