use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Mira tarafından kullanılabilen uzman ajanlar.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExpertAgent {
    OpenAi,
    Gemini,
    Ollama,
    LlamaCpp,
}

/// Uzman ajana verilebilecek görev türleri.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentTaskType {
    AcademicStructure,
    Classification,
    SourceGapAnalysis,
    TerminologyProtection,
    IndependentVerification,
}

/// Mira tarafından uzman ajana hazırlanan görev.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentAssignment {
    pub assignment_id: Uuid,
    pub mira_task_id: Uuid,
    pub agent: ExpertAgent,
    pub task_type: AgentTaskType,
    pub instruction: String,
}

/// Göreve uygun uzman ajan dağılımını hazırlar.
///
/// Bu ilk sürüm yalnızca görev planı üretir.
/// Henüz dış yapay zekâ servislerine çağrı yapmaz.
#[derive(Debug, Default)]
pub struct AgentDispatcher;

impl AgentDispatcher {
    pub fn new() -> Self {
        Self
    }

    pub fn prepare_academic_review(
        &self,
        mira_task_id: Uuid,
        article_instruction: impl Into<String>,
    ) -> Vec<AgentAssignment> {
        let instruction = article_instruction.into();

        vec![
            AgentAssignment {
                assignment_id: Uuid::new_v4(),
                mira_task_id,
                agent: ExpertAgent::OpenAi,
                task_type: AgentTaskType::AcademicStructure,
                instruction: format!(
                    "Akademik yapıyı değerlendir. Orijinal metni değiştirme. {instruction}"
                ),
            },
            AgentAssignment {
                assignment_id: Uuid::new_v4(),
                mira_task_id,
                agent: ExpertAgent::Gemini,
                task_type: AgentTaskType::Classification,
                instruction: format!(
                    "Makale türünü, konu kümesini ve kaynak boşluklarını analiz et. {instruction}"
                ),
            },
            AgentAssignment {
                assignment_id: Uuid::new_v4(),
                mira_task_id,
                agent: ExpertAgent::Ollama,
                task_type: AgentTaskType::TerminologyProtection,
                instruction: format!(
                    "Hebûn, Zanabûn, Mabûn, Rabûn, Rasterast ve Newroza Kawa terminolojisini denetle. {instruction}"
                ),
            },
            AgentAssignment {
                assignment_id: Uuid::new_v4(),
                mira_task_id,
                agent: ExpertAgent::LlamaCpp,
                task_type: AgentTaskType::IndependentVerification,
                instruction: format!(
                    "Bağımsız ikinci doğrulama yap; çelişki ve riskleri raporla. {instruction}"
                ),
            },
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dispatcher_assigns_all_active_expert_agents() {
        let dispatcher = AgentDispatcher::new();
        let mira_task_id = Uuid::new_v4();

        let assignments = dispatcher.prepare_academic_review(
            mira_task_id,
            "Hebûn makale adayını incele.",
        );

        assert_eq!(assignments.len(), 4);

        assert!(assignments
            .iter()
            .any(|assignment| assignment.agent == ExpertAgent::OpenAi));

        assert!(assignments
            .iter()
            .any(|assignment| assignment.agent == ExpertAgent::Gemini));

        assert!(assignments
            .iter()
            .any(|assignment| assignment.agent == ExpertAgent::Ollama));

        assert!(assignments
            .iter()
            .any(|assignment| assignment.agent == ExpertAgent::LlamaCpp));
    }

    #[test]
    fn every_assignment_belongs_to_mira_task() {
        let dispatcher = AgentDispatcher::new();
        let mira_task_id = Uuid::new_v4();

        let assignments = dispatcher.prepare_academic_review(
            mira_task_id,
            "Rasterast doğrulaması için görev hazırla.",
        );

        assert!(assignments
            .iter()
            .all(|assignment| assignment.mira_task_id == mira_task_id));
    }
}



