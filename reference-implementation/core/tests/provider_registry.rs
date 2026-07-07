use chrono::Utc;
use serde_json::json;
use zanistarast_core::provider_registry::ProviderRegistry;
use zanistarast_core::{CssId, ScientificObject};
use zanistarast_ai::provider::NativeAiProvider;

fn test_object() -> ScientificObject {
    ScientificObject {
        css_id: CssId("CSS-ZANISTARAST-PROVIDER-001".to_string()),
        title: "Provider Registry Test Object".to_string(),
        version: "1.0.0".to_string(),
        created_at: Utc::now(),
        payload: json!({
            "type": "scientific_claim",
            "claim": "A provider registry should execute registered scientific providers.",
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
fn provider_registry_registers_native_ai_provider() {
    let mut registry = ProviderRegistry::new();

    registry.register(NativeAiProvider::new());

    assert!(registry.contains("native-ai"));
    assert_eq!(registry.len(), 1);
    assert_eq!(registry.provider_ids(), vec!["native-ai".to_string()]);
}

#[test]
fn provider_registry_executes_native_ai_provider() {
    let mut registry = ProviderRegistry::new();

    registry.register(NativeAiProvider::new());

    let result = registry.execute("native-ai", &test_object());

    assert!(result.is_ok());
    assert_eq!(result.unwrap().css_id.0, "CSS-ZANISTARAST-PROVIDER-001");
}

#[test]
fn provider_registry_rejects_unknown_provider() {
    let registry = ProviderRegistry::new();

    let result = registry.execute("missing-provider", &test_object());

    assert!(result.is_err());
}



