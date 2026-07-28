use crate::academic_output::{
    generate_academic_output,
    AcademicOutput,
    AcademicOutputInput,
};
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
use crate::publication_package::{
    build_publication_package,
    PublicationPackage,
};
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
    pub output: AcademicOutput,
    pub publication_package: PublicationPackage,
}

/// Akademik analiz ile kaynak doğrulamasının birleşik sonucu.
#[derive(Debug, Clone)]
pub struct VerifiedAcademicRunnerOutput {
    pub academic: AcademicRunnerOutput,
    pub source_verification: SourceVerificationReport,
}

impl VerifiedAcademicRunnerOutput {
    /// Akademik yapı ve kaynaklar birlikte doğrulanmışsa yayın için hazırdır.
    pub fn is_ready_for_publication(&self) -> bool {
        self.academic.report.ready_for_publication
            && self.source_verification.is_verified()
    }

pub fn publication_package_with_bibtex(
    &self,
    bibtex: impl Into<String>,
) -> PublicationPackage {
    self.academic
        .publication_package
        .clone()
        .with_bibtex(bibtex)
   }
}

/// Akademik değerlendirme modüllerini tek akışta çalıştırır.
pub fn run_academic_analysis(
    input: AcademicRunnerInput,
) -> AcademicRunnerOutput {
    let article_type = input.article_type;
    
    let pipeline = run_pipeline(
        input.article_type,
        input.has_abstract,
        input.has_references,
        input.has_conclusion,
        input.has_math,
        input.has_experiments,
    );

    let report = build_report(&pipeline);

    let output = generate_academic_output(
        AcademicOutputInput {
            title: format!("{article_type:?}"),
            author: "Veysi yê MALA SAF".to_string(),
            abstract_text: String::new(),
            body: String::new(),
            bibliography: None,
        },
    );
let publication_package = build_publication_package(
    format!("{article_type:?}"),
    &output,
    None,
);

   AcademicRunnerOutput {
    pipeline,
    report,
    output,
    publication_package,
  }
}

/// Akademik analiz ile kaynak doğrulamasını tek çıktıda birleştirir.
pub fn run_verified_academic_analysis(
    input: AcademicRunnerInput,
    source_verification: SourceVerificationReport,
) -> VerifiedAcademicRunnerOutput {
    let academic = run_academic_analysis(input);

    VerifiedAcademicRunnerOutput {
        academic,
        source_verification,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::citation_reference_matcher::CitationReferenceMatchReport;
    use crate::publication_priority::PublicationPriority;

    #[test]
    fn complete_article_produces_publication_ready_report() {
        let output = run_academic_analysis(
            AcademicRunnerInput {
                article_type: AcademicArticleType::Mathematical,
                has_abstract: true,
                has_references: true,
                has_conclusion: true,
                has_math: true,
                has_experiments: false,
            },
        );

        assert_eq!(
            output.pipeline.priority,
            PublicationPriority::Critical
        );

        assert!(output.pipeline.rules.passed);
        assert!(output.report.ready_for_publication);
        assert!(output.report.recommendations.is_empty());
        assert!(output.output.is_valid);
        assert!(output.publication_package.is_complete());
        assert!(!output.publication_package.has_bibliography());
        assert!(!output.publication_package.is_ready_for_publication());
        
    }

    #[test]
    fn incomplete_academic_structure_prevents_publication_readiness() {
        let citation_report = CitationReferenceMatchReport {
            citation_numbers: vec![1],
            reference_numbers: vec![1],
            missing_references: Vec::new(),
            unused_references: Vec::new(),
        };

        let source_verification =
            SourceVerificationReport::from_validation_results(
                1,
                1,
                1,
                1,
                &citation_report,
            );

        let output = run_verified_academic_analysis(
            AcademicRunnerInput {
                article_type: AcademicArticleType::Theoretical,
                has_abstract: false,
                has_references: true,
                has_conclusion: true,
                has_math: false,
                has_experiments: false,
            },
            source_verification,
        );

        assert!(!output.academic.report.ready_for_publication);
        assert!(output.source_verification.is_verified());
        assert!(output.academic.output.is_valid);
        assert!(!output.is_ready_for_publication());
    }

    #[test]
    fn incomplete_article_produces_academic_warnings() {
        let output = run_academic_analysis(
            AcademicRunnerInput {
                article_type: AcademicArticleType::Theoretical,
                has_abstract: false,
                has_references: false,
                has_conclusion: true,
                has_math: false,
                has_experiments: false,
            },
        );

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

        assert!(output.output.is_valid);
    }

    #[test]
    fn verified_analysis_requires_valid_academic_and_source_results() {
        let citation_report = CitationReferenceMatchReport {
            citation_numbers: vec![1],
            reference_numbers: vec![1],
            missing_references: Vec::new(),
            unused_references: Vec::new(),
        };

        let source_verification =
            SourceVerificationReport::from_validation_results(
                1,
                1,
                1,
                1,
                &citation_report,
            );

        let output = run_verified_academic_analysis(
            AcademicRunnerInput {
                article_type: AcademicArticleType::Mathematical,
                has_abstract: true,
                has_references: true,
                has_conclusion: true,
                has_math: true,
                has_experiments: false,
            },
            source_verification,
        );

        assert!(output.academic.report.ready_for_publication);
        assert!(output.source_verification.is_verified());
        assert!(output.academic.output.is_valid);
        assert!(output.is_ready_for_publication());
    }

    #[test]
    fn invalid_sources_prevent_publication_readiness() {
        let citation_report = CitationReferenceMatchReport {
            citation_numbers: vec![1],
            reference_numbers: vec![1],
            missing_references: Vec::new(),
            unused_references: Vec::new(),
        };

        let source_verification =
            SourceVerificationReport::from_validation_results(
                1,
                0,
                1,
                1,
                &citation_report,
            );

        let output = run_verified_academic_analysis(
            AcademicRunnerInput {
                article_type: AcademicArticleType::Mathematical,
                has_abstract: true,
                has_references: true,
                has_conclusion: true,
                has_math: true,
                has_experiments: false,
            },
            source_verification,
        );

        assert!(output.academic.report.ready_for_publication);
        assert!(!output.source_verification.is_verified());
        assert!(output.academic.output.is_valid);
        assert!(!output.is_ready_for_publication());
    }

    #[test]
    fn complete_verified_pipeline_generates_valid_academic_output() {
        let citation_report = CitationReferenceMatchReport {
            citation_numbers: vec![1],
            reference_numbers: vec![1],
            missing_references: Vec::new(),
            unused_references: Vec::new(),
        };

        let source_verification =
            SourceVerificationReport::from_validation_results(
                1,
                1,
                1,
                1,
                &citation_report,
            );

        let output = run_verified_academic_analysis(
            AcademicRunnerInput {
                article_type: AcademicArticleType::Mathematical,
                has_abstract: true,
                has_references: true,
                has_conclusion: true,
                has_math: true,
                has_experiments: false,
            },
            source_verification,
        );

        assert!(output.academic.pipeline.rules.passed);
        assert!(output.academic.report.ready_for_publication);
        assert!(output.source_verification.is_verified());
        assert!(output.academic.output.is_valid);
        assert!(output.is_ready_for_publication());

        assert!(
            output
                .academic
                .output
                .latex_source
                .contains("\\begin{document}")
        );

        assert!(
            output
                .academic
                .output
                .latex_source
                .contains("\\end{document}")
        );

        assert!(!output.academic.output.pdf_bytes.is_empty());
    }

#[test]
fn verified_analysis_can_build_publishable_package() {
    let citation_report = CitationReferenceMatchReport {
        citation_numbers: vec![1],
        reference_numbers: vec![1],
        missing_references: Vec::new(),
        unused_references: Vec::new(),
    };

    let source_verification =
        SourceVerificationReport::from_validation_results(
            1,
            1,
            1,
            1,
            &citation_report,
        );

    let output = run_verified_academic_analysis(
        AcademicRunnerInput {
            article_type: AcademicArticleType::Mathematical,
            has_abstract: true,
            has_references: true,
            has_conclusion: true,
            has_math: true,
            has_experiments: false,
        },
        source_verification,
    );

    let package = output.publication_package_with_bibtex(
        "@article{rasterast2026}",
    );

    assert!(package.has_bibliography());
    assert!(package.is_complete());
    assert!(package.is_ready_for_publication());
    }

}

