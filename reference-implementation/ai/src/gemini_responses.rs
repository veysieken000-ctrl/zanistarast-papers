use serde::{Deserialize, Serialize};

use zanistarast_core::provider::ProviderError;

const GEMINI_API_BASE_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models";

#[derive(Debug, Clone, Serialize)]
pub struct GeminiPart {
    pub text: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeminiContent {
    pub parts: Vec<GeminiPart>,
}

#[derive(Debug, Clone, Serialize)]
pub struct GeminiGenerateRequest {
    pub contents: Vec<GeminiContent>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GeminiGenerateResponse {
    #[serde(default)]
    pub candidates: Vec<GeminiCandidate>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GeminiCandidate {
    pub content: GeminiResponseContent,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GeminiResponseContent {
    #[serde(default)]
    pub parts: Vec<GeminiResponsePart>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct GeminiResponsePart {
    #[serde(default)]
    pub text: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GeminiGenerateClient {
    api_key: String,
    model: String,
    base_url: String,
}

impl GeminiGenerateClient {
    pub fn new(api_key: String, model: String) -> Self {
        Self {
            api_key,
            model,
            base_url: GEMINI_API_BASE_URL.to_string(),
        }
    }

    pub fn with_base_url(
        api_key: String,
        model: String,
        base_url: String,
    ) -> Self {
        Self {
            api_key,
            model,
            base_url,
        }
    }

    pub async fn generate(
        &self,
        input: &str,
    ) -> Result<String, ProviderError> {
        let client = reqwest::Client::new();

        let request = GeminiGenerateRequest {
            contents: vec![GeminiContent {
                parts: vec![GeminiPart {
                    text: input.to_string(),
                }],
            }],
        };

        let endpoint = format!(
            "{}/{}:generateContent",
            self.base_url, self.model
        );

        let response = client
            .post(endpoint)
            .header("x-goog-api-key", &self.api_key)
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|error| {
                ProviderError::Internal(format!(
                    "Gemini request failed: {error}"
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
                "Gemini API returned HTTP {status}: {body}"
            )));
        }

        let body = response
            .json::<GeminiGenerateResponse>()
            .await
            .map_err(|error| {
                ProviderError::Internal(format!(
                    "Gemini response parsing failed: {error}"
                ))
            })?;

        body.candidates
            .iter()
            .flat_map(|candidate| candidate.content.parts.iter())
            .find_map(|part| part.text.clone())
            .filter(|text| !text.trim().is_empty())
            .ok_or_else(|| {
                ProviderError::Internal(
                    "Gemini response did not contain text".to_string(),
                )
            })
    }
}


