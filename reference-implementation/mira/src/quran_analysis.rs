//! Kur'an-ı Kerim analiz modeli.
//!
//! Bu modül Kur'an-ı Kerim'in vahiy statüsü ile
//! Zanistarast tarafından yapılan insanî analiz ve
//! açıklamayı birbirinden ayırır.
//!
//! Kur'an-ı Kerim mutlak vahyî hakikattir.
//! İnsan tarafından yapılan meal, yorum, açıklama,
//! sınıflandırma ve bilimsel ilişkilendirmeler ise
//! denetlenebilir ve düzeltilebilir çalışmalardır.
//!
//! Risale-i Nur'un kurucu akıl, mantık ve ispat
//! yöntemleri yalnızca doğrulanmış yöntem bağlantısı
//! üzerinden kullanılabilir.
//!
//! Rasterast Kur'an-ı Kerim'i değil, insanın kaynak
//! kullanımını, yorumunu, çıkarımını ve uygulamasını
//! denetler.
//!
//! Nihai karar Müdebbir'e aittir.

use crate::{
    ProofPath,
    ProofPathKind,
    RisaleMethodBinding,
    TruthFoundation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum QuranAnalysisType {
    DirectVerseAnalysis,
    MultiVerseAnalysis,
    ConceptAnalysis,
    ThemeAnalysis,
    LinguisticAnalysis,
    ContextualAnalysis,
    CreationBookRelation,
    FitrahRelation,
    ScientificRelation,
    HistoricalRelation,
    ComparativeAnalysis,
    RisaleMethodAnalysis,
    ZanistarastSynthesis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuranAnalysisStatus {
    Draft,
    SourceReview,
    AnalysisReview,
    AwaitingRasterast,
    RasterastVerified,
    RequiresRevision,
    AwaitingMudebbir,
    Approved,
    Rejected,
    Archived,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuranStatementStatus {
    /// Kur'an-ı Kerim'in açık bildirimi.
    RevealedStatement,

    /// Ayetlerden insan tarafından çıkarılan doğrudan
    /// anlam veya açıklama.
    HumanInterpretation,

    /// Ayetlerle ilişkilendirilen insanî araştırma sonucu.
    HumanResearchRelation,

    /// Zanistarast tarafından önerilen ve denetlenebilir
    /// sentez veya model.
    ZanistarastSynthesis,
}

impl QuranStatementStatus {
    pub fn is_revealed(self) -> bool {
        self == Self::RevealedStatement
    }

    pub fn is_humanly_fallible(self) -> bool {
        matches!(
            self,
            Self::HumanInterpretation
                | Self::HumanResearchRelation
                | Self::ZanistarastSynthesis
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuranSourceVerificationStatus {
    Unverified,
    ReferenceVerified,
    TextVerified,
    FullyVerified,
    Rejected,
}

impl QuranSourceVerificationStatus {
    pub fn is_verified(self) -> bool {
        self == Self::FullyVerified
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuranAnalysisLimitationKind {
    MissingSource,
    UnverifiedVerseText,
    TranslationDependency,
    LinguisticAmbiguity,
    ContextInsufficient,
    InterpretationDifference,
    LogicalGap,
    UnsupportedInference,
    ScientificOverreach,
    HistoricalUncertainty,
    RisaleMethodBindingMissing,
    RisaleSourceUnverified,
    CategoryError,
    ScopeError,
    Unknown,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuranVerseReference {
    pub surah_number: u16,
    pub verse_number: u16,
    pub reference_label: String,
}

impl QuranVerseReference {
    pub fn new(
        surah_number: u16,
        verse_number: u16,
    ) -> Self {
        Self {
            surah_number,
            verse_number,
            reference_label: String::new(),
        }
    }

    pub fn with_reference_label(
        mut self,
        reference_label: impl Into<String>,
    ) -> Self {
        self.reference_label = reference_label.into();
        self
    }

    pub fn is_valid(&self) -> bool {
        self.surah_number >= 1
            && self.surah_number <= 114
            && self.verse_number >= 1
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuranSourceRecord {
    pub source_id: String,
    pub verse_reference: QuranVerseReference,

    /// Kur'an-ı Kerim'in kaynak kaydında kullanılan
    /// Arapça ayet metnidir.
    pub original_text: String,

    /// Kullanılan meal veya anlam aktarımıdır.
    ///
    /// Bu alan vahyin kendisi değil, insan tarafından
    /// yapılan anlam aktarımıdır.
    pub translation_text: String,

    /// Meal veya tercümenin kaynak bilgisidir.
    pub translation_source: String,

    pub verification_status: QuranSourceVerificationStatus,
    pub source_notes: Vec<String>,
}

impl QuranSourceRecord {
    pub fn new(
        source_id: impl Into<String>,
        verse_reference: QuranVerseReference,
    ) -> Self {
        Self {
            source_id: source_id.into(),
            verse_reference,
            original_text: String::new(),
            translation_text: String::new(),
            translation_source: String::new(),
            verification_status:
                QuranSourceVerificationStatus::Unverified,
            source_notes: Vec::new(),
        }
    }

    pub fn with_original_text(
        mut self,
        original_text: impl Into<String>,
    ) -> Self {
        self.original_text = original_text.into();
        self
    }

    pub fn with_translation(
        mut self,
        translation_text: impl Into<String>,
        translation_source: impl Into<String>,
    ) -> Self {
        self.translation_text = translation_text.into();
        self.translation_source = translation_source.into();
        self
    }

    pub fn with_verification_status(
        mut self,
        verification_status: QuranSourceVerificationStatus,
    ) -> Self {
        self.verification_status = verification_status;
        self
    }

    pub fn with_source_notes(
        mut self,
        source_notes: Vec<String>,
    ) -> Self {
        self.source_notes = source_notes;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.source_id.trim().is_empty()
            && self.verse_reference.is_valid()
            && !self.original_text.trim().is_empty()
    }

    pub fn can_be_used_as_verified_source(&self) -> bool {
        self.is_complete()
            && self.verification_status.is_verified()
    }

    pub fn translation_is_separate_from_revelation(&self) -> bool {
        self.translation_text.trim().is_empty()
            || self.translation_text.trim()
                != self.original_text.trim()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuranAnalysisLimitation {
    pub kind: QuranAnalysisLimitationKind,
    pub description: String,
    pub proposed_correction: String,
}

impl QuranAnalysisLimitation {
    pub fn new(
        kind: QuranAnalysisLimitationKind,
        description: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            description: description.into(),
            proposed_correction: String::new(),
        }
    }

    pub fn with_proposed_correction(
        mut self,
        proposed_correction: impl Into<String>,
    ) -> Self {
        self.proposed_correction =
            proposed_correction.into();
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.description.trim().is_empty()
    }

    pub fn has_correction_path(&self) -> bool {
        !self.proposed_correction.trim().is_empty()
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuranAnalysisStatement {
    pub statement_id: String,
    pub status: QuranStatementStatus,
    pub statement: String,

    /// Bu hükmün dayandığı kaynak kayıtlarının
    /// kimlikleridir.
    pub source_ids: Vec<String>,

    /// İnsan tarafından kurulan açıklama ve çıkarım
    /// basamaklarıdır.
    pub reasoning_steps: Vec<String>,

    /// Açıklamanın kesinlik veya kapsam sınırlarıdır.
    pub limitations: Vec<String>,
}

impl QuranAnalysisStatement {
    pub fn new(
        statement_id: impl Into<String>,
        status: QuranStatementStatus,
        statement: impl Into<String>,
    ) -> Self {
        Self {
            statement_id: statement_id.into(),
            status,
            statement: statement.into(),
            source_ids: Vec::new(),
            reasoning_steps: Vec::new(),
            limitations: Vec::new(),
        }
    }

    pub fn with_source_ids(
        mut self,
        source_ids: Vec<String>,
    ) -> Self {
        self.source_ids = source_ids;
        self
    }

    pub fn with_reasoning_steps(
        mut self,
        reasoning_steps: Vec<String>,
    ) -> Self {
        self.reasoning_steps = reasoning_steps;
        self
    }

    pub fn with_limitations(
        mut self,
        limitations: Vec<String>,
    ) -> Self {
        self.limitations = limitations;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.statement_id.trim().is_empty()
            && !self.statement.trim().is_empty()
            && self
                .source_ids
                .iter()
                .all(|item| !item.trim().is_empty())
            && self
                .reasoning_steps
                .iter()
                .all(|item| !item.trim().is_empty())
            && self
                .limitations
                .iter()
                .all(|item| !item.trim().is_empty())
    }

    /// Vahyî bildirimin insan yorumu gibi
    /// etiketlenmesini engelleyen temel ayrımdır.
    pub fn preserves_statement_status(&self) -> bool {
        if self.status.is_revealed() {
            self.reasoning_steps.is_empty()
                && self.limitations.is_empty()
        } else {
            true
        }
    }

    /// İnsan yorumunun vahyin kendisi gibi
    /// sunulmasını engeller.
    pub fn human_statement_remains_fallible(&self) -> bool {
        self.status.is_humanly_fallible()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuranAnalysis {
    pub analysis_id: String,
    pub title: String,
    pub subject: String,
    pub analysis_type: QuranAnalysisType,
    pub status: QuranAnalysisStatus,

    /// Bu analizin dayandığı Kur'an hakikat temelidir.
    ///
    /// Temel kaydı başka modülde korunur; burada
    /// değiştirilmez.
    pub foundation: TruthFoundation,

    /// Kullanılan Kur'an kaynaklarıdır.
    pub sources: Vec<QuranSourceRecord>,

    /// Vahyî bildirimler ile insanî açıklamalar ayrı
    /// kayıtlar olarak tutulur.
    pub statements: Vec<QuranAnalysisStatement>,

    /// Analizde kullanılan farklı ispat yollarıdır.
    pub proof_paths: Vec<ProofPath>,

    /// Analizde Risale-i Nur yöntemi kullanılıyorsa
    /// doğrulanmış yöntem bağlantısıdır.
    pub risale_method_binding: Option<RisaleMethodBinding>,

    pub limitations: Vec<QuranAnalysisLimitation>,
    pub alternative_interpretations: Vec<String>,

    /// Rasterast'ın Kur'an'ı değil, bu insanî analiz
    /// kaydını doğruladığını gösterir.
    pub rasterast_verified: bool,

    /// Analizin nihai karar için Müdebbir'e sunulması
    /// gerektiğini gösterir.
    pub requires_mudebbir_decision: bool,
}

impl QuranAnalysis {
    pub fn new(
        analysis_id: impl Into<String>,
        title: impl Into<String>,
        subject: impl Into<String>,
        analysis_type: QuranAnalysisType,
        foundation: TruthFoundation,
    ) -> Self {
        Self {
            analysis_id: analysis_id.into(),
            title: title.into(),
            subject: subject.into(),
            analysis_type,
            status: QuranAnalysisStatus::Draft,
            foundation,
            sources: Vec::new(),
            statements: Vec::new(),
            proof_paths: Vec::new(),
            risale_method_binding: None,
            limitations: Vec::new(),
            alternative_interpretations: Vec::new(),
            rasterast_verified: false,
            requires_mudebbir_decision: true,
        }
    }
 pub fn with_status(
        mut self,
        status: QuranAnalysisStatus,
    ) -> Self {
        self.status = status;
        self
    }

    pub fn with_sources(
        mut self,
        sources: Vec<QuranSourceRecord>,
    ) -> Self {
        self.sources = sources;
        self
    }

    pub fn with_statements(
        mut self,
        statements: Vec<QuranAnalysisStatement>,
    ) -> Self {
        self.statements = statements;
        self
    }

    pub fn with_proof_paths(
        mut self,
        proof_paths: Vec<ProofPath>,
    ) -> Self {
        self.proof_paths = proof_paths;
        self
    }

    pub fn with_risale_method_binding(
        mut self,
        risale_method_binding: RisaleMethodBinding,
    ) -> Self {
        self.risale_method_binding =
            Some(risale_method_binding);
        self
    }

    pub fn with_limitations(
        mut self,
        limitations: Vec<QuranAnalysisLimitation>,
    ) -> Self {
        self.limitations = limitations;
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

    pub fn mark_rasterast_verified(mut self) -> Self {
        self.rasterast_verified = true;
        self.status = QuranAnalysisStatus::RasterastVerified;
        self
    }

    pub fn await_mudebbir(mut self) -> Self {
        self.status = QuranAnalysisStatus::AwaitingMudebbir;
        self
    }

    pub fn approve(mut self) -> Self {
        self.status = QuranAnalysisStatus::Approved;
        self
    }

    pub fn reject(mut self) -> Self {
        self.status = QuranAnalysisStatus::Rejected;
        self
    }

    pub fn add_source(
        &mut self,
        source: QuranSourceRecord,
    ) {
        self.sources.push(source);
    }

    pub fn add_statement(
        &mut self,
        statement: QuranAnalysisStatement,
    ) {
        self.statements.push(statement);
    }

    pub fn add_proof_path(
        &mut self,
        proof_path: ProofPath,
    ) {
        self.proof_paths.push(proof_path);
    }

    pub fn add_limitation(
        &mut self,
        limitation: QuranAnalysisLimitation,
    ) {
        self.limitations.push(limitation);
    }

    pub fn is_identity_complete(&self) -> bool {
        !self.analysis_id.trim().is_empty()
            && !self.title.trim().is_empty()
            && !self.subject.trim().is_empty()
    }

    pub fn sources_are_complete(&self) -> bool {
        !self.sources.is_empty()
            && self
                .sources
                .iter()
                .all(QuranSourceRecord::is_complete)
    }

    pub fn sources_are_verified(&self) -> bool {
        !self.sources.is_empty()
            && self.sources.iter().all(
                QuranSourceRecord::
                    can_be_used_as_verified_source,
            )
    }

    pub fn translations_remain_separate(&self) -> bool {
        self.sources.iter().all(
            QuranSourceRecord::
                translation_is_separate_from_revelation,
        )
    }

    pub fn statements_are_valid(&self) -> bool {
        !self.statements.is_empty()
            && self.statements.iter().all(|statement| {
                statement.is_complete()
                    && statement.preserves_statement_status()
            })
    }

    pub fn limitations_are_valid(&self) -> bool {
        self.limitations
            .iter()
            .all(QuranAnalysisLimitation::is_complete)
    }

    pub fn alternatives_are_valid(&self) -> bool {
        self.alternative_interpretations
            .iter()
            .all(|item| !item.trim().is_empty())
    }
 pub fn has_revealed_statement(&self) -> bool {
        self.statements
            .iter()
            .any(|statement| statement.status.is_revealed())
    }

    pub fn has_human_interpretation(&self) -> bool {
        self.statements.iter().any(|statement| {
            statement.status.is_humanly_fallible()
        })
    }

    pub fn has_risale_proof_path(&self) -> bool {
        self.proof_paths.iter().any(|path| {
            path.kind == ProofPathKind::RisaleMethod
        })
    }

    /// Risale yöntemi kullanılmışsa hem analiz düzeyinde
    /// hem de ilgili ispat yollarında doğrulanmış yöntem
    /// bağlantısı bulunmasını zorunlu tutar.
    pub fn has_valid_risale_method_use(&self) -> bool {
        if self.has_risale_proof_path()
            || self.analysis_type
                == QuranAnalysisType::RisaleMethodAnalysis
        {
            let analysis_binding_is_valid = self
                .risale_method_binding
                .as_ref()
                .map(RisaleMethodBinding::is_verified)
                .unwrap_or(false);

            let proof_paths_are_valid = self
                .proof_paths
                .iter()
                .filter(|path| {
                    path.kind == ProofPathKind::RisaleMethod
                })
                .all(|path| {
                    path.has_valid_risale_method_binding()
                });

            analysis_binding_is_valid
                && proof_paths_are_valid
        } else {
            self.risale_method_binding.is_none()
        }
    }

    pub fn proof_paths_are_valid(&self) -> bool {
        self.proof_paths
            .iter()
            .all(ProofPath::is_constitutionally_valid)
    }

    /// Vahyî hüküm bulunmadığında bir insan yorumunun
    /// vahiy gibi sunulmasını engeller.
    pub fn revelation_and_interpretation_are_separate(
        &self,
    ) -> bool {
        self.statements.iter().all(|statement| {
            statement.preserves_statement_status()
        })
    }

    /// Rasterast doğrulamasının yalnızca insanî analiz
    /// kaydına ait olduğunu model düzeyinde açık tutar.
    pub fn rasterast_verifies_analysis_not_revelation(
        &self,
    ) -> bool {
        if self.rasterast_verified {
            self.has_revealed_statement()
                || self.has_human_interpretation()
        } else {
            true
        }
    }

    pub fn has_unresolved_items(&self) -> bool {
        !self.limitations.is_empty()
            || !self.alternative_interpretations.is_empty()
            || self.proof_paths.iter().any(
                ProofPath::has_unresolved_items,
            )
    }

    pub fn can_await_mudebbir_decision(&self) -> bool {
        self.is_complete()
            && self.sources_are_verified()
            && self.proof_paths_are_valid()
            && self.has_valid_risale_method_use()
            && self.rasterast_verified
            && self.requires_mudebbir_decision
    }

    pub fn can_be_approved(&self) -> bool {
        self.can_await_mudebbir_decision()
            && self.status
                == QuranAnalysisStatus::AwaitingMudebbir
            && !self.has_unresolved_items()
    }

    pub fn is_complete(&self) -> bool {
        self.is_identity_complete()
            && self.sources_are_complete()
            && self.translations_remain_separate()
            && self.statements_are_valid()
            && self.limitations_are_valid()
            && self.alternatives_are_valid()
            && self.proof_paths_are_valid()
            && self.has_valid_risale_method_use()
            && self.revelation_and_interpretation_are_separate()
            && self.rasterast_verifies_analysis_not_revelation()
            && self.requires_mudebbir_decision
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuranAnalysisSet {
    pub subject: String,
    pub analyses: Vec<QuranAnalysis>,
}

impl QuranAnalysisSet {
    pub fn new(subject: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
            analyses: Vec::new(),
        }
    }

    pub fn with_analyses(
        mut self,
        analyses: Vec<QuranAnalysis>,
    ) -> Self {
        self.analyses = analyses;
        self
    }

    pub fn add_analysis(
        &mut self,
        analysis: QuranAnalysis,
    ) {
        self.analyses.push(analysis);
    }

    pub fn approved_analysis_count(&self) -> usize {
        self.analyses
            .iter()
            .filter(|analysis| {
                analysis.status == QuranAnalysisStatus::Approved
            })
            .count()
    }

    pub fn analyses_awaiting_mudebbir(
        &self,
    ) -> Vec<&QuranAnalysis> {
        self.analyses
            .iter()
            .filter(|analysis| {
                analysis.status
                    == QuranAnalysisStatus::AwaitingMudebbir
            })
            .collect()
    }

    pub fn invalid_analyses(&self) -> Vec<&QuranAnalysis> {
        self.analyses
            .iter()
            .filter(|analysis| !analysis.is_complete())
            .collect()
    }

    pub fn is_complete(&self) -> bool {
        !self.subject.trim().is_empty()
            && !self.analyses.is_empty()
            && self.invalid_analyses().is_empty()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    fn verified_source() -> QuranSourceRecord {
        QuranSourceRecord::new(
            "quran-source-001",
            QuranVerseReference::new(1, 1)
                .with_reference_label("Fâtiha 1"),
        )
        .with_original_text(
            "Doğrulanmış örnek Arapça kaynak metni.",
        )
        .with_translation(
            "Doğrulanmış örnek anlam aktarımı.",
            "Doğrulanmış meal kaynağı",
        )
        .with_verification_status(
            QuranSourceVerificationStatus::FullyVerified,
        )
    }

    #[test]
    fn verse_reference_requires_valid_surah_and_verse() {
        let valid = QuranVerseReference::new(1, 1);
        let invalid_surah =
            QuranVerseReference::new(115, 1);
        let invalid_verse =
            QuranVerseReference::new(1, 0);

        assert!(valid.is_valid());
        assert!(!invalid_surah.is_valid());
        assert!(!invalid_verse.is_valid());
    }

    #[test]
    fn verified_source_requires_original_text() {
        let incomplete = QuranSourceRecord::new(
            "quran-source-002",
            QuranVerseReference::new(2, 255),
        )
        .with_verification_status(
            QuranSourceVerificationStatus::FullyVerified,
        );

        assert!(!incomplete.is_complete());
        assert!(
            !incomplete.can_be_used_as_verified_source()
        );

        let complete = verified_source();

        assert!(complete.is_complete());
        assert!(
            complete.can_be_used_as_verified_source()
        );
    }

    #[test]
    fn translation_must_remain_separate_from_revelation() {
        let source = QuranSourceRecord::new(
            "quran-source-003",
            QuranVerseReference::new(112, 1),
        )
        .with_original_text("Aynı metin")
        .with_translation(
            "Aynı metin",
            "Örnek kaynak",
        );

        assert!(
            !source.translation_is_separate_from_revelation()
        );
    }

    #[test]
    fn revealed_statement_has_no_human_reasoning_steps() {
        let statement = QuranAnalysisStatement::new(
            "statement-001",
            QuranStatementStatus::RevealedStatement,
            "Kur'an-ı Kerim'in açık bildirimi.",
        )
        .with_source_ids(vec![
            "quran-source-001".to_string(),
        ])
        .with_reasoning_steps(vec![
            "İnsanî çıkarım basamağı.".to_string(),
        ]);

        assert!(!statement.preserves_statement_status());
    }

    #[test]
    fn revealed_statement_can_preserve_its_status() {
        let statement = QuranAnalysisStatement::new(
            "statement-002",
            QuranStatementStatus::RevealedStatement,
            "Kur'an-ı Kerim'in açık bildirimi.",
        )
        .with_source_ids(vec![
            "quran-source-001".to_string(),
        ]);

        assert!(statement.is_complete());
        assert!(statement.preserves_statement_status());
        assert!(!statement.human_statement_remains_fallible());
    }

    #[test]
    fn human_interpretation_remains_fallible() {
        let statement = QuranAnalysisStatement::new(
            "statement-003",
            QuranStatementStatus::HumanInterpretation,
            "Ayet hakkında insan tarafından yapılan yorum.",
        )
        .with_source_ids(vec![
            "quran-source-001".to_string(),
        ])
        .with_reasoning_steps(vec![
            "Dil ve bağlam incelemesi.".to_string(),
            "İnsanî yorum sonucu.".to_string(),
        ]);

        assert!(statement.is_complete());
        assert!(statement.preserves_statement_status());
        assert!(statement.human_statement_remains_fallible());
    }

    #[test]
    fn limitation_can_define_correction_path() {
        let limitation = QuranAnalysisLimitation::new(
            QuranAnalysisLimitationKind::
                LinguisticAmbiguity,
            "Kelimenin anlam alanı yeniden incelenmelidir.",
        )
        .with_proposed_correction(
            "Klasik sözlükler ve ayet bağlamları karşılaştırılmalıdır.",
        );

        assert!(limitation.is_complete());
        assert!(limitation.has_correction_path());
    }

    #[test]
    fn source_verification_status_is_explicit() {
        assert!(
            QuranSourceVerificationStatus::FullyVerified
                .is_verified()
        );

        assert!(
            !QuranSourceVerificationStatus::Unverified
                .is_verified()
        );

        assert!(
            !QuranSourceVerificationStatus::Rejected
                .is_verified()
        );
    }

    #[test]
    fn analysis_status_keeps_mudebbir_gate() {
        assert_ne!(
            QuranAnalysisStatus::RasterastVerified,
            QuranAnalysisStatus::Approved,
        );

        assert_ne!(
            QuranAnalysisStatus::AwaitingMudebbir,
            QuranAnalysisStatus::Approved,
        );
    }
}
