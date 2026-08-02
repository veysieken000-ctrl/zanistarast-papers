use std::time::SystemTime;

use crate::{
    FileHashComparison,
    FileHashRecord,
    FileHashRole,
};

/// Orijinal dosyanın bütünlük doğrulama sonucunu belirtir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileIntegrityStatus {
    /// Kayıtlı orijinal hash ile güncel hash aynıdır.
    Intact,

    /// Güncel dosya içeriği kayıtlı orijinalden farklıdır.
    Modified,

    /// İki kayıt farklı hash algoritmaları kullanmaktadır.
    AlgorithmMismatch,

    /// Hash kayıtlarından biri eksik veya geçersizdir.
    InvalidRecord,
}

/// Orijinal dosya bütünlük doğrulamasının ayrıntılı kaydıdır.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileIntegrityReport {
    pub original: FileHashRecord,
    pub current: FileHashRecord,
    pub status: FileIntegrityStatus,
    pub verified_at: SystemTime,
}

impl FileIntegrityReport {
    /// Kayıtlı orijinal hash ile güncel dosya hash kaydını
    /// karşılaştırarak bütünlük raporu oluşturur.
    pub fn verify(
        original: FileHashRecord,
        current: FileHashRecord,
        verified_at: SystemTime,
    ) -> Self {
        let status = if !original.is_complete()
            || !current.is_complete()
            || original.role != FileHashRole::Original
        {
            FileIntegrityStatus::InvalidRecord
        } else {
            match original.compare_with(&current) {
                FileHashComparison::Identical => {
                    FileIntegrityStatus::Intact
                }
                FileHashComparison::Changed => {
                    FileIntegrityStatus::Modified
                }
                FileHashComparison::AlgorithmMismatch => {
                    FileIntegrityStatus::AlgorithmMismatch
                }
            }
        };

        Self {
            original,
            current,
            status,
            verified_at,
        }
    }

    /// Orijinal dosyanın değişmeden korunduğunu bildirir.
    pub fn is_intact(&self) -> bool {
        self.status == FileIntegrityStatus::Intact
    }

    /// Orijinal dosyada içerik değişikliği bulunduğunu bildirir.
    pub fn is_modified(&self) -> bool {
        self.status == FileIntegrityStatus::Modified
    }

    /// Raporun güvenlik açısından çözümlenmemiş bir durum
    /// taşıyıp taşımadığını bildirir.
    pub fn requires_review(&self) -> bool {
        self.status != FileIntegrityStatus::Intact
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hash_record(
        role: FileHashRole,
        algorithm: &str,
        digest: &str,
    ) -> FileHashRecord {
        FileHashRecord::new(
            "articles/hebun.md",
            role,
            algorithm,
            digest,
            SystemTime::now(),
        )
    }

    #[test]
    fn reports_intact_original_file() {
        let original = hash_record(
            FileHashRole::Original,
            "SHA-256",
            "same-digest",
        );

        let current = hash_record(
            FileHashRole::Revised,
            "sha-256",
            "same-digest",
        );

        let report = FileIntegrityReport::verify(
            original,
            current,
            SystemTime::now(),
        );

        assert_eq!(
            report.status,
            FileIntegrityStatus::Intact,
        );

        assert!(report.is_intact());
        assert!(!report.is_modified());
        assert!(!report.requires_review());
    }

    #[test]
    fn reports_modified_original_file() {
        let original = hash_record(
            FileHashRole::Original,
            "SHA-256",
            "original-digest",
        );

        let current = hash_record(
            FileHashRole::Revised,
            "SHA-256",
            "changed-digest",
        );

        let report = FileIntegrityReport::verify(
            original,
            current,
            SystemTime::now(),
        );

        assert_eq!(
            report.status,
            FileIntegrityStatus::Modified,
        );

        assert!(!report.is_intact());
        assert!(report.is_modified());
        assert!(report.requires_review());
    }

    #[test]
    fn reports_algorithm_mismatch() {
        let original = hash_record(
            FileHashRole::Original,
            "SHA-256",
            "same-digest",
        );

        let current = hash_record(
            FileHashRole::Revised,
            "SHA-512",
            "same-digest",
        );

        let report = FileIntegrityReport::verify(
            original,
            current,
            SystemTime::now(),
        );

        assert_eq!(
            report.status,
            FileIntegrityStatus::AlgorithmMismatch,
        );

        assert!(report.requires_review());
    }

    #[test]
    fn rejects_non_original_baseline_record() {
        let original = hash_record(
            FileHashRole::Revised,
            "SHA-256",
            "same-digest",
        );

        let current = hash_record(
            FileHashRole::Revised,
            "SHA-256",
            "same-digest",
        );

        let report = FileIntegrityReport::verify(
            original,
            current,
            SystemTime::now(),
        );

        assert_eq!(
            report.status,
            FileIntegrityStatus::InvalidRecord,
        );

        assert!(report.requires_review());
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

        let current = hash_record(
            FileHashRole::Revised,
            "SHA-256",
            "same-digest",
        );

        let report = FileIntegrityReport::verify(
            original,
            current,
            SystemTime::now(),
        );

        assert_eq!(
            report.status,
            FileIntegrityStatus::InvalidRecord,
        );

        assert!(report.requires_review());
    }
}



