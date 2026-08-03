use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::{
    FileHashRecord,
    FileHashRole,
    FileVersionPair,
    MudebbirDecisionRecord,
};

/// Güvenli yeni sürüm oluşturma işleminin sonucudur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafeFileVersionResult {
    pub original_path: PathBuf,
    pub revised_path: PathBuf,
    pub original_hash: FileHashRecord,
    pub revised_hash: FileHashRecord,
    pub version_pair: FileVersionPair,
}

impl SafeFileVersionResult {
    /// Güvenli sürüm sonucunun geçerli bir orijinal–revize
    /// sürüm çifti taşıyıp taşımadığını bildirir.
    pub fn is_valid(&self) -> bool {
        self.original_path != self.revised_path
            && self.original_hash.is_complete()
            && self.original_hash.is_original()
            && self.revised_hash.is_complete()
            && self.revised_hash.is_revised()
            && self.version_pair.is_matched()
    }
}

/// Müdebbir onayına dayanarak orijinal dosyayı değiştirmeden
/// güvenli bir revize dosya oluşturur.
///
/// Güvenlik kuralları:
///
/// - Müdebbir onayı bulunmalıdır.
/// - Orijinal yol ile revize yol aynı olamaz.
/// - Revize hedef zaten varsa üzerine yazılmaz.
/// - Orijinal dosyanın içeriği değiştirilmez.
/// - Her iki dosyanın SHA-256 kayıtları oluşturulur.
pub fn create_safe_file_version(
    original_path: impl AsRef<Path>,
    revised_path: impl AsRef<Path>,
    decision: &MudebbirDecisionRecord,
) -> std::io::Result<SafeFileVersionResult> {
    let original_path = original_path.as_ref().to_path_buf();
    let revised_path = revised_path.as_ref().to_path_buf();

    if !decision.approves_version() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "Mudebbir approval is required",
        ));
    }

    if original_path == revised_path {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "original and revised paths must be different",
        ));
    }

    let mut original_file = std::fs::File::open(&original_path)?;
    let mut original_content = Vec::new();

    original_file.read_to_end(&mut original_content)?;

    let mut revised_file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&revised_path)?;

    if let Err(error) = revised_file.write_all(&original_content) {
        let _ = std::fs::remove_file(&revised_path);
        return Err(error);
    }

    if let Err(error) = revised_file.flush() {
        let _ = std::fs::remove_file(&revised_path);
        return Err(error);
    }

    drop(revised_file);

    let original_hash = match FileHashRecord::from_file_sha256(
        &original_path,
        FileHashRole::Original,
    ) {
        Ok(record) => record,
        Err(error) => {
            let _ = std::fs::remove_file(&revised_path);
            return Err(error);
        }
    };

    let revised_hash = match FileHashRecord::from_file_sha256(
        &revised_path,
        FileHashRole::Revised,
    ) {
        Ok(record) => record,
        Err(error) => {
            let _ = std::fs::remove_file(&revised_path);
            return Err(error);
        }
    };

    let version_pair = FileVersionPair::new(
        original_hash.clone(),
        revised_hash.clone(),
        SystemTime::now(),
    );

    if !version_pair.is_matched() {
        let _ = std::fs::remove_file(&revised_path);

        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            "created file version pair is not valid",
        ));
    }

    Ok(SafeFileVersionResult {
        original_path,
        revised_path,
        original_hash,
        revised_hash,
        version_pair,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MudebbirDecision;
    use uuid::Uuid;

    fn temporary_paths(
        test_name: &str,
    ) -> (PathBuf, PathBuf) {
        let unique_id = Uuid::new_v4();

        let original_path = std::env::temp_dir().join(
            format!(
                "mira-{test_name}-original-{unique_id}.txt",
            ),
        );

        let revised_path = std::env::temp_dir().join(
            format!(
                "mira-{test_name}-revised-{unique_id}.txt",
            ),
        );

        (original_path, revised_path)
    }

    #[test]
    fn creates_safe_revised_file_after_mudebbir_approval() {
        let (
            original_path,
            revised_path,
        ) = temporary_paths("approved-version");

        std::fs::write(
            &original_path,
            b"Hebun original",
        )
        .expect("original test file should be created");

        let decision = MudebbirDecisionRecord::new(
            Uuid::new_v4(),
            MudebbirDecision::Approved,
        );

        let result = create_safe_file_version(
            &original_path,
            &revised_path,
            &decision,
        )
        .expect("approved revised file should be created");

        assert!(result.is_valid());
        assert!(revised_path.exists());

        assert_eq!(
            std::fs::read(&original_path)
                .expect("original file should be readable"),
            b"Hebun original",
        );

        assert_eq!(
            std::fs::read(&revised_path)
                .expect("revised file should be readable"),
            b"Hebun original",
        );

        assert!(result.original_hash.is_original());
        assert!(result.revised_hash.is_revised());
        assert!(result.version_pair.is_matched());

        std::fs::remove_file(original_path)
            .expect("original test file should be removed");

        std::fs::remove_file(revised_path)
            .expect("revised test file should be removed");
    }

    #[test]
    fn rejects_original_path_as_revised_target() {
        let (
            original_path,
            _,
        ) = temporary_paths("same-path");

        std::fs::write(
            &original_path,
            b"Protected original",
        )
        .expect("original test file should be created");

        let decision = MudebbirDecisionRecord::new(
            Uuid::new_v4(),
            MudebbirDecision::Approved,
        );

        let result = create_safe_file_version(
            &original_path,
            &original_path,
            &decision,
        );

        assert!(result.is_err());

        assert_eq!(
            result
                .expect_err("same path must be rejected")
                .kind(),
            std::io::ErrorKind::InvalidInput,
        );

        assert_eq!(
            std::fs::read(&original_path)
                .expect("original file should remain readable"),
            b"Protected original",
        );

        std::fs::remove_file(original_path)
            .expect("original test file should be removed");
    }

    #[test]
    fn refuses_to_overwrite_existing_revised_file() {
        let (
            original_path,
            revised_path,
        ) = temporary_paths("existing-target");

        std::fs::write(
            &original_path,
            b"Original content",
        )
        .expect("original test file should be created");

        std::fs::write(
            &revised_path,
            b"Existing protected content",
        )
        .expect("existing target should be created");

        let decision = MudebbirDecisionRecord::new(
            Uuid::new_v4(),
            MudebbirDecision::Approved,
        );

        let result = create_safe_file_version(
            &original_path,
            &revised_path,
            &decision,
        );

        assert!(result.is_err());

        assert_eq!(
            result
                .expect_err("existing target must be rejected")
                .kind(),
            std::io::ErrorKind::AlreadyExists,
        );

        assert_eq!(
            std::fs::read(&revised_path)
                .expect("existing target should be readable"),
            b"Existing protected content",
        );

        std::fs::remove_file(original_path)
            .expect("original test file should be removed");

        std::fs::remove_file(revised_path)
            .expect("existing target should be removed");
    }

    #[test]
    fn rejects_version_creation_without_mudebbir_approval() {
        let (
            original_path,
            revised_path,
        ) = temporary_paths("without-approval");

        std::fs::write(
            &original_path,
            b"Original content",
        )
        .expect("original test file should be created");

        let decision = MudebbirDecisionRecord::new(
            Uuid::new_v4(),
            MudebbirDecision::Pending,
        );

        let result = create_safe_file_version(
            &original_path,
            &revised_path,
            &decision,
        );

        assert!(result.is_err());

        assert_eq!(
            result
                .expect_err("approval must be required")
                .kind(),
            std::io::ErrorKind::PermissionDenied,
        );

        assert!(!revised_path.exists());

        assert_eq!(
            std::fs::read(&original_path)
                .expect("original file should remain readable"),
            b"Original content",
        );

        std::fs::remove_file(original_path)
            .expect("original test file should be removed");
    }
}






