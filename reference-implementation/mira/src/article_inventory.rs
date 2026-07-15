use crate::repository_scanner::RepositoryScanReport;
use crate::website_scanner::WebsiteScanReport;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Makale adayının kaynak türü.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ArticleSourceType {
    Markdown,
    Html,
    Latex,
}

/// Zanistarast bilimsel sentezindeki temel konu alanları.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ZanistarastDomain {
    Hebun,
    Zanabun,
    Mabun,
    Rabun,
    Rasterast,
    NewrozaKawa,
    Unclassified,
}

/// Envantere alınan tek bir bilimsel makale adayı.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArticleCandidate {
    pub relative_path: PathBuf,
    pub title: Option<String>,
    pub source_type: ArticleSourceType,
    pub domains: Vec<ZanistarastDomain>,
    pub size_bytes: u64,
}

/// Salt okunur makale envanteri sonucu.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArticleInventoryReport {
    pub candidates: Vec<ArticleCandidate>,
    pub repository_candidate_count: usize,
    pub website_candidate_count: usize,
}

impl ArticleInventoryReport {
    pub fn total_candidate_count(&self) -> usize {
        self.candidates.len()
    }

    pub fn candidates_for_domain(
        &self,
        domain: &ZanistarastDomain,
    ) -> Vec<&ArticleCandidate> {
        self.candidates
            .iter()
            .filter(|candidate| candidate.domains.contains(domain))
            .collect()
    }
}

/// Repository ve website tarama raporlarından makale envanteri üretir.
#[derive(Debug, Default)]
pub struct ArticleInventoryBuilder;

impl ArticleInventoryBuilder {
    pub fn new() -> Self {
        Self
    }

    /// Tarama raporlarını birleştirerek salt okunur makale envanteri oluşturur.
    pub fn build(
        &self,
        repository_report: &RepositoryScanReport,
        website_report: &WebsiteScanReport,
    ) -> ArticleInventoryReport {
        let mut candidates = Vec::new();
        let mut known_paths = BTreeSet::new();
        let mut repository_candidate_count = 0;
        let mut website_candidate_count = 0;

        for file in &repository_report.files {
            let Some(source_type) = Self::source_type_from_path(&file.relative_path) else {
                continue;
            };

            if known_paths.insert(file.relative_path.clone()) {
                candidates.push(ArticleCandidate {
                    relative_path: file.relative_path.clone(),
                    title: None,
                    source_type,
                    domains: Self::classify_domains(
                        &file.relative_path,
                        None,
                    ),
                    size_bytes: file.size_bytes,
                });

                repository_candidate_count += 1;
            }
        }

        for page in &website_report.pages {
            if let Some(existing) = candidates
                .iter_mut()
                .find(|candidate| candidate.relative_path == page.relative_path)
            {
                existing.title = page.title.clone();
                existing.domains = Self::classify_domains(
                    &page.relative_path,
                    page.title.as_deref(),
                );
                continue;
            }

            candidates.push(ArticleCandidate {
                relative_path: page.relative_path.clone(),
                title: page.title.clone(),
                source_type: ArticleSourceType::Html,
                domains: Self::classify_domains(
                    &page.relative_path,
                    page.title.as_deref(),
                ),
                size_bytes: page.size_bytes,
            });

            known_paths.insert(page.relative_path.clone());
            website_candidate_count += 1;
        }

        candidates.sort_by(|left, right| {
            left.relative_path.cmp(&right.relative_path)
        });

        ArticleInventoryReport {
            candidates,
            repository_candidate_count,
            website_candidate_count,
        }
    }

    fn source_type_from_path(path: &Path) -> Option<ArticleSourceType> {
        let extension = path.extension()?.to_str()?;

        if extension.eq_ignore_ascii_case("md") {
            return Some(ArticleSourceType::Markdown);
        }

        if extension.eq_ignore_ascii_case("html")
            || extension.eq_ignore_ascii_case("htm")
        {
            return Some(ArticleSourceType::Html);
        }

        if extension.eq_ignore_ascii_case("tex") {
            return Some(ArticleSourceType::Latex);
        }

        None
    }

    fn classify_domains(
        path: &Path,
        title: Option<&str>,
    ) -> Vec<ZanistarastDomain> {
        let mut searchable_text = path.to_string_lossy().to_lowercase();

        if let Some(title) = title {
            searchable_text.push(' ');
            searchable_text.push_str(&title.to_lowercase());
        }

        let mut domains = BTreeSet::new();

        if searchable_text.contains("hebun")
            || searchable_text.contains("hebûn")
        {
            domains.insert(ZanistarastDomain::Hebun);
        }

        if searchable_text.contains("zanabun")
            || searchable_text.contains("zanabûn")
        {
            domains.insert(ZanistarastDomain::Zanabun);
        }

        if searchable_text.contains("mabun")
            || searchable_text.contains("mabûn")
        {
            domains.insert(ZanistarastDomain::Mabun);
        }

        if searchable_text.contains("rabun")
            || searchable_text.contains("rabûn")
        {
            domains.insert(ZanistarastDomain::Rabun);
        }

        if searchable_text.contains("rasterast") {
            domains.insert(ZanistarastDomain::Rasterast);
        }

        if searchable_text.contains("newroza kawa")
            || searchable_text.contains("newroza-kawa")
            || searchable_text.contains("newroza_kawa")
        {
            domains.insert(ZanistarastDomain::NewrozaKawa);
        }

        if domains.is_empty() {
            domains.insert(ZanistarastDomain::Unclassified);
        }

        domains.into_iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository_scanner::{
        RepositoryFile,
        RepositoryScanReport,
    };
    use crate::website_scanner::{
        WebsitePage,
        WebsiteScanReport,
    };

    #[test]
    fn inventory_combines_repository_and_website_candidates() {
        let repository_report = RepositoryScanReport {
            root: PathBuf::from("/tmp/zanistarast"),
            files: vec![
                RepositoryFile {
                    relative_path: PathBuf::from("papers/hebun.md"),
                    extension: Some("md".to_string()),
                    size_bytes: 120,
                },
                RepositoryFile {
                    relative_path: PathBuf::from(
                        "articles/rabun.html",
                    ),
                    extension: Some("html".to_string()),
                    size_bytes: 240,
                },
                RepositoryFile {
                    relative_path: PathBuf::from("images/logo.png"),
                    extension: Some("png".to_string()),
                    size_bytes: 300,
                },
            ],
            directory_count: 3,
            total_size_bytes: 660,
        };

        let website_report = WebsiteScanReport {
            root: PathBuf::from("/tmp/zanistarast"),
            pages: vec![
                WebsitePage {
                    relative_path: PathBuf::from(
                        "articles/rabun.html",
                    ),
                    title: Some(
                        "Rabûn Yönetim Modeli".to_string(),
                    ),
                    links: Vec::new(),
                    images: Vec::new(),
                    size_bytes: 240,
                },
                WebsitePage {
                    relative_path: PathBuf::from(
                        "articles/rasterast.html",
                    ),
                    title: Some(
                        "Rasterast Doğrulama Sistemi".to_string(),
                    ),
                    links: Vec::new(),
                    images: Vec::new(),
                    size_bytes: 180,
                },
            ],
            total_link_count: 0,
            total_image_count: 0,
            total_size_bytes: 420,
        };

        let builder = ArticleInventoryBuilder::new();
        let report = builder.build(
            &repository_report,
            &website_report,
        );

        assert_eq!(report.total_candidate_count(), 3);
        assert_eq!(report.repository_candidate_count, 2);
        assert_eq!(report.website_candidate_count, 1);

        assert_eq!(
            report
                .candidates_for_domain(&ZanistarastDomain::Hebun)
                .len(),
            1
        );

        assert_eq!(
            report
                .candidates_for_domain(&ZanistarastDomain::Rabun)
                .len(),
            1
        );

        assert_eq!(
            report
                .candidates_for_domain(
                    &ZanistarastDomain::Rasterast,
                )
                .len(),
            1
        );
    }

    #[test]
    fn unknown_article_is_marked_unclassified() {
        let domains = ArticleInventoryBuilder::classify_domains(
            Path::new("papers/general-methodology.md"),
            Some("General Methodology"),
        );

        assert_eq!(
            domains,
            vec![ZanistarastDomain::Unclassified]
        );
    }
}







