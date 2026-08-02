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

    /// İki hash kaydının aynı algoritma ve aynı özet
/// üzerinden aynı dosya içeriğini temsil edip
/// etmediğini bildirir.
pub fn has_same_content_as(
    &self,
    other: &FileHashRecord,
) -> bool {
    self.algorithm
        .eq_ignore_ascii_case(&other.algorithm)
        && self.digest == other.digest
}

    
    /// Dosyanın SHA-256 özetini hesaplayarak yeni bir hash kaydı oluşturur.
    pub fn from_file_sha256(
        path: impl Into<PathBuf>,
        role: FileHashRole,
    ) -> std::io::Result<Self> {
        use sha2::{Digest, Sha256};
        use std::fs;

        let path = path.into();
        let bytes = fs::read(&path)?;

        let mut hasher = Sha256::new();
        hasher.update(&bytes);

        let digest = format!("{:x}", hasher.finalize());

        Ok(Self {
            path,
            role,
            algorithm: "SHA-256".to_string(),
            digest,
            recorded_at: SystemTime::now(),
        })
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

    #[test]
    fn creates_sha256_hash_record_from_file() {
        use std::fs;

        let file_path = std::env::temp_dir().join(
            format!(
                "mira-hash-test-{}.txt",
                std::process::id(),
            ),
        );

        fs::write(&file_path, b"Rasterast")
            .expect("temporary file should be created");

        let record = FileHashRecord::from_file_sha256(
            &file_path,
            FileHashRole::Original,
        )
        .expect("hash should be computed");

        assert!(record.is_complete());
        assert!(record.is_original());
        assert_eq!(record.algorithm, "SHA-256");
        assert_eq!(record.digest.len(), 64);

        fs::remove_file(&file_path)
            .expect("temporary file should be removed");
    }
}
#[test]
fn sha256_returns_different_hashes_for_different_files() {
    use std::fs;

    let temp_dir = std::env::temp_dir();

    let file1 = temp_dir.join("mira_hash_test_1.txt");
    let file2 = temp_dir.join("mira_hash_test_2.txt");

    fs::write(&file1, b"Hebun").unwrap();
    fs::write(&file2, b"Rabun").unwrap();

    let hash1 =
        FileHashRecord::from_file_sha256(&file1, FileHashRole::Original)
            .unwrap();

    let hash2 =
        FileHashRecord::from_file_sha256(&file2, FileHashRole::Original)
            .unwrap();

    assert_ne!(hash1.digest, hash2.digest);

    let _ = fs::remove_file(file1);
    let _ = fs::remove_file(file2);
}

#[test]
fn compares_original_and_revised_file_content_hashes() {
    use std::fs;

    let temp_directory = std::env::temp_dir();

    let original_path = temp_directory.join(
        format!(
            "mira-original-hash-{}.txt",
            std::process::id(),
        ),
    );

    let revised_path = temp_directory.join(
        format!(
            "mira-revised-hash-{}.txt",
            std::process::id(),
        ),
    );

    fs::write(&original_path, b"Hebun")
        .expect("original test file should be created");

    fs::write(&revised_path, b"Hebun")
        .expect("revised test file should be created");

    let original = FileHashRecord::from_file_sha256(
        &original_path,
        FileHashRole::Original,
    )
    .expect("original hash should be computed");

    let revised = FileHashRecord::from_file_sha256(
        &revised_path,
        FileHashRole::Revised,
    )
    .expect("revised hash should be computed");

    assert!(original.has_same_content_as(&revised));

    fs::write(&revised_path, b"Hebun revised")
        .expect("revised test file should be updated");

    let changed_revised =
        FileHashRecord::from_file_sha256(
            &revised_path,
            FileHashRole::Revised,
        )
        .expect("changed hash should be computed");

    assert!(
        !original.has_same_content_as(
            &changed_revised,
        )
    );

    fs::remove_file(&original_path)
        .expect("original test file should be removed");

    fs::remove_file(&revised_path)
        .expect("revised test file should be removed");
}


