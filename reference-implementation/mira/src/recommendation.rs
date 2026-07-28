use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::rasterast::RasterastReport;

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



