use std::env;

pub struct ApiKeyManager;

impl ApiKeyManager {
    pub fn openai() -> Option<String> {
        env::var("OPENAI_API_KEY").ok()
    }

    pub fn anthropic() -> Option<String> {
        env::var("ANTHROPIC_API_KEY").ok()
    }

    pub fn gemini() -> Option<String> {
        env::var("GEMINI_API_KEY").ok()
    }

    pub fn has_openai() -> bool {
        Self::openai().is_some()
    }

    pub fn has_anthropic() -> bool {
        Self::anthropic().is_some()
    }

    pub fn has_gemini() -> bool {
        Self::gemini().is_some()
    }
}
