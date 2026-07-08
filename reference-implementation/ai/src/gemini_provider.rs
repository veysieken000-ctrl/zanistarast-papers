use zanistarast_core::provider::{ProviderError, ProviderMetadata, ScientificProvider};
use zanistarast_core::ScientificObject;

pub struct GeminiProvider;

impl GeminiProvider {
    pub fn new() -> Self {
        Self
    }
}

impl ScientificProvider for GeminiProvider {
    fn id(&self) -> &'static str {
        "gemini"
    }

    fn name(&self) -> &'static str {
        "Zanistarast Gemini Provider"
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
        metadata.insert("provider".to_string(), "gemini".to_string());
        metadata.insert("deterministic_wrapper".to_string(), "true".to_string());
        metadata.insert("api_enabled".to_string(), "false".to_string());
        metadata
    }
}

impl Default for GeminiProvider {
    fn default() -> Self {
        Self::new()
    }
}


