use crate::anthropic_provider::AnthropicProvider;
use crate::gemini_provider::GeminiProvider;
use crate::ollama_provider::OllamaProvider;
use crate::openai_provider::OpenAiProvider;
use crate::provider::NativeAiProvider;
use crate::llamacpp_provider::LlamaCppProvider;

use zanistarast_core::provider::ScientificProvider;

pub struct AiProviderFactory;

impl AiProviderFactory {
    pub fn create(id: &str) -> Option<Box<dyn ScientificProvider>> {
        match id {
            "native-ai" => Some(Box::new(NativeAiProvider::new())),
            "openai" => Some(Box::new(OpenAiProvider::new())),
            "anthropic" => Some(Box::new(AnthropicProvider::new())),
            "gemini" => Some(Box::new(GeminiProvider::new())),
            "ollama" => Some(Box::new(OllamaProvider::new())),
            "llama.cpp" => Some(Box::new(LlamaCppProvider::new())),
            _ => None,
        }
    }
}



