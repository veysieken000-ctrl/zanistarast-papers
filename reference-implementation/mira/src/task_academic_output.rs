use uuid::Uuid;

use crate::academic_runner::VerifiedAcademicRunnerOutput;

/// Bir akademik çıktıyı onu üreten Mira göreviyle ilişkilendirir.
#[derive(Debug, Clone)]
pub struct TaskAcademicOutput {
    pub task_id: Uuid,
    pub output: VerifiedAcademicRunnerOutput,
}


