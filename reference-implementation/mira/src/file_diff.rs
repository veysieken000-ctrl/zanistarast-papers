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

/// Diff raporunun sürüm çiftiyle güvenlik
/// doğrulama sonucunu belirtir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileDiffSecurityStatus {
    /// Diff raporu, dosya yolları ve hash sonuçları
    /// sürüm çiftiyle tamamen uyumludur.
    Verified,

    /// Sürüm çifti geçerli değildir.
    InvalidVersionPair,

    /// Diff raporundaki dosya yolları sürüm
    /// çiftindeki yollarla eşleşmemektedir.
    PathMismatch,

    /// Diff raporundaki satır değişiklik kayıtları
    /// kendi içinde tutarlı değildir.
    InconsistentDiff,

    /// Hash karşılaştırması ile diff sonucu
    /// birbiriyle çelişmektedir.
    HashDiffMismatch,
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

    /// İki satır listesinin en uzun ortak alt dizisini
/// hesaplamak için dinamik programlama tablosu oluşturur.
fn build_lcs_table(
    original_lines: &[&str],
    revised_lines: &[&str],
) -> Vec<Vec<usize>> {
    let original_count = original_lines.len();
    let revised_count = revised_lines.len();

    let mut table = vec![
        vec![0; revised_count + 1];
        original_count + 1
    ];

    for original_index in (0..original_count).rev() {
        for revised_index in (0..revised_count).rev() {
            table[original_index][revised_index] =
                if original_lines[original_index]
                    == revised_lines[revised_index]
                {
                    table[original_index + 1]
                        [revised_index + 1]
                        + 1
                } else {
                    table[original_index + 1]
                        [revised_index]
                        .max(
                            table[original_index]
                                [revised_index + 1],
                        )
                };
        }
    }

    table
}

/// LCS tablosunu kullanarak eklenen, kaldırılan,
/// değiştirilen ve değişmeden kalan satırları çıkarır.
fn classify_lines_with_lcs(
    original_lines: &[&str],
    revised_lines: &[&str],
) -> Vec<FileLineChange> {
    let table = Self::build_lcs_table(
        original_lines,
        revised_lines,
    );

    let mut changes = Vec::new();
    let mut original_index = 0;
    let mut revised_index = 0;

    while original_index < original_lines.len()
        && revised_index < revised_lines.len()
    {
        if original_lines[original_index]
            == revised_lines[revised_index]
        {
            changes.push(FileLineChange::new(
                FileLineChangeKind::Unchanged,
                Some(original_index + 1),
                Some(revised_index + 1),
                Some(
                    original_lines[original_index]
                        .to_string(),
                ),
                Some(
                    revised_lines[revised_index]
                        .to_string(),
                ),
            ));

            original_index += 1;
            revised_index += 1;
        } else {
            let removing_preserves_more =
                table[original_index + 1]
                    [revised_index]
                    > table[original_index]
                        [revised_index + 1];

            let adding_preserves_more =
                table[original_index + 1]
                    [revised_index]
                    < table[original_index]
                        [revised_index + 1];

            if removing_preserves_more {
                changes.push(FileLineChange::new(
                    FileLineChangeKind::Removed,
                    Some(original_index + 1),
                    None,
                    Some(
                        original_lines[original_index]
                            .to_string(),
                    ),
                    None,
                ));

                original_index += 1;
            } else if adding_preserves_more {
                changes.push(FileLineChange::new(
                    FileLineChangeKind::Added,
                    None,
                    Some(revised_index + 1),
                    None,
                    Some(
                        revised_lines[revised_index]
                            .to_string(),
                    ),
                ));

                revised_index += 1;
            } else {
                changes.push(FileLineChange::new(
                    FileLineChangeKind::Modified,
                    Some(original_index + 1),
                    Some(revised_index + 1),
                    Some(
                        original_lines[original_index]
                            .to_string(),
                    ),
                    Some(
                        revised_lines[revised_index]
                            .to_string(),
                    ),
                ));

                original_index += 1;
                revised_index += 1;
            }
        }
    }

    while original_index < original_lines.len() {
        changes.push(FileLineChange::new(
            FileLineChangeKind::Removed,
            Some(original_index + 1),
            None,
            Some(
                original_lines[original_index]
                    .to_string(),
            ),
            None,
        ));

        original_index += 1;
    }

    while revised_index < revised_lines.len() {
        changes.push(FileLineChange::new(
            FileLineChangeKind::Added,
            None,
            Some(revised_index + 1),
            None,
            Some(
                revised_lines[revised_index]
                    .to_string(),
            ),
        ));

        revised_index += 1;
    }

    changes
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

/// İki UTF-8 metin dosyasını LCS tabanlı olarak
/// karşılaştırıp satır kaymalarını koruyan diff raporu üretir.
pub fn from_text_files_lcs(
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

    let changes = Self::classify_lines_with_lcs(
        &original_lines,
        &revised_lines,
    );

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

    /// Doğrulanmış orijinal–revize sürüm çiftinden
/// LCS tabanlı gelişmiş metin diff raporu oluşturur.
///
/// Geçersiz sürüm çiftleri kabul edilmez.
pub fn from_version_pair_lcs(
    pair: &FileVersionPair,
    generated_at: SystemTime,
) -> std::io::Result<Self> {
    if !pair.is_matched() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "file version pair must be matched",
        ));
    }

    Self::from_text_files_lcs(
        pair.original.path.clone(),
        pair.revised.path.clone(),
        generated_at,
    )
}

    /// Diff raporunun doğrulanmış sürüm çifti, dosya yolları
/// ve hash karşılaştırma sonucuyla tutarlı olup olmadığını
/// bildirir.
pub fn matches_version_pair(
    &self,
    pair: &FileVersionPair,
) -> bool {
    if !pair.is_matched()
        || !self.is_consistent()
        || self.original_path != pair.original.path
        || self.revised_path != pair.revised.path
    {
        return false;
    }

    if pair.content_identical() {
        return !self.has_changes();
    }

    if pair.content_changed() {
        return self.has_changes();
    }

    false
}

    /// Diff raporunu sürüm çifti, dosya yolları,
/// hash karşılaştırması ve satır kayıtları üzerinden
/// ayrıntılı olarak doğrular.
pub fn security_status(
    &self,
    pair: &FileVersionPair,
) -> FileDiffSecurityStatus {
    if !pair.is_matched() {
        return FileDiffSecurityStatus::InvalidVersionPair;
    }

    if self.original_path != pair.original.path
        || self.revised_path != pair.revised.path
    {
        return FileDiffSecurityStatus::PathMismatch;
    }

    if !self.is_consistent() {
        return FileDiffSecurityStatus::InconsistentDiff;
    }

    let hash_and_diff_agree =
        (pair.content_identical() && !self.has_changes())
            || (pair.content_changed() && self.has_changes());

    if !hash_and_diff_agree {
        return FileDiffSecurityStatus::HashDiffMismatch;
    }

    FileDiffSecurityStatus::Verified
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

#[test]
fn lcs_diff_detects_inserted_line_without_shifting_following_lines() {
    use std::fs;

    let original_path = std::env::temp_dir().join(
        format!(
            "mira-lcs-insert-original-{}.txt",
            std::process::id(),
        ),
    );

    let revised_path = std::env::temp_dir().join(
        format!(
            "mira-lcs-insert-revised-{}.txt",
            std::process::id(),
        ),
    );

    fs::write(
        &original_path,
        "Hebun\nRabun\nRasterast\n",
    )
    .expect("original file should be created");

    fs::write(
        &revised_path,
        "Hebun\nNew line\nRabun\nRasterast\n",
    )
    .expect("revised file should be created");

    let report = FileDiffReport::from_text_files_lcs(
        &original_path,
        &revised_path,
        SystemTime::now(),
    )
    .expect("LCS diff should be generated");

    assert!(report.is_complete());
    assert!(report.is_consistent());

    assert_eq!(report.added_count(), 1);
    assert_eq!(report.removed_count(), 0);
    assert_eq!(report.modified_count(), 0);
    assert_eq!(report.unchanged_count(), 3);

    let added = report
        .changes
        .iter()
        .find(|change| change.is_added())
        .expect("inserted line should be detected");

    assert_eq!(
        added.revised_line_number,
        Some(2),
    );

    assert_eq!(
        added.revised_content.as_deref(),
        Some("New line"),
    );

    fs::remove_file(&original_path)
        .expect("original file should be removed");

    fs::remove_file(&revised_path)
        .expect("revised file should be removed");
}

#[test]
fn lcs_diff_detects_removed_line_without_shifting_following_lines() {
    use std::fs;

    let original_path = std::env::temp_dir().join(
        format!(
            "mira-lcs-remove-original-{}.txt",
            std::process::id(),
        ),
    );

    let revised_path = std::env::temp_dir().join(
        format!(
            "mira-lcs-remove-revised-{}.txt",
            std::process::id(),
        ),
    );

    fs::write(
        &original_path,
        "Hebun\nRemoved line\nRabun\nRasterast\n",
    )
    .expect("original file should be created");

    fs::write(
        &revised_path,
        "Hebun\nRabun\nRasterast\n",
    )
    .expect("revised file should be created");

    let report = FileDiffReport::from_text_files_lcs(
        &original_path,
        &revised_path,
        SystemTime::now(),
    )
    .expect("LCS diff should be generated");

    assert!(report.is_consistent());

    assert_eq!(report.added_count(), 0);
    assert_eq!(report.removed_count(), 1);
    assert_eq!(report.modified_count(), 0);
    assert_eq!(report.unchanged_count(), 3);

    let removed = report
        .changes
        .iter()
        .find(|change| change.is_removed())
        .expect("removed line should be detected");

    assert_eq!(
        removed.original_line_number,
        Some(2),
    );

    assert_eq!(
        removed.original_content.as_deref(),
        Some("Removed line"),
    );

    fs::remove_file(&original_path)
        .expect("original file should be removed");

    fs::remove_file(&revised_path)
        .expect("revised file should be removed");
}

#[test]
fn creates_lcs_diff_from_matched_version_pair() {
    use std::fs;

    let original_path = std::env::temp_dir().join(
        format!(
            "mira-pair-lcs-original-{}.txt",
            std::process::id(),
        ),
    );

    let revised_path = std::env::temp_dir().join(
        format!(
            "mira-pair-lcs-revised-{}.txt",
            std::process::id(),
        ),
    );

    fs::write(
        &original_path,
        "Hebun\nRabun\nRasterast\n",
    )
    .expect("original file should be created");

    fs::write(
        &revised_path,
        "Hebun\nNew line\nRabun\nRasterast\n",
    )
    .expect("revised file should be created");

    let pair = FileVersionPair::from_files_sha256(
        &original_path,
        &revised_path,
        SystemTime::now(),
    )
    .expect("version pair should be created");

    assert!(pair.is_matched());

    let report = FileDiffReport::from_version_pair_lcs(
        &pair,
        SystemTime::now(),
    )
    .expect(
        "LCS diff should be created from matched pair",
    );

    assert!(report.is_complete());
    assert!(report.is_consistent());
    assert!(report.has_changes());

    assert_eq!(report.added_count(), 1);
    assert_eq!(report.removed_count(), 0);
    assert_eq!(report.modified_count(), 0);
    assert_eq!(report.unchanged_count(), 3);

    let added = report
        .changes
        .iter()
        .find(|change| change.is_added())
        .expect("added line should be recorded");

    assert_eq!(
        added.revised_line_number,
        Some(2),
    );

    assert_eq!(
        added.revised_content.as_deref(),
        Some("New line"),
    );

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
fn rejects_lcs_diff_for_invalid_version_pair() {
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

    let result = FileDiffReport::from_version_pair_lcs(
        &pair,
        SystemTime::now(),
    );

    assert!(result.is_err());

    assert_eq!(
        result
            .expect_err(
                "invalid pair should not create an LCS diff",
            )
            .kind(),
        std::io::ErrorKind::InvalidInput,
    );
}

#[test]
fn changed_diff_matches_changed_version_pair() {
    use std::fs;

    let original_path = std::env::temp_dir().join(
        format!(
            "mira-cross-check-original-{}.txt",
            std::process::id(),
        ),
    );

    let revised_path = std::env::temp_dir().join(
        format!(
            "mira-cross-check-revised-{}.txt",
            std::process::id(),
        ),
    );

    fs::write(
        &original_path,
        "Hebun\nRasterast\n",
    )
    .expect("original file should be created");

    fs::write(
        &revised_path,
        "Hebun\nNew line\nRasterast\n",
    )
    .expect("revised file should be created");

    let pair = FileVersionPair::from_files_sha256(
        &original_path,
        &revised_path,
        SystemTime::now(),
    )
    .expect("version pair should be created");

    let report = FileDiffReport::from_version_pair_lcs(
        &pair,
        SystemTime::now(),
    )
    .expect("diff report should be created");

    assert!(pair.content_changed());
    assert!(report.has_changes());
    assert!(report.matches_version_pair(&pair));

    fs::remove_file(&original_path)
        .expect("original file should be removed");

    fs::remove_file(&revised_path)
        .expect("revised file should be removed");
}

#[test]
fn unchanged_diff_matches_identical_version_pair() {
    use std::fs;

    let original_path = std::env::temp_dir().join(
        format!(
            "mira-cross-check-identical-original-{}.txt",
            std::process::id(),
        ),
    );

    let revised_path = std::env::temp_dir().join(
        format!(
            "mira-cross-check-identical-revised-{}.txt",
            std::process::id(),
        ),
    );

    fs::write(&original_path, "Hebun\nRasterast\n")
        .expect("original file should be created");

    fs::write(&revised_path, "Hebun\nRasterast\n")
        .expect("revised file should be created");

    let pair = FileVersionPair::from_files_sha256(
        &original_path,
        &revised_path,
        SystemTime::now(),
    )
    .expect("identical version pair should be created");

    let report = FileDiffReport::from_version_pair_lcs(
        &pair,
        SystemTime::now(),
    )
    .expect("identical diff report should be created");

    assert!(pair.content_identical());
    assert!(!report.has_changes());
    assert!(report.matches_version_pair(&pair));

    fs::remove_file(&original_path)
        .expect("original file should be removed");

    fs::remove_file(&revised_path)
        .expect("revised file should be removed");
}

#[test]
fn diff_report_rejects_unrelated_version_pair() {
    use std::fs;

    let original_path = std::env::temp_dir().join(
        format!(
            "mira-cross-check-unrelated-original-{}.txt",
            std::process::id(),
        ),
    );

    let revised_path = std::env::temp_dir().join(
        format!(
            "mira-cross-check-unrelated-revised-{}.txt",
            std::process::id(),
        ),
    );

    let other_original_path = std::env::temp_dir().join(
        format!(
            "mira-cross-check-other-original-{}.txt",
            std::process::id(),
        ),
    );

    let other_revised_path = std::env::temp_dir().join(
        format!(
            "mira-cross-check-other-revised-{}.txt",
            std::process::id(),
        ),
    );

    fs::write(&original_path, "Hebun\n")
        .expect("original file should be created");

    fs::write(&revised_path, "Hebun revised\n")
        .expect("revised file should be created");

    fs::write(&other_original_path, "Rasterast\n")
        .expect("other original file should be created");

    fs::write(&other_revised_path, "Rasterast revised\n")
        .expect("other revised file should be created");

    let pair = FileVersionPair::from_files_sha256(
        &original_path,
        &revised_path,
        SystemTime::now(),
    )
    .expect("version pair should be created");

    let unrelated_pair = FileVersionPair::from_files_sha256(
        &other_original_path,
        &other_revised_path,
        SystemTime::now(),
    )
    .expect("unrelated version pair should be created");

    let report = FileDiffReport::from_version_pair_lcs(
        &pair,
        SystemTime::now(),
    )
    .expect("diff report should be created");

    assert!(!report.matches_version_pair(
        &unrelated_pair,
    ));

    fs::remove_file(&original_path)
        .expect("original file should be removed");

    fs::remove_file(&revised_path)
        .expect("revised file should be removed");

    fs::remove_file(&other_original_path)
        .expect("other original file should be removed");

    fs::remove_file(&other_revised_path)
        .expect("other revised file should be removed");
}

#[test]
fn security_status_verifies_matching_diff_and_version_pair() {
    use std::fs;

    let original_path = std::env::temp_dir().join(
        format!(
            "mira-security-diff-original-{}.txt",
            std::process::id(),
        ),
    );

    let revised_path = std::env::temp_dir().join(
        format!(
            "mira-security-diff-revised-{}.txt",
            std::process::id(),
        ),
    );

    fs::write(
        &original_path,
        "Hebun\nRasterast\n",
    )
    .expect("original file should be created");

    fs::write(
        &revised_path,
        "Hebun\nNew line\nRasterast\n",
    )
    .expect("revised file should be created");

    let pair = FileVersionPair::from_files_sha256(
        &original_path,
        &revised_path,
        SystemTime::now(),
    )
    .expect("version pair should be created");

    let report = FileDiffReport::from_version_pair_lcs(
        &pair,
        SystemTime::now(),
    )
    .expect("diff report should be created");

    assert_eq!(
        report.security_status(&pair),
        FileDiffSecurityStatus::Verified,
    );

    fs::remove_file(&original_path)
        .expect("original file should be removed");

    fs::remove_file(&revised_path)
        .expect("revised file should be removed");
}

#[test]
fn security_status_reports_path_mismatch() {
    let original = crate::FileHashRecord::new(
        "articles/hebun.md",
        crate::FileHashRole::Original,
        "SHA-256",
        "original-digest",
        SystemTime::now(),
    );

    let revised = crate::FileHashRecord::new(
        "articles/hebun-v2.md",
        crate::FileHashRole::Revised,
        "SHA-256",
        "revised-digest",
        SystemTime::now(),
    );

    let pair = FileVersionPair::new(
        original,
        revised,
        SystemTime::now(),
    );

    let report = FileDiffReport::new(
        "articles/other.md",
        "articles/other-v2.md",
        vec![
            FileLineChange::new(
                FileLineChangeKind::Modified,
                Some(1),
                Some(1),
                Some("Old".to_string()),
                Some("New".to_string()),
            ),
        ],
        SystemTime::now(),
    );

    assert_eq!(
        report.security_status(&pair),
        FileDiffSecurityStatus::PathMismatch,
    );
}

#[test]
fn security_status_reports_hash_diff_mismatch() {
    let original = crate::FileHashRecord::new(
        "articles/hebun.md",
        crate::FileHashRole::Original,
        "SHA-256",
        "same-digest",
        SystemTime::now(),
    );

    let revised = crate::FileHashRecord::new(
        "articles/hebun-v2.md",
        crate::FileHashRole::Revised,
        "SHA-256",
        "same-digest",
        SystemTime::now(),
    );

    let pair = FileVersionPair::new(
        original,
        revised,
        SystemTime::now(),
    );

    let report = FileDiffReport::new(
        "articles/hebun.md",
        "articles/hebun-v2.md",
        vec![
            FileLineChange::new(
                FileLineChangeKind::Modified,
                Some(1),
                Some(1),
                Some("Old".to_string()),
                Some("New".to_string()),
            ),
        ],
        SystemTime::now(),
    );

    assert!(pair.content_identical());
    assert!(report.has_changes());

    assert_eq!(
        report.security_status(&pair),
        FileDiffSecurityStatus::HashDiffMismatch,
    );
}





