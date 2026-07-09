use serde::{Deserialize, Serialize};
use zanistarast_core::provider::{ProviderError, ProviderMetadata, ScientificProvider};
use zanistarast_core::ScientificObject;

#[derive(Debug, Clone, Serialize)]
pub struct OllamaGenerateRequest {
    pub model: String,
    pub prompt: String,
    pub stream: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OllamaGenerateResponse {
    pub response: String,
    pub done: bool,
}

pub struct OllamaProvider {
    endpoint: String,
    model: String,
}

impl OllamaProvider {
    pub fn new() -> Self {
        Self {
            endpoint: "http://localhost:11434".to_string(),
            model: "llama3".to_string(),
        }
    }

pub fn with_config(endpoint: Option<String>, model: Option<String>) -> Self {
    Self {
        endpoint: endpoint.unwrap_or_else(|| "http://localhost:11434".to_string()),
        model: model.unwrap_or_else(|| "llama3".to_string()),
    }
}
    
    pub async fn generate(&self, prompt: &str) -> Result<String, ProviderError> {
        let client = reqwest::Client::new();

        let request = OllamaGenerateRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let url = format!("{}/api/generate", self.endpoint);

        let response = client
            .post(url)
            .json(&request)
            .send()
            .await
            .map_err(|error| ProviderError::Internal(error.to_string()))?;

        let body = response
            .json::<OllamaGenerateResponse>()
            .await
            .map_err(|error| ProviderError::Internal(error.to_string()))?;

        Ok(body.response)
    }
}

impl ScientificProvider for OllamaProvider {
    fn id(&self) -> &'static str {
        "ollama"
    }

    fn name(&self) -> &'static str {
        "Zanistarast Ollama Provider"
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
        metadata.insert("type".to_string(), "local-ai-provider".to_string());
        metadata.insert("provider".to_string(), "ollama".to_string());
        metadata.insert("endpoint".to_string(), self.endpoint.clone());
        metadata.insert("model".to_string(), self.model.clone());
        metadata.insert("deterministic_wrapper".to_string(), "true".to_string());
        metadata.insert("api_enabled".to_string(), "true".to_string());
        metadata
    }
}

impl Default for OllamaProvider {
    fn default() -> Self {
        Self::new()
    }
}



