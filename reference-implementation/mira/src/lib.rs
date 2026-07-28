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
pub mod citation_reference_matcher;
pub mod source_verification_report;
pub mod bibtex_generator;
pub mod latex_generator;
pub mod pdf_generator;
pub mod academic_output;
pub mod publication_package;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::academic_runner::{
    run_verified_academic_analysis,
    AcademicRunnerInput,
    VerifiedAcademicRunnerOutput,
};
use crate::source_verification_report::SourceVerificationReport;

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
    rasterast_reports: Vec<RasterastReport>,
    academic_outputs: Vec<VerifiedAcademicRunnerOutput>,

}

impl MiraCore {
    pub fn new() -> Self {
    Self {
        tasks: Vec::new(),
        rasterast_reports: Vec::new(),
    academic_outputs: Vec::new(),
    }
}

    /// Görevi Mira’nın iç kuyruğuna ekler.
    pub fn register_task(&mut self, task: MiraTask) -> Uuid {
        let task_id = task.id;
        self.tasks.push(task);
        task_id
    }

    /// Müdebbir onayı gerektiren yüksek riskli bir akademik görev oluşturur ve kaydeder.
pub fn register_academic_task(
    &mut self,
    title: impl Into<String>,
    instruction: impl Into<String>,
) -> Uuid {
    let task = MiraTask::new(
        title,
        instruction,
        MiraRiskLevel::High,
        true,
    );

    self.register_task(task)
}

/// Mira tarafından saklanan doğrulanmış akademik çıktıları döndürür.
pub fn academic_outputs(&self) -> &[VerifiedAcademicRunnerOutput] {
    &self.academic_outputs
}
    
   /// Oluşturulmuş bir görevi planlama aşamasına geçirir.
pub fn start_planning(&mut self, task_id: Uuid) -> bool {
    let Some(task) = self.find_task_mut(task_id) else {
        return false;
    };

    if task.status != MiraTaskStatus::Created {
        return false;
    }

    task.update_status(MiraTaskStatus::Planning);
    true
}
   
   /// Planlanan görevi çalıştırma aşamasına geçirir.
pub fn start_running(&mut self, task_id: Uuid) -> bool {
    let Some(task) = self.find_task_mut(task_id) else {
        return false;
    };

    if task.status != MiraTaskStatus::Planning {
        return false;
    }

    task.update_status(MiraTaskStatus::Running);
    true
}

/// Çalışan görevi Rasterast doğrulaması bekleme aşamasına geçirir.
pub fn await_rasterast(&mut self, task_id: Uuid) -> bool {
    let Some(task) = self.find_task_mut(task_id) else {
        return false;
    };

    if task.status != MiraTaskStatus::Running {
        return false;
    }

    task.update_status(MiraTaskStatus::AwaitingRasterast);
    true
}

/// Rasterast raporunu ilgili göreve bağlar.
pub fn attach_rasterast_report(
    &mut self,
    report: RasterastReport,
) -> bool {
    let Some(task) = self.find_task(report.task_id) else {
        return false;
    };

    if task.status != MiraTaskStatus::AwaitingRasterast {
        return false;
    }

    self.rasterast_reports.push(report);
    true
}

/// Rasterast raporu bulunan görevi Müdebbir kararı bekleme aşamasına geçirir.
pub fn await_mudebbir(&mut self, task_id: Uuid) -> bool {
    let has_rasterast_report = self
        .rasterast_reports
        .iter()
        .any(|report| report.task_id == task_id);

    if !has_rasterast_report {
        return false;
    }

    let Some(task) = self.find_task_mut(task_id) else {
        return false;
    };

    if task.status != MiraTaskStatus::AwaitingRasterast {
        return false;
    }

    task.update_status(MiraTaskStatus::AwaitingMudebbir);
    true
}

    /// Müdebbir onayını bekleyen görevi onaylanmış duruma geçirir.
pub fn approve_task(&mut self, task_id: Uuid) -> bool {
    let Some(task) = self.find_task_mut(task_id) else {
        return false;
    };

    if task.status != MiraTaskStatus::AwaitingMudebbir {
        return false;
    }

    task.update_status(MiraTaskStatus::Approved);
    true
}

pub fn store_academic_output(
    &mut self,
    output: VerifiedAcademicRunnerOutput,
) {
    self.academic_outputs.push(output);
}

pub fn academic_output_count(&self) -> usize {
    self.academic_outputs.len()
}

pub fn has_academic_output(&self) -> bool {
    !self.academic_outputs.is_empty()
}

  /// Yalnızca Müdebbir tarafından onaylanmış görev için
/// doğrulanmış akademik üretim hattını çalıştırır.
pub fn run_verified_analysis(
    &mut self,
    task_id: Uuid,
    input: AcademicRunnerInput,
    source_verification: SourceVerificationReport,
) -> bool {
    if !self.can_start_academic_pipeline(task_id) {
        return false;
    }

    let output =
        run_verified_academic_analysis(input, source_verification);

    self.store_academic_output(output);
    true
} 
 
/// Müdebbir onayını bekleyen görevi reddedilmiş duruma geçirir.
pub fn reject_task(&mut self, task_id: Uuid) -> bool {
    let Some(task) = self.find_task_mut(task_id) else {
        return false;
    };

    if task.status != MiraTaskStatus::AwaitingMudebbir {
        return false;
    }

    task.update_status(MiraTaskStatus::Rejected);
    true
}

    /// Onaylanan akademik görevin akademik üretim hattını başlatmaya uygun olup olmadığını bildirir.
pub fn can_start_academic_pipeline(
    &self,
    task_id: Uuid,
) -> bool {
    let Some(task) = self.find_task(task_id) else {
        return false;
    };

    task.status == MiraTaskStatus::Approved
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
#[test]
fn mira_registers_academic_task_with_mudebbir_approval() {
    let mut mira = MiraCore::new();

    let task_id = mira.register_academic_task(
        "Hebûn makalesini hazırla",
        "Hebûn içeriğini akademik üretim hattından geçir.",
    );

    let task = mira
        .find_task(task_id)
        .expect("Akademik görev kayıtlı olmalıdır.");

    assert_eq!(task.status, MiraTaskStatus::Created);
    assert_eq!(task.risk_level, MiraRiskLevel::High);
    assert!(task.requires_mudebbir_approval);
    assert!(!task.may_execute_autonomously());
}

#[test]
fn mira_moves_created_academic_task_to_planning() {
    let mut mira = MiraCore::new();

    let task_id = mira.register_academic_task(
        "Hebûn makalesini hazırla",
        "Hebûn içeriğini akademik üretim hattından geçir.",
    );

    assert!(mira.start_planning(task_id));

    let task = mira
        .find_task(task_id)
        .expect("Akademik görev kayıtlı olmalıdır.");

    assert_eq!(task.status, MiraTaskStatus::Planning);
}

#[test]
fn mira_moves_planning_task_to_running() {
    let mut mira = MiraCore::new();

    let task_id = mira.register_academic_task(
        "Hebûn makalesini hazırla",
        "Hebûn içeriğini akademik üretim hattından geçir.",
    );

    assert!(mira.start_planning(task_id));
    assert!(mira.start_running(task_id));

    let task = mira.find_task(task_id).unwrap();

    assert_eq!(task.status, MiraTaskStatus::Running);
}

#[test]
fn mira_moves_running_task_to_awaiting_rasterast() {
    let mut mira = MiraCore::new();

    let task_id = mira.register_academic_task(
        "Hebûn makalesini hazırla",
        "Hebûn içeriğini akademik üretim hattından geçir.",
    );

    assert!(mira.start_planning(task_id));
    assert!(mira.start_running(task_id));
    assert!(mira.await_rasterast(task_id));

    let task = mira
        .find_task(task_id)
        .expect("Akademik görev kayıtlı olmalıdır.");

    assert_eq!(task.status, MiraTaskStatus::AwaitingRasterast);
}

#[test]
fn mira_attaches_rasterast_report_to_awaiting_task() {
    let mut mira = MiraCore::new();

    let task_id = mira.register_academic_task(
        "Hebûn makalesini hazırla",
        "Hebûn içeriğini akademik üretim hattından geçir.",
    );

    assert!(mira.start_planning(task_id));
    assert!(mira.start_running(task_id));
    assert!(mira.await_rasterast(task_id));

    let report = RasterastReport {
        task_id,
        verified: true,
        verified_items: vec!["Akademik görev doğrulandı.".to_string()],
        unverified_items: Vec::new(),
        contradictions: Vec::new(),
        risks: Vec::new(),
        requires_mudebbir_decision: true,
        created_at: Utc::now(),
    };

    assert!(mira.attach_rasterast_report(report));
    assert_eq!(mira.rasterast_reports.len(), 1);
    assert_eq!(mira.rasterast_reports[0].task_id, task_id);
}

#[test]
fn mira_moves_verified_task_to_awaiting_mudebbir() {
    let mut mira = MiraCore::new();

    let task_id = mira.register_academic_task(
        "Hebûn makalesini hazırla",
        "Hebûn içeriğini akademik üretim hattından geçir.",
    );

    assert!(mira.start_planning(task_id));
    assert!(mira.start_running(task_id));
    assert!(mira.await_rasterast(task_id));

    let report = RasterastReport {
        task_id,
        verified: true,
        verified_items: vec!["Akademik görev doğrulandı.".to_string()],
        unverified_items: Vec::new(),
        contradictions: Vec::new(),
        risks: Vec::new(),
        requires_mudebbir_decision: true,
        created_at: Utc::now(),
    };

    assert!(mira.attach_rasterast_report(report));
    assert!(mira.await_mudebbir(task_id));

    let task = mira
        .find_task(task_id)
        .expect("Akademik görev kayıtlı olmalıdır.");

    assert_eq!(task.status, MiraTaskStatus::AwaitingMudebbir);
}

#[test]
fn mudebbir_approves_awaiting_academic_task() {
    let mut mira = MiraCore::new();

    let task_id = mira.register_academic_task(
        "Hebûn makalesini hazırla",
        "Hebûn içeriğini akademik üretim hattından geçir.",
    );

    assert!(mira.start_planning(task_id));
    assert!(mira.start_running(task_id));
    assert!(mira.await_rasterast(task_id));

    let report = RasterastReport {
        task_id,
        verified: true,
        verified_items: vec!["Akademik görev doğrulandı.".to_string()],
        unverified_items: Vec::new(),
        contradictions: Vec::new(),
        risks: Vec::new(),
        requires_mudebbir_decision: true,
        created_at: Utc::now(),
    };

    assert!(mira.attach_rasterast_report(report));
    assert!(mira.await_mudebbir(task_id));
    assert!(mira.approve_task(task_id));

    let task = mira
        .find_task(task_id)
        .expect("Akademik görev kayıtlı olmalıdır.");

    assert_eq!(task.status, MiraTaskStatus::Approved);
}

#[test]
fn mudebbir_rejects_awaiting_academic_task() {
    let mut mira = MiraCore::new();

    let task_id = mira.register_academic_task(
        "Hebûn makalesini hazırla",
        "Hebûn içeriğini akademik üretim hattından geçir.",
    );

    assert!(mira.start_planning(task_id));
    assert!(mira.start_running(task_id));
    assert!(mira.await_rasterast(task_id));

    let report = RasterastReport {
        task_id,
        verified: true,
        verified_items: vec!["Akademik görev doğrulandı.".to_string()],
        unverified_items: Vec::new(),
        contradictions: Vec::new(),
        risks: Vec::new(),
        requires_mudebbir_decision: true,
        created_at: Utc::now(),
    };

    assert!(mira.attach_rasterast_report(report));
    assert!(mira.await_mudebbir(task_id));
    assert!(mira.reject_task(task_id));

    let task = mira
        .find_task(task_id)
        .expect("Akademik görev kayıtlı olmalıdır.");

    assert_eq!(task.status, MiraTaskStatus::Rejected);
}

#[test]
fn approved_task_can_start_academic_pipeline() {
    let mut mira = MiraCore::new();

    let task_id = mira.register_academic_task(
        "Hebûn makalesini hazırla",
        "Hebûn içeriğini akademik üretim hattından geçir.",
    );

    assert!(mira.start_planning(task_id));
    assert!(mira.start_running(task_id));
    assert!(mira.await_rasterast(task_id));

    let report = RasterastReport {
        task_id,
        verified: true,
        verified_items: vec!["Doğrulandı.".to_string()],
        unverified_items: Vec::new(),
        contradictions: Vec::new(),
        risks: Vec::new(),
        requires_mudebbir_decision: true,
        created_at: Utc::now(),
    };

    assert!(mira.attach_rasterast_report(report));
    assert!(mira.await_mudebbir(task_id));
    assert!(mira.approve_task(task_id));

    assert!(mira.can_start_academic_pipeline(task_id));
}

#[test]
fn approved_task_runs_and_stores_verified_academic_analysis() {
    let mut mira = MiraCore::new();

    let task_id = mira.register_academic_task(
        "Hebûn makalesini hazırla",
        "Hebûn içeriğini akademik üretim hattından geçir.",
    );

    assert!(mira.start_planning(task_id));
    assert!(mira.start_running(task_id));
    assert!(mira.await_rasterast(task_id));

    let rasterast_report = RasterastReport {
        task_id,
        verified: true,
        verified_items: vec![
            "Akademik görev doğrulandı.".to_string(),
        ],
        unverified_items: Vec::new(),
        contradictions: Vec::new(),
        risks: Vec::new(),
        requires_mudebbir_decision: true,
        created_at: Utc::now(),
    };

    assert!(mira.attach_rasterast_report(rasterast_report));
    assert!(mira.await_mudebbir(task_id));
    assert!(mira.approve_task(task_id));

    let input = AcademicRunnerInput {
        article_type:
            crate::article_classifier::AcademicArticleType::Mathematical,
        has_abstract: true,
        has_references: true,
        has_conclusion: true,
        has_math: true,
        has_experiments: false,
    };

    let source_verification = SourceVerificationReport {
        doi_count: 0,
        valid_doi_count: 0,
        invalid_doi_count: 0,
        url_count: 0,
        valid_url_count: 0,
        invalid_url_count: 0,
        citation_count: 0,
        reference_count: 0,
        missing_references: Vec::new(),
        unused_references: Vec::new(),
    };

    assert_eq!(mira.academic_output_count(), 0);

    assert!(mira.run_verified_analysis(
        task_id,
        input,
        source_verification,
    ));

    assert_eq!(mira.academic_output_count(), 1);
    assert!(mira.has_academic_output());

    let output = mira
        .academic_outputs()
        .last()
        .expect("Mira akademik çıktıyı saklamalıdır.");

    assert!(output.is_ready_for_publication());
}









