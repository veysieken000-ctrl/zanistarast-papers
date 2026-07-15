use crate::provider_bridge::MiraProvider;
use crate::provider_executor::ProviderExecutionResult;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;
use zanistarast_core::provider::ProviderMetadata;
use zanistarast_core::ScientificObject;

/// Tek bir uzman sağlayıcıdan toplanan standartlaştırılmış sonuç.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectedProviderResult {
    pub provider: MiraProvider,
    pub provider_metadata: ProviderMetadata,
    pub succeeded: bool,
    pub output: Option<ScientificObject>,
    pub error: Option<String>,
}

/// Mira'nın aynı görev için topladığı birleşik uzman sonuç raporu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResultCollection {
    pub mira_task_id: Uuid,
    pub results: Vec<CollectedProviderResult>,
    pub successful_provider_count: usize,
    pub failed_provider_count: usize,
    pub duplicate_provider_count: usize,
    pub requires_rasterast_review: bool,
}

impl AgentResultCollection {
    /// Toplanan toplam uzman sonucunu döndürür.
    pub fn result_count(&self) -> usize {
        self.results.len()
    }

    /// Belirli bir provider'ın sonucunu döndürür.
    pub fn result_for_provider(
        &self,
        provider: MiraProvider,
    ) -> Option<&CollectedProviderResult> {
        self.results
            .iter()
            .find(|result| result.provider == provider)
    }

    /// Bütün aktif uzman sağlayıcıların başarılı olup olmadığını bildirir.
    pub fn all_providers_succeeded(&self) -> bool {
        self.failed_provider_count == 0
            && self.successful_provider_count == self.results.len()
    }
}

/// Uzman sağlayıcılardan gelen sonuçları tek bir raporda toplar.
///
/// Bu katman:
/// - uzman sonuçlarını değiştirmez,
/// - başarısızlıkları gizlemez,
/// - aynı provider'dan gelen tekrarları tespit eder,
/// - sonuçları doğrulanmış gerçek olarak kabul etmez,
/// - birleşik raporu Rasterast incelemesine hazırlar.
#[derive(Debug, Default)]
pub struct ResultCollector;

impl ResultCollector {
    pub fn new() -> Self {
        Self
    }

    /// Aynı Mira görevi için gelen provider sonuçlarını birleştirir.
    pub fn collect(
        &self,
        mira_task_id: Uuid,
        execution_results: Vec<ProviderExecutionResult>,
    ) -> AgentResultCollection {
        let mut unique_results =
            BTreeMap::<String, CollectedProviderResult>::new();

        let mut duplicate_provider_count = 0;

        for result in execution_results {
            let provider_id = result.provider.provider_id().to_string();

            let collected = CollectedProviderResult {
                provider: result.provider,
                provider_metadata: result.provider_metadata,
                succeeded: result.succeeded(),
                output: result.output,
                error: result.error,
            };

            if unique_results
                .insert(provider_id, collected)
                .is_some()
            {
                duplicate_provider_count += 1;
            }
        }

        let results = unique_results.into_values().collect::<Vec<_>>();

        let successful_provider_count = results
            .iter()
            .filter(|result| result.succeeded)
            .count();

        let failed_provider_count =
            results.len() - successful_provider_count;

        AgentResultCollection {
            mira_task_id,
            results,
            successful_provider_count,
            failed_provider_count,
            duplicate_provider_count,
            requires_rasterast_review: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;
    use zanistarast_core::CssId;

    fn test_object(title: &str) -> ScientificObject {
        ScientificObject {
            css_id: CssId(format!(
                "mira-result-{}",
                title.to_lowercase().replace(' ', "-")
            )),
            title: title.to_string(),
            version: "0.1.0".to_string(),
            created_at: Utc::now(),
            payload: json!({
                "result": title
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
    fn collector_combines_successful_and_failed_results() {
        let collector = ResultCollector::new();
        let mira_task_id = Uuid::new_v4();

        let report = collector.collect(
            mira_task_id,
            vec![
                successful_result(
                    MiraProvider::OpenAi,
                    "OpenAI sonucu",
                ),
                successful_result(
                    MiraProvider::Gemini,
                    "Gemini sonucu",
                ),
                failed_result(
                    MiraProvider::Ollama,
                    "Ollama runtime kullanılamıyor.",
                ),
                successful_result(
                    MiraProvider::LlamaCpp,
                    "llama.cpp sonucu",
                ),
            ],
        );

        assert_eq!(report.mira_task_id, mira_task_id);
        assert_eq!(report.result_count(), 4);
        assert_eq!(report.successful_provider_count, 3);
        assert_eq!(report.failed_provider_count, 1);
        assert!(report.requires_rasterast_review);
        assert!(!report.all_providers_succeeded());
    }

    #[test]
    fn collector_allows_provider_result_lookup() {
        let collector = ResultCollector::new();

        let report = collector.collect(
            Uuid::new_v4(),
            vec![successful_result(
                MiraProvider::Gemini,
                "Gemini sınıflandırması",
            )],
        );

        let result = report
            .result_for_provider(MiraProvider::Gemini)
            .expect("Gemini result should exist");

        assert!(result.succeeded);
        assert!(result.output.is_some());
    }

    #[test]
    fn duplicate_provider_results_are_detected() {
        let collector = ResultCollector::new();

        let report = collector.collect(
            Uuid::new_v4(),
            vec![
                successful_result(
                    MiraProvider::OpenAi,
                    "İlk OpenAI sonucu",
                ),
                successful_result(
                    MiraProvider::OpenAi,
                    "İkinci OpenAI sonucu",
                ),
            ],
        );

        assert_eq!(report.result_count(), 1);
        assert_eq!(report.duplicate_provider_count, 1);
    }

    #[test]
    fn all_providers_succeeded_requires_no_failure() {
        let collector = ResultCollector::new();

        let report = collector.collect(
            Uuid::new_v4(),
            vec![
                successful_result(
                    MiraProvider::OpenAi,
                    "OpenAI sonucu",
                ),
                successful_result(
                    MiraProvider::Gemini,
                    "Gemini sonucu",
                ),
            ],
        );

        assert!(report.all_providers_succeeded());
    }
}


