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



