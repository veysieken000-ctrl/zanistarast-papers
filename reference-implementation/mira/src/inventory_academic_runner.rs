use crate::article_classifier::AcademicArticleType;
use crate::article_inventory::ArticleInventoryReport;
use crate::repository_academic_scan::{
    scan_repository_articles,
    RepositoryArticleAnalysis,
};
use std::path::Path;

/// Makale envanterindeki tüm adayları analiz eder.
pub fn run_inventory_analysis<F>(
    repository_root: &Path,
    inventory: &ArticleInventoryReport,
    classify: F,
) -> Vec<RepositoryArticleAnalysis>
where
    F: Fn(&crate::article_inventory::ArticleCandidate) -> AcademicArticleType,
{
    scan_repository_articles(
        repository_root,
        &inventory.candidates,
        classify,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::article_inventory::{
        ArticleCandidate,
        ArticleInventoryReport,
        ArticleSourceType,
    };
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_repository() -> PathBuf {
        let unique_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "zanistarast_inventory_runner_{}_{}",
            std::process::id(),
            unique_id
        ))
    }

    #[test]
    fn inventory_candidates_are_sent_to_academic_scan() {
        let repository_root = temporary_repository();

        std::fs::create_dir_all(repository_root.join("papers"))
            .expect("papers directory should be created");

        let content = r#"
# Abstract

Rasterast verification model.

# Conclusion

The analysis is complete.

# References

[1] Reference.
"#;

        std::fs::write(
            repository_root.join("papers/rasterast.md"),
            content,
        )
        .expect("article should be written");

        let inventory = ArticleInventoryReport {
            candidates: vec![ArticleCandidate {
                relative_path: PathBuf::from("papers/rasterast.md"),
                title: Some("Rasterast".to_string()),
                source_type: ArticleSourceType::Markdown,
                domains: Vec::new(),
                size_bytes: content.len() as u64,
            }],
            repository_candidate_count: 1,
            website_candidate_count: 0,
        };

        let results = run_inventory_analysis(
            &repository_root,
            &inventory,
            |_| AcademicArticleType::Theoretical,
        );

        assert_eq!(results.len(), 1);
        assert_eq!(
            results[0].relative_path,
            "papers/rasterast.md"
        );
        assert!(results[0].result.is_ok());

        std::fs::remove_dir_all(repository_root)
            .expect("temporary repository should be removed");
    }
}


