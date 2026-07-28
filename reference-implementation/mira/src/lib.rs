pub mod task;
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

pub use task::{MiraRiskLevel, MiraTask, MiraTaskStatus};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::academic_runner::{
    run_verified_academic_analysis,
    AcademicRunnerInput,
    VerifiedAcademicRunnerOutput,
};
use crate::source_verification_report::SourceVerificationReport;

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

/// Müdebbirin bir Mira görevi için verdiği kalıcı karar kaydı.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MudebbirDecisionRecord {
    pub task_id: Uuid,
    pub decision: MudebbirDecision,
    pub decided_at: DateTime<Utc>,
}

/// Bir akademik çıktıyı onu üreten Mira göreviyle ilişkilendirir.
#[derive(Debug, Clone)]
pub struct TaskAcademicOutput {
    pub task_id: Uuid,
    pub output: VerifiedAcademicRunnerOutput,
}

/// Mira’nın temel yönetim çekirdeği.
#[derive(Debug, Default)]
pub struct MiraCore {
    tasks: Vec<MiraTask>,
    rasterast_reports: Vec<RasterastReport>,
    academic_outputs: Vec<TaskAcademicOutput>,
    recommendations: Vec<MiraRecommendation>,
    mudebbir_decisions: Vec<MudebbirDecisionRecord>,
}

impl MiraCore {
    pub fn new() -> Self {
    Self {
        tasks: Vec::new(),
        rasterast_reports: Vec::new(),
        academic_outputs: Vec::new(),
        recommendations: Vec::new(),
        mudebbir_decisions: Vec::new(),
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

/// Mira tarafından saklanan ve görevlerle ilişkilendirilmiş
/// akademik çıktıları salt okunur olarak döndürür.
pub fn academic_outputs(&self) -> &[TaskAcademicOutput] {
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

/// Rasterast raporuna dayanarak Müdebbire sunulacak öneriyi oluşturur.
pub fn create_recommendation(
    &mut self,
    task_id: Uuid,
    rationale: impl Into<String>,
    benefits: Vec<String>,
    alternatives: Vec<String>,
    proposed_next_step: impl Into<String>,
) -> bool {
    let Some(task) = self.find_task(task_id) else {
        return false;
    };

    if task.status != MiraTaskStatus::AwaitingRasterast {
        return false;
    }

    let Some(report) = self
        .rasterast_reports
        .iter()
        .find(|report| report.task_id == task_id)
        .cloned()
    else {
        return false;
    };

    let recommendation = MiraRecommendation {
        task_id,
        rationale: rationale.into(),
        benefits,
        risks: report.risks.clone(),
        alternatives,
        rasterast_report: Some(report),
        proposed_next_step: proposed_next_step.into(),
        requires_mudebbir_approval: task.requires_mudebbir_approval,
    };

    self.recommendations.push(recommendation);
    true
}

/// Mira tarafından oluşturulan önerileri salt okunur döndürür.
pub fn recommendations(&self) -> &[MiraRecommendation] {
    &self.recommendations
}

/// Belirtilen göreve ait Mira önerisini bulur.
pub fn recommendation_for_task(
    &self,
    task_id: Uuid,
) -> Option<&MiraRecommendation> {
    self.recommendations
        .iter()
        .find(|recommendation| recommendation.task_id == task_id)
}
   
/// Rasterast raporu ve Mira önerisi bulunan görevi
/// Müdebbir kararı bekleme aşamasına geçirir.
pub fn await_mudebbir(&mut self, task_id: Uuid) -> bool {
    let has_rasterast_report = self
        .rasterast_reports
        .iter()
        .any(|report| report.task_id == task_id);

    if !has_rasterast_report {
        return false;
    }

    let has_recommendation = self
        .recommendations
        .iter()
        .any(|recommendation| recommendation.task_id == task_id);

    if !has_recommendation {
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

   /// Müdebbir onayını bekleyen görevi onaylar ve kararı kalıcı olarak kaydeder.
pub fn approve_task(&mut self, task_id: Uuid) -> bool {
    {
        let Some(task) = self.find_task_mut(task_id) else {
            return false;
        };

        if task.status != MiraTaskStatus::AwaitingMudebbir {
            return false;
        }

        task.update_status(MiraTaskStatus::Approved);
    }

    self.mudebbir_decisions.push(MudebbirDecisionRecord {
        task_id,
        decision: MudebbirDecision::Approved,
        decided_at: Utc::now(),
    });

    true
}


/// Akademik çıktıyı onu üreten görev kimliğiyle birlikte saklar.
pub fn store_academic_output(
    &mut self,
    task_id: Uuid,
    output: VerifiedAcademicRunnerOutput,
) {
    self.academic_outputs.push(TaskAcademicOutput {
        task_id,
        output,
    });
}

   /// Belirtilen Mira görevine ait akademik çıktıyı bulur.
pub fn academic_output_for_task(
    &self,
    task_id: Uuid,
) -> Option<&VerifiedAcademicRunnerOutput> {
    self.academic_outputs
        .iter()
        .find(|stored| stored.task_id == task_id)
        .map(|stored| &stored.output)
}
    
pub fn academic_output_count(&self) -> usize {
    self.academic_outputs.len()
}

pub fn has_academic_output(&self) -> bool {
    !self.academic_outputs.is_empty()
}

 /// Müdebbir tarafından onaylanmış görev için akademik üretim hattını
/// çalıştırır, çıktıyı saklar ve görevi tamamlanmış duruma geçirir.
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

    self.store_academic_output(task_id, output);

    let Some(task) = self.find_task_mut(task_id) else {
        return false;
    };

    task.update_status(MiraTaskStatus::Completed);
    true
}
 
/// Müdebbir onayını bekleyen görevi reddeder ve kararı kalıcı olarak kaydeder.
pub fn reject_task(&mut self, task_id: Uuid) -> bool {
    {
        let Some(task) = self.find_task_mut(task_id) else {
            return false;
        };

        if task.status != MiraTaskStatus::AwaitingMudebbir {
            return false;
        }

        task.update_status(MiraTaskStatus::Rejected);
    }

    self.mudebbir_decisions.push(MudebbirDecisionRecord {
        task_id,
        decision: MudebbirDecision::Rejected,
        decided_at: Utc::now(),
    });

    true
}

    /// Müdebbirin verdiği bütün karar kayıtlarını salt okunur döndürür.
pub fn mudebbir_decisions(&self) -> &[MudebbirDecisionRecord] {
    &self.mudebbir_decisions
}

/// Belirtilen göreve ait en son Müdebbir kararını bulur.
pub fn mudebbir_decision_for_task(
    &self,
    task_id: Uuid,
) -> Option<&MudebbirDecisionRecord> {
    self.mudebbir_decisions
        .iter()
        .rev()
        .find(|record| record.task_id == task_id)
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
    assert!(mira.create_recommendation(
    task_id,
    "Görev Rasterast doğrulamasından geçti.",
    vec!["Akademik üretim için uygun bulundu.".to_string()],
    vec!["Taslak üretimle sınırlı tutulabilir.".to_string()],
    "Müdebbir kararına sun.",
));
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
    assert!(mira.create_recommendation(
    task_id,
    "Görev Rasterast doğrulamasından geçti.",
    vec!["Akademik üretim için uygun bulundu.".to_string()],
    vec!["Taslak üretimle sınırlı tutulabilir.".to_string()],
    "Müdebbir kararına sun.",
));
    assert!(mira.await_mudebbir(task_id));
    assert!(mira.approve_task(task_id));

    let task = mira
        .find_task(task_id)
        .expect("Akademik görev kayıtlı olmalıdır.");

    assert_eq!(task.status, MiraTaskStatus::Approved);

assert_eq!(mira.mudebbir_decisions().len(), 1);

let decision = mira
    .mudebbir_decision_for_task(task_id)
    .expect("Göreve ait Müdebbir onay kaydı bulunmalıdır.");

assert_eq!(decision.task_id, task_id);
assert_eq!(decision.decision, MudebbirDecision::Approved);
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
    assert!(mira.create_recommendation(
    task_id,
    "Görev Rasterast doğrulamasından geçti.",
    vec!["Akademik üretim için uygun bulundu.".to_string()],
    vec!["Taslak üretimle sınırlı tutulabilir.".to_string()],
    "Müdebbir kararına sun.",
));
    assert!(mira.await_mudebbir(task_id));
    assert!(mira.reject_task(task_id));

    let task = mira
        .find_task(task_id)
        .expect("Akademik görev kayıtlı olmalıdır.");

    assert_eq!(task.status, MiraTaskStatus::Rejected);

assert_eq!(mira.mudebbir_decisions().len(), 1);

let decision = mira
    .mudebbir_decision_for_task(task_id)
    .expect("Göreve ait Müdebbir ret kaydı bulunmalıdır.");

assert_eq!(decision.task_id, task_id);
assert_eq!(decision.decision, MudebbirDecision::Rejected);
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
    assert!(mira.create_recommendation(
    task_id,
    "Görev Rasterast doğrulamasından geçti.",
    vec!["Akademik üretim için uygun bulundu.".to_string()],
    vec!["Taslak üretimle sınırlı tutulabilir.".to_string()],
    "Müdebbir kararına sun.",
));
    assert!(mira.await_mudebbir(task_id));
    assert!(mira.approve_task(task_id));

    assert!(mira.can_start_academic_pipeline(task_id));
}

#[test]
fn hebun_academic_task_completes_end_to_end() {
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
    assert!(mira.create_recommendation(
    task_id,
    "Görev Rasterast doğrulamasından geçti.",
    vec!["Akademik üretim için uygun bulundu.".to_string()],
    vec!["Taslak üretimle sınırlı tutulabilir.".to_string()],
    "Müdebbir kararına sun.",
));
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

    let stored_output = mira
    .academic_outputs()
    .last()
    .expect("Mira akademik çıktıyı görevle birlikte saklamalıdır.");

assert_eq!(stored_output.task_id, task_id);
assert!(stored_output.output.is_ready_for_publication());

let task_output = mira
    .academic_output_for_task(task_id)
    .expect("Göreve ait akademik çıktı bulunmalıdır.");

assert!(task_output.is_ready_for_publication());

assert_eq!(mira.academic_output_count(), 1);
assert!(mira.has_academic_output());

let stored_output = mira
    .academic_outputs()
    .last()
    .expect("Mira akademik çıktıyı görevle birlikte saklamalıdır.");

assert_eq!(stored_output.task_id, task_id);
assert!(stored_output.output.is_ready_for_publication());

let task_output = mira
    .academic_output_for_task(task_id)
    .expect("Göreve ait akademik çıktı bulunmalıdır.");

assert!(task_output.is_ready_for_publication());

let task = mira
    .find_task(task_id)
    .expect("Akademik görev Mira içinde kayıtlı olmalıdır.");

assert_eq!(task.status, MiraTaskStatus::Completed);
let recommendation = mira
    .recommendation_for_task(task_id)
    .expect("Hebûn görevi için Mira önerisi bulunmalıdır.");

assert_eq!(recommendation.task_id, task_id);
assert!(recommendation.rasterast_report.is_some());
assert!(recommendation.requires_mudebbir_approval);

let decision = mira
    .mudebbir_decision_for_task(task_id)
    .expect("Hebûn görevi için Müdebbir karar kaydı bulunmalıdır.");

assert_eq!(decision.task_id, task_id);
assert_eq!(decision.decision, MudebbirDecision::Approved);

let academic_output = mira
    .academic_output_for_task(task_id)
    .expect("Hebûn görevine bağlı akademik çıktı bulunmalıdır.");

assert!(academic_output.is_ready_for_publication());

}

#[test]
fn mira_creates_recommendation_from_rasterast_report() {
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
        verified_items: vec![
            "Akademik görev doğrulandı.".to_string(),
        ],
        unverified_items: Vec::new(),
        contradictions: Vec::new(),
        risks: vec![
            "Kaynakların yayın öncesinde yeniden kontrol edilmesi gerekir."
                .to_string(),
        ],
        requires_mudebbir_decision: true,
        created_at: Utc::now(),
    };

    assert!(mira.attach_rasterast_report(report));

    assert!(mira.create_recommendation(
        task_id,
        "Hebûn çalışması akademik üretim hattına alınmaya uygundur.",
        vec![
            "Hebûn kuramının matematiksel sunumunu güçlendirir."
                .to_string(),
        ],
        vec![
            "Makalenin önce yalnızca taslak olarak hazırlanması."
                .to_string(),
        ],
        "Müdebbir onayından sonra akademik üretim hattını çalıştır.",
    ));

    assert_eq!(mira.recommendations().len(), 1);

    let recommendation = mira
        .recommendation_for_task(task_id)
        .expect("Göreve ait Mira önerisi bulunmalıdır.");

    assert_eq!(recommendation.task_id, task_id);
    assert!(recommendation.rasterast_report.is_some());
    assert!(recommendation.requires_mudebbir_approval);
    assert_eq!(recommendation.risks.len(), 1);
}

#[test]
fn task_cannot_await_mudebbir_without_recommendation() {
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
        verified_items: vec![
            "Akademik görev doğrulandı.".to_string(),
        ],
        unverified_items: Vec::new(),
        contradictions: Vec::new(),
        risks: Vec::new(),
        requires_mudebbir_decision: true,
        created_at: Utc::now(),
    };

    assert!(mira.attach_rasterast_report(report));

    assert!(!mira.await_mudebbir(task_id));

    let task = mira
        .find_task(task_id)
        .expect("Akademik görev kayıtlı olmalıdır.");

    assert_eq!(task.status, MiraTaskStatus::AwaitingRasterast);
}





