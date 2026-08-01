use uuid::Uuid;

use crate::academic_runner::VerifiedAcademicRunnerOutput;

/// Bir akademik çıktıyı onu üreten Mira göreviyle ilişkilendirir.
#[derive(Debug, Clone)]
pub struct TaskAcademicOutput {
    pub task_id: Uuid,
    pub output: VerifiedAcademicRunnerOutput,
}

/// Bir yayın isteğini onu hazırlayan Mira göreviyle
/// açık biçimde ilişkilendirir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskPublicationRequestLink {
    pub task_id: Uuid,
    pub publication_request_id: Uuid,
}

impl TaskPublicationRequestLink {
    /// Mira görevi ile yayın isteği arasında yeni bağlantı oluşturur.
    pub fn new(
        task_id: Uuid,
        publication_request_id: Uuid,
    ) -> Self {
        Self {
            task_id,
            publication_request_id,
        }
    }

    /// Bağlantının belirtilen Mira görevine ait olup
    /// olmadığını bildirir.
    pub fn belongs_to_task(&self, task_id: Uuid) -> bool {
        self.task_id == task_id
    }

    /// Bağlantının belirtilen yayın isteğine ait olup
    /// olmadığını bildirir.
    pub fn references_publication_request(
        &self,
        publication_request_id: Uuid,
    ) -> bool {
        self.publication_request_id
            == publication_request_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_publication_request_to_mira_task() {
        let task_id = Uuid::new_v4();
        let publication_request_id = Uuid::new_v4();

        let link = TaskPublicationRequestLink::new(
            task_id,
            publication_request_id,
        );

        assert!(link.belongs_to_task(task_id));

        assert!(
            link.references_publication_request(
                publication_request_id,
            )
        );
    }

    #[test]
    fn rejects_unrelated_task_and_request_identifiers() {
        let link = TaskPublicationRequestLink::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
        );

        assert!(
            !link.belongs_to_task(Uuid::new_v4())
        );

        assert!(
            !link.references_publication_request(
                Uuid::new_v4(),
            )
        );
    }
}



