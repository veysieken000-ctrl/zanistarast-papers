use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Rasterast doğrulama sonucu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RasterastReport {
    pub task_id: Uuid,
    pub verified: bool,
    pub verified_items: Vec<String>,
    pub unverified_items: Vec<String>,
    pub contradictions: Vec<String>,
    pub risks: Vec<String>,
    pub requires_mudebbir_decision: bool,
    pub created_at: DateTime<Utc>,
}

impl RasterastReport {
    /// Yeni bir Rasterast doğrulama raporu oluşturur.
    pub fn new(
        task_id: Uuid,
        verified_items: Vec<String>,
        unverified_items: Vec<String>,
        contradictions: Vec<String>,
        risks: Vec<String>,
        requires_mudebbir_decision: bool,
    ) -> Self {
        let verified = unverified_items.is_empty()
            && contradictions.is_empty()
            && risks.is_empty();

        Self {
            task_id,
            verified,
            verified_items,
            unverified_items,
            contradictions,
            risks,
            requires_mudebbir_decision,
            created_at: Utc::now(),
        }
    }

    /// Raporun doğrulamadan başarıyla geçip geçmediğini bildirir.
    pub fn is_verified(&self) -> bool {
        self.verified
    }

    /// Raporun çözümlenmemiş bir sorun taşıyıp taşımadığını bildirir.
    pub fn requires_review(&self) -> bool {
        !self.unverified_items.is_empty()
            || !self.contradictions.is_empty()
            || !self.risks.is_empty()
    }

    /// Raporun Müdebbir kararı bekleyip beklemediğini bildirir.
    pub fn awaits_mudebbir(&self) -> bool {
        self.requires_mudebbir_decision
    }

/// Hash, bütünlük, sürüm çifti ve diff güvenlik
/// sonuçlarını tek bir Rasterast raporunda birleştirir.
pub fn from_security_results(
    task_id: Uuid,
    original_hash: &crate::FileHashRecord,
    revised_hash: &crate::FileHashRecord,
    integrity_report: &crate::FileIntegrityReport,
    version_pair: &crate::FileVersionPair,
    diff_report: &crate::FileDiffReport,
) -> Self {
    let mut verified_items = Vec::new();
    let mut unverified_items = Vec::new();
    let mut contradictions = Vec::new();
    let mut risks = Vec::new();

    if original_hash.is_complete()
        && original_hash.is_original()
    {
        verified_items.push(
            "Orijinal dosya hash kaydı doğrulandı."
                .to_string(),
        );
    } else {
        unverified_items.push(
            "Orijinal dosya hash kaydı geçersizdir."
                .to_string(),
        );
    }

    if revised_hash.is_complete()
        && revised_hash.is_revised()
    {
        verified_items.push(
            "Revize dosya hash kaydı doğrulandı."
                .to_string(),
        );
    } else {
        unverified_items.push(
            "Revize dosya hash kaydı geçersizdir."
                .to_string(),
        );
    }

    match integrity_report.status {
        crate::FileIntegrityStatus::Intact => {
            verified_items.push(
                "Dosya bütünlüğü doğrulandı."
                    .to_string(),
            );
        }

        crate::FileIntegrityStatus::Modified => {
            risks.push(
                "Orijinal dosyada içerik değişikliği tespit edildi."
                    .to_string(),
            );
        }

        crate::FileIntegrityStatus::AlgorithmMismatch => {
            contradictions.push(
                "Bütünlük kayıtlarında hash algoritması uyuşmazlığı bulundu."
                    .to_string(),
            );
        }

        crate::FileIntegrityStatus::InvalidRecord => {
            unverified_items.push(
                "Dosya bütünlük kaydı geçersizdir."
                    .to_string(),
            );
        }
    }

    if version_pair.is_matched() {
        verified_items.push(
            "Orijinal ve revize sürüm çifti doğrulandı."
                .to_string(),
        );
    } else {
        unverified_items.push(
            "Dosya sürüm çifti doğrulanamadı."
                .to_string(),
        );
    }

    match diff_report.security_status(version_pair) {
        crate::FileDiffSecurityStatus::Verified => {
            verified_items.push(
                "Diff güvenlik doğrulaması tamamlandı."
                    .to_string(),
            );
        }

        crate::FileDiffSecurityStatus::InvalidVersionPair => {
            unverified_items.push(
                "Diff raporu geçersiz sürüm çiftine bağlıdır."
                    .to_string(),
            );
        }

        crate::FileDiffSecurityStatus::PathMismatch => {
            contradictions.push(
                "Diff raporu ile sürüm çifti yolları uyuşmuyor."
                    .to_string(),
            );
        }

        crate::FileDiffSecurityStatus::InconsistentDiff => {
            contradictions.push(
                "Diff satır değişikliği kayıtları tutarsızdır."
                    .to_string(),
            );
        }

        crate::FileDiffSecurityStatus::HashDiffMismatch => {
            contradictions.push(
                "Hash sonucu ile diff sonucu çelişmektedir."
                    .to_string(),
            );
        }
    }

    let requires_mudebbir_decision =
        !unverified_items.is_empty()
            || !contradictions.is_empty()
            || !risks.is_empty();

    Self::new(
        task_id,
        verified_items,
        unverified_items,
        contradictions,
        risks,
        requires_mudebbir_decision,
    )
}

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_verified_rasterast_report() {
        let report = RasterastReport::new(
            Uuid::new_v4(),
            vec!["Kaynak doğrulandı.".to_string()],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            false,
        );

        assert!(report.is_verified());
        assert!(!report.requires_review());
        assert!(!report.awaits_mudebbir());
    }

    #[test]
    fn report_with_risk_requires_review_and_mudebbir() {
        let report = RasterastReport::new(
            Uuid::new_v4(),
            vec!["Hash doğrulandı.".to_string()],
            Vec::new(),
            Vec::new(),
            vec!["Orijinal dosya değişikliği riski.".to_string()],
            true,
        );

        assert!(!report.is_verified());
        assert!(report.requires_review());
        assert!(report.awaits_mudebbir());
    }

    #[test]
    fn report_with_contradiction_is_not_verified() {
        let report = RasterastReport::new(
            Uuid::new_v4(),
            Vec::new(),
            Vec::new(),
            vec!["Hash ve diff sonuçları çelişiyor.".to_string()],
            Vec::new(),
            false,
        );

        assert!(!report.is_verified());
        assert!(report.requires_review());
    }
#[test]
fn combines_verified_security_results() {
    let original = crate::FileHashRecord::new(
        "articles/hebun.md",
        crate::FileHashRole::Original,
        "SHA-256",
        "original-digest",
        std::time::SystemTime::now(),
    );

    let revised = crate::FileHashRecord::new(
        "articles/hebun-v2.md",
        crate::FileHashRole::Revised,
        "SHA-256",
        "revised-digest",
        std::time::SystemTime::now(),
    );

    let integrity = crate::FileIntegrityReport::verify(
        original.clone(),
        original.clone(),
        std::time::SystemTime::now(),
    );

    let pair = crate::FileVersionPair::new(
        original.clone(),
        revised.clone(),
        std::time::SystemTime::now(),
    );

    let diff = crate::FileDiffReport::new(
        original.path.clone(),
        revised.path.clone(),
        vec![
            crate::FileLineChange::new(
                crate::FileLineChangeKind::Modified,
                Some(1),
                Some(1),
                Some("Hebun".to_string()),
                Some("Hebun revised".to_string()),
            ),
        ],
        std::time::SystemTime::now(),
    );

    let report = RasterastReport::from_security_results(
        Uuid::new_v4(),
        &original,
        &revised,
        &integrity,
        &pair,
        &diff,
    );

    assert!(report.is_verified());
    assert!(!report.requires_review());
    assert!(!report.awaits_mudebbir());
    assert!(report.unverified_items.is_empty());
    assert!(report.contradictions.is_empty());
    assert!(report.risks.is_empty());
}

#[test]
fn security_problem_requires_mudebbir_decision() {
    let original = crate::FileHashRecord::new(
        "articles/hebun.md",
        crate::FileHashRole::Original,
        "SHA-256",
        "original-digest",
        std::time::SystemTime::now(),
    );

    let revised = crate::FileHashRecord::new(
        "articles/hebun-v2.md",
        crate::FileHashRole::Revised,
        "SHA-256",
        "changed-digest",
        std::time::SystemTime::now(),
    );

    let integrity = crate::FileIntegrityReport::verify(
        original.clone(),
        revised.clone(),
        std::time::SystemTime::now(),
    );

    let pair = crate::FileVersionPair::new(
        original.clone(),
        revised.clone(),
        std::time::SystemTime::now(),
    );

    let diff = crate::FileDiffReport::new(
        "articles/other.md",
        "articles/other-v2.md",
        Vec::new(),
        std::time::SystemTime::now(),
    );

    let report = RasterastReport::from_security_results(
        Uuid::new_v4(),
        &original,
        &revised,
        &integrity,
        &pair,
        &diff,
    );

    assert!(!report.is_verified());
    assert!(report.requires_review());
    assert!(report.awaits_mudebbir());
    assert!(!report.risks.is_empty());
    assert!(!report.contradictions.is_empty());
}

}



