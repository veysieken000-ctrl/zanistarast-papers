use chrono::Utc;
use serde_json::json;
use zanistarast_ai::provider::NativeAiProvider;
use zanistarast_core::provider::ScientificProvider;
use zanistarast_core::{CssId, ScientificObject};

fn test_object() -> ScientificObject {
    ScientificObject {
        css_id: CssId("CSS-ZANISTARAST-NATIVE-AI-001".to_string()),
        title: "Native AI Provider Test Object".to_string(),
        version: "1.0.0".to_string(),
        created_at: Utc::now(),
        payload: json!({
            "type": "scientific_claim",
            "claim": "Native AI provider should execute a valid scientific object.",
            "layers": {
                "hebun": "existence verified",
                "zanabun": "meaning verified",
                "mabun": "structure verified",
                "rabun": "operation verified",
                "rasterast": "final verification completed"
            }
        }),
    }
}

#[test]
fn native_ai_provider_executes_valid_object() {
    let provider = NativeAiProvider::new();

    let result = provider.execute(&test_object());

    assert!(result.is_ok());
    assert_eq!(provider.id(), "native-ai");
    assert_eq!(provider.metadata().get("deterministic"), Some(&"true".to_string()));
}

#[test]
fn native_ai_provider_rejects_invalid_object() {
    let provider = NativeAiProvider::new();

    let mut object = test_object();
    object.title = "".to_string();

    let result = provider.execute(&object);

    assert!(result.is_err());
}

