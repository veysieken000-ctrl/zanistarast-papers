use zanistarast_ai::ollama_provider::OllamaProvider;

#[tokio::test]
async fn ollama_provider_has_metadata() {
    let provider = OllamaProvider::new();
    let metadata = provider.metadata();

    assert_eq!(metadata.get("provider"), Some(&"ollama".to_string()));
    assert_eq!(metadata.get("api_enabled"), Some(&"true".to_string()));
    assert_eq!(metadata.get("deterministic_wrapper"), Some(&"true".to_string()));
}


