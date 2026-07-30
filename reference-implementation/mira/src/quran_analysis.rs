/// Kur'an ayetinin insan tarafından analiz edilme sürecinin
/// mevcut durumudur.
///
/// Bu durum Kur'an'ın doğruluk durumunu değil, insan
/// analizinin tamamlanma ve doğrulanma durumunu gösterir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuranAnalysisStatus {
    NotStarted,
    InProgress,
    RequiresMoreEvidence,
    RequiresArabicReview,
    RequiresLinguisticReview,
    RequiresContextReview,
    RequiresRelatedVerseReview,
    RequiresRisaleReview,
    RequiresHadithReview,
    RequiresCreationBookReview,
    RequiresFitrahReview,
    RequiresRationalReview,
    RequiresRasterastReview,
    AwaitingMudebbirDecision,
    Completed,
}

/// Kur'an analizinde uygulanabilecek temel kontrol
/// alanlarıdır.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuranReviewArea {
    ArabicOriginalText,
    VerseReference,
    Translation,
    RootAnalysis,
    Grammar,
    SemanticRange,
    ImmediateContext,
    SurahContext,
    QuranicWhole,
    RelatedVerses,
    RevelationContext,
    AuthenticHadith,
    RisaleNurOriginalText,
    ClassicalTafsir,
    ContemporaryTafsir,
    CreationBook,
    Fitrah,
    Reason,
    Logic,
    Observation,
    Experiment,
    Mathematics,
    ZanistarastInterpretation,
}

/// Bir ayete ilişkin insan yorumunun güven durumudur.
///
/// Ayetin hakikat statüsü ile insan yorumunun güven seviyesi
/// birbirinden ayrıdır.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterpretationConfidence {
    /// Ayetin açık lafzına doğrudan dayanır.
    ExplicitText,

    /// Ayet bütünlüğü, dil ve güçlü kaynaklarla sağlam
    /// biçimde desteklenir.
    StronglyGrounded,

    /// Makul delillere dayanır; başka yorumlar mümkündür.
    Probable,

    /// Ek dilsel, bağlamsal veya kaynak araştırması gerekir.
    NeedsReview,

    /// Mevcut çıkarım ayetin anlamını aşmaktadır.
    Overextended,

    /// Kaynak ve bağlamla çelişen insan yorumu.
    RejectedInterpretation,
}

/// Ayet analizindeki tek bir kontrolün sonucudur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuranReviewFinding {
    pub area: QuranReviewArea,
    pub reviewed: bool,
    pub finding: String,
    pub risks: Vec<String>,
    pub open_questions: Vec<String>,
}

impl QuranReviewFinding {
    pub fn new(
        area: QuranReviewArea,
        finding: impl Into<String>,
    ) -> Self {
        Self {
            area,
            reviewed: false,
            finding: finding.into(),
            risks: Vec::new(),
            open_questions: Vec::new(),
        }
    }

    pub fn mark_reviewed(mut self) -> Self {
        self.reviewed = true;
        self
    }

    pub fn with_risks(mut self, risks: Vec<String>) -> Self {
        self.risks = risks;
        self
    }

    pub fn with_open_questions(
        mut self,
        open_questions: Vec<String>,
    ) -> Self {
        self.open_questions = open_questions;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.finding.trim().is_empty()
    }

    pub fn has_unresolved_items(&self) -> bool {
        !self.risks.is_empty() || !self.open_questions.is_empty()
    }
}

/// Risale-i Nur'un orijinal metninden yapılan bir analiz
/// kaydıdır.
///
/// Risale-i Nur, Kur'an ile özdeşleştirilmez; Kur'an'ın
/// okunması ve ispat yollarının anlaşılması bakımından
/// başlıca yorum ve yöntem referansı olarak incelenir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RisaleAnalysisReference {
    pub work_name: String,
    pub section_reference: String,
    pub original_text: String,
    pub proof_method: String,
    pub zanistarast_analysis: String,
    pub interpretation_risks: Vec<String>,
}

impl RisaleAnalysisReference {
    pub fn new(
        work_name: impl Into<String>,
        section_reference: impl Into<String>,
    ) -> Self {
        Self {
            work_name: work_name.into(),
            section_reference: section_reference.into(),
            original_text: String::new(),
            proof_method: String::new(),
            zanistarast_analysis: String::new(),
            interpretation_risks: Vec::new(),
        }
    }

    pub fn with_original_text(
        mut self,
        original_text: impl Into<String>,
    ) -> Self {
        self.original_text = original_text.into();
        self
    }

    pub fn with_proof_method(
        mut self,
        proof_method: impl Into<String>,
    ) -> Self {
        self.proof_method = proof_method.into();
        self
    }

    pub fn with_zanistarast_analysis(
        mut self,
        zanistarast_analysis: impl Into<String>,
    ) -> Self {
        self.zanistarast_analysis =
            zanistarast_analysis.into();
        self
    }

    pub fn with_interpretation_risks(
        mut self,
        interpretation_risks: Vec<String>,
    ) -> Self {
        self.interpretation_risks = interpretation_risks;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.work_name.trim().is_empty()
            && !self.section_reference.trim().is_empty()
            && !self.original_text.trim().is_empty()
    }

    pub fn separates_original_from_analysis(&self) -> bool {
        self.zanistarast_analysis.trim().is_empty()
            || self.original_text.trim()
                != self.zanistarast_analysis.trim()
    }
}

/// Ayetle ilişkilendirilen hadis kaydıdır.
///
/// Hadislerin sıhhat değerlendirmesi insan ilmî çalışmasına
/// dayandığı için kaynak ve sıhhat bilgisi ayrıca tutulur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuranHadithReference {
    pub reference: String,
    pub text: String,
    pub authenticity_grade: String,
    pub authenticity_source: String,
    pub relation_to_verse: String,
    pub review_notes: Vec<String>,
}

impl QuranHadithReference {
    pub fn new(
        reference: impl Into<String>,
        authenticity_grade: impl Into<String>,
    ) -> Self {
        Self {
            reference: reference.into(),
            text: String::new(),
            authenticity_grade: authenticity_grade.into(),
            authenticity_source: String::new(),
            relation_to_verse: String::new(),
            review_notes: Vec::new(),
        }
    }

    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn with_authenticity_source(
        mut self,
        authenticity_source: impl Into<String>,
    ) -> Self {
        self.authenticity_source =
            authenticity_source.into();
        self
    }

    pub fn with_relation_to_verse(
        mut self,
        relation_to_verse: impl Into<String>,
    ) -> Self {
        self.relation_to_verse =
            relation_to_verse.into();
        self
    }

    pub fn with_review_notes(
        mut self,
        review_notes: Vec<String>,
    ) -> Self {
        self.review_notes = review_notes;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.reference.trim().is_empty()
            && !self.text.trim().is_empty()
            && !self.authenticity_grade.trim().is_empty()
            && !self.authenticity_source.trim().is_empty()
    }
}

/// Kur'an-ı Kerim'deki bir ayetin insan tarafından yapılan
/// analiz kaydıdır.
///
/// Bu yapı Kur'an'ın doğruluğunu sınamaz. Ayetin metninin,
/// bağlamının ve insan yorumunun doğru ele alınıp
/// alınmadığını denetler.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuranAnalysis {
    pub analysis_id: String,
    pub verse_reference: String,
    pub arabic_text: String,
    pub verified_translation: String,
    pub topic: String,
    pub status: QuranAnalysisStatus,
    pub interpretation_confidence: InterpretationConfidence,

    pub immediate_context: Vec<String>,
    pub related_verses: Vec<String>,
    pub linguistic_notes: Vec<String>,
    pub root_analysis: Vec<String>,
    pub semantic_alternatives: Vec<String>,
    pub revelation_context: Vec<String>,

    pub related_hadiths: Vec<QuranHadithReference>,
    pub risale_references: Vec<RisaleAnalysisReference>,

    pub creation_book_observations: Vec<String>,
    pub fitrah_evidence: Vec<String>,
    pub rational_arguments: Vec<String>,
    pub logical_arguments: Vec<String>,
    pub mathematical_models: Vec<String>,
    pub empirical_research: Vec<String>,

    pub human_interpretation: String,
    pub zanistarast_interpretation: String,
    pub alternative_interpretations: Vec<String>,
    pub review_findings: Vec<QuranReviewFinding>,

    pub contradictions: Vec<String>,
    pub interpretation_risks: Vec<String>,
    pub open_questions: Vec<String>,

    pub rasterast_verified: bool,
    pub requires_mudebbir_decision: bool,
}

impl QuranAnalysis {
    pub fn new(
        analysis_id: impl Into<String>,
        verse_reference: impl Into<String>,
        topic: impl Into<String>,
    ) -> Self {
        Self {
            analysis_id: analysis_id.into(),
            verse_reference: verse_reference.into(),
            arabic_text: String::new(),
            verified_translation: String::new(),
            topic: topic.into(),
            status: QuranAnalysisStatus::NotStarted,
            interpretation_confidence:
                InterpretationConfidence::NeedsReview,

            immediate_context: Vec::new(),
            related_verses: Vec::new(),
            linguistic_notes: Vec::new(),
            root_analysis: Vec::new(),
            semantic_alternatives: Vec::new(),
            revelation_context: Vec::new(),

            related_hadiths: Vec::new(),
            risale_references: Vec::new(),

            creation_book_observations: Vec::new(),
            fitrah_evidence: Vec::new(),
            rational_arguments: Vec::new(),
            logical_arguments: Vec::new(),
            mathematical_models: Vec::new(),
            empirical_research: Vec::new(),

            human_interpretation: String::new(),
            zanistarast_interpretation: String::new(),
            alternative_interpretations: Vec::new(),
            review_findings: Vec::new(),

            contradictions: Vec::new(),
            interpretation_risks: Vec::new(),
            open_questions: Vec::new(),

            rasterast_verified: false,
            requires_mudebbir_decision: true,
        }
    }

    pub fn with_arabic_text(
        mut self,
        arabic_text: impl Into<String>,
    ) -> Self {
        self.arabic_text = arabic_text.into();
        self
    }

    pub fn with_verified_translation(
        mut self,
        verified_translation: impl Into<String>,
    ) -> Self {
        self.verified_translation =
            verified_translation.into();
        self
    }

    pub fn with_status(
        mut self,
        status: QuranAnalysisStatus,
    ) -> Self {
        self.status = status;
        self
    }

    pub fn with_interpretation_confidence(
        mut self,
        interpretation_confidence: InterpretationConfidence,
    ) -> Self {
        self.interpretation_confidence =
            interpretation_confidence;
        self
    }

    pub fn with_immediate_context(
        mut self,
        immediate_context: Vec<String>,
    ) -> Self {
        self.immediate_context = immediate_context;
        self
    }

    pub fn with_related_verses(
        mut self,
        related_verses: Vec<String>,
    ) -> Self {
        self.related_verses = related_verses;
        self
    }

    pub fn with_linguistic_notes(
        mut self,
        linguistic_notes: Vec<String>,
    ) -> Self {
        self.linguistic_notes = linguistic_notes;
        self
    }

    pub fn with_root_analysis(
        mut self,
        root_analysis: Vec<String>,
    ) -> Self {
        self.root_analysis = root_analysis;
        self
    }

    pub fn with_semantic_alternatives(
        mut self,
        semantic_alternatives: Vec<String>,
    ) -> Self {
        self.semantic_alternatives =
            semantic_alternatives;
        self
    }

    pub fn with_revelation_context(
        mut self,
        revelation_context: Vec<String>,
    ) -> Self {
        self.revelation_context = revelation_context;
        self
    }

    pub fn with_related_hadiths(
        mut self,
        related_hadiths: Vec<QuranHadithReference>,
    ) -> Self {
        self.related_hadiths = related_hadiths;
        self
    }

    pub fn with_risale_references(
        mut self,
        risale_references: Vec<RisaleAnalysisReference>,
    ) -> Self {
        self.risale_references = risale_references;
        self
    }

    pub fn with_creation_book_observations(
        mut self,
        observations: Vec<String>,
    ) -> Self {
        self.creation_book_observations = observations;
        self
    }

    pub fn with_fitrah_evidence(
        mut self,
        fitrah_evidence: Vec<String>,
    ) -> Self {
        self.fitrah_evidence = fitrah_evidence;
        self
    }

    pub fn with_rational_arguments(
        mut self,
        rational_arguments: Vec<String>,
    ) -> Self {
        self.rational_arguments = rational_arguments;
        self
    }

    pub fn with_logical_arguments(
        mut self,
        logical_arguments: Vec<String>,
    ) -> Self {
        self.logical_arguments = logical_arguments;
        self
    }

    pub fn with_mathematical_models(
        mut self,
        mathematical_models: Vec<String>,
    ) -> Self {
        self.mathematical_models = mathematical_models;
        self
    }

    pub fn with_empirical_research(
        mut self,
        empirical_research: Vec<String>,
    ) -> Self {
        self.empirical_research = empirical_research;
        self
    }

    pub fn with_human_interpretation(
        mut self,
        human_interpretation: impl Into<String>,
    ) -> Self {
        self.human_interpretation =
            human_interpretation.into();
        self
    }

    pub fn with_zanistarast_interpretation(
        mut self,
        zanistarast_interpretation: impl Into<String>,
    ) -> Self {
        self.zanistarast_interpretation =
            zanistarast_interpretation.into();
        self
    }

    pub fn with_alternative_interpretations(
        mut self,
        alternative_interpretations: Vec<String>,
    ) -> Self {
        self.alternative_interpretations =
            alternative_interpretations;
        self
    }

    pub fn with_review_findings(
        mut self,
        review_findings: Vec<QuranReviewFinding>,
    ) -> Self {
        self.review_findings = review_findings;
        self
    }

    pub fn with_contradictions(
        mut self,
        contradictions: Vec<String>,
    ) -> Self {
        self.contradictions = contradictions;
        self
    }

    pub fn with_interpretation_risks(
        mut self,
        interpretation_risks: Vec<String>,
    ) -> Self {
        self.interpretation_risks =
            interpretation_risks;
        self
    }

    pub fn with_open_questions(
        mut self,
        open_questions: Vec<String>,
    ) -> Self {
        self.open_questions = open_questions;
        self
    }

    pub fn mark_rasterast_verified(mut self) -> Self {
        self.rasterast_verified = true;
        self
    }

    pub fn is_identity_complete(&self) -> bool {
        !self.analysis_id.trim().is_empty()
            && !self.verse_reference.trim().is_empty()
            && !self.arabic_text.trim().is_empty()
            && !self.verified_translation.trim().is_empty()
            && !self.topic.trim().is_empty()
    }

    /// Kur'an'ın kendi metni ile Zanistarast yorumunun
    /// birbirine karıştırılmasını engeller.
    pub fn separates_quran_from_zanistarast_interpretation(
        &self,
    ) -> bool {
        self.zanistarast_interpretation.trim().is_empty()
            || self.arabic_text.trim()
                != self.zanistarast_interpretation.trim()
    }

    /// Genel insan yorumuyla Zanistarast'ın özel yorumunun
    /// açık biçimde ayrılıp ayrılmadığını denetler.
    pub fn separates_human_and_zanistarast_interpretation(
        &self,
    ) -> bool {
        self.human_interpretation.trim().is_empty()
            || self.zanistarast_interpretation.trim().is_empty()
            || self.human_interpretation.trim()
                != self.zanistarast_interpretation.trim()
    }

    pub fn has_context_review(&self) -> bool {
        !self.immediate_context.is_empty()
            || self.review_findings.iter().any(|finding| {
                finding.reviewed
                    && matches!(
                        finding.area,
                        QuranReviewArea::ImmediateContext
                            | QuranReviewArea::SurahContext
                            | QuranReviewArea::QuranicWhole
                    )
            })
    }

    pub fn has_related_verse_review(&self) -> bool {
        !self.related_verses.is_empty()
            || self.review_findings.iter().any(|finding| {
                finding.reviewed
                    && finding.area
                        == QuranReviewArea::RelatedVerses
            })
    }

    pub fn has_linguistic_review(&self) -> bool {
        !self.linguistic_notes.is_empty()
            || !self.root_analysis.is_empty()
            || self.review_findings.iter().any(|finding| {
                finding.reviewed
                    && matches!(
                        finding.area,
                        QuranReviewArea::RootAnalysis
                            | QuranReviewArea::Grammar
                            | QuranReviewArea::SemanticRange
                    )
            })
    }

    pub fn has_risale_review(&self) -> bool {
        !self.risale_references.is_empty()
            && self
                .risale_references
                .iter()
                .all(RisaleAnalysisReference::is_complete)
            && self.risale_references.iter().all(
                RisaleAnalysisReference::
                    separates_original_from_analysis,
            )
    }

    pub fn has_hadith_review(&self) -> bool {
        !self.related_hadiths.is_empty()
            && self
                .related_hadiths
                .iter()
                .all(QuranHadithReference::is_complete)
    }

    pub fn has_creation_book_review(&self) -> bool {
        !self.creation_book_observations.is_empty()
    }

    pub fn has_fitrah_review(&self) -> bool {
        !self.fitrah_evidence.is_empty()
    }

    pub fn has_rational_review(&self) -> bool {
        !self.rational_arguments.is_empty()
            || !self.logical_arguments.is_empty()
    }

    pub fn has_unresolved_items(&self) -> bool {
        !self.contradictions.is_empty()
            || !self.interpretation_risks.is_empty()
            || !self.open_questions.is_empty()
            || self
                .review_findings
                .iter()
                .any(QuranReviewFinding::has_unresolved_items)
    }

    /// İnsan yorumunun ayetin açık hükmünü aştığını gösteren
    /// güven seviyelerini denetler.
    pub fn interpretation_overreaches_text(&self) -> bool {
        matches!(
            self.interpretation_confidence,
            InterpretationConfidence::Overextended
                | InterpretationConfidence::
                    RejectedInterpretation
        )
    }

    /// Ayet analizinin tamamlanmış kabul edilebilmesi için
    /// gereken asgari anayasal koşulları denetler.
    pub fn can_be_completed(&self) -> bool {
        self.is_identity_complete()
            && self.has_context_review()
            && self.has_related_verse_review()
            && self.has_linguistic_review()
            && self
                .review_findings
                .iter()
                .all(QuranReviewFinding::is_complete)
            && self
                .risale_references
                .iter()
                .all(RisaleAnalysisReference::is_complete)
            && self
                .risale_references
                .iter()
                .all(
                    RisaleAnalysisReference::
                        separates_original_from_analysis,
                )
            && self
                .related_hadiths
                .iter()
                .all(QuranHadithReference::is_complete)
            && self
                .separates_quran_from_zanistarast_interpretation()
            && self
                .separates_human_and_zanistarast_interpretation()
            && !self.interpretation_overreaches_text()
            && !self.has_unresolved_items()
            && self.rasterast_verified
    }

    pub fn is_constitutionally_valid(&self) -> bool {
        self.is_identity_complete()
            && self
                .separates_quran_from_zanistarast_interpretation()
            && self
                .separates_human_and_zanistarast_interpretation()
            && self.requires_mudebbir_decision
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn risale_reference() -> RisaleAnalysisReference {
        RisaleAnalysisReference::new(
            "Sözler",
            "Örnek bölüm",
        )
        .with_original_text(
            "Risale-i Nur orijinal metin bölümü.",
        )
        .with_proof_method(
            "Temsil ve akli kıyas yöntemi.",
        )
        .with_zanistarast_analysis(
            "İspat yolunun Zanistarast açısından analizi.",
        )
    }

    fn hadith_reference() -> QuranHadithReference {
        QuranHadithReference::new(
            "Örnek hadis kaynağı",
            "Sahih",
        )
        .with_text("Hadis metni.")
        .with_authenticity_source(
            "Hadis sıhhat değerlendirmesi kaynağı.",
        )
        .with_relation_to_verse(
            "Ayetin konusu ile ilişkili açıklama.",
        )
    }

    fn complete_analysis() -> QuranAnalysis {
        QuranAnalysis::new(
            "analysis-001",
            "Örnek sure ve ayet",
            "Meleklerin varlığı",
        )
        .with_arabic_text(
            "Doğrulanmış Arapça ayet metni.",
        )
        .with_verified_translation(
            "Doğrulanmış anlam kaydı.",
        )
        .with_status(
            QuranAnalysisStatus::RequiresRasterastReview,
        )
        .with_interpretation_confidence(
            InterpretationConfidence::StronglyGrounded,
        )
        .with_immediate_context(vec![
            "Önceki ve sonraki ayetlerin bağlamı.".to_string(),
        ])
        .with_related_verses(vec![
            "Aynı konudaki ilgili ayet.".to_string(),
        ])
        .with_linguistic_notes(vec![
            "Dilsel kullanım notu.".to_string(),
        ])
        .with_root_analysis(vec![
            "Kelime kökü analizi.".to_string(),
        ])
        .with_related_hadiths(vec![hadith_reference()])
        .with_risale_references(vec![risale_reference()])
        .with_creation_book_observations(vec![
            "Kâinat kitabı araştırma notu.".to_string(),
        ])
        .with_fitrah_evidence(vec![
            "Fıtrat delili değerlendirmesi.".to_string(),
        ])
        .with_rational_arguments(vec![
            "Akli çıkarım.".to_string(),
        ])
        .with_logical_arguments(vec![
            "Mantıksal çıkarım.".to_string(),
        ])
        .with_human_interpretation(
            "İnsan yorumunun ayrı kaydı.",
        )
        .with_zanistarast_interpretation(
            "Zanistarast yorumunun ayrı kaydı.",
        )
        .with_review_findings(vec![
            QuranReviewFinding::new(
                QuranReviewArea::QuranicWhole,
                "Ayet Kur'an bütünlüğü içinde incelendi.",
            )
            .mark_reviewed(),
        ])
        .mark_rasterast_verified()
    }

    #[test]
    fn analysis_reviews_human_understanding_not_quranic_truth() {
        let analysis = complete_analysis();

        assert!(analysis.is_identity_complete());
        assert!(analysis.rasterast_verified);
        assert!(analysis.requires_mudebbir_decision);
    }

    #[test]
    fn quran_and_zanistarast_interpretation_remain_separate() {
        let analysis = complete_analysis();

        assert!(
            analysis
                .separates_quran_from_zanistarast_interpretation()
        );
        assert!(
            analysis
                .separates_human_and_zanistarast_interpretation()
        );
    }
#[test]
    fn identical_quran_text_and_interpretation_are_rejected() {
        let analysis = QuranAnalysis::new(
            "analysis-002",
            "Örnek ayet",
            "Örnek konu",
        )
        .with_arabic_text("Aynı ifade")
        .with_verified_translation("Doğrulanmış anlam")
        .with_zanistarast_interpretation("Aynı ifade");

        assert!(
            !analysis
                .separates_quran_from_zanistarast_interpretation()
        );
        assert!(!analysis.is_constitutionally_valid());
    }

    #[test]
    fn overextended_interpretation_cannot_be_completed() {
        let analysis = complete_analysis()
            .with_interpretation_confidence(
                InterpretationConfidence::Overextended,
            );

        assert!(analysis.interpretation_overreaches_text());
        assert!(!analysis.can_be_completed());
    }

    #[test]
    fn unresolved_question_blocks_completion() {
        let analysis = complete_analysis()
            .with_open_questions(vec![
                "Dilsel alternatif yeniden incelenmelidir."
                    .to_string(),
            ]);

        assert!(analysis.has_unresolved_items());
        assert!(!analysis.can_be_completed());
    }

    #[test]
    fn complete_analysis_requires_rasterast_verification() {
        let mut analysis = complete_analysis();
        analysis.rasterast_verified = false;

        assert!(!analysis.can_be_completed());
    }

    #[test]
    fn complete_analysis_preserves_mudebbir_gate() {
        let analysis = complete_analysis();

        assert!(analysis.can_be_completed());
        assert!(analysis.requires_mudebbir_decision);
    }

    #[test]
    fn risale_original_text_and_analysis_remain_separate() {
        let reference = risale_reference();

        assert!(reference.is_complete());
        assert!(reference.separates_original_from_analysis());
    }

    #[test]
    fn risale_cannot_replace_original_text_with_analysis() {
        let reference = RisaleAnalysisReference::new(
            "Sözler",
            "Örnek bölüm",
        )
        .with_original_text("Aynı ifade")
        .with_zanistarast_analysis("Aynı ifade");

        assert!(!reference.separates_original_from_analysis());
    }

    #[test]
    fn hadith_reference_requires_authenticity_source() {
        let reference = QuranHadithReference::new(
            "Örnek hadis kaynağı",
            "Sahih",
        )
        .with_text("Hadis metni.");

        assert!(!reference.is_complete());
    }
}

          
