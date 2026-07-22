use crate::article_classifier::AcademicArticleType;
use crate::article_inventory::{
    ArticleCandidate,
    ArticleInventoryBuilder,
    ArticleInventoryReport,
};
use crate::inventory_academic_runner::run_inventory_analysis;
use crate::repository_academic_scan::RepositoryArticleAnalysis;
use crate::repository_scanner::{
    RepositoryScanReport,
    RepositoryScanner,
};
use crate::website_scanner::{
    WebsiteScanReport,
    WebsiteScanner,
};
use std::io;
use std::path::Path;

#[derive(Debug)]
pub struct FullAcademicScanReport {
    pub repository_report: RepositoryScanReport,
    pub website_report: WebsiteScanReport,
    pub inventory: ArticleInventoryReport,
    pub analyses: Vec<RepositoryArticleAnalysis>,
}

/// Repository taraması, website taraması, makale envanteri
/// ve akademik analizi tek salt okunur akışta çalıştırır.
pub fn run_full_academic_scan<F>(
    repository_root: &Path,
    website_root: &Path,
    classify: F,
) -> io::Result<FullAcademicScanReport>
where
    F: Fn(&ArticleCandidate) -> AcademicArticleType,
{
    let repository_report =
        RepositoryScanner::new().scan(repository_root)?;

    let website_report =
        WebsiteScanner::new().scan(website_root)?;

    let inventory = ArticleInventoryBuilder::new().build(
        &repository_report,
        &website_report,
    );

    let analyses = run_inventory_analysis(
        repository_root,
        &inventory,
        classify,
    );

    Ok(FullAcademicScanReport {
        repository_report,
        website_report,
        inventory,
        analyses,
    })
}


