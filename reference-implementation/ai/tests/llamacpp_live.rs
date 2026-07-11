use chrono::Utc;
use serde_json::json;

use zanistarast_ai::llamacpp_provider::LlamaCppProvider;
use zanistarast_core::{CssId, ScientificObject};

#[tokio::test]
#[ignore = "requires a running llama.cpp server and a loaded model"]
async fn llamacpp_live_request_returns_scientific_object() {
    let endpoint = std::env::var("LLAMACPP_ENDPOINT")
        .unwrap_or_else(|_| "http://localhost:8080".to_string());

    let model = std::env::var("LLAMACPP_MODEL")
        .unwrap_or_else(|_| "local-model".to_string());

    let provider = LlamaCppProvider::with_config(
        Some(endpoint),
        Some(model),
    );

    let object = ScientificObject {
        css_id: CssId("CSS-LLAMACPP-LIVE-001".to_string()),
        title: "Zanistarast llama.cpp Live Test".to_string(),
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
        .expect("real llama.cpp request should succeed");

    let response = result
        .payload
        .get("llamacpp_response")
        .and_then(|value| value.as_str())
        .expect("response should contain llamacpp_response");

    assert!(!response.trim().is_empty());
}


