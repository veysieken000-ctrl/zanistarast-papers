use chrono::Utc;
use serde_json::json;

use zanistarast_ai::openai_provider::OpenAiProvider;
use zanistarast_core::{CssId, ScientificObject};

#[tokio::test]
#[ignore = "requires OPENAI_API_KEY and makes a real paid API request"]
async fn openai_live_request_returns_scientific_object() {
    let model = std::env::var("OPENAI_MODEL")
        .unwrap_or_else(|_| "gpt-5.2".to_string());

    let provider = OpenAiProvider::with_model(model);

    let object = ScientificObject {
        css_id: CssId("CSS-OPENAI-LIVE-001".to_string()),
        title: "Zanistarast OpenAI Live Test".to_string(),
        version: "1.0.0".to_string(),
        created_at: Utc::now(),
        payload: json!({
            "type": "scientific_claim",
            "claim": "Explain deterministic scientific verification in one short sentence."
        }),
    };

    let result = provider
        .execute_remote(&object)
        .await
        .expect("real OpenAI request should succeed");

    let response = result
        .payload
        .get("openai_response")
        .and_then(|value| value.as_str())
        .expect("response should contain openai_response");

    assert!(!response.trim().is_empty());
}


