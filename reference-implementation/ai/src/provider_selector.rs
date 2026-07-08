use crate::provider_config::ProviderConfig;
use crate::provider_factory::AiProviderFactory;
use zanistarast_core::provider::ScientificProvider;

pub struct ProviderSelector;

impl ProviderSelector {
    pub fn select(config: &ProviderConfig) -> Option<Box<dyn ScientificProvider>> {
        if config.is_enabled(&config.default_provider) {
            return AiProviderFactory::create(&config.default_provider);
        }

        for provider_id in config.providers.keys() {
            if config.is_enabled(provider_id) {
                return AiProviderFactory::create(provider_id);
            }
        }

        None
    }
}
