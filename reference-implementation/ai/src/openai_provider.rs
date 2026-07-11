use serde_json::json;

use zanistarast_core::provider::{
    ProviderError,
    ProviderMetadata,
    ScientificProvider,
};
use zanistarast_core::ScientificObject;

use crate::api_key_manager::ApiKeyManager;
use crate::openai_responses::OpenAiResponsesClient;

pub struct OpenAiProvider {
    model: String,
}

impl OpenAiProvider {
    pub fn new() -> Self {
        Self {
            model: "gpt-5".to_string(),
        }
    }

    pub fn with_model(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
        }
    }

    /// Sends a real request to the OpenAI Responses API.
    ///
    /// The API key is read from the OPENAI_API_KEY environment variable.
    /// The key is never stored in the source code.
    pub async fn generate(
        &self,
        prompt: &str,
    ) -> Result<String, ProviderError> {
        let api_key = ApiKeyManager::openai().ok_or_else(|| {
            ProviderError::InvalidInput(
                "OPENAI_API_KEY environment variable is not configured"
                    .to_string(),
            )
        })?;

        let client =
            OpenAiResponsesClient::new(api_key, self.model.clone());

        client.generate(prompt).await
    }

    /// Sends the scientific object to OpenAI and places the returned text
    /// inside a cloned ScientificObject.
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
                "openai_response".to_string(),
                json!(response_text),
            );
            payload.insert(
                "openai_model".to_string(),
                json!(self.model),
            );
        } else {
            output.payload = json!({
                "original_payload": object.payload.clone(),
                "openai_response": response_text,
                "openai_model": self.model
            });
        }

        Ok(output)
    }
}

impl ScientificProvider for OpenAiProvider {
    fn id(&self) -> &'static str {
        "openai"
    }

    fn name(&self) -> &'static str {
        "Zanistarast OpenAI Provider"
    }

    fn version(&self) -> &'static str {
        "0.1.0"
    }

    /// The shared ScientificProvider interface is currently synchronous.
    ///
    /// Real OpenAI network execution is therefore provided through
    /// execute_remote(), which is asynchronous.
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
            "openai".to_string(),
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
            ApiKeyManager::openai().is_some().to_string(),
        );

        metadata
    }
}

impl Default for OpenAiProvider {
    fn default() -> Self {
        Self::new()
    }
}


