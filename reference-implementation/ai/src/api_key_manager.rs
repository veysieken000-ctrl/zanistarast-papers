use std::env;

pub struct ApiKeyManager;

impl ApiKeyManager {
    pub fn get(key_name: &str) -> Option<String> {
        env::var(key_name).ok()
    }

    pub fn openai() -> Option<String> {
        Self::get("OPENAI_API_KEY")
    }

    pub fn anthropic() -> Option<String> {
        Self::get("ANTHROPIC_API_KEY")
    }

    pub fn gemini() -> Option<String> {
        Self::get("GEMINI_API_KEY")
    }

    pub fn has(key_name: &str) -> bool {
        Self::get(key_name).is_some()
    }
}


