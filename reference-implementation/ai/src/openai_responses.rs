use serde::{Deserialize, Serialize};
use zanistarast_core::provider::ProviderError;

const OPENAI_RESPONSES_URL: &str = "https://api.openai.com/v1/responses";

#[derive(Debug, Clone, Serialize)]
pub struct OpenAiResponseRequest {
    pub model: String,
    pub input: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenAiResponse {
    #[serde(default)]
    pub output: Vec<OpenAiOutputItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenAiOutputItem {
    #[serde(default)]
    pub content: Vec<OpenAiContentItem>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OpenAiContentItem {
    #[serde(rename = "type")]
    pub content_type: String,

    #[serde(default)]
    pub text: Option<String>,
}

#[derive(Debug, Clone)]
pub struct OpenAiResponsesClient {
    api_key: String,
    model: String,
    endpoint: String,
}

impl OpenAiResponsesClient {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            endpoint: OPENAI_RESPONSES_URL.to_string(),
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

        let request = OpenAiResponseRequest {
            model: self.model.clone(),
            input: input.to_string(),
        };

        let response = client
            .post(&self.endpoint)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await
            .map_err(|error| {
                ProviderError::Internal(format!(
                    "OpenAI request failed: {error}"
                ))
            })?;

        let status = response.status();

        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "response body unavailable".to_string());

            return Err(ProviderError::Internal(format!(
                "OpenAI API returned HTTP {status}: {body}"
            )));
        }

        let body = response
            .json::<OpenAiResponse>()
            .await
            .map_err(|error| {
                ProviderError::Internal(format!(
                    "OpenAI response parsing failed: {error}"
                ))
            })?;

        body.output
            .iter()
            .flat_map(|item| item.content.iter())
            .find(|content| {
                content.content_type == "output_text"
                    && content.text.is_some()
            })
            .and_then(|content| content.text.clone())
            .ok_or_else(|| {
                ProviderError::Internal(
                    "OpenAI response did not contain output_text".to_string(),
                )
            })
    }
}

