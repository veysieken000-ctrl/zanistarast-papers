use zanistarast_ai::ollama_provider::OllamaGenerateRequest;

#[test]
fn ollama_generate_request_is_serializable() {
    let request = OllamaGenerateRequest {
        model: "llama3".to_string(),
        prompt: "Hebûn test".to_string(),
        stream: false,
    };

    let json = serde_json::to_value(request).expect("request should serialize");

    assert_eq!(json["model"], "llama3");
    assert_eq!(json["prompt"], "Hebûn test");
    assert_eq!(json["stream"], false);
}



