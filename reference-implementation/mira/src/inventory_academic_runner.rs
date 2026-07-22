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


