use serde::{Deserialize, Serialize};

use zanistarast_core::provider::{
    ProviderError,
    ProviderMetadata,
    ScientificProvider,
};
use zanistarast_core::ScientificObject;

#[derive(Debug, Clone, Serialize)]
struct LlamaCppChatRequest {
    model: String,
    messages: Vec<LlamaCppMessage>,
    stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LlamaCppMessage {
    role: String,
    content: String,
}

#[derive(Debug, Clone, Deserialize)]
struct LlamaCppChatResponse {
    choices: Vec<LlamaCppChoice>,
}

#[derive(Debug, Clone, Deserialize)]
struct LlamaCppChoice {
    message: LlamaCppMessage,
}

pub struct LlamaCppProvider {
    endpoint: String,
    model: String,
}

impl LlamaCppProvider {
    pub fn new() -> Self {
        Self {
            endpoint: "http://localhost:8080".to_string(),
            model: "local-model".to_string(),
        }
    }

    pub fn with_config(
        endpoint: Option<String>,
        model: Option<String>,
    ) -> Self {
        Self {
            endpoint: endpoint
                .unwrap_or_else(|| "http://localhost:8080".to_string()),
            model: model
                .unwrap_or_else(|| "local-model".to_string()),
        }
    }

    pub async fn generate(
        &self,
        prompt: &str,
    ) -> Result<String, ProviderError> {
        let client = reqwest::Client::new();

        let request = LlamaCppChatRequest {
            model: self.model.clone(),
            messages: vec![LlamaCppMessage {
                role: "user".to_string(),
                content: prompt.to_string(),
            }],
            stream: false,
        };

        let url = format!(
            "{}/v1/chat/completions",
            self.endpoint.trim_end_matches('/')
        );

        let response = client
            .post(url)
            .json(&request)
            .send()
            .await
            .map_err(|error| {
                ProviderError::Internal(format!(
                    "llama.cpp request failed: {error}"
                ))
            })?;

        let status = response.status();

        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| {
                    "response body unavailable".to_string()
                });

            return Err(ProviderError::Internal(format!(
                "llama.cpp API returned HTTP {status}: {body}"
            )));
        }

        let body = response
            .json::<LlamaCppChatResponse>()
            .await
            .map_err(|error| {
                ProviderError::Internal(format!(
                    "llama.cpp response parsing failed: {error}"
                ))
            })?;

        body.choices
            .into_iter()
            .next()
            .map(|choice| choice.message.content)
            .filter(|content| !content.trim().is_empty())
            .ok_or_else(|| {
                ProviderError::Internal(
                    "llama.cpp response did not contain generated text"
                        .to_string(),
                )
            })
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
                "llamacpp_response".to_string(),
                serde_json::json!(response_text),
            );
            payload.insert(
                "llamacpp_model".to_string(),
                serde_json::json!(self.model.clone()),
            );
        } else {
            output.payload = serde_json::json!({
                "original_payload": object.payload.clone(),
                "llamacpp_response": response_text,
                "llamacpp_model": self.model.clone()
            });
        }

        Ok(output)
    }
}

impl ScientificProvider for LlamaCppProvider {
    fn id(&self) -> &'static str {
        "llama.cpp"
    }

    fn name(&self) -> &'static str {
        "Zanistarast llama.cpp Provider"
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
            "local-ai-provider".to_string(),
        );
        metadata.insert(
            "provider".to_string(),
            "llama.cpp".to_string(),
        );
        metadata.insert(
            "endpoint".to_string(),
            self.endpoint.clone(),
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
            "true".to_string(),
        );

        metadata
    }
}

impl Default for LlamaCppProvider {
    fn default() -> Self {
        Self::new()
    }
}



