use std::path::PathBuf;
use std::time::SystemTime;

/// Bir metin satırında belirlenen değişiklik türünü gösterir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileLineChangeKind {
    /// Revize dosyaya yeni bir satır eklenmiştir.
    Added,

    /// Orijinal dosyadaki bir satır kaldırılmıştır.
    Removed,

    /// Orijinal satır değiştirilerek yeni içerik üretilmiştir.
    Modified,

    /// Satır iki sürümde de aynı kalmıştır.
    Unchanged,
}

/// Bir dosya sürümündeki tek satır değişikliğini temsil eder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileLineChange {
    pub kind: FileLineChangeKind,
    pub original_line_number: Option<usize>,
    pub revised_line_number: Option<usize>,
    pub original_content: Option<String>,
    pub revised_content: Option<String>,
}

impl FileLineChange {
    /// Yeni bir satır değişiklik kaydı oluşturur.
    pub fn new(
        kind: FileLineChangeKind,
        original_line_number: Option<usize>,
        revised_line_number: Option<usize>,
        original_content: Option<String>,
        revised_content: Option<String>,
    ) -> Self {
        Self {
            kind,
            original_line_number,
            revised_line_number,
            original_content,
            revised_content,
        }
    }

    /// Kaydın eklenen satırı temsil edip etmediğini bildirir.
    pub fn is_added(&self) -> bool {
        self.kind == FileLineChangeKind::Added
    }

    /// Kaydın silinen satırı temsil edip etmediğini bildirir.
    pub fn is_removed(&self) -> bool {
        self.kind == FileLineChangeKind::Removed
    }

    /// Kaydın değiştirilmiş satırı temsil edip etmediğini bildirir.
    pub fn is_modified(&self) -> bool {
        self.kind == FileLineChangeKind::Modified
    }

    /// Kaydın değişmeden kalan satırı temsil edip etmediğini bildirir.
    pub fn is_unchanged(&self) -> bool {
        self.kind == FileLineChangeKind::Unchanged
    }
}

/// Orijinal ve revize dosya arasındaki satır değişikliklerinin
/// birleşik raporudur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileDiffReport {
    pub original_path: PathBuf,
    pub revised_path: PathBuf,
    pub changes: Vec<FileLineChange>,
    pub generated_at: SystemTime,
}

impl FileDiffReport {
    /// Yeni bir dosya diff raporu oluşturur.
    pub fn new(
        original_path: impl Into<PathBuf>,
        revised_path: impl Into<PathBuf>,
        changes: Vec<FileLineChange>,
        generated_at: SystemTime,
    ) -> Self {
        Self {
            original_path: original_path.into(),
            revised_path: revised_path.into(),
            changes,
            generated_at,
        }
    }

    /// Eklenen satırların sayısını döndürür.
    pub fn added_count(&self) -> usize {
        self.changes
            .iter()
            .filter(|change| change.is_added())
            .count()
    }

    /// Silinen satırların sayısını döndürür.
    pub fn removed_count(&self) -> usize {
        self.changes
            .iter()
            .filter(|change| change.is_removed())
            .count()
    }

    /// Değiştirilen satırların sayısını döndürür.
    pub fn modified_count(&self) -> usize {
        self.changes
            .iter()
            .filter(|change| change.is_modified())
            .count()
    }

    /// Değişmeden kalan satırların sayısını döndürür.
    pub fn unchanged_count(&self) -> usize {
        self.changes
            .iter()
            .filter(|change| change.is_unchanged())
            .count()
    }

    /// Raporda herhangi bir içerik değişikliği bulunup
    /// bulunmadığını bildirir.
    pub fn has_changes(&self) -> bool {
        self.added_count() > 0
            || self.removed_count() > 0
            || self.modified_count() > 0
    }

    /// Raporun temel alanlarının eksiksiz olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.original_path.as_os_str().is_empty()
            && !self.revised_path.as_os_str().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_complete_file_diff_report() {
        let changes = vec![
            FileLineChange::new(
                FileLineChangeKind::Unchanged,
                Some(1),
                Some(1),
                Some("Hebun".to_string()),
                Some("Hebun".to_string()),
            ),
            FileLineChange::new(
                FileLineChangeKind::Modified,
                Some(2),
                Some(2),
                Some("Original line".to_string()),
                Some("Revised line".to_string()),
            ),
            FileLineChange::new(
                FileLineChangeKind::Added,
                None,
                Some(3),
                None,
                Some("New line".to_string()),
            ),
            FileLineChange::new(
                FileLineChangeKind::Removed,
                Some(4),
                None,
                Some("Removed line".to_string()),
                None,
            ),
        ];

        let report = FileDiffReport::new(
            "articles/hebun.md",
            "articles/hebun-v2.md",
            changes,
            SystemTime::now(),
        );

        assert!(report.is_complete());
        assert!(report.has_changes());

        assert_eq!(report.added_count(), 1);
        assert_eq!(report.removed_count(), 1);
        assert_eq!(report.modified_count(), 1);
        assert_eq!(report.unchanged_count(), 1);
    }

    #[test]
    fn unchanged_report_has_no_content_changes() {
        let changes = vec![
            FileLineChange::new(
                FileLineChangeKind::Unchanged,
                Some(1),
                Some(1),
                Some("Rasterast".to_string()),
                Some("Rasterast".to_string()),
            ),
        ];

        let report = FileDiffReport::new(
            "articles/rasterast.md",
            "articles/rasterast-copy.md",
            changes,
            SystemTime::now(),
        );

        assert!(!report.has_changes());
        assert_eq!(report.unchanged_count(), 1);
    }

    #[test]
    fn incomplete_diff_report_is_rejected() {
        let report = FileDiffReport::new(
            "",
            "",
            Vec::new(),
            SystemTime::now(),
        );

        assert!(!report.is_complete());
    }

    #[test]
    fn line_change_reports_its_kind() {
        let added = FileLineChange::new(
            FileLineChangeKind::Added,
            None,
            Some(1),
            None,
            Some("New line".to_string()),
        );

        let removed = FileLineChange::new(
            FileLineChangeKind::Removed,
            Some(1),
            None,
            Some("Old line".to_string()),
            None,
        );

        let modified = FileLineChange::new(
            FileLineChangeKind::Modified,
            Some(2),
            Some(2),
            Some("Old".to_string()),
            Some("New".to_string()),
        );

        assert!(added.is_added());
        assert!(removed.is_removed());
        assert!(modified.is_modified());
    }
}

