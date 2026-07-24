use crate::academic_pipeline::{
    run_pipeline,
    AcademicPipelineResult,
};
use crate::academic_report::{
    build_report,
    AcademicReport,
};
use crate::article_classifier::AcademicArticleType;
use crate::source_verification_report::SourceVerificationReport;

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

/// Akademik analiz ile kaynak doğrulamasının birleşik sonucu.
#[derive(Debug, Clone)]
pub struct VerifiedAcademicRunnerOutput {
    pub academic: AcademicRunnerOutput,
    pub source_verification: SourceVerificationReport,
}

impl VerifiedAcademicRunnerOutput {
    pub fn is_ready_for_publication(&self) -> bool {
        self.academic.report.ready_for_publication
            && self.source_verification.is_verified()
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
