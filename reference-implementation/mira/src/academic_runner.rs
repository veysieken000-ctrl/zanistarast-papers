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
use crate::development_synthesis::DevelopmentSynthesisReport;
use crate::publication_package::{
    build_publication_package,
    PublicationPackage,
};
use crate::source_verification_report::SourceVerificationReport;

/// Bir makalenin akademik analizinde kullanılacak girdiler.
///
/// Bu yapı mevcut akademik üretim hattıyla uyumluluğu
/// korur. Sentez, Rasterast ve Müdebbir kapıları ayrı
/// çıktı katmanlarında uygulanır.
#[derive(Debug, Clone)]
pub struct AcademicRunnerInput {
    pub article_type: AcademicArticleType,
    pub has_abstract: bool,
    pub has_references: bool,
    pub has_conclusion: bool,
    pub has_math: bool,
    pub has_experiments: bool,
}

/// Akademik üretim hattının birleşik sonucudur.
#[derive(Debug, Clone)]
pub struct AcademicRunnerOutput {
    pub pipeline: AcademicPipelineResult,
    pub report: AcademicReport,
    pub output: AcademicOutput,
    pub publication_package: PublicationPackage,
}

impl AcademicRunnerOutput {
    /// Akademik yapı denetiminin başarıyla tamamlanıp
    /// tamamlanmadığını bildirir.
    pub fn has_valid_academic_structure(&self) -> bool {
        self.pipeline.rules.passed
            && self.report.ready_for_publication
            && self.output.is_valid
    }

    /// Kaynak doğrulaması ve Müdebbir kararı henüz bu
    /// aşamada bulunmadığından bu metot yalnızca akademik
    /// üretim katmanını değerlendirir.
    pub fn can_enter_source_verification(&self) -> bool {
        self.has_valid_academic_structure()
    }
}

/// Akademik analiz ile kaynak doğrulamasının birleşik
/// sonucudur.
///
/// Bu katman kaynakların ve akademik yapının doğrulanmasını
/// sağlar. Tek başına Müdebbir yayın onayı anlamına gelmez.
#[derive(Debug, Clone)]
pub struct VerifiedAcademicRunnerOutput {
    pub academic: AcademicRunnerOutput,
    pub source_verification: SourceVerificationReport,
}

impl VerifiedAcademicRunnerOutput {
    /// Akademik yapı ve kaynakların birlikte doğrulanıp
    /// doğrulanmadığını bildirir.
    ///
    /// Bu sonuç teknik yayın hazırlığını ifade eder;
    /// Müdebbir'in nihai yayın kararı değildir.
    pub fn is_ready_for_publication(&self) -> bool {
        self.academic.has_valid_academic_structure()
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

/// Akademik üretim, kaynak doğrulaması ve gelişim
/// sentezinin birlikte değerlendirildiği anayasal çıktıdır.
///
/// Bu yapı üç ayrı kapıyı birbirinden ayırır:
///
/// 1. Akademik yapı doğrulaması
/// 2. Kaynak ve Rasterast destekli sentez doğrulaması
/// 3. Müdebbir'in nihai kararı
#[derive(Debug, Clone)]
pub struct SynthesisVerifiedAcademicRunnerOutput {
    pub verified_academic: VerifiedAcademicRunnerOutput,
    pub development_synthesis: DevelopmentSynthesisReport,

    /// Bu alan yalnızca Müdebbir'in açık kararını temsil
    /// eder. Rasterast veya başka bir yapay zekâ sistemi
    /// bu alanı kendiliğinden true yapamaz.
    pub mudebbir_approved: bool,
}
impl SynthesisVerifiedAcademicRunnerOutput {
    /// Gelişim sentezinin eksiksiz, çelişkisiz ve
    /// Rasterast tarafından doğrulanmış olduğunu bildirir.
    pub fn has_verified_synthesis(&self) -> bool {
        self.development_synthesis
            .can_support_academic_synthesis()
    }

    /// Çalışmanın Müdebbir kararına sunulabilecek durumda
    /// olup olmadığını bildirir.
    ///
    /// Müdebbir onayı henüz verilmemiş olsa bile teknik ve
    /// epistemolojik kapılar tamamlanmış olabilir.
    pub fn can_await_mudebbir_decision(&self) -> bool {
        self.verified_academic.is_ready_for_publication()
            && self.has_verified_synthesis()
            && self
                .development_synthesis
                .requires_mudebbir_decision
    }

    /// Nihai yayın hazırlığı ancak Müdebbir'in açık
    /// onayıyla oluşur.
    pub fn is_ready_for_publication(&self) -> bool {
        self.can_await_mudebbir_decision()
            && self.mudebbir_approved
    }

    /// Müdebbir onayı olmadan yayın paketi üretmez.
    pub fn approved_publication_package_with_bibtex(
        &self,
        bibtex: impl Into<String>,
    ) -> Option<PublicationPackage> {
        if !self.is_ready_for_publication() {
            return None;
        }

        Some(
            self.verified_academic
                .publication_package_with_bibtex(bibtex),
        )
    }
}

/// Akademik değerlendirme modüllerini tek akışta
/// çalıştırır.
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

/// Akademik analiz ile kaynak doğrulamasını tek çıktıda
/// birleştirir.
///
/// Bu fonksiyon mevcut `MiraCore` çağrılarıyla uyumluluğu
/// korur.
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

/// Akademik analiz, kaynak doğrulaması ve gelişim sentezini
/// tek anayasal çıktı içinde birleştirir.
///
/// `mudebbir_approved` yalnızca Müdebbir'in gerçek ve açık
/// kararından alınmalıdır.
pub fn run_synthesis_verified_academic_analysis(
    input: AcademicRunnerInput,
    source_verification: SourceVerificationReport,
    development_synthesis: DevelopmentSynthesisReport,
    mudebbir_approved: bool,
) -> SynthesisVerifiedAcademicRunnerOutput {
    let verified_academic =
        run_verified_academic_analysis(
            input,
            source_verification,
        );

    SynthesisVerifiedAcademicRunnerOutput {
        verified_academic,
        development_synthesis,
        mudebbir_approved,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::citation_reference_matcher::
        CitationReferenceMatchReport;
    use crate::publication_package::
        export_publication_package;
    use crate::publication_priority::
        PublicationPriority;

    fn verified_source_report() -> SourceVerificationReport {
        let citation_report =
            CitationReferenceMatchReport {
                citation_numbers: vec![1],
                reference_numbers: vec![1],
                missing_references: Vec::new(),
                unused_references: Vec::new(),
            };

        SourceVerificationReport::from_validation_results(
            1,
            1,
            1,
            1,
            &citation_report,
        )
    }

    fn invalid_source_report() -> SourceVerificationReport {
        let citation_report =
            CitationReferenceMatchReport {
                citation_numbers: vec![1],
                reference_numbers: vec![1],
                missing_references: Vec::new(),
                unused_references: Vec::new(),
            };

        SourceVerificationReport::from_validation_results(
            1,
            0,
            1,
            1,
            &citation_report,
        )
    }

    fn complete_mathematical_input() -> AcademicRunnerInput {
        AcademicRunnerInput {
            article_type:
                AcademicArticleType::Mathematical,
            has_abstract: true,
            has_references: true,
            has_conclusion: true,
            has_math: true,
            has_experiments: false,
        }
    }

    #[test]
    fn complete_article_produces_publication_ready_report() {
        let output = run_academic_analysis(
            complete_mathematical_input(),
        );

        assert_eq!(
            output.pipeline.priority,
            PublicationPriority::Critical
        );
        assert!(output.pipeline.rules.passed);
        assert!(output.report.ready_for_publication);
        assert!(output.report.recommendations.is_empty());
        assert!(output.output.is_valid);
        assert!(output.has_valid_academic_structure());
        assert!(output.can_enter_source_verification());
        assert!(output.publication_package.is_complete());
        assert!(
            !output.publication_package.has_bibliography()
        );
        assert!(
            !output
                .publication_package
                .is_ready_for_publication()
        );
    }

    #[test]
    fn incomplete_academic_structure_prevents_readiness() {
        let output = run_verified_academic_analysis(
            AcademicRunnerInput {
                article_type:
                    AcademicArticleType::Theoretical,
                has_abstract: false,
                has_references: true,
                has_conclusion: true,
                has_math: false,
                has_experiments: false,
            },
            verified_source_report(),
        );

        assert!(
            !output.academic.report.ready_for_publication
        );
        assert!(
            output.source_verification.is_verified()
        );
        assert!(output.academic.output.is_valid);
        assert!(!output.is_ready_for_publication());
    }

    #[test]
    fn incomplete_article_produces_academic_warnings() {
        let output = run_academic_analysis(
            AcademicRunnerInput {
                article_type:
                    AcademicArticleType::Theoretical,
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
        assert!(!output.has_valid_academic_structure());
    }

    #[test]
    fn verified_analysis_requires_academic_and_sources() {
        let output = run_verified_academic_analysis(
            complete_mathematical_input(),
            verified_source_report(),
        );

        assert!(
            output.academic.report.ready_for_publication
        );
        assert!(
            output.source_verification.is_verified()
        );
        assert!(output.academic.output.is_valid);
        assert!(output.is_ready_for_publication());
    }

    #[test]
    fn invalid_sources_prevent_publication_readiness() {
        let output = run_verified_academic_analysis(
            complete_mathematical_input(),
            invalid_source_report(),
        );

        assert!(
            output.academic.report.ready_for_publication
        );
        assert!(
            !output.source_verification.is_verified()
        );
        assert!(output.academic.output.is_valid);
        assert!(!output.is_ready_for_publication());
    }
#[test]
    fn complete_pipeline_generates_academic_output() {
        let output = run_verified_academic_analysis(
            complete_mathematical_input(),
            verified_source_report(),
        );

        assert!(output.academic.pipeline.rules.passed);
        assert!(
            output.academic.report.ready_for_publication
        );
        assert!(
            output.source_verification.is_verified()
        );
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

        assert!(
            !output.academic.output.pdf_bytes.is_empty()
        );
    }

    #[test]
    fn verified_analysis_can_build_publishable_package() {
        let output = run_verified_academic_analysis(
            complete_mathematical_input(),
            verified_source_report(),
        );

        let package =
            output.publication_package_with_bibtex(
                "@article{rasterast2026}",
            );

        assert!(package.has_bibliography());
        assert!(package.is_complete());
        assert!(package.is_ready_for_publication());
    }

    #[test]
    fn verified_analysis_exports_complete_package() {
        let output = run_verified_academic_analysis(
            complete_mathematical_input(),
            verified_source_report(),
        );

        assert!(output.is_ready_for_publication());

        let package =
            output.publication_package_with_bibtex(
                "@article{rasterast2026}",
            );

        let output_directory =
            std::env::temp_dir().join(
                format!(
                    "mira-verified-publication-flow-{}",
                    std::process::id()
                ),
            );

        let output_directory_text = output_directory
            .to_str()
            .expect(
                "temporary directory should be valid UTF-8",
            );

        let written_files = export_publication_package(
            &package,
            output_directory_text,
            "Rasterast Verification 2026",
        )
        .expect(
            "verified publication package should be exported",
        );

        assert_eq!(written_files.len(), 3);

        assert!(
            output_directory
                .join(
                    "Rasterast_Verification_2026.tex"
                )
                .exists()
        );

        assert!(
            output_directory
                .join(
                    "Rasterast_Verification_2026.pdf"
                )
                .exists()
        );

        assert!(
            output_directory
                .join(
                    "Rasterast_Verification_2026.bib"
                )
                .exists()
        );

        std::fs::remove_dir_all(&output_directory)
            .expect(
                "temporary directory should be removed",
            );
    }

    #[test]
    fn verified_academic_output_is_not_mudebbir_approval() {
        let output = run_verified_academic_analysis(
            complete_mathematical_input(),
            verified_source_report(),
        );

        assert!(output.is_ready_for_publication());

        // Bu teknik hazırlık sonucudur.
        // Müdebbir onayı yalnızca
        // SynthesisVerifiedAcademicRunnerOutput içinde
        // ayrı olarak tutulur.
    }
}
