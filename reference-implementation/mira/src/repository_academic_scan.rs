use crate::academic_runner::AcademicRunnerOutput;
use crate::article_analysis_service::{
    analyze_article_candidate,
    ArticleAnalysisServiceError,
};
use crate::article_classifier::AcademicArticleType;
use crate::article_inventory::ArticleCandidate;
use crate::repository_change_tracker::{
    RepositoryChangeTracker,
    RepositoryFileChange,
};
use crate::repository_file_inventory::RepositoryFileInventory;
use std::path::Path;

#[derive(Debug)]
pub struct RepositoryArticleAnalysis {
    pub relative_path: String,
    pub result: Result<
        AcademicRunnerOutput,
        ArticleAnalysisServiceError,
    >,
}

/// Makale analizlerini ve iki depo envanteri arasında
/// belirlenen dosya değişikliklerini birlikte taşır.
#[derive(Debug)]
pub struct RepositoryAcademicScanResult {
    pub analyses: Vec<RepositoryArticleAnalysis>,
    pub changes: Vec<RepositoryFileChange>,
}

/// Repository envanterindeki makale adaylarını
/// salt okunur biçimde analiz eder.
///
/// Her aday bağımsız işlenir.
/// Bir dosyanın hata vermesi diğer adayların
/// analizini durdurmaz.
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

/// Makale adaylarını analiz ederken önceki ve güncel
/// depo envanterlerini de karşılaştırır.
///
/// Böylece akademik analiz sonuçları ile eklenen,
/// değiştirilen, silinen veya taşınan dosyalar
/// aynı sonuç içinde döndürülür.
pub fn scan_repository_articles_with_changes<F>(
    repository_root: &Path,
    candidates: &[ArticleCandidate],
    previous_inventory: &RepositoryFileInventory,
    current_inventory: &RepositoryFileInventory,
    classify: F,
) -> RepositoryAcademicScanResult
where
    F: Fn(&ArticleCandidate) -> AcademicArticleType,
{
    let analyses = scan_repository_articles(
        repository_root,
        candidates,
        classify,
    );

    let tracker = RepositoryChangeTracker::new();

    let changes = tracker.detect_changes(
        previous_inventory,
        current_inventory,
    );

    RepositoryAcademicScanResult {
        analyses,
        changes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::article_classifier::AcademicArticleType;
    use crate::article_inventory::{
        ArticleCandidate,
        ArticleSourceType,
    };
    use crate::repository_change_tracker::RepositoryChangeKind;
    use crate::repository_file_inventory::{
        RepositoryEntryKind,
        RepositoryFileRecord,
    };
    use std::path::PathBuf;
    use std::time::{
        SystemTime,
        UNIX_EPOCH,
    };
    use uuid::Uuid;

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

        std::fs::create_dir_all(
            repository.join("papers"),
        )
        .unwrap();

        let markdown = r#"# Abstract
Example
# Conclusion
Done
# References
[1] Ref"#;

        std::fs::write(
            repository.join("papers/good.md"),
            markdown,
        )
        .unwrap();

        let candidates = vec![
            ArticleCandidate {
                relative_path:
                    PathBuf::from("papers/good.md"),
                title: Some("Good".into()),
                source_type:
                    ArticleSourceType::Markdown,
                domains: Vec::new(),
                size_bytes: markdown.len() as u64,
            },
            ArticleCandidate {
                relative_path:
                    PathBuf::from("papers/missing.md"),
                title: Some("Missing".into()),
                source_type:
                    ArticleSourceType::Markdown,
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

    #[test]
    fn academic_scan_includes_repository_changes() {
        let repository = temporary_repository();

        std::fs::create_dir_all(
            repository.join("papers"),
        )
        .unwrap();

        let markdown = r#"# Abstract
Example
# Conclusion
Done
# References
[1] Ref"#;

        std::fs::write(
            repository.join("papers/article.md"),
            markdown,
        )
        .unwrap();

        let candidates = vec![
            ArticleCandidate {
                relative_path:
                    PathBuf::from("papers/article.md"),
                title: Some("Article".into()),
                source_type:
                    ArticleSourceType::Markdown,
                domains: Vec::new(),
                size_bytes: markdown.len() as u64,
            },
        ];

        let repository_id = Uuid::new_v4();

        let mut previous_inventory =
            RepositoryFileInventory::new();

        let mut current_inventory =
            RepositoryFileInventory::new();

        assert!(previous_inventory.register(
            RepositoryFileRecord::new(
                repository_id,
                "papers/article.md",
                repository.join("papers/article.md"),
                RepositoryEntryKind::File,
                markdown.len() as u64,
                None,
            )
            .with_sha256("old-digest"),
        ));

        assert!(current_inventory.register(
            RepositoryFileRecord::new(
                repository_id,
                "papers/article.md",
                repository.join("papers/article.md"),
                RepositoryEntryKind::File,
                markdown.len() as u64,
                None,
            )
            .with_sha256("new-digest"),
        ));

        let result =
            scan_repository_articles_with_changes(
                &repository,
                &candidates,
                &previous_inventory,
                &current_inventory,
                |_| AcademicArticleType::Theoretical,
            );

        assert_eq!(result.analyses.len(), 1);
        assert!(result.analyses[0].result.is_ok());

        assert_eq!(result.changes.len(), 1);
        assert_eq!(
            result.changes[0].kind,
            RepositoryChangeKind::Modified,
        );

        std::fs::remove_dir_all(repository).unwrap();
    }
}

