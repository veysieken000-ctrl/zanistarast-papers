use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use zanistarast_certification::{
    CertificationManager,
    MachineCertificationRecord,
};
use zanistarast_core::{
    CertificationResult,
    RegistryEntry,
    RuntimeId,
    RuntimeState,
    ScientificObject,
    VerificationResult,
};
use zanistarast_registry::{
    RegistryPublication,
    ScientificRegistry,
};
use zanistarast_verification::{
    VerificationEngine,
    VerificationTrace,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeTraceEntry {
    pub runtime_id: RuntimeId,
    pub state: RuntimeState,
    pub message: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeTrace {
    pub runtime_id: RuntimeId,
    pub entries: Vec<RuntimeTraceEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeExecutionResult {
    pub runtime_id: RuntimeId,
    pub verification: VerificationResult,
    pub verification_trace: VerificationTrace,
    pub certification: CertificationResult,
    pub certification_record: MachineCertificationRecord,
    pub publication: RegistryPublication,
    pub runtime_trace: RuntimeTrace,
}

pub struct DeterministicRuntime {
    verification_engine: VerificationEngine,
    certification_manager: CertificationManager,
    registry: ScientificRegistry,
}

impl DeterministicRuntime {
    pub fn new() -> Self {
        Self {
            verification_engine: VerificationEngine::new(),
            certification_manager: CertificationManager::new(),
            registry: ScientificRegistry::new(),
        }
    }

    pub fn execute(&mut self, object: ScientificObject) -> RuntimeExecutionResult {
        let runtime_id = RuntimeId(Uuid::new_v4());
        let mut trace = RuntimeTrace {
            runtime_id: runtime_id.clone(),
            entries: Vec::new(),
        };

        Self::push_trace(
            &mut trace,
            RuntimeState::Created,
            "runtime created",
        );

        Self::push_trace(
            &mut trace,
            RuntimeState::Initialized,
            "runtime initialized",
        );

        Self::push_trace(
            &mut trace,
            RuntimeState::Executing,
            "verification started",
        );

        let (verification, verification_trace) =
            self.verification_engine.verify(&object);

        Self::push_trace(
            &mut trace,
            RuntimeState::Verifying,
            "verification completed",
        );

        let (certification, certification_record) =
            self.certification_manager.certify(&object, &verification);

        let certification_state = if certification.verified {
            RuntimeState::Certified
        } else {
            RuntimeState::Failed
        };

        Self::push_trace(
            &mut trace,
            certification_state,
            "certification evaluated",
        );

        let publication = self.registry.publish(
            object,
            certification.clone(),
            certification_record.clone(),
        );

        Self::push_trace(
            &mut trace,
            RuntimeState::Published,
            "registry publication completed",
        );

        Self::push_trace(
            &mut trace,
            RuntimeState::Completed,
            "runtime execution completed",
        );

        RuntimeExecutionResult {
            runtime_id,
            verification,
            verification_trace,
            certification,
            certification_record,
            publication,
            runtime_trace: trace,
        }
    }

    pub fn registry_entries(&self) -> &[RegistryEntry] {
        self.registry.entries()
    }

    fn push_trace(
        trace: &mut RuntimeTrace,
        state: RuntimeState,
        message: impl Into<String>,
    ) {
        trace.entries.push(RuntimeTraceEntry {
            runtime_id: trace.runtime_id.clone(),
            state,
            message: message.into(),
            timestamp: Utc::now(),
        });
    }
}

impl Default for DeterministicRuntime {
    fn default() -> Self {
        Self::new()
    }
}



