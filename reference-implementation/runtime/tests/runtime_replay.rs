use chrono::Utc;
use serde_json::json;
use zanistarast_core::{CssId, ScientificObject};
use zanistarast_runtime::DeterministicRuntime;

#[test]
fn runtime_replay_verifies_successful_execution() {
    let object = ScientificObject {
        css_id: CssId("CSS-ZANISTARAST-REPLAY-001".to_string()),
        title: "Replay Test Object".to_string(),
        version: "1.0.0".to_string(),
        created_at: Utc::now(),
        payload: json!({
            "type": "scientific_claim",
            "claim": "A certified runtime execution must be replayable.",
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
    let replay = runtime.replay(&result);

    assert!(result.verification.passed);
    assert!(result.certification.verified);
    assert!(result.publication.is_some());

    assert!(replay.replay_verified);
    assert!(replay.expected_certified);
    assert!(replay.actual_certified);
    assert!(replay.diagnostics.is_empty());
}

#[test]
fn runtime_replay_verifies_failed_execution_without_publication() {
    let object = ScientificObject {
        css_id: CssId("CSS-ZANISTARAST-REPLAY-FAIL-001".to_string()),
        title: "".to_string(),
        version: "1.0.0".to_string(),
        created_at: Utc::now(),
        payload: json!({
            "type": "scientific_claim",
            "claim": "A failed runtime execution must not be published."
        }),
    };

    let mut runtime = DeterministicRuntime::new();
    let result = runtime.execute(object);
    let replay = runtime.replay(&result);

    assert!(!result.verification.passed);
    assert!(!result.certification.verified);
    assert!(result.publication.is_none());

    assert!(replay.replay_verified);
    assert!(!replay.expected_certified);
    assert!(!replay.actual_certified);
    assert!(replay.diagnostics.is_empty());
}



