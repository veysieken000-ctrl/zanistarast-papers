use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Repository taramasında bulunan tek bir dosyanın kaydı.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepositoryFile {
    pub relative_path: PathBuf,
    pub extension: Option<String>,
    pub size_bytes: u64,
}

/// Salt okunur repository taramasının sonucu.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepositoryScanReport {
    pub root: PathBuf,
    pub files: Vec<RepositoryFile>,
    pub directory_count: usize,
    pub total_size_bytes: u64,
}

impl RepositoryScanReport {
    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}

/// Depoyu yalnızca okuyarak dosya envanteri çıkarır.
///
/// Bu tarayıcı:
/// - dosya oluşturmaz,
/// - dosya değiştirmez,
/// - dosya silmez,
/// - bulunan içerikleri yalnızca raporlar.
#[derive(Debug, Default)]
pub struct RepositoryScanner;

impl RepositoryScanner {
    pub fn new() -> Self {
        Self
    }

    /// Verilen kök klasörü salt okunur biçimde tarar.
    pub fn scan(&self, root: impl AsRef<Path>) -> io::Result<RepositoryScanReport> {
        let root = root.as_ref().canonicalize()?;
        let mut files = Vec::new();
        let mut directory_count = 0;

        Self::scan_directory(
            &root,
            &root,
            &mut files,
            &mut directory_count,
        )?;

        files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));

        let total_size_bytes = files.iter().map(|file| file.size_bytes).sum();

        Ok(RepositoryScanReport {
            root,
            files,
            directory_count,
            total_size_bytes,
        })
    }

    fn scan_directory(
        root: &Path,
        current: &Path,
        files: &mut Vec<RepositoryFile>,
        directory_count: &mut usize,
    ) -> io::Result<()> {
        *directory_count += 1;

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
                Self::scan_directory(
                    root,
                    &path,
                    files,
                    directory_count,
                )?;
                continue;
            }

            if !file_type.is_file() {
                continue;
            }

            let metadata = entry.metadata()?;
            let relative_path = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_path_buf();

            let extension = path
                .extension()
                .and_then(|value| value.to_str())
                .map(str::to_owned);

            files.push(RepositoryFile {
                relative_path,
                extension,
                size_bytes: metadata.len(),
            });
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn scanner_creates_read_only_inventory() {
        let test_root = std::env::temp_dir().join(format!(
            "zanistarast-mira-scanner-{}",
            uuid::Uuid::new_v4()
        ));

        let nested = test_root.join("papers");
        fs::create_dir_all(&nested).expect("test directory should be created");
        fs::write(test_root.join("README.md"), "Zanistarast")
            .expect("test README should be written");
        fs::write(nested.join("hebun.md"), "Hebûn")
            .expect("test article should be written");

        let scanner = RepositoryScanner::new();
        let report = scanner
            .scan(&test_root)
            .expect("repository scan should succeed");

        assert_eq!(report.file_count(), 2);
        assert_eq!(report.directory_count, 2);
        assert!(report.total_size_bytes > 0);

        fs::remove_dir_all(&test_root)
            .expect("test directory should be removed");
    }
}


