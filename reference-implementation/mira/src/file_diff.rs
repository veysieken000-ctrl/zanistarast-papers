use std::path::PathBuf;
use std::time::SystemTime;

use crate::FileVersionPair;

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

    /// İki UTF-8 metin dosyasını satır satır karşılaştırarak
/// diff raporu oluşturur.
///
/// Aynı sıradaki eşit satırlar `Unchanged`, farklı satırlar
/// `Modified`, yalnızca revize dosyada bulunan satırlar
/// `Added`, yalnızca orijinal dosyada bulunan satırlar
/// `Removed` olarak kaydedilir.
pub fn from_text_files(
    original_path: impl Into<PathBuf>,
    revised_path: impl Into<PathBuf>,
    generated_at: SystemTime,
) -> std::io::Result<Self> {
    let original_path = original_path.into();
    let revised_path = revised_path.into();

    let original_text =
        std::fs::read_to_string(&original_path)?;

    let revised_text =
        std::fs::read_to_string(&revised_path)?;

    let original_lines: Vec<&str> =
        original_text.lines().collect();

    let revised_lines: Vec<&str> =
        revised_text.lines().collect();

    let maximum_line_count =
        original_lines.len().max(revised_lines.len());

    let mut changes =
        Vec::with_capacity(maximum_line_count);

    for index in 0..maximum_line_count {
        let original_line = original_lines.get(index);
        let revised_line = revised_lines.get(index);

        let change = match (original_line, revised_line) {
            (Some(original), Some(revised))
                if original == revised =>
            {
                FileLineChange::new(
                    FileLineChangeKind::Unchanged,
                    Some(index + 1),
                    Some(index + 1),
                    Some((*original).to_string()),
                    Some((*revised).to_string()),
                )
            }

            (Some(original), Some(revised)) => {
                FileLineChange::new(
                    FileLineChangeKind::Modified,
                    Some(index + 1),
                    Some(index + 1),
                    Some((*original).to_string()),
                    Some((*revised).to_string()),
                )
            }

            (None, Some(revised)) => {
                FileLineChange::new(
                    FileLineChangeKind::Added,
                    None,
                    Some(index + 1),
                    None,
                    Some((*revised).to_string()),
                )
            }

            (Some(original), None) => {
                FileLineChange::new(
                    FileLineChangeKind::Removed,
                    Some(index + 1),
                    None,
                    Some((*original).to_string()),
                    None,
                )
            }

            (None, None) => continue,
        };

        changes.push(change);
    }

    Ok(Self::new(
        original_path,
        revised_path,
        changes,
        generated_at,
    ))
}

/// Doğrulanmış orijinal–revize sürüm çiftine bağlı
/// metin diff raporu oluşturur.
///
/// Sürüm çifti geçerli değilse veya kayıtlı dosya yolları
/// okunamıyorsa rapor oluşturulmaz.
pub fn from_version_pair(
    pair: &FileVersionPair,
    generated_at: SystemTime,
) -> std::io::Result<Self> {
    if !pair.is_matched() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "file version pair must be matched",
        ));
    }

    Self::from_text_files(
        pair.original.path.clone(),
        pair.revised.path.clone(),
        generated_at,
    )
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

    /// Diff raporundaki bütün satır değişiklik
/// kayıtlarının toplam sayısını döndürür.
pub fn total_change_count(&self) -> usize {
    self.changes.len()
}

/// Değişiklik türlerine göre hesaplanan toplam
/// satır kaydı sayısını döndürür.
pub fn classified_change_count(&self) -> usize {
    self.added_count()
        + self.removed_count()
        + self.modified_count()
        + self.unchanged_count()
}

/// Tek bir satır değişiklik kaydının türü ile
/// satır numarası ve içerik alanlarının uyumlu
/// olup olmadığını bildirir.
fn is_change_consistent(
    change: &FileLineChange,
) -> bool {
    match change.kind {
        FileLineChangeKind::Added => {
            change.original_line_number.is_none()
                && change.revised_line_number.is_some()
                && change.original_content.is_none()
                && change.revised_content.is_some()
        }

        FileLineChangeKind::Removed => {
            change.original_line_number.is_some()
                && change.revised_line_number.is_none()
                && change.original_content.is_some()
                && change.revised_content.is_none()
        }

        FileLineChangeKind::Modified
        | FileLineChangeKind::Unchanged => {
            change.original_line_number.is_some()
                && change.revised_line_number.is_some()
                && change.original_content.is_some()
                && change.revised_content.is_some()
        }
    }
}

    /// Diff raporunun yollarının, sınıflandırma toplamlarının
/// ve bütün satır değişiklik kayıtlarının tutarlı
/// olup olmadığını bildirir.
pub fn is_consistent(&self) -> bool {
    self.is_complete()
        && self.total_change_count()
            == self.classified_change_count()
        && self
            .changes
            .iter()
            .all(Self::is_change_consistent)
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
#[test]
fn creates_diff_report_from_text_files() {
    use std::fs;

    let original_path = std::env::temp_dir().join(
        format!(
            "mira-diff-original-{}.txt",
            std::process::id(),
        ),
    );

    let revised_path = std::env::temp_dir().join(
        format!(
            "mira-diff-revised-{}.txt",
            std::process::id(),
        ),
    );

    fs::write(
        &original_path,
        "Hebun\nOriginal line\nRemoved line\n",
    )
    .expect("original text file should be created");

    fs::write(
        &revised_path,
        "Hebun\nRevised line\nAdded line\nExtra line\n",
    )
    .expect("revised text file should be created");

    let report = FileDiffReport::from_text_files(
        &original_path,
        &revised_path,
        SystemTime::now(),
    )
    .expect("text file diff report should be created");

    assert!(report.is_complete());
    assert!(report.has_changes());

    assert_eq!(report.unchanged_count(), 1);
    assert_eq!(report.modified_count(), 2);
    assert_eq!(report.added_count(), 1);
    assert_eq!(report.removed_count(), 0);

    assert_eq!(
        report.changes[0].kind,
        FileLineChangeKind::Unchanged,
    );

    assert_eq!(
        report.changes[1].kind,
        FileLineChangeKind::Modified,
    );

    assert_eq!(
        report.changes[3].kind,
        FileLineChangeKind::Added,
    );

    fs::remove_file(&original_path)
        .expect("original text file should be removed");

    fs::remove_file(&revised_path)
        .expect("revised text file should be removed");
}

}

#[test]
fn detects_removed_trailing_lines_from_text_files() {
    use std::fs;

    let original_path = std::env::temp_dir().join(
        format!(
            "mira-diff-removed-original-{}.txt",
            std::process::id(),
        ),
    );

    let revised_path = std::env::temp_dir().join(
        format!(
            "mira-diff-removed-revised-{}.txt",
            std::process::id(),
        ),
    );

    fs::write(
        &original_path,
        "Hebun\nSecond line\nRemoved line\n",
    )
    .expect("original text file should be created");

    fs::write(
        &revised_path,
        "Hebun\nSecond line\n",
    )
    .expect("revised text file should be created");

    let report = FileDiffReport::from_text_files(
        &original_path,
        &revised_path,
        SystemTime::now(),
    )
    .expect("removed line diff should be created");

    assert_eq!(report.unchanged_count(), 2);
    assert_eq!(report.removed_count(), 1);
    assert_eq!(report.added_count(), 0);
    assert_eq!(report.modified_count(), 0);

    let removed = report
        .changes
        .iter()
        .find(|change| change.is_removed())
        .expect("removed line should be recorded");

    assert_eq!(
        removed.original_line_number,
        Some(3),
    );

    assert_eq!(
        removed.original_content.as_deref(),
        Some("Removed line"),
    );

    fs::remove_file(&original_path)
        .expect("original text file should be removed");

    fs::remove_file(&revised_path)
        .expect("revised text file should be removed");
}

#[test]
fn creates_diff_report_from_matched_version_pair() {
    use std::fs;

    let original_path = std::env::temp_dir().join(
        format!(
            "mira-pair-diff-original-{}.txt",
            std::process::id(),
        ),
    );

    let revised_path = std::env::temp_dir().join(
        format!(
            "mira-pair-diff-revised-{}.txt",
            std::process::id(),
        ),
    );

    fs::write(
        &original_path,
        "Hebun\nOriginal line\n",
    )
    .expect("original file should be created");

    fs::write(
        &revised_path,
        "Hebun\nRevised line\nNew line\n",
    )
    .expect("revised file should be created");

    let pair = FileVersionPair::from_files_sha256(
        &original_path,
        &revised_path,
        SystemTime::now(),
    )
    .expect("version pair should be created");

    assert!(pair.is_matched());

    let report = FileDiffReport::from_version_pair(
        &pair,
        SystemTime::now(),
    )
    .expect(
        "diff report should be created from matched pair",
    );

    assert!(report.is_complete());
    assert!(report.has_changes());
    assert_eq!(report.unchanged_count(), 1);
    assert_eq!(report.modified_count(), 1);
    assert_eq!(report.added_count(), 1);

    assert_eq!(
        report.original_path,
        original_path,
    );

    assert_eq!(
        report.revised_path,
        revised_path,
    );

    fs::remove_file(&original_path)
        .expect("original file should be removed");

    fs::remove_file(&revised_path)
        .expect("revised file should be removed");
}

#[test]
fn rejects_diff_for_invalid_version_pair() {
    use crate::{
        FileHashRecord,
        FileHashRole,
        FileVersionPair,
    };

    let invalid_original = FileHashRecord::new(
        "articles/hebun.md",
        FileHashRole::Revised,
        "SHA-256",
        "original-digest",
        SystemTime::now(),
    );

    let revised = FileHashRecord::new(
        "articles/hebun-v2.md",
        FileHashRole::Revised,
        "SHA-256",
        "revised-digest",
        SystemTime::now(),
    );

    let pair = FileVersionPair::new(
        invalid_original,
        revised,
        SystemTime::now(),
    );

    assert!(!pair.is_matched());

    let result = FileDiffReport::from_version_pair(
        &pair,
        SystemTime::now(),
    );

    assert!(result.is_err());

    assert_eq!(
        result
            .expect_err(
                "invalid pair should not create a diff",
            )
            .kind(),
        std::io::ErrorKind::InvalidInput,
    );
}

#[test]
fn complete_diff_report_is_consistent() {
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
            Some("Old line".to_string()),
            Some("New line".to_string()),
        ),
        FileLineChange::new(
            FileLineChangeKind::Added,
            None,
            Some(3),
            None,
            Some("Added line".to_string()),
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

    assert_eq!(report.total_change_count(), 4);

    assert_eq!(
        report.classified_change_count(),
        4,
    );

    assert!(report.is_consistent());
}




#[test]
fn inconsistent_added_line_record_is_rejected() {
    let changes = vec![
        FileLineChange::new(
            FileLineChangeKind::Added,
            Some(1),
            Some(1),
            Some("Invalid original content".to_string()),
            Some("Added line".to_string()),
        ),
    ];

    let report = FileDiffReport::new(
        "articles/hebun.md",
        "articles/hebun-v2.md",
        changes,
        SystemTime::now(),
    );

    assert!(!report.is_consistent());
}

#[test]
fn inconsistent_removed_line_record_is_rejected() {
    let changes = vec![
        FileLineChange::new(
            FileLineChangeKind::Removed,
            Some(1),
            Some(1),
            Some("Removed line".to_string()),
            Some("Invalid revised content".to_string()),
        ),
    ];

    let report = FileDiffReport::new(
        "articles/hebun.md",
        "articles/hebun-v2.md",
        changes,
        SystemTime::now(),
    );

    assert!(!report.is_consistent());
}


