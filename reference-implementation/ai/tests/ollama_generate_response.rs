use zanistarast_ai::ollama_provider::OllamaGenerateResponse;

#[test]
fn ollama_generate_response_is_deserializable() {
    let raw = r#"
    {
        "response": "Hebûn response test",
        "done": true
    }
    "#;

    let response: OllamaGenerateResponse =
        serde_json::from_str(raw).expect("response should deserialize");

    assert_eq!(response.response, "Hebûn response test");
    assert!(response.done);
}


