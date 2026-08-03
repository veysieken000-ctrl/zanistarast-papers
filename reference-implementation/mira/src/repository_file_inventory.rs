use std::path::{Path, PathBuf};
use std::time::SystemTime;

use uuid::Uuid;

/// Depo envanterinde kayıtlı yolun türünü belirtir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryEntryKind {
    File,
    Directory,
}

/// Bir depodaki tek dosya veya dizinin
/// salt okunur envanter kaydıdır.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryFileRecord {
    pub id: Uuid,
    pub repository_id: Uuid,
    pub relative_path: PathBuf,
    pub absolute_path: PathBuf,
    pub kind: RepositoryEntryKind,
    pub size_bytes: u64,
    pub modified_at: Option<SystemTime>,
    pub sha256_digest: Option<String>,
}

impl RepositoryFileRecord {
    /// Yeni bir depo dosya envanter kaydı oluşturur.
    pub fn new(
        repository_id: Uuid,
        relative_path: impl Into<PathBuf>,
        absolute_path: impl Into<PathBuf>,
        kind: RepositoryEntryKind,
        size_bytes: u64,
        modified_at: Option<SystemTime>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            repository_id,
            relative_path: relative_path.into(),
            absolute_path: absolute_path.into(),
            kind,
            size_bytes,
            modified_at,
            sha256_digest: None,
        }
    }

    /// Dosya kaydına SHA-256 özeti ekler.
    pub fn with_sha256(
        mut self,
        digest: impl Into<String>,
    ) -> Self {
        self.sha256_digest = Some(digest.into());
        self
    }

    /// Kaydın zorunlu alanlarının eksiksiz
    /// olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.relative_path.as_os_str().is_empty()
            && !self.absolute_path.as_os_str().is_empty()
    }

    /// Kaydın normal bir dosyayı temsil ettiğini bildirir.
    pub fn is_file(&self) -> bool {
        self.kind == RepositoryEntryKind::File
    }

    /// Kaydın bir dizini temsil ettiğini bildirir.
    pub fn is_directory(&self) -> bool {
        self.kind == RepositoryEntryKind::Directory
    }

    /// SHA-256 özetinin kaydedilip kaydedilmediğini bildirir.
    pub fn has_sha256(&self) -> bool {
        self.sha256_digest
            .as_ref()
            .is_some_and(|digest| !digest.trim().is_empty())
    }

    /// Kaydın belirtilen göreli yola ait olup olmadığını bildirir.
    pub fn matches_relative_path(
        &self,
        path: impl AsRef<Path>,
    ) -> bool {
        self.relative_path == path.as_ref()
    }
}

/// Mira’nın erişebildiği depolardaki dosya ve dizilerin
/// salt okunur envanter koleksiyonudur.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RepositoryFileInventory {
    records: Vec<RepositoryFileRecord>,
}

impl RepositoryFileInventory {
    /// Boş bir dosya envanteri oluşturur.
    pub fn new() -> Self {
        Self {
            records: Vec::new(),
        }
    }

    /// Eksiksiz ve aynı depo-yol çiftiyle daha önce
    /// kaydedilmemiş bir envanter kaydı ekler.
    pub fn register(
        &mut self,
        record: RepositoryFileRecord,
    ) -> bool {
        if !record.is_complete() {
            return false;
        }

        if self.records.iter().any(|stored| {
            stored.repository_id == record.repository_id
                && stored.relative_path == record.relative_path
        }) {
            return false;
        }

        self.records.push(record);
        true
    }

    /// Bütün envanter kayıtlarını salt okunur döndürür.
    pub fn records(&self) -> &[RepositoryFileRecord] {
        &self.records
    }

    /// Toplam kayıt sayısını döndürür.
    pub fn len(&self) -> usize {
        self.records.len()
    }

    /// Envanterin boş olup olmadığını bildirir.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Kimliğine göre envanter kaydı bulur.
    pub fn find(
        &self,
        record_id: Uuid,
    ) -> Option<&RepositoryFileRecord> {
        self.records
            .iter()
            .find(|record| record.id == record_id)
    }

    /// Belirtilen depoya ait bütün kayıtları döndürür.
    pub fn records_for_repository(
        &self,
        repository_id: Uuid,
    ) -> Vec<&RepositoryFileRecord> {
        self.records
            .iter()
            .filter(|record| {
                record.repository_id == repository_id
            })
            .collect()
    }

    /// Belirtilen depo ve göreli yola ait kaydı bulur.
    pub fn find_by_relative_path(
        &self,
        repository_id: Uuid,
        relative_path: impl AsRef<Path>,
    ) -> Option<&RepositoryFileRecord> {
        self.records.iter().find(|record| {
            record.repository_id == repository_id
                && record.relative_path
                    == relative_path.as_ref()
        })
    }

    /// Envanterdeki normal dosya sayısını döndürür.
    pub fn file_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.is_file())
            .count()
    }

    /// Envanterdeki dizin sayısını döndürür.
    pub fn directory_count(&self) -> usize {
        self.records
            .iter()
            .filter(|record| record.is_directory())
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_complete_repository_file_record() {
        let repository_id = Uuid::new_v4();

        let record = RepositoryFileRecord::new(
            repository_id,
            "src/lib.rs",
            "/projects/zanistarast/src/lib.rs",
            RepositoryEntryKind::File,
            1024,
            Some(SystemTime::now()),
        )
        .with_sha256("0123456789abcdef");

        assert!(record.is_complete());
        assert!(record.is_file());
        assert!(!record.is_directory());
        assert!(record.has_sha256());

        assert!(
            record.matches_relative_path(
                "src/lib.rs",
            )
        );
    }

    #[test]
    fn registers_file_and_directory_records() {
        let repository_id = Uuid::new_v4();
        let mut inventory = RepositoryFileInventory::new();

        let directory = RepositoryFileRecord::new(
            repository_id,
            "src",
            "/projects/zanistarast/src",
            RepositoryEntryKind::Directory,
            0,
            Some(SystemTime::now()),
        );

        let file = RepositoryFileRecord::new(
            repository_id,
            "src/lib.rs",
            "/projects/zanistarast/src/lib.rs",
            RepositoryEntryKind::File,
            2048,
            Some(SystemTime::now()),
        );

        assert!(inventory.register(directory));
        assert!(inventory.register(file));

        assert_eq!(inventory.len(), 2);
        assert_eq!(inventory.file_count(), 1);
        assert_eq!(inventory.directory_count(), 1);
    }

    #[test]
    fn rejects_duplicate_repository_relative_path() {
        let repository_id = Uuid::new_v4();
        let mut inventory = RepositoryFileInventory::new();

        let first = RepositoryFileRecord::new(
            repository_id,
            "src/lib.rs",
            "/projects/zanistarast/src/lib.rs",
            RepositoryEntryKind::File,
            1024,
            None,
        );

        let duplicate = RepositoryFileRecord::new(
            repository_id,
            "src/lib.rs",
            "/other/path/lib.rs",
            RepositoryEntryKind::File,
            2048,
            None,
        );

        assert!(inventory.register(first));
        assert!(!inventory.register(duplicate));
        assert_eq!(inventory.len(), 1);
    }

    #[test]
    fn allows_same_relative_path_in_different_repositories() {
        let first_repository_id = Uuid::new_v4();
        let second_repository_id = Uuid::new_v4();

        let mut inventory = RepositoryFileInventory::new();

        assert!(inventory.register(
            RepositoryFileRecord::new(
                first_repository_id,
                "README.md",
                "/projects/first/README.md",
                RepositoryEntryKind::File,
                100,
                None,
            ),
        ));

        assert!(inventory.register(
            RepositoryFileRecord::new(
                second_repository_id,
                "README.md",
                "/projects/second/README.md",
                RepositoryEntryKind::File,
                200,
                None,
            ),
        ));

        assert_eq!(inventory.len(), 2);

        assert_eq!(
            inventory
                .records_for_repository(
                    first_repository_id,
                )
                .len(),
            1,
        );

        assert_eq!(
            inventory
                .records_for_repository(
                    second_repository_id,
                )
                .len(),
            1,
        );
    }

    #[test]
    fn rejects_incomplete_file_record() {
        let mut inventory = RepositoryFileInventory::new();

        let record = RepositoryFileRecord::new(
            Uuid::new_v4(),
            "",
            "",
            RepositoryEntryKind::File,
            0,
            None,
        );

        assert!(!inventory.register(record));
        assert!(inventory.is_empty());
    }
}



