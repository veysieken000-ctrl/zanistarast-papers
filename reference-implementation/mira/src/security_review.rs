use serde::{Deserialize, Serialize};

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



