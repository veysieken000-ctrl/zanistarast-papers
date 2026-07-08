use crate::provider_config::ProviderConfig;
use std::fs;
use std::path::Path;

pub struct ProviderConfigLoader;

impl ProviderConfigLoader {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<ProviderConfig> {
        let content = fs::read_to_string(path)?;
        let config: ProviderConfig = toml::from_str(&content)?;
        Ok(config)
    }
}

