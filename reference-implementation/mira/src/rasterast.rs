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
}



