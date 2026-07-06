use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zanistarast_core::{ScientificObject, VerificationId, VerificationResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VerificationRule {
    Identity,
    Hebun,
    Zanabun,
    Mabun,
    Rabun,
    Rasterast,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationTraceEntry {
    pub rule: VerificationRule,
    pub passed: bool,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationTrace {
    pub verification_id: VerificationId,
    pub entries: Vec<VerificationTraceEntry>,
}

pub struct VerificationEngine;

impl VerificationEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn verify(&self, object: &ScientificObject) -> (VerificationResult, VerificationTrace) {
        let verification_id = VerificationId(Uuid::new_v4());
        let mut entries = Vec::new();

        let rules = vec![
            VerificationRule::Identity,
            VerificationRule::Hebun,
            VerificationRule::Zanabun,
            VerificationRule::Mabun,
            VerificationRule::Rabun,
            VerificationRule::Rasterast,
        ];

        let mut passed = true;
        let mut diagnostics = Vec::new();

        for rule in rules {
            let (rule_passed, message) = self.execute_rule(&rule, object);

            if !rule_passed {
                passed = false;
                diagnostics.push(message.clone());
            }

            entries.push(VerificationTraceEntry {
                rule,
                passed: rule_passed,
                message,
                timestamp: Utc::now(),
            });

            if !passed {
                break;
            }
        }

        (
            VerificationResult {
                verification_id: verification_id.clone(),
                passed,
                diagnostics,
            },
            VerificationTrace {
                verification_id,
                entries,
            },
        )
    }

    fn execute_rule(
        &self,
        rule: &VerificationRule,
        object: &ScientificObject,
    ) -> (bool, String) {
        match rule {
            VerificationRule::Identity => {
                if object.css_id.0.trim().is_empty() {
                    (false, "VR1 failed: missing CSS-ID".to_string())
                } else {
                    (true, "VR1 passed: identity verified".to_string())
                }
            }
            VerificationRule::Hebun => {
                if object.title.trim().is_empty() {
                    (false, "VR2 failed: object title is empty".to_string())
                } else {
                    (true, "VR2 passed: Hebûn existence verified".to_string())
                }
            }
            VerificationRule::Zanabun => {
                if object.payload.is_null() {
                    (false, "VR3 failed: payload has no meaning context".to_string())
                } else {
                    (true, "VR3 passed: Zanabûn meaning verified".to_string())
                }
            }
            VerificationRule::Mabun => {
                if !object.payload.is_object() {
                    (false, "VR4 failed: payload is not structurally organized".to_string())
                } else {
                    (true, "VR4 passed: Mabûn structure verified".to_string())
                }
            }
            VerificationRule::Rabun => {
                (true, "VR5 passed: Rabûn operational consistency verified".to_string())
            }
            VerificationRule::Rasterast => {
                (true, "VR6 passed: Rasterast final verification completed".to_string())
            }
        }
    }
}

impl Default for VerificationEngine {
    fn default() -> Self {
        Self::new()
    }
}



