use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Hash kaydının temsil ettiği dosya sürümünü belirtir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileHashRole {
    /// Korunması gereken değişiklik öncesi kaynak dosya.
    Original,

    /// Değişiklik veya üretim sonrasında oluşan yeni sürüm.
    Revised,
}

/// Bir dosyanın bütünlük doğrulamasında kullanılacak
/// hash kaydını temsil eder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileHashRecord {
    pub path: PathBuf,
    pub role: FileHashRole,
    pub algorithm: String,
    pub digest: String,
    pub recorded_at: SystemTime,
}

impl FileHashRecord {
    /// Yeni bir dosya hash kaydı oluşturur.
    pub fn new(
        path: impl Into<PathBuf>,
        role: FileHashRole,
        algorithm: impl Into<String>,
        digest: impl Into<String>,
        recorded_at: SystemTime,
    ) -> Self {
        Self {
            path: path.into(),
            role,
            algorithm: algorithm.into(),
            digest: digest.into(),
            recorded_at,
        }
    }

    /// Hash kaydının zorunlu alanlarının eksiksiz
    /// olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.path.as_os_str().is_empty()
            && !self.algorithm.trim().is_empty()
            && !self.digest.trim().is_empty()
    }

    /// Kaydın belirtilen dosyaya ait olup olmadığını bildirir.
    pub fn belongs_to_path(
        &self,
        path: impl AsRef<Path>,
    ) -> bool {
        self.path == path.as_ref()
    }

    /// Kaydın orijinal dosyayı temsil edip etmediğini bildirir.
    pub fn is_original(&self) -> bool {
        self.role == FileHashRole::Original
    }

    /// Kaydın yeni sürümü temsil edip etmediğini bildirir.
    pub fn is_revised(&self) -> bool {
        self.role == FileHashRole::Revised
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_complete_original_file_hash_record() {
        let recorded_at = SystemTime::now();

        let record = FileHashRecord::new(
            "articles/hebun.md",
            FileHashRole::Original,
            "SHA-256",
            "0123456789abcdef",
            recorded_at,
        );

        assert!(record.is_complete());
        assert!(record.is_original());
        assert!(!record.is_revised());

        assert!(
            record.belongs_to_path(
                "articles/hebun.md",
            )
        );
    }

    #[test]
    fn creates_complete_revised_file_hash_record() {
        let record = FileHashRecord::new(
            "articles/hebun-v2.md",
            FileHashRole::Revised,
            "SHA-256",
            "fedcba9876543210",
            SystemTime::now(),
        );

        assert!(record.is_complete());
        assert!(record.is_revised());
        assert!(!record.is_original());
    }

    #[test]
    fn incomplete_hash_record_is_rejected() {
        let record = FileHashRecord::new(
            "",
            FileHashRole::Original,
            "",
            "",
            SystemTime::now(),
        );

        assert!(!record.is_complete());
    }

    #[test]
    fn unrelated_path_is_not_accepted() {
        let record = FileHashRecord::new(
            "articles/hebun.md",
            FileHashRole::Original,
            "SHA-256",
            "0123456789abcdef",
            SystemTime::now(),
        );

        assert!(
            !record.belongs_to_path(
                "articles/rabun.md",
            )
        );
    }
}



