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


