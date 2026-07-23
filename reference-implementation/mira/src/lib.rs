pub mod task_engine;
pub mod repository_scanner;
pub mod website_scanner;
pub mod article_inventory;
pub mod article_classifier;
pub mod article_candidate_analysis;
pub mod topic_clustering;
pub mod knowledge_map;
pub mod agent_dispatch;
pub mod provider_bridge;
pub mod provider_executor;
pub mod result_collector;
pub mod rasterast_review;
pub mod approval_workflow;
pub mod recommendation_report;
pub mod chat_interface;
pub mod command_router;
pub mod chat_session;
pub mod chat_orchestrator;
pub mod chat_service;
pub mod publication_priority;
pub mod article_templates;
pub mod template_sections;
pub mod template_validator;
pub mod academic_rules;
pub mod academic_pipeline;
pub mod academic_report;
pub mod academic_runner;
pub mod article_analysis_adapter;
pub mod content_signal_detector;
pub mod article_file_analyzer;
pub mod article_analysis_service;
pub mod repository_academic_scan;
pub mod inventory_academic_runner;
pub mod full_academic_scan;
pub mod reference_signal_detector;
pub mod doi_validator;
pub mod url_validator;
pub mod bibtex_parser;

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

/// Uzman ajanın Mira’ya sunduğu çalışma sonucu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentContribution {
    pub agent_id: String,
    pub task_id: Uuid,
    pub summary: String,
    pub evidence: Vec<String>,
    pub uncertainties: Vec<String>,
    pub created_at: DateTime<Utc>,
}

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

/// Mira’nın Müdebbire sunduğu nihai öneri paketi.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MiraRecommendation {
    pub task_id: Uuid,
    pub rationale: String,
    pub benefits: Vec<String>,
    pub risks: Vec<String>,
    pub alternatives: Vec<String>,
    pub rasterast_report: Option<RasterastReport>,
    pub proposed_next_step: String,
    pub requires_mudebbir_approval: bool,
}

/// Müdebbirin açık karar kaydı.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MudebbirDecision {
    Pending,
    Approved,
    Rejected,
    RevisionRequested,
}

/// Mira’nın temel yönetim çekirdeği.
#[derive(Debug, Default)]
pub struct MiraCore {
    tasks: Vec<MiraTask>,
}

impl MiraCore {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    /// Görevi Mira’nın iç kuyruğuna ekler.
    pub fn register_task(&mut self, task: MiraTask) -> Uuid {
        let task_id = task.id;
        self.tasks.push(task);
        task_id
    }

    /// Kayıtlı görevleri salt okunur olarak döndürür.
    pub fn tasks(&self) -> &[MiraTask] {
        &self.tasks
    }

    /// Kimliğine göre görevi bulur.
    pub fn find_task(&self, task_id: Uuid) -> Option<&MiraTask> {
        self.tasks.iter().find(|task| task.id == task_id)
    }

    /// Kimliğine göre görevi değiştirilebilir olarak bulur.
    pub fn find_task_mut(&mut self, task_id: Uuid) -> Option<&mut MiraTask> {
        self.tasks.iter_mut().find(|task| task.id == task_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn low_risk_task_can_run_autonomously() {
        let task = MiraTask::new(
            "Repository scan",
            "Scan the repository without modifying files.",
            MiraRiskLevel::Low,
            false,
        );

        assert!(task.may_execute_autonomously());
    }

    #[test]
    fn approval_required_task_cannot_run_autonomously() {
        let task = MiraTask::new(
            "Publish article",
            "Publish an approved article.",
            MiraRiskLevel::High,
            true,
        );

        assert!(!task.may_execute_autonomously());
    }

    #[test]
    fn mira_core_registers_and_finds_task() {
        let mut mira = MiraCore::new();

        let task = MiraTask::new(
            "Build inventory",
            "Create a read-only article inventory.",
            MiraRiskLevel::Low,
            false,
        );

        let task_id = mira.register_task(task);

        assert!(mira.find_task(task_id).is_some());
        assert_eq!(mira.tasks().len(), 1);
    }
}


