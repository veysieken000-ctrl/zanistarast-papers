use std::path::PathBuf;

use zanistarast_ai::configured_provider::ConfiguredProviderFactory;
use zanistarast_ai::provider_loader::ProviderConfigLoader;

#[test]
fn creates_provider_from_config() {
    let config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../config/providers.toml");

    let config = ProviderConfigLoader::load_from_file(&config_path)
        .expect("providers.toml should load successfully");

    let provider = ConfiguredProviderFactory::create(&config);

    assert!(provider.is_some());
}


