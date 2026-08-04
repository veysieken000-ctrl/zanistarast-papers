use std::path::PathBuf;

use crate::repository_file_inventory::RepositoryFileInventory;

/// İki repository taraması arasında belirlenen
/// dosya değişikliğinin türü.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryChangeKind {
    Added,
    Modified,
    Removed,
    Moved,
}

/// Repository içinde belirlenen tek bir dosya
/// değişikliğinin kaydı.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryFileChange {
    pub kind: RepositoryChangeKind,
    pub previous_path: Option<PathBuf>,
    pub current_path: Option<PathBuf>,
}

impl RepositoryFileChange {
    /// Yeni eklenen bir dosya değişikliği oluşturur.
    pub fn added(
        current_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            kind: RepositoryChangeKind::Added,
            previous_path: None,
            current_path: Some(current_path.into()),
        }
    }

    /// İçeriği değişen bir dosya kaydı oluşturur.
    pub fn modified(
        path: impl Into<PathBuf>,
    ) -> Self {
        let path = path.into();

        Self {
            kind: RepositoryChangeKind::Modified,
            previous_path: Some(path.clone()),
            current_path: Some(path),
        }
    }

    /// Silinen bir dosya değişikliği oluşturur.
    pub fn removed(
        previous_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            kind: RepositoryChangeKind::Removed,
            previous_path: Some(previous_path.into()),
            current_path: None,
        }
    }

    /// Taşınan veya yeniden adlandırılan bir dosya
    /// değişikliği oluşturur.
    pub fn moved(
        previous_path: impl Into<PathBuf>,
        current_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            kind: RepositoryChangeKind::Moved,
            previous_path: Some(previous_path.into()),
            current_path: Some(current_path.into()),
        }
    }
}

/// İki repository dosya envanteri arasındaki
/// değişiklikleri belirler.
#[derive(Debug, Default)]
pub struct RepositoryChangeTracker;

impl RepositoryChangeTracker {
    /// Yeni bir repository değişiklik izleyicisi oluşturur.
    pub fn new() -> Self {
        Self
    }

    /// Önceki ve güncel envanteri karşılaştırarak
    /// yeni eklenen dosyaları belirler.
    pub fn detect_changes(
        &self,
        previous: &RepositoryFileInventory,
        current: &RepositoryFileInventory,
    ) -> Vec<RepositoryFileChange> {
        let mut changes = Vec::new();

        for record in current.records() {
           if previous
    .find_by_relative_path(
        record.repository_id,
        &record.relative_path,
    )
    .is_none()
{
    changes.push(
        RepositoryFileChange::added(
            record.relative_path.clone(),
        ),
    );
} else if let Some(previous_record) = previous.find_by_relative_path(
    record.repository_id,
    &record.relative_path,
) && previous_record.sha256_digest != record.sha256_digest
{
    changes.push(
        RepositoryFileChange::modified(
            record.relative_path.clone(),
        ),
    );
}
            }
      
        for record in previous.records() {
    if current
        .find_by_relative_path(
            record.repository_id,
            &record.relative_path,
        )
        .is_none()
    {
        changes.push(
            RepositoryFileChange::removed(
                record.relative_path.clone(),
            ),
        );
    }
}

        changes
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository_file_inventory::{
        RepositoryEntryKind,
        RepositoryFileInventory,
        RepositoryFileRecord,
    };
    use std::time::SystemTime;
    use uuid::Uuid;

    #[test]
    fn detects_added_repository_file() {
        let repository_id = Uuid::new_v4();

        let previous = RepositoryFileInventory::new();

        let mut current = RepositoryFileInventory::new();

        assert!(current.register(
            RepositoryFileRecord::new(
                repository_id,
                "src/lib.rs",
                "/tmp/src/lib.rs",
                RepositoryEntryKind::File,
                1024,
                Some(SystemTime::now()),
            )
        ));

        let tracker = RepositoryChangeTracker::new();

        let changes = tracker.detect_changes(
            &previous,
            &current,
        );

        assert_eq!(changes.len(), 1);
        assert_eq!(
            changes[0].kind,
            RepositoryChangeKind::Added
        );
        assert_eq!(
            changes[0].current_path.as_deref(),
            Some(std::path::Path::new("src/lib.rs"))
        );
    }


#[test]
fn detects_removed_repository_file() {
    let repository_id = Uuid::new_v4();

    let mut previous = RepositoryFileInventory::new();

    assert!(previous.register(
        RepositoryFileRecord::new(
            repository_id,
            "src/lib.rs",
            "/tmp/src/lib.rs",
            RepositoryEntryKind::File,
            1024,
            Some(SystemTime::now()),
        )
    ));

    let current = RepositoryFileInventory::new();

    let tracker = RepositoryChangeTracker::new();

    let changes = tracker.detect_changes(
        &previous,
        &current,
    );

    assert_eq!(changes.len(), 1);
    assert_eq!(
        changes[0].kind,
        RepositoryChangeKind::Removed
    );
    assert_eq!(
        changes[0].previous_path.as_deref(),
        Some(std::path::Path::new("src/lib.rs"))
    );
}
}
