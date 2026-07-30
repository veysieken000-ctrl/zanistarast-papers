//! Zanistarast hakikat temellerinin anayasal veri modelidir.
//!
//! # Temel ayrım
//!
//! Kur'an-ı Kerim mutlak vahyî hakikatin ve Zanistarast
//! bilim paradigmasının başlangıç kaynağıdır.
//!
//! Üstad Bediüzzaman Said-i Kürdî'nin Risale-i Nur'daki
//! aklî, mantıkî, imanî ve ispatlayıcı yöntemleri
//! Zanistarast için bağlayıcı kurucu yöntemlerdir.
//!
//! Risale-i Nur, Kur'an-ı Kerim ile aynı vahiy statüsüne
//! yerleştirilmez. Ancak Zanistarast içinde sıradan,
//! isteğe bağlı veya birbirinin alternatifi olan insanî
//! yöntemlerden biri olarak da değerlendirilmez.
//!
//! Zanistarast'ın kaynakları anlama, sınıflandırma ve
//! bilimsel alanlara uygulama biçimi insanîdir. Bu nedenle
//! Rasterast denetimine ve Müdebbir kararına açıktır.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TruthFoundationKind {
    Quran,
    OriginalRisaleNur,
    AuthenticHadith,
    Creation,
    Fitrah,
    HumanEssence,
    Spirit,
    Heart,
    Conscience,
    Emotion,
    Reason,
    Logic,
    Morality,
    Observation,
    Experiment,
    Mathematics,
    ReliableScience,
}

impl TruthFoundationKind {
    pub fn is_revelation(self) -> bool {
        matches!(self, Self::Quran)
    }

    pub fn is_binding_risale_method(self) -> bool {
        matches!(self, Self::OriginalRisaleNur)
    }

    pub fn is_human_verification_instrument(self) -> bool {
        matches!(
            self,
            Self::Observation
                | Self::Experiment
                | Self::Mathematics
                | Self::ReliableScience
        )
    }

    pub fn is_human_inner_dimension(self) -> bool {
        matches!(
            self,
            Self::HumanEssence
                | Self::Spirit
                | Self::Heart
                | Self::Conscience
                | Self::Emotion
                | Self::Reason
                | Self::Logic
                | Self::Morality
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FoundationAuthority {
    /// Kur'an-ı Kerim'e mahsus mutlak vahyî otorite.
    AbsoluteRevealedTruth,

    /// Risale-i Nur'un Zanistarast içindeki bağlayıcı,
    /// kurucu akıl, mantık, iman ve ispat yöntemi.
    BindingFoundationalMethod,

    /// Sıhhati doğrulanmış hadislerin bağlayıcı nebevî
    /// açıklama ve uygulama otoritesi.
    AuthenticatedPropheticGuidance,

    /// Yaratılışta ve varlık düzeninde okunan şahitlik.
    CreationWitness,

    /// İnsanın yaratılış yapısında bulunan şahitlik.
    FitrahWitness,

    /// İnsan özü, ruh, kalp, vicdan, duygu, akıl ve
    /// ahlak alanlarında bulunan iç şahitlik.
    HumanInnerWitness,

    /// Deney, gözlem ve matematikle elde edilen fakat
    /// insanî sınırlar taşıyan doğrulama.
    EmpiricalVerification,

    /// Güvenilir bilimsel çalışmalardan yararlanmayı,
    /// fakat onları mutlaklaştırmamayı ifade eder.
    ReliableScientificEvidence,
}

impl FoundationAuthority {
    pub fn is_absolute_revelation(self) -> bool {
        matches!(self, Self::AbsoluteRevealedTruth)
    }

    pub fn is_binding_foundational_method(self) -> bool {
        matches!(self, Self::BindingFoundationalMethod)
    }

    pub fn is_humanly_reviewable(self) -> bool {
        !self.is_absolute_revelation()
            && !self.is_binding_foundational_method()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FoundationUse {
    OntologicalTruth,
    QuranicStartingPoint,
    FoundationalProofMethod,
    FoundationalReasoningMethod,
    FoundationalLogicalMethod,
    FaithReasonIntegration,
    HumanUnderstanding,
    CreationReading,
    FitrahReading,
    MoralEvaluation,
    PropheticExplanation,
    InterpretationSupport,
    EmpiricalInvestigation,
    MathematicalFormalization,
    ScientificComparison,
    RasterastVerification,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationReviewStatus {
    NotStarted,
    SourceLocated,
    SourceVerificationInProgress,
    SourceVerified,
    InterpretationReviewRequired,
    MethodApplicationReviewRequired,
    EmpiricalReviewRequired,
    RequiresRasterastReview,
    AwaitingMudebbirDecision,
    ApprovedForUse,
    ApplicationRequiresCorrection,
    RejectedApplication,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TruthFoundation {
    pub foundation_id: String,
    pub name: String,
    pub description: String,

    pub kind: TruthFoundationKind,
    pub authority: FoundationAuthority,
    pub uses: Vec<FoundationUse>,
    pub review_status: FoundationReviewStatus,

    pub source_references: Vec<String>,
    pub original_text_references: Vec<String>,
    pub related_topics: Vec<String>,

    pub zanistarast_interpretation: String,
    pub scientific_application_notes: Vec<String>,

    pub source_verified: bool,
    pub original_preserved: bool,
    pub interpretation_separated: bool,
    pub rasterast_verified: bool,
    pub requires_mudebbir_decision: bool,

    pub risks: Vec<String>,
    pub uncertainties: Vec<String>,
    pub contradictions: Vec<String>,
}

impl TruthFoundation {
    pub fn new(
        foundation_id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        kind: TruthFoundationKind,
        authority: FoundationAuthority,
    ) -> Self {
        Self {
            foundation_id: foundation_id.into(),
            name: name.into(),
            description: description.into(),

            kind,
            authority,
            uses: Vec::new(),
            review_status: FoundationReviewStatus::NotStarted,

            source_references: Vec::new(),
            original_text_references: Vec::new(),
            related_topics: Vec::new(),

            zanistarast_interpretation: String::new(),
            scientific_application_notes: Vec::new(),

            source_verified: false,
            original_preserved: false,
            interpretation_separated: false,
            rasterast_verified: false,
            requires_mudebbir_decision: true,

            risks: Vec::new(),
            uncertainties: Vec::new(),
            contradictions: Vec::new(),
        }
    }

    pub fn with_uses(
        mut self,
        uses: Vec<FoundationUse>,
    ) -> Self {
        self.uses = uses;
        self
    }

    pub fn with_review_status(
        mut self,
        status: FoundationReviewStatus,
    ) -> Self {
        self.review_status = status;
        self
    }

    pub fn with_source_references(
        mut self,
        references: Vec<String>,
    ) -> Self {
        self.source_references = references;
        self
    }

    pub fn with_original_text_references(
        mut self,
        references: Vec<String>,
    ) -> Self {
        self.original_text_references = references;
        self
    }

    pub fn with_related_topics(
        mut self,
        topics: Vec<String>,
    ) -> Self {
        self.related_topics = topics;
        self
    }

    pub fn with_zanistarast_interpretation(
        mut self,
        interpretation: impl Into<String>,
    ) -> Self {
        self.zanistarast_interpretation =
            interpretation.into();
        self
    }

    pub fn with_scientific_application_notes(
        mut self,
        notes: Vec<String>,
    ) -> Self {
        self.scientific_application_notes = notes;
        self
    }

    pub fn with_risks(
        mut self,
        risks: Vec<String>,
    ) -> Self {
        self.risks = risks;
        self
    }

    pub fn with_uncertainties(
        mut self,
        uncertainties: Vec<String>,
    ) -> Self {
        self.uncertainties = uncertainties;
        self
    }

    pub fn with_contradictions(
        mut self,
        contradictions: Vec<String>,
    ) -> Self {
        self.contradictions = contradictions;
        self
    }

    pub fn mark_source_verified(mut self) -> Self {
        self.source_verified = true;
        self
    }

    pub fn mark_original_preserved(mut self) -> Self {
        self.original_preserved = true;
        self
    }

    pub fn mark_interpretation_separated(mut self) -> Self {
        self.interpretation_separated = true;
        self
    }

    pub fn mark_rasterast_verified(mut self) -> Self {
        self.rasterast_verified = true;
        self
    }

    pub fn is_identity_complete(&self) -> bool {
        !self.foundation_id.trim().is_empty()
            && !self.name.trim().is_empty()
            && !self.description.trim().is_empty()
    }

    pub fn has_source_basis(&self) -> bool {
        !self.source_references.is_empty()
    }

    pub fn has_original_text_basis(&self) -> bool {
        !self.original_text_references.is_empty()
    }

    pub fn is_quran_foundation(&self) -> bool {
        self.kind == TruthFoundationKind::Quran
    }

    pub fn is_original_risale_foundation(&self) -> bool {
        self.kind == TruthFoundationKind::OriginalRisaleNur
    }

    pub fn preserves_quran_authority(&self) -> bool {
        if self.is_quran_foundation() {
            self.authority
                == FoundationAuthority::AbsoluteRevealedTruth
        } else {
            !self.authority.is_absolute_revelation()
        }
    }

    pub fn preserves_risale_method_authority(&self) -> bool {
        if self.is_original_risale_foundation() {
            self.authority
                == FoundationAuthority::BindingFoundationalMethod
                && self.uses.contains(
                    &FoundationUse::FoundationalProofMethod,
                )
        } else {
            self.authority
                != FoundationAuthority::BindingFoundationalMethod
        }
    }

    pub fn separates_quran_and_risale_status(&self) -> bool {
        if self.is_quran_foundation() {
            self.authority.is_absolute_revelation()
        } else if self.is_original_risale_foundation() {
            self.authority.is_binding_foundational_method()
                && !self.authority.is_absolute_revelation()
        } else {
            !self.authority.is_absolute_revelation()
                && !self.authority
                    .is_binding_foundational_method()
        }
    }

    pub fn application_has_open_issues(&self) -> bool {
        !self.risks.is_empty()
            || !self.uncertainties.is_empty()
            || !self.contradictions.is_empty()
    }

    pub fn requires_source_verification(&self) -> bool {
        matches!(
            self.kind,
            TruthFoundationKind::Quran
                | TruthFoundationKind::OriginalRisaleNur
                | TruthFoundationKind::AuthenticHadith
                | TruthFoundationKind::ReliableScience
        )
    }

    pub fn is_constitutionally_valid(&self) -> bool {
        self.is_identity_complete()
            && self.preserves_quran_authority()
            && self.preserves_risale_method_authority()
            && self.separates_quran_and_risale_status()
            && self.requires_mudebbir_decision
    }

    pub fn can_be_approved_for_use(&self) -> bool {
        let source_condition =
            !self.requires_source_verification()
                || self.source_verified;

        self.is_constitutionally_valid()
            && source_condition
            && self.original_preserved
            && self.interpretation_separated
            && self.rasterast_verified
            && !self.application_has_open_issues()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TruthFoundationSet {
    pub set_id: String,
    pub title: String,
    pub foundations: Vec<TruthFoundation>,
}

impl TruthFoundationSet {
    pub fn new(
        set_id: impl Into<String>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            set_id: set_id.into(),
            title: title.into(),
            foundations: Vec::new(),
        }
    }

    pub fn with_foundations(
        mut self,
        foundations: Vec<TruthFoundation>,
    ) -> Self {
        self.foundations = foundations;
        self
    }

    pub fn add_foundation(
        &mut self,
        foundation: TruthFoundation,
    ) {
        self.foundations.push(foundation);
    }

    pub fn find_foundation(
        &self,
        foundation_id: &str,
    ) -> Option<&TruthFoundation> {
        self.foundations
            .iter()
            .find(|foundation| {
                foundation.foundation_id == foundation_id
            })
    }

    pub fn foundations_by_kind(
        &self,
        kind: TruthFoundationKind,
    ) -> Vec<&TruthFoundation> {
        self.foundations
            .iter()
            .filter(|foundation| foundation.kind == kind)
            .collect()
    }

    pub fn quran_foundation(
        &self,
    ) -> Option<&TruthFoundation> {
        self.foundations
            .iter()
            .find(|foundation| foundation.is_quran_foundation())
    }

    pub fn risale_foundation(
        &self,
    ) -> Option<&TruthFoundation> {
        self.foundations
            .iter()
            .find(|foundation| {
                foundation.is_original_risale_foundation()
            })
    }

    pub fn invalid_foundations(
        &self,
    ) -> Vec<&TruthFoundation> {
        self.foundations
            .iter()
            .filter(|foundation| {
                !foundation.is_constitutionally_valid()
            })
            .collect()
    }

    pub fn approved_foundation_count(&self) -> usize {
        self.foundations
            .iter()
            .filter(|foundation| {
                foundation.can_be_approved_for_use()
            })
            .count()
    }

    pub fn preserves_foundation_order(&self) -> bool {
        let Some(quran_index) = self
            .foundations
            .iter()
            .position(|foundation| {
                foundation.is_quran_foundation()
            })
        else {
            return false;
        };

        let Some(risale_index) = self
            .foundations
            .iter()
            .position(|foundation| {
                foundation.is_original_risale_foundation()
            })
        else {
            return false;
        };

        quran_index < risale_index
    }

    pub fn is_complete(&self) -> bool {
        !self.set_id.trim().is_empty()
            && !self.title.trim().is_empty()
            && !self.foundations.is_empty()
            && self.quran_foundation().is_some()
            && self.risale_foundation().is_some()
            && self.preserves_foundation_order()
            && self.invalid_foundations().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quran_foundation() -> TruthFoundation {
        TruthFoundation::new(
            "foundation-quran",
            "Kur'an-ı Kerim",
            "Zanistarast'ın mutlak vahyî hakikat ve başlangıç kaynağıdır.",
            TruthFoundationKind::Quran,
            FoundationAuthority::AbsoluteRevealedTruth,
        )
        .with_uses(vec![
            FoundationUse::OntologicalTruth,
            FoundationUse::QuranicStartingPoint,
        ])
        .with_source_references(vec![
            "Kur'an-ı Kerim".to_string(),
        ])
        .with_original_text_references(vec![
            "Doğrulanmış mushaf metni".to_string(),
        ])
        .mark_source_verified()
        .mark_original_preserved()
        .mark_interpretation_separated()
        .mark_rasterast_verified()
    }

    fn risale_foundation() -> TruthFoundation {
        TruthFoundation::new(
            "foundation-risale",
            "Risale-i Nur",
            "Zanistarast için bağlayıcı kurucu akıl, mantık, iman ve ispat yöntemidir.",
            TruthFoundationKind::OriginalRisaleNur,
            FoundationAuthority::BindingFoundationalMethod,
        )
        .with_uses(vec![
            FoundationUse::FoundationalProofMethod,
            FoundationUse::FoundationalReasoningMethod,
            FoundationUse::FoundationalLogicalMethod,
            FoundationUse::FaithReasonIntegration,
            FoundationUse::HumanUnderstanding,
            FoundationUse::CreationReading,
        ])
        .with_source_references(vec![
            "Risale-i Nur Külliyatı".to_string(),
        ])
        .with_original_text_references(vec![
            "Doğrulanmış orijinal Risale metni".to_string(),
        ])
        .mark_source_verified()
        .mark_original_preserved()
        .mark_interpretation_separated()
        .mark_rasterast_verified()
    }

    #[test]
    fn quran_has_absolute_revealed_authority() {
        let foundation = quran_foundation();

        assert!(foundation.is_quran_foundation());
        assert!(
            foundation.authority.is_absolute_revelation()
        );
        assert!(foundation.preserves_quran_authority());
        assert!(foundation.can_be_approved_for_use());
    }

    #[test]
    fn risale_has_binding_foundational_method_authority() {
        let foundation = risale_foundation();

        assert!(foundation.is_original_risale_foundation());
        assert!(
            foundation
                .authority
                .is_binding_foundational_method()
        );
        assert!(!foundation.authority.is_absolute_revelation());
        assert!(
            foundation.preserves_risale_method_authority()
        );
        assert!(foundation.can_be_approved_for_use());
    }

    #[test]
    fn risale_cannot_be_assigned_revelation_authority() {
        let foundation = TruthFoundation::new(
            "invalid-risale",
            "Risale-i Nur",
            "Geçersiz otorite eşleştirmesi.",
            TruthFoundationKind::OriginalRisaleNur,
            FoundationAuthority::AbsoluteRevealedTruth,
        )
        .with_uses(vec![
            FoundationUse::FoundationalProofMethod,
        ]);

        assert!(
            !foundation.preserves_risale_method_authority()
        );
        assert!(
            !foundation.separates_quran_and_risale_status()
        );
        assert!(!foundation.is_constitutionally_valid());
    }

    #[test]
    fn ordinary_foundation_cannot_use_risale_authority() {
        let foundation = TruthFoundation::new(
            "foundation-science",
            "Güvenilir bilim",
            "Doğrulanmış bilimsel bulgular.",
            TruthFoundationKind::ReliableScience,
            FoundationAuthority::BindingFoundationalMethod,
        );

        assert!(
            !foundation.preserves_risale_method_authority()
        );
        assert!(!foundation.is_constitutionally_valid());
    }

    #[test]
    fn source_verification_is_required_for_risale() {
        let foundation = TruthFoundation::new(
            "foundation-risale-unverified",
            "Risale-i Nur",
            "Kaynak doğrulaması tamamlanmamış yöntem kaydı.",
            TruthFoundationKind::OriginalRisaleNur,
            FoundationAuthority::BindingFoundationalMethod,
        )
        .with_uses(vec![
            FoundationUse::FoundationalProofMethod,
        ])
        .mark_original_preserved()
        .mark_interpretation_separated()
        .mark_rasterast_verified();

        assert!(foundation.requires_source_verification());
        assert!(!foundation.can_be_approved_for_use());
    }

    #[test]
    fn application_issue_blocks_approval_not_authority() {
        let foundation = risale_foundation()
            .with_risks(vec![
                "Zanistarast uygulaması yeniden incelenmelidir."
                    .to_string(),
            ]);

        assert!(
            foundation.preserves_risale_method_authority()
        );
        assert!(foundation.application_has_open_issues());
        assert!(!foundation.can_be_approved_for_use());
    }

    #[test]
    fn foundation_set_requires_quran_before_risale() {
        let valid_set = TruthFoundationSet::new(
            "foundation-set-001",
            "Zanistarast Hakikat Temelleri",
        )
        .with_foundations(vec![
            quran_foundation(),
            risale_foundation(),
        ]);

        assert!(valid_set.preserves_foundation_order());
        assert!(valid_set.is_complete());
        assert_eq!(valid_set.approved_foundation_count(), 2);
    }

    #[test]
    fn reversed_foundation_order_is_rejected() {
        let invalid_set = TruthFoundationSet::new(
            "foundation-set-002",
            "Ters sıralanmış temeller",
        )
        .with_foundations(vec![
            risale_foundation(),
            quran_foundation(),
        ]);

        assert!(!invalid_set.preserves_foundation_order());
        assert!(!invalid_set.is_complete());
    }
}







