use zanistarast_ai::anthropic_provider::AnthropicProvider;
use zanistarast_core::provider::ScientificProvider;

#[test]
fn anthropic_provider_metadata() {
    let provider = AnthropicProvider::new();

    assert_eq!(provider.id(), "anthropic");
    assert_eq!(provider.name(), "Zanistarast Anthropic Provider");
    assert_eq!(provider.version(), "0.1.0");

    let metadata = provider.metadata();
    assert_eq!(metadata.get("provider").unwrap(), "anthropic");
}


