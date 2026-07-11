use crate::anthropic_responses::AnthropicMessagesClient;
use crate::api_key_manager::ApiKeyManager;

use zanistarast_core::provider::{ProviderError, ProviderMetadata, ScientificProvider};
use zanistarast_core::ScientificObject;

pub struct AnthropicProvider {
    client: Option<AnthropicMessagesClient>,
}

impl AnthropicProvider {
    pub fn new() -> Self {
        let client = ApiKeyManager::anthropic().map(|api_key| {
            AnthropicMessagesClient::new(
                api_key,
                "claude-3-5-haiku-latest".to_string(),
            )
        });

        Self { client }
    }
}

impl ScientificProvider for AnthropicProvider {
    fn id(&self) -> &'static str {
        "anthropic"
    }

    fn name(&self) -> &'static str {
        "Zanistarast Anthropic Provider"
    }

    fn version(&self) -> &'static str {
        "0.1.0"
    }

    fn execute(
        &self,
        object: &ScientificObject,
    ) -> Result<ScientificObject, ProviderError> {
        Ok(object.clone())
    }

    fn metadata(&self) -> ProviderMetadata {
        let mut metadata = ProviderMetadata::new();
        metadata.insert("type".to_string(), "cloud-ai-provider".to_string());
        metadata.insert("provider".to_string(), "anthropic".to_string());
        metadata.insert("deterministic_wrapper".to_string(), "true".to_string());
        metadata.insert(
            "api_enabled".to_string(),
            self.client.is_some().to_string(),
        );
        metadata
    }
}

impl Default for AnthropicProvider {
    fn default() -> Self {
        Self::new()
    }
}


