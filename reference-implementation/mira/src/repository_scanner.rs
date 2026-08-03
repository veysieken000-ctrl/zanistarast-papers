use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::{
    RepositoryEntryKind,
    RepositoryFileInventory,
    RepositoryFileRecord,
    RepositoryRoot,
};

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
/// Kayıtlı bir depo kökünü salt okunur biçimde tarayarak
    /// dosya ve dizin kayıtlarını RepositoryFileInventory
    /// modeline aktarır.
    ///
    /// Normal dosyalar için SHA-256 özeti hesaplanır.
    /// Sembolik bağlantılar takip edilmez.
    pub fn scan_inventory(
        &self,
        repository: &RepositoryRoot,
    ) -> io::Result<RepositoryFileInventory> {
        if !repository.is_complete()
            || !repository.read_only
        {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "repository must be complete and read-only",
            ));
        }

        if !repository.root_path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "repository root directory was not found",
            ));
        }

        let root = repository.root_path.canonicalize()?;
        let mut inventory = RepositoryFileInventory::new();

        Self::scan_inventory_directory(
            repository,
            &root,
            &root,
            &mut inventory,
        )?;

        Ok(inventory)
    }

    fn scan_inventory_directory(
        repository: &RepositoryRoot,
        root: &Path,
        current: &Path,
        inventory: &mut RepositoryFileInventory,
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

            let relative_path = path
                .strip_prefix(root)
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "repository entry is outside root",
                    )
                })?
                .to_path_buf();

            let metadata = entry.metadata()?;

            if file_type.is_dir() {
                let record = RepositoryFileRecord::new(
                    repository.id,
                    relative_path,
                    path.clone(),
                    RepositoryEntryKind::Directory,
                    0,
                    metadata.modified().ok(),
                );

                Self::register_inventory_record(
                    inventory,
                    record,
                )?;

                Self::scan_inventory_directory(
                    repository,
                    root,
                    &path,
                    inventory,
                )?;

                continue;
            }

            if !file_type.is_file() {
                continue;
            }

            let digest = Self::calculate_sha256(&path)?;

            let record = RepositoryFileRecord::new(
                repository.id,
                relative_path,
                path,
                RepositoryEntryKind::File,
                metadata.len(),
                metadata.modified().ok(),
            )
            .with_sha256(digest);

            Self::register_inventory_record(
                inventory,
                record,
            )?;
        }

        Ok(())
    }

    fn calculate_sha256(
        path: &Path,
    ) -> io::Result<String> {
        let bytes = fs::read(path)?;

        let mut hasher = Sha256::new();
        hasher.update(bytes);

        Ok(format!("{:x}", hasher.finalize()))
    }

    fn register_inventory_record(
        inventory: &mut RepositoryFileInventory,
        record: RepositoryFileRecord,
    ) -> io::Result<()> {
        if inventory.register(record) {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "repository inventory record was rejected",
            ))
        }
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
#[test]
    fn scanner_builds_hashed_repository_file_inventory() {
        let test_root = std::env::temp_dir().join(
            format!(
                "zanistarast-mira-inventory-{}",
                uuid::Uuid::new_v4(),
            ),
        );

        let nested = test_root.join("src");
        let source_path = nested.join("lib.rs");
        let readme_path = test_root.join("README.md");

        fs::create_dir_all(&nested)
            .expect("test directory should be created");

        fs::write(
            &source_path,
            b"pub fn hebun() {}",
        )
        .expect("source file should be written");

        fs::write(
            &readme_path,
            b"# Zanistarast",
        )
        .expect("README should be written");

        let original_source = fs::read(&source_path)
            .expect("source file should be readable");

        let repository = RepositoryRoot::new(
            "zanistarast-test",
            &test_root,
        );

        let scanner = RepositoryScanner::new();

        let inventory = scanner
            .scan_inventory(&repository)
            .expect("repository inventory scan should succeed");

        assert_eq!(inventory.file_count(), 2);
        assert_eq!(inventory.directory_count(), 1);
        assert_eq!(inventory.len(), 3);

        let source_record = inventory
            .find_by_relative_path(
                repository.id,
                "src/lib.rs",
            )
            .expect("source file should be inventoried");

        assert!(source_record.is_file());
        assert!(source_record.has_sha256());

        assert_eq!(
            source_record
                .sha256_digest
                .as_ref()
                .expect("SHA-256 digest should exist")
                .len(),
            64,
        );

        let directory_record = inventory
            .find_by_relative_path(
                repository.id,
                "src",
            )
            .expect("source directory should be inventoried");

        assert!(directory_record.is_directory());

        assert_eq!(
            fs::read(&source_path)
                .expect("source file should remain readable"),
            original_source,
        );

        fs::remove_dir_all(&test_root)
            .expect("test directory should be removed");
    }
}


