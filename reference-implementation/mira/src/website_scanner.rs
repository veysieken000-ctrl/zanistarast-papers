use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// İncelenen tek bir HTML sayfasının kaydı.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WebsitePage {
    pub relative_path: PathBuf,
    pub title: Option<String>,
    pub links: Vec<String>,
    pub images: Vec<String>,
    pub size_bytes: u64,
}

/// Yerel web sitesi taramasının salt okunur sonucu.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WebsiteScanReport {
    pub root: PathBuf,
    pub pages: Vec<WebsitePage>,
    pub total_link_count: usize,
    pub total_image_count: usize,
    pub total_size_bytes: u64,
}

impl WebsiteScanReport {
    pub fn page_count(&self) -> usize {
        self.pages.len()
    }
}

/// Yerel HTML dosyalarını salt okunur biçimde tarar.
///
/// Bu tarayıcı:
/// - yalnızca `.html` ve `.htm` dosyalarını okur,
/// - dosya oluşturmaz,
/// - dosya değiştirmez,
/// - dosya silmez,
/// - sayfa başlıklarını, bağlantıları ve görselleri raporlar.
#[derive(Debug, Default)]
pub struct WebsiteScanner;

impl WebsiteScanner {
    pub fn new() -> Self {
        Self
    }

    /// Verilen web sitesi klasörünü salt okunur biçimde tarar.
    pub fn scan(&self, root: impl AsRef<Path>) -> io::Result<WebsiteScanReport> {
        let root = root.as_ref().canonicalize()?;
        let mut pages = Vec::new();

        Self::scan_directory(&root, &root, &mut pages)?;

        pages.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));

        let total_link_count = pages.iter().map(|page| page.links.len()).sum();
        let total_image_count = pages.iter().map(|page| page.images.len()).sum();
        let total_size_bytes = pages.iter().map(|page| page.size_bytes).sum();

        Ok(WebsiteScanReport {
            root,
            pages,
            total_link_count,
            total_image_count,
            total_size_bytes,
        })
    }

    fn scan_directory(
        root: &Path,
        current: &Path,
        pages: &mut Vec<WebsitePage>,
    ) -> io::Result<()> {
        let mut entries = fs::read_dir(current)?
            .collect::<Result<Vec<_>, io::Error>>()?;

        entries.sort_by_key(|entry| entry.path());

        for entry in entries {
            let path = entry.path();
            let file_type = entry.file_type()?;

            if file_type.is_symlink() {
                continue;
            }

            if file_type.is_dir() {
                Self::scan_directory(root, &path, pages)?;
                continue;
            }

            if !file_type.is_file() || !Self::is_html_file(&path) {
                continue;
            }

            let content = fs::read_to_string(&path)?;
            let metadata = entry.metadata()?;

            let relative_path = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_path_buf();

            pages.push(WebsitePage {
                relative_path,
                title: Self::extract_title(&content),
                links: Self::extract_attribute_values(&content, "href"),
                images: Self::extract_attribute_values(&content, "src"),
                size_bytes: metadata.len(),
            });
        }

        Ok(())
    }

    fn is_html_file(path: &Path) -> bool {
        path.extension()
            .and_then(|extension| extension.to_str())
            .map(|extension| {
                extension.eq_ignore_ascii_case("html")
                    || extension.eq_ignore_ascii_case("htm")
            })
            .unwrap_or(false)
    }

    fn extract_title(content: &str) -> Option<String> {
        let lowercase = content.to_lowercase();
        let start_tag = "<title>";
        let end_tag = "</title>";

        let start = lowercase.find(start_tag)? + start_tag.len();
        let remaining = &lowercase[start..];
        let end = remaining.find(end_tag)? + start;

        let title = content[start..end].trim();

        if title.is_empty() {
            None
        } else {
            Some(title.to_string())
        }
    }

    fn extract_attribute_values(content: &str, attribute: &str) -> Vec<String> {
        let mut values = Vec::new();
        let lowercase = content.to_lowercase();
        let patterns = [
            format!("{attribute}=\""),
            format!("{attribute}='"),
        ];

        for pattern in patterns {
            let quote = pattern
                .chars()
                .last()
                .expect("attribute pattern must contain a quote");

            let mut search_start = 0;

            while let Some(relative_start) = lowercase[search_start..].find(&pattern) {
                let value_start = search_start + relative_start + pattern.len();
                let remaining = &content[value_start..];

                let Some(value_end) = remaining.find(quote) else {
                    break;
                };

                let value = remaining[..value_end].trim();

                if !value.is_empty() {
                    values.push(value.to_string());
                }

                search_start = value_start + value_end + 1;
            }
        }

        values.sort();
        values.dedup();
        values
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scanner_reads_html_pages_without_modifying_them() {
        let test_root = std::env::temp_dir().join(format!(
            "zanistarast-mira-website-scanner-{}",
            uuid::Uuid::new_v4()
        ));

        let articles = test_root.join("articles");
        fs::create_dir_all(&articles)
            .expect("test website directory should be created");

        fs::write(
            test_root.join("index.html"),
            r#"
                <html>
                    <head>
                        <title>Zanistarast</title>
                    </head>
                    <body>
                        <a href="articles/hebun.html">Hebûn</a>
                        <img src="images/logo.png">
                    </body>
                </html>
            "#,
        )
        .expect("test index page should be written");

        fs::write(
            articles.join("hebun.html"),
            r#"
                <html>
                    <head>
                        <title>Hebûn Makalesi</title>
                    </head>
                    <body>
                        <a href="../index.html">Ana Sayfa</a>
                    </body>
                </html>
            "#,
        )
        .expect("test article page should be written");

        fs::write(test_root.join("README.md"), "Not an HTML page")
            .expect("test non-HTML file should be written");

        let scanner = WebsiteScanner::new();
        let report = scanner
            .scan(&test_root)
            .expect("website scan should succeed");

        assert_eq!(report.page_count(), 2);
        assert_eq!(report.total_link_count, 2);
        assert_eq!(report.total_image_count, 1);
        assert!(report.total_size_bytes > 0);

        let index_page = report
            .pages
            .iter()
            .find(|page| page.relative_path == PathBuf::from("index.html"))
            .expect("index page should exist");

        assert_eq!(index_page.title.as_deref(), Some("Zanistarast"));
        assert_eq!(index_page.links, vec!["articles/hebun.html"]);
        assert_eq!(index_page.images, vec!["images/logo.png"]);

        fs::remove_dir_all(&test_root)
            .expect("test website directory should be removed");
    }
}



pub mod website_scanner;
