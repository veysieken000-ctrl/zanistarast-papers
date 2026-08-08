use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// Zanistarast güvenlik ve Rasterast doğrulama
/// sürecindeki bir incelemenin durumunu belirtir.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub enum SecurityReviewStatus {
    Pending,
    InProgress,
    Passed,
    Failed,
    RequiresMudebbirApproval,
}

/// Tek bir güvenlik veya Rasterast doğrulama
/// incelemesinin temel kaydıdır.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct SecurityReview {
    pub id: String,
    pub subject: String,
    pub status: SecurityReviewStatus,
}

impl SecurityReview {
    /// Yeni bir güvenlik incelemesi oluşturur.
    pub fn new(
        id: impl Into<String>,
        subject: impl Into<String>,
        status: SecurityReviewStatus,
    ) -> Self {
        Self {
            id: id.into(),
            subject: subject.into(),
            status,
        }
    }

    /// İncelemenin zorunlu bilgilerinin
    /// eksiksiz olup olmadığını bildirir.
    pub fn is_valid(&self) -> bool {
        !self.id.trim().is_empty()
            && !self.subject.trim().is_empty()
    }

    /// İncelemenin başarıyla tamamlanıp
    /// tamamlanmadığını bildirir.
    pub fn is_passed(&self) -> bool {
        self.status == SecurityReviewStatus::Passed
    }

    /// Müdebbir kararı gerekip gerekmediğini bildirir.
    pub fn requires_mudebbir_approval(&self) -> bool {
        self.status
            == SecurityReviewStatus::RequiresMudebbirApproval
    }
    }

/// Korunan bir metnin değişiklik öncesi ve güncel
/// SHA-256 bütünlük değerlerini temsil eder.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct OriginalTextIntegrityRecord {
    pub relative_path: String,
    pub original_sha256: String,
    pub current_sha256: String,
}

impl OriginalTextIntegrityRecord {
    /// Yeni bir metin bütünlük kaydı oluşturur.
    pub fn new(
        relative_path: impl Into<String>,
        original_sha256: impl Into<String>,
        current_sha256: impl Into<String>,
    ) -> Self {
        Self {
            relative_path: relative_path.into(),
            original_sha256: original_sha256.into(),
            current_sha256: current_sha256.into(),
        }
    }

    /// Bütünlük kaydının zorunlu bilgilerinin
    /// eksiksiz olup olmadığını bildirir.
    pub fn is_valid(&self) -> bool {
        !self.relative_path.trim().is_empty()
            && !self.original_sha256.trim().is_empty()
            && !self.current_sha256.trim().is_empty()
    }

    /// Korunan metnin SHA-256 değerinin
    /// değişmeden kalıp kalmadığını bildirir.
    pub fn is_unchanged(&self) -> bool {
        self.is_valid()
            && self.original_sha256 == self.current_sha256
    }

    /// Korunan metnin değişmiş olup
    /// olmadığını bildirir.
    pub fn has_changed(&self) -> bool {
        self.is_valid() && !self.is_unchanged()
    }
}

/// Orijinal ve güncel metinlerin SHA-256
    /// değerlerini hesaplayarak bütünlük kaydı oluşturur.
    pub fn from_texts(
        relative_path: impl Into<String>,
        original_text: &str,
        current_text: &str,
    ) -> Self {
        let original_sha256 =
            format!("{:x}", Sha256::digest(original_text.as_bytes()));

        let current_sha256 =
            format!("{:x}", Sha256::digest(current_text.as_bytes()));

        Self::new(
            relative_path,
            original_sha256,
            current_sha256,
        )
    }







