use std::path::PathBuf;

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

/// İki repository dosya envanteri arasındaki değişiklikleri belirler.
#[derive(Debug, Default)]
pub struct RepositoryChangeTracker;

impl RepositoryChangeTracker {
    pub fn new() -> Self {
        Self
    }
}
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
        }
    }

    changes
}
