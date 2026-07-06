use chrono::Utc;
use serde_json::json;
use zanistarast_certification::CertificationManager;
use zanistarast_core::{CssId, ScientificObject};
use zanistarast_registry::ScientificRegistry;
use zanistarast_verification::VerificationEngine;

#[test]
fn certified_object_is_published_once() {
    let object = ScientificObject {
        css_id: CssId("CSS-ZANISTARAST-REGISTRY-001".to_string()),
        title: "Registry Test Object".to_string(),
        version: "1.0.0".to_string(),
        created_at: Utc::now(),
        payload: json!({
            "type": "scientific_claim",
            "claim": "A certified object may be published to the registry.",
            "layers": {
                "hebun": "existence verified",
                "zanabun": "meaning verified",
                "mabun": "structure verified",
                "rabun": "operation verified",
                "rasterast": "final verification completed"
            }
        }),
    };

    let verification_engine = VerificationEngine::new();
    let certification_manager = CertificationManager::new();
    let mut registry = ScientificRegistry::new();

    let (verification, _) = verification_engine.verify(&object);
    let (certification, record) = certification_manager.certify(&object, &verification);

    assert!(verification.passed);
    assert!(certification.verified);

    let publication = registry.publish(object, certification, record);

    assert_eq!(registry.len(), 1);
    assert_eq!(registry.entries()[0].registry_id, publication.registry_id);
}

#[test]
fn registry_entries_are_append_only() {
    let mut registry = ScientificRegistry::new();

    let verification_engine = VerificationEngine::new();
    let certification_manager = CertificationManager::new();

    for index in 1..=2 {
        let object = ScientificObject {
            css_id: CssId(format!("CSS-ZANISTARAST-REGISTRY-00{}", index)),
            title: format!("Registry Append Object {}", index),
            version: "1.0.0".to_string(),
            created_at: Utc::now(),
            payload: json!({
                "type": "scientific_claim",
                "claim": "Registry entries are append-only.",
                "index": index
            }),
        };

        let (verification, _) = verification_engine.verify(&object);
        let (certification, record) = certification_manager.certify(&object, &verification);

        assert!(verification.passed);
        assert!(certification.verified);

        registry.publish(object, certification, record);
    }

    assert_eq!(registry.len(), 2);
    assert!(!registry.is_empty());
}


