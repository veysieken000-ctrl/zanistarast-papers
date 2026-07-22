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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::article_inventory::{
        ArticleCandidate,
        ArticleSourceType,
    };
    use crate::publication_priority::PublicationPriority;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_directory() -> PathBuf {
        let unique_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "zanistarast_mira_analysis_{}_{}",
            std::process::id(),
            unique_id
        ))
    }

    #[test]
    fn service_analyzes_real_article_candidate() {
        let repository_root = temporary_directory();
        let papers_directory = repository_root.join("papers");

        std::fs::create_dir_all(&papers_directory)
            .expect("temporary papers directory should be created");

        let relative_path = PathBuf::from("papers/rasterast.md");
        let full_path = repository_root.join(&relative_path);

        let content = r#"
# Abstract

Rasterast verification method.

# Conclusion

The method was validated.

# References

[1] Test reference.

$$
R(x) = true
$$

Experimental benchmark results.
"#;

        std::fs::write(&full_path, content)
            .expect("temporary article should be written");

        let candidate = ArticleCandidate {
            relative_path,
            title: Some("Rasterast".to_string()),
            source_type: ArticleSourceType::Markdown,
            domains: Vec::new(),
            size_bytes: content.len() as u64,
        };

        let output = analyze_article_candidate(
            &repository_root,
            &candidate,
            AcademicArticleType::Methodology,
        )
        .expect("article candidate should be analyzed");

        assert_eq!(
            output.pipeline.priority,
            PublicationPriority::Critical
        );
        assert!(output.pipeline.rules.passed);
        assert!(output.report.ready_for_publication);

        std::fs::remove_dir_all(&repository_root)
            .expect("temporary repository should be removed");
    }

    #[test]
    fn service_returns_error_when_candidate_file_is_missing() {
        let repository_root = temporary_directory();

        let candidate = ArticleCandidate {
            relative_path: PathBuf::from("papers/missing.md"),
            title: Some("Missing".to_string()),
            source_type: ArticleSourceType::Markdown,
            domains: Vec::new(),
            size_bytes: 0,
        };

        let result = analyze_article_candidate(
            &repository_root,
            &candidate,
            AcademicArticleType::Theoretical,
        );

        assert!(matches!(
            result,
            Err(ArticleAnalysisServiceError::FileAnalysis(
                ArticleFileAnalysisError::ReadFailed(_)
            ))
        ));
    }
}


