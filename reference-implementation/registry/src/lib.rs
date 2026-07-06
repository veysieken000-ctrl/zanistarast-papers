use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use zanistarast_certification::MachineCertificationRecord;
use zanistarast_core::{
    RegistryEntry,
    RegistryId,
    ScientificObject,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryPublication {
    pub registry_id: RegistryId,
    pub published_at: DateTime<Utc>,
    pub record: MachineCertificationRecord,
}

#[derive(Debug, Default)]
pub struct ScientificRegistry {
    entries: Vec<RegistryEntry>,
}

impl ScientificRegistry {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn publish(
        &mut self,
        object: ScientificObject,
        certification: zanistarast_core::CertificationResult,
        record: MachineCertificationRecord,
    ) -> RegistryPublication {
        let registry_id = RegistryId(Uuid::new_v4());

        let entry = RegistryEntry {
            registry_id: registry_id.clone(),
            object,
            certification,
        };

        self.entries.push(entry);

        RegistryPublication {
            registry_id,
            published_at: Utc::now(),
            record,
        }
    }

    pub fn entries(&self) -> &[RegistryEntry] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}


