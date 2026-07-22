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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::academic_pipeline::AcademicPipelineResult;
    use crate::academic_rules::AcademicRuleReport;
    use crate::publication_priority::PublicationPriority;

    #[test]
    fn passing_rules_produce_publication_ready_report() {
        let pipeline_result = AcademicPipelineResult {
            priority: PublicationPriority::High,
            rules: AcademicRuleReport {
                passed: true,
                warnings: Vec::new(),
            },
        };

        let report = build_report(&pipeline_result);

        assert!(report.ready_for_publication);
        assert_eq!(report.summary, "Academic validation passed.");
        assert!(report.recommendations.is_empty());
    }

    #[test]
    fn failing_rules_produce_improvement_report() {
        let pipeline_result = AcademicPipelineResult {
            priority: PublicationPriority::Medium,
            rules: AcademicRuleReport {
                passed: false,
                warnings: vec![
                    "Missing Abstract".to_string(),
                    "Missing References".to_string(),
                ],
            },
        };

        let report = build_report(&pipeline_result);

        assert!(!report.ready_for_publication);
        assert_eq!(
            report.summary,
            "Academic validation requires improvements."
        );
        assert_eq!(
            report.recommendations,
            vec![
                "Missing Abstract".to_string(),
                "Missing References".to_string(),
            ]
        );
    }
}


