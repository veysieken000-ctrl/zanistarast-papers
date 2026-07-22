use crate::academic_runner::AcademicRunnerOutput;
use crate::article_analysis_service::{
    analyze_article_candidate,
    ArticleAnalysisServiceError,
};
use crate::article_classifier::AcademicArticleType;
use crate::article_inventory::ArticleCandidate;
use std::path::Path;

#[derive(Debug)]
pub struct RepositoryArticleAnalysis {
    pub relative_path: String,
    pub result: Result<
        AcademicRunnerOutput,
        ArticleAnalysisServiceError,
    >,
}

/// Repository envanterindeki makale adaylarını salt okunur biçimde analiz eder.
///
/// Her aday bağımsız işlenir.
/// Bir dosyanın hata vermesi diğer adayların analizini durdurmaz.
pub fn scan_repository_articles<F>(
    repository_root: &Path,
    candidates: &[ArticleCandidate],
    classify: F,
) -> Vec<RepositoryArticleAnalysis>
where
    F: Fn(&ArticleCandidate) -> AcademicArticleType,
{
    candidates
        .iter()
        .map(|candidate| {
            let article_type = classify(candidate);

            let result = analyze_article_candidate(
                repository_root,
                candidate,
                article_type,
            );

            RepositoryArticleAnalysis {
                relative_path: candidate
                    .relative_path
                    .to_string_lossy()
                    .into_owned(),
                result,
            }
        })
        .collect()
}


