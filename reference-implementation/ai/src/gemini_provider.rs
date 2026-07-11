use serde_json::json;

use zanistarast_core::provider::{
    ProviderError,
    ProviderMetadata,
    ScientificProvider,
};
use zanistarast_core::ScientificObject;

use crate::api_key_manager::ApiKeyManager;
use crate::gemini_responses::GeminiGenerateClient;

pub struct GeminiProvider {
    model: String,
}

impl GeminiProvider {
    pub fn new() -> Self {
        let model = std::env::var("GEMINI_MODEL")
            .unwrap_or_else(|_| "gemini-3-flash-preview".to_string()
        
        Self { model }
    }

    pub fn with_model(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
        }
    }

    pub async fn generate(
        &self,
        prompt: &str,
    ) -> Result<String, ProviderError> {
        let api_key = ApiKeyManager::gemini().ok_or_else(|| {
            ProviderError::InvalidInput(
                "GEMINI_API_KEY environment variable is not configured"
                    .to_string(),
            )
        })?;

        let client =
            GeminiGenerateClient::new(api_key, self.model.clone());

        client.generate(prompt).await
    }

    pub async fn execute_remote(
        &self,
        object: &ScientificObject,
    ) -> Result<ScientificObject, ProviderError> {
        let prompt = object
            .payload
            .get("claim")
            .and_then(|value| value.as_str())
            .unwrap_or(&object.title);

        let response_text = self.generate(prompt).await?;

        let mut output = object.clone();

        if let Some(payload) = output.payload.as_object_mut() {
            payload.insert(
                "gemini_response".to_string(),
                json!(response_text),
            );
            payload.insert(
                "gemini_model".to_string(),
                json!(self.model),
            );
        } else {
            output.payload = json!({
                "original_payload": object.payload.clone(),
                "gemini_response": response_text,
                "gemini_model": self.model
            });
        }

        Ok(output)
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

        metadata.insert(
            "type".to_string(),
            "cloud-ai-provider".to_string(),
        );
        metadata.insert(
            "provider".to_string(),
            "gemini".to_string(),
        );
        metadata.insert(
            "model".to_string(),
            self.model.clone(),
        );
        metadata.insert(
            "deterministic_wrapper".to_string(),
            "true".to_string(),
        );
        metadata.insert(
            "api_enabled".to_string(),
            ApiKeyManager::gemini().is_some().to_string(),
        );

        metadata
    }
}

impl Default for GeminiProvider {
    fn default() -> Self {
        Self::new()
    }
}




