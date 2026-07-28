use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Uzman ajanın Mira'ya sunduğu çalışma sonucu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContribution {
    pub agent_id: String,
    pub task_id: Uuid,
    pub summary: String,
    pub evidence: Vec<String>,
    pub uncertainties: Vec<String>,
    pub created_at: DateTime<Utc>,
}


