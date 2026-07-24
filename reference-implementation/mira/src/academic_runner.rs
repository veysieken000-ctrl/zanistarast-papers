use crate::academic_pipeline::{
    run_pipeline,
    AcademicPipelineResult,
};
use crate::academic_report::{
    build_report,
    AcademicReport,
};
use crate::article_classifier::AcademicArticleType;

/// Bir makalenin akademik analizinde kullanılacak girdiler.
#[derive(Debug, Clone)]
pub struct AcademicRunnerInput {
    pub article_type: AcademicArticleType,
    pub has_abstract: bool,
    pub has_references: bool,
    pub has_conclusion: bool,
    pub has_math: bool,
    pub has_experiments: bool,
}

/// Akademik üretim hattının birleşik sonucu.
#[derive(Debug, Clone)]
pub struct AcademicRunnerOutput {
    pub pipeline: AcademicPipelineResult,
    pub report: AcademicReport,
}

/// Akademik değerlendirme modüllerini tek akışta çalıştırır.
pub fn run_academic_analysis(
    input: AcademicRunnerInput,
) -> AcademicRunnerOutput {
    let pipeline = run_pipeline(
        input.article_type,
        input.has_abstract,
        input.has_references,
        input.has_conclusion,
        input.has_math,
        input.has_experiments,
    );

    let report = build_report(&pipeline);

    AcademicRunnerOutput {
        pipeline,
        report,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::publication_priority::PublicationPriority;

    #[test]
    fn complete_article_produces_publication_ready_report() {
        let output = run_academic_analysis(AcademicRunnerInput {
            article_type: AcademicArticleType::Mathematical,
            has_abstract: true,
            has_references: true,
            has_conclusion: true,
            has_math: true,
            has_experiments: false,
        });

        assert_eq!(
            output.pipeline.priority,
            PublicationPriority::Critical
        );

        assert!(output.pipeline.rules.passed);
        assert!(output.report.ready_for_publication);
        assert!(output.report.recommendations.is_empty());
    }

    #[test]
    fn incomplete_article_produces_academic_warnings() {
        let output = run_academic_analysis(AcademicRunnerInput {
            article_type: AcademicArticleType::Theoretical,
            has_abstract: false,
            has_references: false,
            has_conclusion: true,
            has_math: false,
            has_experiments: false,
        });

        assert_eq!(
            output.pipeline.priority,
            PublicationPriority::Medium
        );

        assert!(!output.pipeline.rules.passed);
        assert!(!output.report.ready_for_publication);

        assert_eq!(
            output.report.recommendations,
            vec![
                "Missing Abstract".to_string(),
                "Missing References".to_string(),
            ]
        );
    }
}
