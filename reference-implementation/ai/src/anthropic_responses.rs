use serde::{Deserialize, Serialize};

use zanistarast_core::provider::ProviderError;

const ANTHROPIC_MESSAGES_URL: &str = "https://api.anthropic.com/v1/messages";
const ANTHROPIC_API_VERSION: &str = "2023-06-01";

#[derive(Debug, Clone, Serialize)]
pub struct AnthropicMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AnthropicMessageRequest {
    pub model: String,
    pub max_tokens: u32,
    pub messages: Vec<AnthropicMessage>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnthropicMessageResponse {
    #[serde(default)]
    pub content: Vec<AnthropicContentBlock>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AnthropicContentBlock {
    #[serde(rename = "type")]
    pub content_type: String,

    #[serde(default)]
    pub text: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AnthropicMessagesClient {
    api_key: String,
    model: String,
    endpoint: String,
}

impl AnthropicMessagesClient {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            endpoint: ANTHROPIC_MESSAGES_URL.to_string(),
        }
    }

    pub fn with_endpoint(
        api_key: String,
        model: String,
        endpoint: String,
    ) -> Self {
        Self {
            api_key,
            model,
            endpoint,
        }
    }

    pub async fn generate(
        &self,
        input: &str,
    ) -> Result<String, ProviderError> {
        let client = reqwest::Client::new();

        let request = AnthropicMessageRequest {
            model: self.model.clone(),
            max_tokens: 512,
            messages: vec![AnthropicMessage {
                role: "user".to_string(),
                content: input.to_string(),
            }],
        };

        let response = client
            .post(&self.endpoint)
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", ANTHROPIC_API_VERSION)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|error| {
                ProviderError::Internal(format!(
                    "Anthropic request failed: {error}"
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
                "Anthropic API returned HTTP {status}: {body}"
            )));
        }

        let body = response
            .json::<AnthropicMessageResponse>()
            .await
            .map_err(|error| {
                ProviderError::Internal(format!(
                    "Anthropic response parsing failed: {error}"
                ))
            })?;

        body.content
            .iter()
            .find(|content| {
                content.content_type == "text"
                    && content.text.is_some()
            })
            .and_then(|content| content.text.clone())
            .filter(|text| !text.trim().is_empty())
            .ok_or_else(|| {
                ProviderError::Internal(
                    "Anthropic response did not contain text"
                        .to_string(),
                )
            })
    }
}
