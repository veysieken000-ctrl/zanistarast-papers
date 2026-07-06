use chrono::Utc;
use serde_json::json;
use zanistarast_core::{CssId, ScientificObject};
use zanistarast_runtime::DeterministicRuntime;

#[test]
fn successful_object_is_verified_certified_and_published() {
    let object = ScientificObject {
        css_id: CssId("CSS-ZANISTARAST-TEST-001".to_string()),
        title: "Successful Test Object".to_string(),
        version: "1.0.0".to_string(),
        created_at: Utc::now(),
        payload: json!({
            "type": "scientific_claim",
            "claim": "A valid object should pass deterministic verification.",
            "layers": {
                "hebun": "existence verified",
                "zanabun": "meaning verified",
                "mabun": "structure verified",
                "rabun": "operation verified",
                "rasterast": "final verification completed"
            }
        }),
    };

    let mut runtime = DeterministicRuntime::new();
    let result = runtime.execute(object);

    assert!(result.verification.passed);
    assert!(result.certification.verified);
    assert!(result.publication.is_some());
    assert_eq!(runtime.registry_entries().len(), 1);
    assert!(!result.verification_trace.entries.is_empty());
    assert!(!result.runtime_trace.entries.is_empty());
}


