use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zanistarast_core::{CertificationId, CertificationResult, ScientificObject, VerificationResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CertificationStatus {
    Certified,
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MachineCertificationRecord {
    pub certification_id: CertificationId,
    pub css_id: String,
    pub status: CertificationStatus,
    pub issued_at: DateTime<Utc>,
    pub evidence: Vec<String>,
}

pub struct CertificationManager;

impl CertificationManager {
    pub fn new() -> Self {
        Self
    }

    pub fn certify(
        &self,
        object: &ScientificObject,
        verification: &VerificationResult,
    ) -> (CertificationResult, MachineCertificationRecord) {
        let certification_id = CertificationId(Uuid::new_v4());

        let status = if verification.passed {
            CertificationStatus::Certified
        } else {
            CertificationStatus::Denied
        };

        let evidence = if verification.passed {
            vec![
                "VR7 passed: certification prerequisites satisfied".to_string(),
                "Machine certification granted".to_string(),
            ]
        } else {
            verification.diagnostics.clone()
        };

        let result = CertificationResult {
            certification_id: certification_id.clone(),
            verified: verification.passed,
        };

        let record = MachineCertificationRecord {
            certification_id,
            css_id: object.css_id.0.clone(),
            status,
            issued_at: Utc::now(),
            evidence,
        };

        (result, record)
    }
}

impl Default for CertificationManager {
    fn default() -> Self {
        Self::new()
    }
}



