use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    pub default_provider: String,
    pub providers: HashMap<String, SingleProviderConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SingleProviderConfig {
    pub enabled: bool,
    pub model: Option<String>,
    pub endpoint: Option<String>,
    pub api_key_env: Option<String>,
}

impl ProviderConfig {
    pub fn is_enabled(&self, provider_id: &str) -> bool {
        self.providers
            .get(provider_id)
            .map(|provider| provider.enabled)
            .unwrap_or(false)
    }
}

