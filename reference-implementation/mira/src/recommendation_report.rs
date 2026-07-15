use crate::{
    MiraRecommendation,
    RasterastReport,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Mira önerisinin mevcut hazırlık durumunu gösterir.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecommendationReadiness {
    Draft,
    RequiresRevision,
    ReadyForMudebbirReview,
}

/// Müdebbire sunulacak açıklamalı öneri raporu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationReport {
    pub recommendation: MiraRecommendation,
    pub readiness: RecommendationReadiness,
    pub transformation_summary: Vec<String>,
    pub protected_elements: Vec<String>,
    pub publication_status: String,
}

impl RecommendationReport {
    /// Raporun Müdebbir incelemesine hazır olup olmadığını bildirir.
    pub fn is_ready_for_mudebbir_review(&self) -> bool {
        self.readiness
            == RecommendationReadiness::ReadyForMudebbirReview
    }

    /// Raporun bağlı olduğu Mira görev kimliğini döndürür.
    pub fn task_id(&self) -> Uuid {
        self.recommendation.task_id
    }
}

/// Rasterast sonuçlarından gerekçe ve fayda–risk raporu oluşturur.
///
/// Bu katman:
/// - orijinal metni değiştirmez,
/// - doğrulanmamış iddiaları doğrulanmış gibi sunmaz,
/// - riskleri gizlemez,
/// - alternatifleri Müdebbire açıkça gösterir,
/// - yayın kararını kendisi vermez.
#[derive(Debug, Default)]
pub struct RecommendationReportBuilder;

impl RecommendationReportBuilder {
    pub fn new() -> Self {
        Self
    }

    #[allow(clippy::too_many_arguments)]
    pub fn build(
        &self,
        task_id: Uuid,
        rationale: impl Into<String>,
        benefits: Vec<String>,
        risks: Vec<String>,
        alternatives: Vec<String>,
        transformation_summary: Vec<String>,
        protected_elements: Vec<String>,
        proposed_next_step: impl Into<String>,
        publication_status: impl Into<String>,
        rasterast_report: RasterastReport,
    ) -> RecommendationReport {
        let readiness = Self::determine_readiness(
            &rasterast_report,
            &risks,
        );

        let requires_mudebbir_approval =
            rasterast_report.requires_mudebbir_decision
                || !risks.is_empty();

        RecommendationReport {
            recommendation: MiraRecommendation {
                task_id,
                rationale: rationale.into(),
                benefits,
                risks,
                alternatives,
                rasterast_report: Some(rasterast_report),
                proposed_next_step: proposed_next_step.into(),
                requires_mudebbir_approval,
            },
            readiness,
            transformation_summary,
            protected_elements,
            publication_status: publication_status.into(),
        }
    }

    fn determine_readiness(
        rasterast_report: &RasterastReport,
        risks: &[String],
    ) -> RecommendationReadiness {
        if !rasterast_report.contradictions.is_empty() {
            return RecommendationReadiness::RequiresRevision;
        }

        if rasterast_report.verified
            && rasterast_report.unverified_items.is_empty()
            && risks.is_empty()
        {
            return RecommendationReadiness::ReadyForMudebbirReview;
        }

        RecommendationReadiness::Draft
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    fn verified_report(task_id: Uuid) -> RasterastReport {
        RasterastReport {
            task_id,
            verified: true,
            verified_items: vec![
                "Yapısal öneri doğrulandı.".to_string(),
            ],
            unverified_items: Vec::new(),
            contradictions: Vec::new(),
            risks: Vec::new(),
            requires_mudebbir_decision: true,
            created_at: Utc::now(),
        }
    }

    fn unverified_report(task_id: Uuid) -> RasterastReport {
        RasterastReport {
            task_id,
            verified: false,
            verified_items: Vec::new(),
            unverified_items: vec![
                "Kaynak doğrulaması tamamlanmadı.".to_string(),
            ],
            contradictions: Vec::new(),
            risks: vec![
                "Eksik kaynak riski.".to_string(),
            ],
            requires_mudebbir_decision: true,
            created_at: Utc::now(),
        }
    }

    #[test]
    fn verified_safe_report_is_ready_for_mudebbir_review() {
        let task_id = Uuid::new_v4();
        let builder = RecommendationReportBuilder::new();

        let report = builder.build(
            task_id,
            "Makale yapısının akademik olarak düzenlenmesi öneriliyor.",
            vec![
                "Okunabilirlik artabilir.".to_string(),
            ],
            Vec::new(),
            vec![
                "Mevcut yapı korunabilir.".to_string(),
            ],
            vec![
                "Başlık hiyerarşisi düzenlenecek.".to_string(),
            ],
            vec![
                "Orijinal terminoloji korunacak.".to_string(),
            ],
            "Müdebbir incelemesine sun.",
            "İç taslak",
            verified_report(task_id),
        );

        assert_eq!(report.task_id(), task_id);
        assert!(report.is_ready_for_mudebbir_review());
        assert!(
            report
                .recommendation
                .requires_mudebbir_approval
        );
    }

    #[test]
    fn unverified_report_remains_draft() {
        let task_id = Uuid::new_v4();
        let builder = RecommendationReportBuilder::new();

        let report = builder.build(
            task_id,
            "Kaynak yapısının güçlendirilmesi öneriliyor.",
            vec![
                "Kaynak güvenilirliği artabilir.".to_string(),
            ],
            vec![
                "Yanlış kaynak ekleme riski.".to_string(),
            ],
            vec![
                "Kaynak eklemeden iç taslakta tut.".to_string(),
            ],
            vec![
                "Eksik kaynaklar işaretlenecek.".to_string(),
            ],
            vec![
                "Orijinal alıntılar değiştirilmeyecek."
                    .to_string(),
            ],
            "Kaynak doğrulaması yap.",
            "Yayın için hazır değil",
            unverified_report(task_id),
        );

        assert_eq!(
            report.readiness,
            RecommendationReadiness::Draft
        );

        assert!(
            report
                .recommendation
                .requires_mudebbir_approval
        );
    }

    #[test]
    fn contradiction_requires_revision() {
        let task_id = Uuid::new_v4();

        let rasterast_report = RasterastReport {
            task_id,
            verified: false,
            verified_items: Vec::new(),
            unverified_items: Vec::new(),
            contradictions: vec![
                "İki uzman sonucu birbiriyle çelişiyor."
                    .to_string(),
            ],
            risks: Vec::new(),
            requires_mudebbir_decision: true,
            created_at: Utc::now(),
        };

        let builder = RecommendationReportBuilder::new();

        let report = builder.build(
            task_id,
            "Çelişkili sonuçlar yeniden incelenmelidir.",
            Vec::new(),
            Vec::new(),
            vec![
                "Yeni uzman incelemesi iste.".to_string(),
            ],
            Vec::new(),
            vec![
                "Orijinal metne dokunma.".to_string(),
            ],
            "Görevi yeniden planla.",
            "İnceleme bekliyor",
            rasterast_report,
        );

        assert_eq!(
            report.readiness,
            RecommendationReadiness::RequiresRevision
        );
    }

    #[test]
    fn protected_elements_are_preserved_in_report() {
        let task_id = Uuid::new_v4();
        let builder = RecommendationReportBuilder::new();

        let report = builder.build(
            task_id,
            "Terminoloji koruması gereklidir.",
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec![
                "Hebûn".to_string(),
                "Zanabûn".to_string(),
                "Mabûn".to_string(),
                "Rabûn".to_string(),
                "Rasterast".to_string(),
                "Newroza Kawa".to_string(),
            ],
            "Müdebbir incelemesine sun.",
            "İç taslak",
            verified_report(task_id),
        );

        assert_eq!(report.protected_elements.len(), 6);
        assert!(
            report
                .protected_elements
                .contains(&"Rasterast".to_string())
        );
    }
}


