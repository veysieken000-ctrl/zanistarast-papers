use zanistarast_ai::ollama_provider::OllamaProvider;

#[tokio::test]
#[ignore = "requires a running Ollama server and a downloaded model"]
async fn ollama_live_request_returns_response() {
    let endpoint = std::env::var("OLLAMA_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:11434".to_string());

    let model = std::env::var("OLLAMA_MODEL")
        .unwrap_or_else(|_| "tinyllama".to_string());

    let provider = OllamaProvider::with_config(
        Some(endpoint),
        Some(model),
    );

    let response = provider
        .generate(
            "Explain deterministic scientific verification in one short sentence.",
        )
        .await
        .expect("real Ollama request should succeed");

    assert!(!response.trim().is_empty());
}


