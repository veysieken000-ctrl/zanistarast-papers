use crate::provider_bridge::{MiraProvider, ProviderBridge};
use serde::{Deserialize, Serialize};
use zanistarast_core::provider::ProviderMetadata;
use zanistarast_core::ScientificObject;

/// Mira'nın bir uzman sağlayıcıya göndereceği yürütme isteği.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderExecutionRequest {
    pub provider: MiraProvider,
    pub object: ScientificObject,
}

impl ProviderExecutionRequest {
    pub fn new(
        provider: MiraProvider,
        object: ScientificObject,
    ) -> Self {
        Self { provider, object }
    }
}

/// Uzman sağlayıcının Mira'ya döndürdüğü sonuç.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderExecutionResult {
    pub provider: MiraProvider,
    pub provider_metadata: ProviderMetadata,
    pub output: Option<ScientificObject>,
    pub error: Option<String>,
}

impl ProviderExecutionResult {
    pub fn succeeded(&self) -> bool {
        self.output.is_some() && self.error.is_none()
    }
}

/// Mira'nın mevcut AI sağlayıcılarını çalıştıran katman.
///
/// Bu katman:
/// - API anahtarı saklamaz,
/// - Provider Factory üzerinden sağlayıcı oluşturur,
/// - bilimsel nesneyi sağlayıcıya gönderir,
/// - sonucu veya hatayı Mira'ya raporlar,
/// - yayınlama, merge veya dosya değiştirme işlemi yapmaz.
#[derive(Debug, Default)]
pub struct ProviderExecutor {
    bridge: ProviderBridge,
}

impl ProviderExecutor {
    pub fn new() -> Self {
        Self {
            bridge: ProviderBridge::new(),
        }
    }

    /// Tek bir uzman sağlayıcı üzerinde görevi yürütür.
    pub fn execute(
        &self,
        request: ProviderExecutionRequest,
    ) -> ProviderExecutionResult {
        let Some(provider) =
            self.bridge.create_provider(request.provider)
        else {
            return ProviderExecutionResult {
                provider: request.provider,
                provider_metadata: ProviderMetadata::new(),
                output: None,
                error: Some(format!(
                    "Provider oluşturulamadı: {}",
                    request.provider.provider_id()
                )),
            };
        };

        let provider_metadata = provider.metadata();

        match provider.execute(&request.object) {
            Ok(output) => ProviderExecutionResult {
                provider: request.provider,
                provider_metadata,
                output: Some(output),
                error: None,
            },
            Err(error) => ProviderExecutionResult {
                provider: request.provider,
                provider_metadata,
                output: None,
                error: Some(format!("{error:?}")),
            },
        }
    }

    /// Sağlayıcıyı çalıştırmadan yalnızca metadata bilgisini döndürür.
    pub fn metadata(
        &self,
        provider: MiraProvider,
    ) -> Option<ProviderMetadata> {
        self.bridge
            .create_provider(provider)
            .map(|provider| provider.metadata())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;
    use zanistarast_core::CssId;

    fn test_object() -> ScientificObject {
        ScientificObject {
            css_id: CssId("mira-test-object".to_string()),
            title: "Mira Provider Test".to_string(),
            version: "0.1.0".to_string(),
            created_at: Utc::now(),
            payload: json!({
                "instruction": "Provider görev yürütme katmanını doğrula."
            }),
        }
    }

    #[test]
    fn execution_request_preserves_provider_and_object() {
        let object = test_object();

        let request = ProviderExecutionRequest::new(
            MiraProvider::OpenAi,
            object,
        );

        assert_eq!(request.provider, MiraProvider::OpenAi);
        assert_eq!(request.object.title, "Mira Provider Test");
    }

    #[test]
    fn executor_reads_active_provider_metadata_without_execution() {
        let executor = ProviderExecutor::new();

        for provider in [
            MiraProvider::OpenAi,
            MiraProvider::Gemini,
            MiraProvider::Ollama,
            MiraProvider::LlamaCpp,
        ] {
            assert!(
                executor.metadata(provider).is_some(),
                "metadata should exist for {}",
                provider.provider_id()
            );
        }
    }

    #[test]
    fn success_requires_output_without_error() {
        let result = ProviderExecutionResult {
            provider: MiraProvider::OpenAi,
            provider_metadata: ProviderMetadata::new(),
            output: Some(test_object()),
            error: None,
        };

        assert!(result.succeeded());
    }
}





