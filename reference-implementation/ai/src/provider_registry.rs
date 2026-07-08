use crate::anthropic_provider::AnthropicProvider;
use crate::gemini_provider::GeminiProvider;
use crate::ollama_provider::OllamaProvider;
use crate::openai_provider::OpenAiProvider;
use crate::provider::NativeAiProvider;
use zanistarast_core::provider::{ProviderMetadata, ScientificProvider};
use crate::llamacpp_provider::LlamaCppProvider;

pub struct AiProviderRegistry;

impl AiProviderRegistry {
    pub fn provider_ids() -> Vec<&'static str> {
        vec![
            "native-ai",
            "openai",
            "anthropic",
            "gemini",
            "ollama",
            "llama.cpp",

        ]
    }

    pub fn metadata_for(provider_id: &str) -> Option<ProviderMetadata> {
        match provider_id {
            "native-ai" => Some(NativeAiProvider::new().metadata()),
            "openai" => Some(OpenAiProvider::new().metadata()),
            "anthropic" => Some(AnthropicProvider::new().metadata()),
            "gemini" => Some(GeminiProvider::new().metadata()),
            "ollama" => Some(OllamaProvider::new().metadata()),
            "llama.cpp" => Some(LlamaCppProvider::new().metadata()),
            _ => None,
        }
    }

    pub fn contains(provider_id: &str) -> bool {
        Self::provider_ids().contains(&provider_id)
    }
}


