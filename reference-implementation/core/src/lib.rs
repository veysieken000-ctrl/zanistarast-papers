use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Canonical Scientific Structure Identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CssId(pub String);

/// Runtime Session Identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RuntimeId(pub Uuid);

/// Verification Identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VerificationId(pub Uuid);

/// Certification Identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CertificationId(pub Uuid);

/// Registry Identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RegistryId(pub Uuid);

/// Scientific Object
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScientificObject {
    pub css_id: CssId,
    pub title: String,
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub payload: serde_json::Value,
}

/// Verification Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub verification_id: VerificationId,
    pub passed: bool,
    pub diagnostics: Vec<String>,
}

/// Certification Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificationResult {
    pub certification_id: CertificationId,
    pub verified: bool,
}

/// Registry Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryEntry {
    pub registry_id: RegistryId,
    pub object: ScientificObject,
    pub certification: CertificationResult,
}

/// Canonical Runtime State
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RuntimeState {
    Created,
    Initialized,
    Scheduled,
    Executing,
    Verifying,
    Certified,
    Published,
    Completed,
    Failed,
}



