use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Müdebbirin açık karar türü.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MudebbirDecision {
    Pending,
    Approved,
    Rejected,
    RevisionRequested,
}

/// Müdebbirin bir Mira görevi için verdiği kalıcı karar kaydı.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MudebbirDecisionRecord {
    pub task_id: Uuid,
    pub decision: MudebbirDecision,
    pub decided_at: DateTime<Utc>,
}


