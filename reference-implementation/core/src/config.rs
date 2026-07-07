use serde::{Deserialize, Serialize};

/// Global configuration for the Zanistarast runtime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeConfig {
    pub runtime: RuntimeSection,
    pub verification: VerificationSection,
    pub provider: ProviderSection,
    pub logging: LoggingSection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeSection {
    pub deterministic: bool,
    pub replay_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationSection {
    pub strict_mode: bool,
    pub stop_on_failure: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderSection {
    pub default_provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSection {
    pub level: String,
}

impl Default for RuntimeConfig {
    fn default() -> Self {
        Self {
            runtime: RuntimeSection {
                deterministic: true,
                replay_enabled: true,
            },
            verification: VerificationSection {
                strict_mode: true,
                stop_on_failure: true,
            },
            provider: ProviderSection {
                default_provider: "native-ai".to_string(),
            },
            logging: LoggingSection {
                level: "info".to_string(),
            },
        }
    }
}




