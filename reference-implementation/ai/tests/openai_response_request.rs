use zanistarast_ai::openai_responses::OpenAiResponseRequest;

#[test]
fn openai_response_request_is_serializable() {
    let request = OpenAiResponseRequest {
        model: "gpt-5".to_string(),
        input: "Hebûn test".to_string(),
    };

    let json =
        serde_json::to_value(request).expect("OpenAI request should serialize");

    assert_eq!(json["model"], "gpt-5");
    assert_eq!(json["input"], "Hebûn test");
}



