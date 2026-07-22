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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::article_inventory::{
        ArticleCandidate,
        ArticleSourceType,
    };
    use crate::article_classifier::AcademicArticleType;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_repository() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        std::env::temp_dir().join(format!(
            "zanistarast_repo_scan_{}_{}",
            std::process::id(),
            unique
        ))
    }

    #[test]
    fn repository_scan_continues_after_errors() {
        let repository = temporary_repository();

        std::fs::create_dir_all(repository.join("papers"))
            .unwrap();

        let markdown = r#"
# Abstract

Example

# Conclusion

Done

# References

[1] Ref
"#;

        std::fs::write(
            repository.join("papers/good.md"),
            markdown,
        )
        .unwrap();

        let candidates = vec![
            ArticleCandidate {
                relative_path: PathBuf::from("papers/good.md"),
                title: Some("Good".into()),
                source_type: ArticleSourceType::Markdown,
                domains: Vec::new(),
                size_bytes: markdown.len() as u64,
            },
            ArticleCandidate {
                relative_path: PathBuf::from("papers/missing.md"),
                title: Some("Missing".into()),
                source_type: ArticleSourceType::Markdown,
                domains: Vec::new(),
                size_bytes: 0,
            },
        ];

        let results = scan_repository_articles(
            &repository,
            &candidates,
            |_| AcademicArticleType::Theoretical,
        );

        assert_eq!(results.len(), 2);

        assert!(results[0].result.is_ok());
        assert!(results[1].result.is_err());

        std::fs::remove_dir_all(repository).unwrap();
    }
}



