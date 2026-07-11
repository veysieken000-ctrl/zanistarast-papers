use chrono::Utc;
use serde_json::json;

use zanistarast_ai::gemini_provider::GeminiProvider;
use zanistarast_core::{CssId, ScientificObject};

#[tokio::test]
#[ignore = "requires GEMINI_API_KEY and makes a real API request"]
async fn gemini_live_request_returns_scientific_object() {
    let model = std::env::var("GEMINI_MODEL")
        .unwrap_or_else(|_| "gemini-3-flash-preview".to_string()

    let provider = GeminiProvider::with_model(model);

    let object = ScientificObject {
        css_id: CssId("CSS-GEMINI-LIVE-001".to_string()),
        title: "Zanistarast Gemini Live Test".to_string(),
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
        .expect("real Gemini request should succeed");

    let response = result
        .payload
        .get("gemini_response")
        .and_then(|value| value.as_str())
        .expect("response should contain gemini_response");

    assert!(!response.trim().is_empty());
}


