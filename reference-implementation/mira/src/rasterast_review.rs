use crate::result_collector::AgentResultCollection;
use crate::RasterastReport;
use chrono::Utc;

/// Çoklu uzman sonuçlarını Rasterast ön denetimine hazırlar.
///
/// Bu katman:
/// - uzman çıktısını doğrudan doğru kabul etmez,
/// - başarısız sağlayıcıları açıkça bildirir,
/// - tekrar eden sağlayıcı sonuçlarını risk olarak kaydeder,
/// - içerik doğrulamasını sonraki Rasterast aşamasına bırakır,
/// - gerekli durumlarda Müdebbir kararını zorunlu kılar.
#[derive(Debug, Default)]
pub struct RasterastReviewer;

impl RasterastReviewer {
    pub fn new() -> Self {
        Self
    }

    /// Toplanmış uzman sonuçlarından ön Rasterast raporu üretir.
    pub fn review(
        &self,
        collection: &AgentResultCollection,
    ) -> RasterastReport {
        let mut verified_items = Vec::new();
        let mut unverified_items = Vec::new();
        let mut contradictions = Vec::new();
        let mut risks = Vec::new();

        for result in &collection.results {
            let provider_id = result.provider.provider_id();

            if result.succeeded {
                verified_items.push(format!(
                    "{provider_id} sağlayıcısından işlenebilir bir çıktı alındı."
                ));

                unverified_items.push(format!(
                    "{provider_id} çıktısındaki bilimsel iddialar henüz kaynak ve içerik bakımından doğrulanmadı."
                ));
            } else {
                let error = result
                    .error
                    .as_deref()
                    .unwrap_or("Bilinmeyen sağlayıcı hatası.");

                risks.push(format!(
                    "{provider_id} sağlayıcısı başarısız oldu: {error}"
                ));
            }
        }

        if collection.results.is_empty() {
            risks.push(
                "Rasterast incelemesine gönderilmiş hiçbir uzman sonucu bulunmuyor."
                    .to_string(),
            );
        }

        if collection.duplicate_provider_count > 0 {
            contradictions.push(format!(
                "{} tekrar eden sağlayıcı sonucu tespit edildi; son kayıt korunmuştur.",
                collection.duplicate_provider_count
            ));
        }

        if collection.failed_provider_count > 0 {
            risks.push(format!(
                "{} sağlayıcı görevi tamamlayamadı.",
                collection.failed_provider_count
            ));
        }

        let has_all_required_results =
            collection.result_count() == 4
                && collection.failed_provider_count == 0
                && collection.duplicate_provider_count == 0;

        let verified = has_all_required_results
            && unverified_items.is_empty()
            && contradictions.is_empty()
            && risks.is_empty();

        let requires_mudebbir_decision =
            !risks.is_empty()
                || !contradictions.is_empty()
                || !unverified_items.is_empty();

        RasterastReport {
            task_id: collection.mira_task_id,
            verified,
            verified_items,
            unverified_items,
            contradictions,
            risks,
            requires_mudebbir_decision,
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider_bridge::MiraProvider;
    use crate::provider_executor::ProviderExecutionResult;
    use crate::result_collector::ResultCollector;
    use chrono::Utc;
    use serde_json::json;
    use uuid::Uuid;
    use zanistarast_core::provider::ProviderMetadata;
    use zanistarast_core::{CssId, ScientificObject};

    fn test_object(title: &str) -> ScientificObject {
        ScientificObject {
            css_id: CssId(format!(
                "rasterast-{}",
                title.to_lowercase().replace(' ', "-")
            )),
            title: title.to_string(),
            version: "0.1.0".to_string(),
            created_at: Utc::now(),
            payload: json!({
                "analysis": title
            }),
        }
    }

    fn successful_result(
        provider: MiraProvider,
        title: &str,
    ) -> ProviderExecutionResult {
        ProviderExecutionResult {
            provider,
            provider_metadata: ProviderMetadata::new(),
            output: Some(test_object(title)),
            error: None,
        }
    }

    fn failed_result(
        provider: MiraProvider,
        error: &str,
    ) -> ProviderExecutionResult {
        ProviderExecutionResult {
            provider,
            provider_metadata: ProviderMetadata::new(),
            output: None,
            error: Some(error.to_string()),
        }
    }

    #[test]
    fn successful_outputs_remain_unverified_until_content_review() {
        let task_id = Uuid::new_v4();
        let collector = ResultCollector::new();

        let collection = collector.collect(
            task_id,
            vec![successful_result(
                MiraProvider::OpenAi,
                "Akademik yapı sonucu",
            )],
        );

        let reviewer = RasterastReviewer::new();
        let report = reviewer.review(&collection);

        assert_eq!(report.task_id, task_id);
        assert!(!report.verified);
        assert_eq!(report.verified_items.len(), 1);
        assert_eq!(report.unverified_items.len(), 1);
        assert!(report.requires_mudebbir_decision);
    }

    #[test]
    fn failed_provider_is_recorded_as_risk() {
        let collector = ResultCollector::new();

        let collection = collector.collect(
            Uuid::new_v4(),
            vec![failed_result(
                MiraProvider::Ollama,
                "Yerel runtime kullanılamıyor.",
            )],
        );

        let reviewer = RasterastReviewer::new();
        let report = reviewer.review(&collection);

        assert!(!report.risks.is_empty());
        assert!(!report.verified);
        assert!(report.requires_mudebbir_decision);
    }

    #[test]
    fn duplicate_provider_result_is_recorded() {
        let collector = ResultCollector::new();

        let collection = collector.collect(
            Uuid::new_v4(),
            vec![
                successful_result(
                    MiraProvider::Gemini,
                    "İlk sınıflandırma",
                ),
                successful_result(
                    MiraProvider::Gemini,
                    "İkinci sınıflandırma",
                ),
            ],
        );

        let reviewer = RasterastReviewer::new();
        let report = reviewer.review(&collection);

        assert_eq!(report.contradictions.len(), 1);
        assert!(report.requires_mudebbir_decision);
    }

    #[test]
    fn empty_collection_cannot_be_verified() {
        let collector = ResultCollector::new();

        let collection = collector.collect(
            Uuid::new_v4(),
            Vec::new(),
        );

        let reviewer = RasterastReviewer::new();
        let report = reviewer.review(&collection);

        assert!(!report.verified);
        assert!(!report.risks.is_empty());
        assert!(report.requires_mudebbir_decision);
    }
}



