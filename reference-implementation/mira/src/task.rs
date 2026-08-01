use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Mira sisteminde görevin mevcut durumunu gösterir.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MiraTaskStatus {
    Created,
    Planning,
    Assigned,
    Running,
    AwaitingRasterast,
    AwaitingMudebbir,
    Approved,
    Rejected,
    Completed,

    /// Tamamlanmış akademik çalışmanın dış yayını
    /// Müdebbir tarafından onaylanmıştır.
    PublicationApproved,

    /// Tamamlanmış akademik çalışmanın dış yayını
    /// Müdebbir tarafından reddedilmiştir.
    PublicationRejected,

    Failed,
}

/// Bir işlemin risk seviyesini gösterir.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MiraRiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Mira tarafından yönetilen temel görev kaydı.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiraTask {
    pub id: Uuid,
    pub title: String,
    pub instruction: String,
    pub status: MiraTaskStatus,
    pub risk_level: MiraRiskLevel,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub requires_mudebbir_approval: bool,
}

impl MiraTask {
    /// Yeni bir Mira görevi oluşturur.
    pub fn new(
        title: impl Into<String>,
        instruction: impl Into<String>,
        risk_level: MiraRiskLevel,
        requires_mudebbir_approval: bool,
    ) -> Self {
        let now = Utc::now();

        Self {
            id: Uuid::new_v4(),
            title: title.into(),
            instruction: instruction.into(),
            status: MiraTaskStatus::Created,
            risk_level,
            created_at: now,
            updated_at: now,
            requires_mudebbir_approval,
        }
    }

    /// Görevin durumunu güvenli biçimde günceller.
    pub fn update_status(&mut self, status: MiraTaskStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    /// Görevin Müdebbir onayı olmadan uygulanıp uygulanamayacağını bildirir.
    pub fn may_execute_autonomously(&self) -> bool {
        !self.requires_mudebbir_approval
            && matches!(self.risk_level, MiraRiskLevel::Low)
    }
}



