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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_repository() -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "zanistarast_full_scan_{}_{}",
            std::process::id(),
            unique
        ))
    }

    #[test]
    fn full_scan_runs_end_to_end() {
        let root = temporary_repository();

        fs::create_dir_all(root.join("papers")).unwrap();
        fs::create_dir_all(root.join("website")).unwrap();

        fs::write(
            root.join("papers/rasterast.md"),
            r#"
# Abstract

Rasterast verification.

# Conclusion

Done.

# References

[1] Reference.
"#,
        )
        .unwrap();

        fs::write(
            root.join("website/index.html"),
            r#"
<html>
<head>
<title>Zanistarast</title>
</head>
<body>
<a href="papers/rasterast.md">Paper</a>
</body>
</html>
"#,
        )
        .unwrap();

        let report = run_full_academic_scan(
            &root,
            &root.join("website"),
            |_| AcademicArticleType::Theoretical,
        )
        .expect("full scan should succeed");

        assert!(report.repository_report.file_count() >= 2);
        assert_eq!(report.website_report.page_count(), 1);
        assert!(!report.inventory.candidates.is_empty());
        assert!(!report.analyses.is_empty());

        fs::remove_dir_all(root).unwrap();
    }
}


