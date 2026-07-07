use std::fs;
use std::path::Path;

use crate::config::RuntimeConfig;

/// Loads RuntimeConfig from a TOML file.
///
/// If the configuration file does not exist,
/// the default configuration is returned.
pub fn load_config<P: AsRef<Path>>(
    path: P,
) -> Result<RuntimeConfig, Box<dyn std::error::Error>> {
    let path = path.as_ref();

    if !path.exists() {
        return Ok(RuntimeConfig::default());
    }

    let text = fs::read_to_string(path)?;

    let config: RuntimeConfig = toml::from_str(&text)?;

    Ok(config)
}


