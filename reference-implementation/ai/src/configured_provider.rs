use crate::ollama_provider::OllamaProvider;
use crate::provider_config::ProviderConfig;
use crate::provider_factory::AiProviderFactory;
use zanistarast_core::provider::ScientificProvider;

pub struct ConfiguredProviderFactory;

impl ConfiguredProviderFactory {
    pub fn create(config: &ProviderConfig) -> Option<Box<dyn ScientificProvider>> {
        let provider_id = config.default_provider.as_str();

        if !config.is_enabled(provider_id) {
            return None;
        }

        let provider_config = config.providers.get(provider_id)?;

        match provider_id {
            "ollama" => Some(Box::new(OllamaProvider::with_config(
                provider_config.endpoint.clone(),
                provider_config.model.clone(),
            ))),
            _ => AiProviderFactory::create(provider_id),
        }
    }
}
