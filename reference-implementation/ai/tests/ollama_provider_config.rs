use zanistarast_ai::ollama_provider::OllamaProvider;
use zanistarast_core::provider::ScientificProvider;

#[test]
fn ollama_provider_accepts_custom_config() {
    let provider = OllamaProvider::with_config(
        Some("http://localhost:11434".to_string()),
        Some("llama3".to_string()),
    );

    let metadata = provider.metadata();

    assert_eq!(
        metadata.get("endpoint"),
        Some(&"http://localhost:11434".to_string())
    );
    assert_eq!(metadata.get("model"), Some(&"llama3".to_string()));
}


