use std::path::Path;
use std::time::SystemTime;

use crate::{
    FileHashComparison,
    FileHashRecord,
    FileHashRole,
};

/// Orijinal ve revize dosya kayıtlarının eşleştirme
/// sonucunu belirtir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileVersionPairStatus {
    /// Orijinal ve revize kayıt geçerli bir sürüm çiftidir.
    Matched,

    /// İlk kayıt orijinal dosyayı temsil etmiyor.
    InvalidOriginalRole,

    /// İkinci kayıt revize dosyayı temsil etmiyor.
    InvalidRevisedRole,

    /// Hash kayıtlarından biri eksik veya geçersizdir.
    InvalidHashRecord,

    /// İki kayıt farklı hash algoritmaları kullanmaktadır.
    AlgorithmMismatch,
}

/// Orijinal dosya ile revize sürüm arasındaki
/// doğrulanmış ilişkiyi temsil eder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileVersionPair {
    pub original: FileHashRecord,
    pub revised: FileHashRecord,
    pub status: FileVersionPairStatus,
    pub matched_at: SystemTime,
}

impl FileVersionPair {
    /// Orijinal ve revize hash kayıtlarını doğrulayarak
    /// bir sürüm çifti oluşturur.
    pub fn new(
        original: FileHashRecord,
        revised: FileHashRecord,
        matched_at: SystemTime,
    ) -> Self {
        let status = if !original.is_complete()
            || !revised.is_complete()
        {
            FileVersionPairStatus::InvalidHashRecord
        } else if original.role != FileHashRole::Original {
            FileVersionPairStatus::InvalidOriginalRole
        } else if revised.role != FileHashRole::Revised {
            FileVersionPairStatus::InvalidRevisedRole
        } else if original.compare_with(&revised)
            == FileHashComparison::AlgorithmMismatch
        {
            FileVersionPairStatus::AlgorithmMismatch
        } else {
            FileVersionPairStatus::Matched
        };

        Self {
            original,
            revised,
            status,
            matched_at,
        }
    }

    /// Orijinal ve revize dosyaları diskten okuyup
/// SHA-256 kayıtlarını oluşturarak sürüm çiftini üretir.
pub fn from_files_sha256(
    original_path: impl AsRef<Path>,
    revised_path: impl AsRef<Path>,
    matched_at: SystemTime,
) -> std::io::Result<Self> {
    let original = FileHashRecord::from_file_sha256(
        original_path.as_ref(),
        FileHashRole::Original,
    )?;

    let revised = FileHashRecord::from_file_sha256(
        revised_path.as_ref(),
        FileHashRole::Revised,
    )?;

    Ok(Self::new(
        original,
        revised,
        matched_at,
    ))
}

    /// Kayıtların geçerli bir orijinal–revize sürüm
    /// çifti oluşturduğunu bildirir.
    pub fn is_matched(&self) -> bool {
        self.status == FileVersionPairStatus::Matched
    }

    /// Orijinal ve revize sürümlerin içeriklerinin
    /// birbirinden farklı olup olmadığını bildirir.
    pub fn content_changed(&self) -> bool {
        self.is_matched()
            && self.original.compare_with(&self.revised)
                == FileHashComparison::Changed
    }

    /// Orijinal ve revize sürümlerin içeriklerinin
    /// aynı olup olmadığını bildirir.
    pub fn content_identical(&self) -> bool {
        self.is_matched()
            && self.original.compare_with(&self.revised)
                == FileHashComparison::Identical
    }

    /// Sürüm çiftinin belirtilen orijinal ve revize
    /// dosya yollarına ait olup olmadığını bildirir.
    pub fn belongs_to_paths(
        &self,
        original_path: impl AsRef<Path>,
        revised_path: impl AsRef<Path>,
    ) -> bool {
        self.original.belongs_to_path(original_path)
            && self.revised.belongs_to_path(revised_path)
    }

    /// Eşleştirmenin insan veya Rasterast incelemesi
    /// gerektirip gerektirmediğini bildirir.
    pub fn requires_review(&self) -> bool {
        !self.is_matched()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash_record(
        path: &str,
        role: FileHashRole,
        algorithm: &str,
        digest: &str,
    ) -> FileHashRecord {
        FileHashRecord::new(
            path,
            role,
            algorithm,
            digest,
            SystemTime::now(),
        )
    }

    #[test]
    fn matches_original_and_changed_revised_version() {
        let original = hash_record(
            "articles/hebun.md",
            FileHashRole::Original,
            "SHA-256",
            "original-digest",
        );

        let revised = hash_record(
            "articles/hebun-v2.md",
            FileHashRole::Revised,
            "SHA-256",
            "revised-digest",
        );

        let pair = FileVersionPair::new(
            original,
            revised,
            SystemTime::now(),
        );

        assert_eq!(
            pair.status,
            FileVersionPairStatus::Matched,
        );

        assert!(pair.is_matched());
        assert!(pair.content_changed());
        assert!(!pair.content_identical());
        assert!(!pair.requires_review());

        assert!(
            pair.belongs_to_paths(
                "articles/hebun.md",
                "articles/hebun-v2.md",
            )
        );
    }

    #[test]
    fn matches_original_and_identical_revised_version() {
        let original = hash_record(
            "articles/hebun.md",
            FileHashRole::Original,
            "SHA-256",
            "same-digest",
        );

        let revised = hash_record(
            "articles/hebun-copy.md",
            FileHashRole::Revised,
            "sha-256",
            "same-digest",
        );

        let pair = FileVersionPair::new(
            original,
            revised,
            SystemTime::now(),
        );

        assert!(pair.is_matched());
        assert!(pair.content_identical());
        assert!(!pair.content_changed());
    }

    #[test]
    fn rejects_non_original_first_record() {
        let original = hash_record(
            "articles/hebun.md",
            FileHashRole::Revised,
            "SHA-256",
            "original-digest",
        );

        let revised = hash_record(
            "articles/hebun-v2.md",
            FileHashRole::Revised,
            "SHA-256",
            "revised-digest",
        );

        let pair = FileVersionPair::new(
            original,
            revised,
            SystemTime::now(),
        );

        assert_eq!(
            pair.status,
            FileVersionPairStatus::InvalidOriginalRole,
        );

        assert!(!pair.is_matched());
        assert!(pair.requires_review());
    }

    #[test]
    fn rejects_non_revised_second_record() {
        let original = hash_record(
            "articles/hebun.md",
            FileHashRole::Original,
            "SHA-256",
            "original-digest",
        );

        let revised = hash_record(
            "articles/hebun-v2.md",
            FileHashRole::Original,
            "SHA-256",
            "revised-digest",
        );

        let pair = FileVersionPair::new(
            original,
            revised,
            SystemTime::now(),
        );

        assert_eq!(
            pair.status,
            FileVersionPairStatus::InvalidRevisedRole,
        );

        assert!(pair.requires_review());
    }

    #[test]
    fn rejects_algorithm_mismatch() {
        let original = hash_record(
            "articles/hebun.md",
            FileHashRole::Original,
            "SHA-256",
            "same-digest",
        );

        let revised = hash_record(
            "articles/hebun-v2.md",
            FileHashRole::Revised,
            "SHA-512",
            "same-digest",
        );

        let pair = FileVersionPair::new(
            original,
            revised,
            SystemTime::now(),
        );

        assert_eq!(
            pair.status,
            FileVersionPairStatus::AlgorithmMismatch,
        );

        assert!(!pair.is_matched());
        assert!(pair.requires_review());
    }

    #[test]
    fn rejects_incomplete_hash_record() {
        let original = FileHashRecord::new(
            "",
            FileHashRole::Original,
            "",
            "",
            SystemTime::now(),
        );

        let revised = hash_record(
            "articles/hebun-v2.md",
            FileHashRole::Revised,
            "SHA-256",
            "revised-digest",
        );

        let pair = FileVersionPair::new(
            original,
            revised,
            SystemTime::now(),
        );

        assert_eq!(
            pair.status,
            FileVersionPairStatus::InvalidHashRecord,
        );

        assert!(pair.requires_review());
    }
#[test]
fn creates_changed_version_pair_from_files() {
    use std::fs;

    let original_path = std::env::temp_dir().join(
        format!(
            "mira-version-original-{}.txt",
            std::process::id(),
        ),
    );

    let revised_path = std::env::temp_dir().join(
        format!(
            "mira-version-revised-{}.txt",
            std::process::id(),
        ),
    );

    fs::write(&original_path, b"Hebun original")
        .expect("original file should be created");

    fs::write(&revised_path, b"Hebun revised")
        .expect("revised file should be created");

    let pair = FileVersionPair::from_files_sha256(
        &original_path,
        &revised_path,
        SystemTime::now(),
    )
    .expect("file version pair should be created");

    assert!(pair.is_matched());
    assert!(pair.content_changed());
    assert!(!pair.content_identical());

    assert!(
        pair.belongs_to_paths(
            &original_path,
            &revised_path,
        )
    );

    fs::remove_file(&original_path)
        .expect("original file should be removed");

    fs::remove_file(&revised_path)
        .expect("revised file should be removed");
}

}

#[test]
fn creates_identical_version_pair_from_files() {
    use std::fs;

    let original_path = std::env::temp_dir().join(
        format!(
            "mira-version-identical-original-{}.txt",
            std::process::id(),
        ),
    );

    let revised_path = std::env::temp_dir().join(
        format!(
            "mira-version-identical-revised-{}.txt",
            std::process::id(),
        ),
    );

    fs::write(&original_path, b"Rasterast")
        .expect("original file should be created");

    fs::write(&revised_path, b"Rasterast")
        .expect("revised file should be created");

    let pair = FileVersionPair::from_files_sha256(
        &original_path,
        &revised_path,
        SystemTime::now(),
    )
    .expect("identical file version pair should be created");

    assert!(pair.is_matched());
    assert!(pair.content_identical());
    assert!(!pair.content_changed());

    fs::remove_file(&original_path)
        .expect("original file should be removed");

    fs::remove_file(&revised_path)
        .expect("revised file should be removed");
}




