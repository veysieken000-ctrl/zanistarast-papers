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

impl MudebbirDecisionRecord {
    /// Bir Mira görevi için yeni Müdebbir karar kaydı oluşturur.
    pub fn new(
        task_id: Uuid,
        decision: MudebbirDecision,
    ) -> Self {
        Self {
            task_id,
            decision,
            decided_at: Utc::now(),
        }
    }

    /// Kararın henüz verilmemiş olup olmadığını bildirir.
    pub fn is_pending(&self) -> bool {
        self.decision == MudebbirDecision::Pending
    }

    /// Kararın kesinleşmiş olup olmadığını bildirir.
    pub fn is_final(&self) -> bool {
        !self.is_pending()
    }

    /// Kararın güvenli yeni sürüm oluşturulmasına
    /// izin verip vermediğini bildirir.
    pub fn approves_version(&self) -> bool {
        self.decision == MudebbirDecision::Approved
    }

    /// Müdebbirin düzeltme veya yeniden çalışma
    /// istediğini bildirir.
    pub fn requires_revision(&self) -> bool {
        self.decision == MudebbirDecision::RevisionRequested
    }

    /// Kararın görevi açıkça reddedip reddetmediğini bildirir.
    pub fn is_rejected(&self) -> bool {
        self.decision == MudebbirDecision::Rejected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approved_decision_allows_version_creation() {
        let record = MudebbirDecisionRecord::new(
            Uuid::new_v4(),
            MudebbirDecision::Approved,
        );

        assert!(record.is_final());
        assert!(record.approves_version());
        assert!(!record.requires_revision());
        assert!(!record.is_rejected());
    }

    #[test]
    fn pending_decision_does_not_allow_version_creation() {
        let record = MudebbirDecisionRecord::new(
            Uuid::new_v4(),
            MudebbirDecision::Pending,
        );

        assert!(record.is_pending());
        assert!(!record.is_final());
        assert!(!record.approves_version());
    }

    #[test]
    fn revision_request_blocks_version_creation() {
        let record = MudebbirDecisionRecord::new(
            Uuid::new_v4(),
            MudebbirDecision::RevisionRequested,
        );

        assert!(record.is_final());
        assert!(record.requires_revision());
        assert!(!record.approves_version());
    }

    #[test]
    fn rejected_decision_blocks_version_creation() {
        let record = MudebbirDecisionRecord::new(
            Uuid::new_v4(),
            MudebbirDecision::Rejected,
        );

        assert!(record.is_final());
        assert!(record.is_rejected());
        assert!(!record.approves_version());
    }
}


