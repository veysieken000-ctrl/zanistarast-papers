use serde::{Deserialize, Serialize};

/// Mira aktif çalışma merkezindeki bir işin durumunu belirtir.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub enum WorkItemStatus {
    Pending,
    InProgress,
    Blocked,
    AwaitingInput,
    AwaitingApproval,
    AwaitingVerification,
    Ready,
}

/// Mira aktif çalışma merkezindeki tek bir işi temsil eder.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct WorkItem {
    pub id: String,
    pub title: String,
    pub status: WorkItemStatus,
}

impl WorkItem {
    pub fn new(
        id: impl Into<String>,
        title: impl Into<String>,
        status: WorkItemStatus,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            status,
        }
    }

    pub fn is_valid(&self) -> bool {
        !self.id.trim().is_empty()
            && !self.title.trim().is_empty()
    }
}

/// Mira'nın görünür aktif çalışma merkezidir.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct WorkCenter {
    pub items: Vec<WorkItem>,
}

