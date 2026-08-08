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

/// Korunan bir metnin bütünlük doğrulama sonucunu temsil eder.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct OriginalTextIntegrityReport {
    pub relative_path: String,
    pub valid: bool,
    pub changed: bool,
}

impl OriginalTextIntegrityRecord {
    /// Mevcut bütünlük kaydından doğrulama raporu üretir.
    pub fn integrity_report(&self) -> OriginalTextIntegrityReport {
        OriginalTextIntegrityReport {
            relative_path: self.relative_path.clone(),
            valid: self.is_valid(),
            changed: self.has_changed(),
        }
    }
}

/// Korunan bir metnin değişiklik özetini temsil eder.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct TextDiffReport {
    pub relative_path: String,
    pub changed: bool,
    pub original_sha256: String,
    pub current_sha256: String,
}

impl OriginalTextIntegrityRecord {
    /// Bütünlük kaydından değişiklik raporu üretir.
    pub fn diff_report(&self) -> TextDiffReport {
        TextDiffReport {
            relative_path: self.relative_path.clone(),
            changed: self.has_changed(),
            original_sha256: self.original_sha256.clone(),
            current_sha256: self.current_sha256.clone(),
        }
    }
}

/// Rasterast doğrulamasının temel sonucunu temsil eder.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct RasterastVerificationResult {
    pub subject: String,
    pub consistent: bool,
    pub source_valid: bool,
    pub integrity_preserved: bool,
}

impl RasterastVerificationResult {
    /// Yeni bir Rasterast doğrulama sonucu oluşturur.
    pub fn new(
        subject: impl Into<String>,
        consistent: bool,
        source_valid: bool,
        integrity_preserved: bool,
    ) -> Self {
        Self {
            subject: subject.into(),
            consistent,
            source_valid,
            integrity_preserved,
        }
    }

    /// Rasterast doğrulamasının tamamen başarılı
    /// olup olmadığını bildirir.
    pub fn is_valid(&self) -> bool {
        !self.subject.trim().is_empty()
            && self.consistent
            && self.source_valid
            && self.integrity_preserved
    }
}

/// Bir güvenlik veya yayın kararının
/// fayda–risk değerlendirmesini temsil eder.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct BenefitRiskAssessment {
    pub subject: String,
    pub benefit_score: u8,
    pub risk_score: u8,
    pub rationale: String,
}

impl BenefitRiskAssessment {
    /// Yeni bir fayda–risk değerlendirmesi oluşturur.
    pub fn new(
        subject: impl Into<String>,
        benefit_score: u8,
        risk_score: u8,
        rationale: impl Into<String>,
    ) -> Self {
        Self {
            subject: subject.into(),
            benefit_score,
            risk_score,
            rationale: rationale.into(),
        }
    }

    /// Değerlendirmenin zorunlu bilgilerinin
    /// geçerli olup olmadığını bildirir.
    pub fn is_valid(&self) -> bool {
        !self.subject.trim().is_empty()
            && !self.rationale.trim().is_empty()
            && self.benefit_score <= 100
            && self.risk_score <= 100
    }

    /// Faydanın riskten büyük veya eşit
    /// olup olmadığını bildirir.
    pub fn benefit_outweighs_risk(&self) -> bool {
        self.is_valid()
            && self.benefit_score >= self.risk_score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn original_text_integrity_report_detects_changes() {
        let unchanged =
            OriginalTextIntegrityRecord::from_texts(
                "papers/hebun.md",
                "Hebûn değişmez çekirdek ilkedir.",
                "Hebûn değişmez çekirdek ilkedir.",
            );

        let unchanged_report =
            unchanged.integrity_report();

        assert!(unchanged_report.valid);
        assert!(!unchanged_report.changed);

        let changed =
            OriginalTextIntegrityRecord::from_texts(
                "papers/hebun.md",
                "Hebûn değişmez çekirdek ilkedir.",
                "Hebûn çekirdek ilkedir.",
            );

        let changed_report =
            changed.integrity_report();

        assert!(changed_report.valid);
        assert!(changed_report.changed);
    }
}








