use zanistarast_core::provider::{ProviderError, ProviderMetadata, ScientificProvider};
use zanistarast_core::provider_registry::ProviderRegistry;
use zanistarast_core::{CssId, ScientificObject};

use chrono::Utc;
use serde_json::json;

struct MockProvider;

impl ScientificProvider for MockProvider {
    fn id(&self) -> &'static str {
        "mock-provider"
    }

    fn name(&self) -> &'static str {
        "Mock Scientific Provider"
    }

    fn version(&self) -> &'static str {
        "0.1.0"
    }

    fn execute(
        &self,
        object: &ScientificObject,
    ) -> Result<ScientificObject, ProviderError> {
        Ok(object.clone())
    }

    fn metadata(&self) -> ProviderMetadata {
        let mut metadata = ProviderMetadata::new();
        metadata.insert("type".to_string(), "mock".to_string());
        metadata
    }
}

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
fn provider_registry_registers_provider() {
    let mut registry = ProviderRegistry::new();

    registry.register(MockProvider);

    assert!(registry.contains("mock-provider"));
    assert_eq!(registry.len(), 1);
    assert_eq!(registry.provider_ids(), vec!["mock-provider".to_string()]);
}

#[test]
fn provider_registry_executes_registered_provider() {
    let mut registry = ProviderRegistry::new();

    registry.register(MockProvider);

    let result = registry.execute("mock-provider", &test_object());

    assert!(result.is_ok());
    assert_eq!(result.unwrap().css_id.0, "CSS-ZANISTARAST-PROVIDER-001");
}

#[test]
fn provider_registry_rejects_unknown_provider() {
    let registry = ProviderRegistry::new();

    let result = registry.execute("missing-provider", &test_object());

    assert!(result.is_err());
}


