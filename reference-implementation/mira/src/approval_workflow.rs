use crate::{
    MiraCore,
    MiraTaskStatus,
    MudebbirDecision,
    RasterastReport,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Müdebbire sunulan tek bir onay paketi.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRequest {
    pub request_id: Uuid,
    pub task_id: Uuid,
    pub summary: String,
    pub rationale: String,
    pub benefits: Vec<String>,
    pub risks: Vec<String>,
    pub rasterast_report: RasterastReport,
    pub decision: MudebbirDecision,
    pub created_at: DateTime<Utc>,
    pub decided_at: Option<DateTime<Utc>>,
    pub decision_note: Option<String>,
}

impl ApprovalRequest {
    /// Yeni bir Müdebbir onay talebi oluşturur.
    pub fn new(
        task_id: Uuid,
        summary: impl Into<String>,
        rationale: impl Into<String>,
        benefits: Vec<String>,
        risks: Vec<String>,
        rasterast_report: RasterastReport,
    ) -> Self {
        Self {
            request_id: Uuid::new_v4(),
            task_id,
            summary: summary.into(),
            rationale: rationale.into(),
            benefits,
            risks,
            rasterast_report,
            decision: MudebbirDecision::Pending,
            created_at: Utc::now(),
            decided_at: None,
            decision_note: None,
        }
    }

    /// Onay talebinin hâlâ karar bekleyip beklemediğini bildirir.
    pub fn is_pending(&self) -> bool {
        self.decision == MudebbirDecision::Pending
    }

    /// Müdebbirin kararını kaydeder.
    pub fn record_decision(
        &mut self,
        decision: MudebbirDecision,
        decision_note: Option<String>,
    ) -> bool {
        if !self.is_pending() {
            return false;
        }

        if decision == MudebbirDecision::Pending {
            return false;
        }

        self.decision = decision;
        self.decision_note = decision_note;
        self.decided_at = Some(Utc::now());

        true
    }
}

/// Müdebbir onay taleplerini yöneten güvenli iş akışı.
///
/// Bu katman:
/// - Müdebbirin yerine karar vermez,
/// - bekleyen talebi otomatik onaylamaz,
/// - aynı talebin kararını sonradan sessizce değiştirmez,
/// - verilen kararı Mira görevinin durumuna uygular,
/// - yayınlama veya dosya değiştirme işlemi yapmaz.
#[derive(Debug, Default)]
pub struct ApprovalWorkflow {
    requests: Vec<ApprovalRequest>,
}

impl ApprovalWorkflow {
    pub fn new() -> Self {
        Self {
            requests: Vec::new(),
        }
    }

    /// Yeni onay talebini kaydeder.
    pub fn submit(&mut self, request: ApprovalRequest) -> Uuid {
        let request_id = request.request_id;
        self.requests.push(request);
        request_id
    }

    /// Bütün onay taleplerini salt okunur döndürür.
    pub fn requests(&self) -> &[ApprovalRequest] {
        &self.requests
    }

    /// Kimliğine göre onay talebini bulur.
    pub fn find_request(
        &self,
        request_id: Uuid,
    ) -> Option<&ApprovalRequest> {
        self.requests
            .iter()
            .find(|request| request.request_id == request_id)
    }

    /// Kimliğine göre onay talebini değiştirilebilir olarak bulur.
    pub fn find_request_mut(
        &mut self,
        request_id: Uuid,
    ) -> Option<&mut ApprovalRequest> {
        self.requests
            .iter_mut()
            .find(|request| request.request_id == request_id)
    }

    /// Müdebbir kararını kaydeder ve ilgili Mira görevine uygular.
    pub fn decide(
        &mut self,
        mira: &mut MiraCore,
        request_id: Uuid,
        decision: MudebbirDecision,
        decision_note: Option<String>,
    ) -> bool {
        let Some(request) = self.find_request_mut(request_id) else {
            return false;
        };

        if !request.record_decision(
            decision.clone(),
            decision_note,
        ) {
            return false;
        }

        let Some(task) = mira.find_task_mut(request.task_id) else {
            return false;
        };

        let status = match decision {
            MudebbirDecision::Pending => {
                MiraTaskStatus::AwaitingMudebbir
            }
            MudebbirDecision::Approved => {
                MiraTaskStatus::Approved
            }
            MudebbirDecision::Rejected => {
                MiraTaskStatus::Rejected
            }
            MudebbirDecision::RevisionRequested => {
                MiraTaskStatus::Planning
            }
        };

        task.update_status(status);

        true
    }

    /// Karar bekleyen talepleri döndürür.
    pub fn pending_requests(&self) -> Vec<&ApprovalRequest> {
        self.requests
            .iter()
            .filter(|request| request.is_pending())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        MiraRiskLevel,
        MiraTask,
    };

    fn rasterast_report(task_id: Uuid) -> RasterastReport {
        RasterastReport {
            task_id,
            verified: false,
            verified_items: vec![
                "Uzman çıktıları alındı.".to_string(),
            ],
            unverified_items: vec![
                "Bilimsel iddialar kaynak bakımından doğrulanmalıdır."
                    .to_string(),
            ],
            contradictions: Vec::new(),
            risks: vec![
                "Müdebbir incelemesi gereklidir.".to_string(),
            ],
            requires_mudebbir_decision: true,
            created_at: Utc::now(),
        }
    }

    fn approval_request(task_id: Uuid) -> ApprovalRequest {
        ApprovalRequest::new(
            task_id,
            "Hebûn makale adayının akademik dönüşüm önerisi",
            "Makalenin yapısal düzeninin güçlendirilmesi öneriliyor.",
            vec![
                "Akademik okunabilirlik artabilir.".to_string(),
            ],
            vec![
                "Özgün vurgu kaybı riski bulunmaktadır.".to_string(),
            ],
            rasterast_report(task_id),
        )
    }

    #[test]
    fn approval_request_starts_as_pending() {
        let task_id = Uuid::new_v4();
        let request = approval_request(task_id);

        assert!(request.is_pending());
        assert_eq!(
            request.decision,
            MudebbirDecision::Pending
        );
        assert!(request.decided_at.is_none());
    }

    #[test]
    fn approved_decision_updates_mira_task() {
        let mut mira = MiraCore::new();

        let task = MiraTask::new(
            "Hebûn dönüşüm incelemesi",
            "Akademik dönüşüm taslağını incele.",
            MiraRiskLevel::High,
            true,
        );

        let task_id = mira.register_task(task);

        mira.request_mudebbir_approval(task_id);

        let mut workflow = ApprovalWorkflow::new();
        let request_id =
            workflow.submit(approval_request(task_id));

        let decided = workflow.decide(
            &mut mira,
            request_id,
            MudebbirDecision::Approved,
            Some("Taslak aşaması için onaylandı.".to_string()),
        );

        assert!(decided);

        let task = mira
            .find_task(task_id)
            .expect("task should exist");

        assert_eq!(
            task.status,
            MiraTaskStatus::Approved
        );

        assert!(workflow.pending_requests().is_empty());
    }

    #[test]
    fn revision_request_returns_task_to_planning() {
        let mut mira = MiraCore::new();

        let task = MiraTask::new(
            "Kaynak doğrulama",
            "Eksik kaynakları incele.",
            MiraRiskLevel::Medium,
            true,
        );

        let task_id = mira.register_task(task);

        mira.request_mudebbir_approval(task_id);

        let mut workflow = ApprovalWorkflow::new();
        let request_id =
            workflow.submit(approval_request(task_id));

        let decided = workflow.decide(
            &mut mira,
            request_id,
            MudebbirDecision::RevisionRequested,
            Some(
                "Kaynaklar doğrulandıktan sonra yeniden sun."
                    .to_string(),
            ),
        );

        assert!(decided);

        let task = mira
            .find_task(task_id)
            .expect("task should exist");

        assert_eq!(
            task.status,
            MiraTaskStatus::Planning
        );
    }

    #[test]
    fn decided_request_cannot_be_decided_again() {
        let mut mira = MiraCore::new();

        let task = MiraTask::new(
            "Yayın hazırlığı",
            "Yayın paketini incele.",
            MiraRiskLevel::Critical,
            true,
        );

        let task_id = mira.register_task(task);

        let mut workflow = ApprovalWorkflow::new();
        let request_id =
            workflow.submit(approval_request(task_id));

        assert!(workflow.decide(
            &mut mira,
            request_id,
            MudebbirDecision::Rejected,
            Some("Yayın için hazır değil.".to_string()),
        ));

        assert!(!workflow.decide(
            &mut mira,
            request_id,
            MudebbirDecision::Approved,
            None,
        ));
    }

    #[test]
    fn pending_decision_is_not_accepted_as_final_decision() {
        let mut mira = MiraCore::new();

        let task = MiraTask::new(
            "Makale incelemesi",
            "Makale adayını incele.",
            MiraRiskLevel::High,
            true,
        );

        let task_id = mira.register_task(task);

        let mut workflow = ApprovalWorkflow::new();
        let request_id =
            workflow.submit(approval_request(task_id));

        assert!(!workflow.decide(
            &mut mira,
            request_id,
            MudebbirDecision::Pending,
            None,
        ));

        assert_eq!(
            workflow.pending_requests().len(),
            1
        );
    }
}



