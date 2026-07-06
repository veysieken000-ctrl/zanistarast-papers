use chrono::Utc;
use serde_json::json;
use zanistarast_core::{CssId, ScientificObject};
use zanistarast_runtime::DeterministicRuntime;

#[test]
fn invalid_object_is_not_certified_or_published() {
    let object = ScientificObject {
        css_id: CssId("CSS-ZANISTARAST-TEST-FAIL-001".to_string()),
        title: "".to_string(),
        version: "1.0.0".to_string(),
        created_at: Utc::now(),
        payload: json!({
            "type": "scientific_claim",
            "claim": "This object should fail because title is empty."
        }),
    };

    let mut runtime = DeterministicRuntime::new();
    let result = runtime.execute(object);

    assert!(!result.verification.passed);
    assert!(!result.certification.verified);
    assert!(result.publication.is_none());
    assert_eq!(runtime.registry_entries().len(), 0);
    assert!(!result.verification.diagnostics.is_empty());
    assert!(!result.verification_trace.entries.is_empty());
    assert!(!result.runtime_trace.entries.is_empty());
}



