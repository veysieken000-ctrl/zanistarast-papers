use crate::academic_runner::{
    run_academic_analysis,
    AcademicRunnerOutput,
};
use crate::article_analysis_adapter::adapt_article_candidate;
use crate::article_classifier::AcademicArticleType;
use crate::article_file_analyzer::{
    analyze_article_file,
    ArticleFileAnalysisError,
};
use crate::article_inventory::ArticleCandidate;
use std::path::Path;

#[derive(Debug)]
pub enum ArticleAnalysisServiceError {
    FileAnalysis(ArticleFileAnalysisError),
}

/// Bir makale adayının gerçek dosyasını salt okunur biçimde analiz eder
/// ve sonucu Academic Runner üzerinden üretir.
pub fn analyze_article_candidate(
    repository_root: &Path,
    candidate: &ArticleCandidate,
    article_type: AcademicArticleType,
) -> Result<AcademicRunnerOutput, ArticleAnalysisServiceError> {
    let full_path = repository_root.join(&candidate.relative_path);

    let signals = analyze_article_file(
        &full_path,
        &candidate.source_type,
    )
    .map_err(ArticleAnalysisServiceError::FileAnalysis)?;

    let adapted = adapt_article_candidate(
        candidate,
        article_type,
        signals,
    );

    Ok(run_academic_analysis(adapted.runner_input))
}


