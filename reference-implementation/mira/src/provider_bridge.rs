use serde::{Deserialize, Serialize};
use zanistarast_ai::provider_factory::AiProviderFactory;
use zanistarast_core::provider::ScientificProvider;

/// Mira tarafından aktif olarak kullanılacak uzman sağlayıcılar.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum MiraProvider {
    OpenAi,
    Gemini,
    Ollama,
    LlamaCpp,
}

impl MiraProvider {
    /// Provider Factory tarafından kullanılan kimliği döndürür.
    pub fn provider_id(self) -> &'static str {
        match self {
            Self::OpenAi => "openai",
            Self::Gemini => "gemini",
            Self::Ollama => "ollama",
            Self::LlamaCpp => "llama.cpp",
        }
    }
}

/// Mira ile mevcut AI provider altyapısı arasındaki bağlantı katmanı.
///
/// Bu ilk sürüm:
/// - aktif provider kimliklerini doğrular,
/// - mevcut Provider Factory üzerinden provider oluşturur,
/// - API anahtarlarını saklamaz,
/// - doğrudan yayın veya kalıcı işlem yapmaz.
#[derive(Debug, Default)]
pub struct ProviderBridge;

impl ProviderBridge {
    pub fn new() -> Self {
        Self
    }

    /// Seçilen uzman provider'ı mevcut AI Factory üzerinden oluşturur.
    pub fn create_provider(
        &self,
        provider: MiraProvider,
    ) -> Option<Box<dyn ScientificProvider>> {
        AiProviderFactory::create(provider.provider_id())
    }

    /// Mira için tanımlı bütün aktif provider'ları döndürür.
    pub fn active_providers(&self) -> [MiraProvider; 4] {
        [
            MiraProvider::OpenAi,
            MiraProvider::Gemini,
            MiraProvider::Ollama,
            MiraProvider::LlamaCpp,
        ]
    }

    /// Provider'ın mevcut factory tarafından oluşturulabildiğini doğrular.
    pub fn is_available(&self, provider: MiraProvider) -> bool {
        self.create_provider(provider).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bridge_contains_only_active_mira_providers() {
        let bridge = ProviderBridge::new();
        let providers = bridge.active_providers();

        assert_eq!(providers.len(), 4);
        assert!(providers.contains(&MiraProvider::OpenAi));
        assert!(providers.contains(&MiraProvider::Gemini));
        assert!(providers.contains(&MiraProvider::Ollama));
        assert!(providers.contains(&MiraProvider::LlamaCpp));
    }

    #[test]
    fn provider_ids_match_existing_factory_ids() {
        assert_eq!(MiraProvider::OpenAi.provider_id(), "openai");
        assert_eq!(MiraProvider::Gemini.provider_id(), "gemini");
        assert_eq!(MiraProvider::Ollama.provider_id(), "ollama");
        assert_eq!(MiraProvider::LlamaCpp.provider_id(), "llama.cpp");
    }

    #[test]
    fn bridge_creates_all_active_providers() {
        let bridge = ProviderBridge::new();

        for provider in bridge.active_providers() {
            assert!(
                bridge.is_available(provider),
                "provider should be available: {}",
                provider.provider_id()
            );
        }
    }
}

