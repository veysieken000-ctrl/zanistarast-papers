use zanistarast_ai::openai_responses::OpenAiResponse;

#[test]
fn openai_response_is_deserializable() {
    let raw = r#"
    {
        "output": [
            {
                "content": [
                    {
                        "type": "output_text",
                        "text": "Zanistarast OpenAI response test"
                    }
                ]
            }
        ]
    }
    "#;

    let response: OpenAiResponse =
        serde_json::from_str(raw).expect("OpenAI response should deserialize");

    assert_eq!(response.output.len(), 1);
    assert_eq!(response.output[0].content.len(), 1);
    assert_eq!(
        response.output[0].content[0].content_type,
        "output_text"
    );
    assert_eq!(
        response.output[0].content[0].text.as_deref(),
        Some("Zanistarast OpenAI response test")
    );
}


