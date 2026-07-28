use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Rasterast doğrulama sonucu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RasterastReport {
    pub task_id: Uuid,
    pub verified: bool,
    pub verified_items: Vec<String>,
    pub unverified_items: Vec<String>,
    pub contradictions: Vec<String>,
    pub risks: Vec<String>,
    pub requires_mudebbir_decision: bool,
    pub created_at: DateTime<Utc>,
}
