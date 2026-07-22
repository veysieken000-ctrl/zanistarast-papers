use crate::academic_pipeline::AcademicPipelineResult;

#[derive(Debug, Clone)]
pub struct AcademicReport {
    pub ready_for_publication: bool,
    pub summary: String,
    pub recommendations: Vec<String>,
}

pub fn build_report(
    result: &AcademicPipelineResult,
) -> AcademicReport {
    let mut recommendations = Vec::new();

    if !result.rules.passed {
        recommendations.extend(result.rules.warnings.clone());
    }

    let ready = result.rules.passed;

    let summary = if ready {
        "Academic validation passed.".to_string()
    } else {
        "Academic validation requires improvements.".to_string()
    };

    AcademicReport {
        ready_for_publication: ready,
        summary,
        recommendations,
    }
}


