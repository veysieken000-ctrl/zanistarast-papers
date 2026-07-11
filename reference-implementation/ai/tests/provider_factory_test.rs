use zanistarast_ai::configured_provider::ConfiguredProviderFactory;
use zanistarast_ai::provider_config::ProviderConfig;

#[test]
fn creates_provider_from_config() {
    let config = ProviderConfig::load_default().unwrap();

    let provider = ConfiguredProviderFactory::create(&config);

    assert!(provider.is_some());
}


