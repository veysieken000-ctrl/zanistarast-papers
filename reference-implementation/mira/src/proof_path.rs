//! Zanistarast içinde kullanılan ispat ve araştırma
//! yollarının veri modelidir.
//!
//! Bir hakikatin birden fazla ispat yolu bulunabilir.
//! Bu yollar birbirini dışlamak yerine destekleyebilir,
//! karşılaştırabilir ve denetleyebilir.
//!
//! Kur'an-ı Kerim'in açık hükmü, insanın oluşturduğu
//! ispat yolunun mevcut başarısına indirgenmez.
//!
//! Üstad Bediüzzaman Said-i Kürdî'nin Risale-i Nur'daki
//! kurucu akıl, mantık ve ispat yöntemleri Zanistarast
//! için bağlayıcı yöntemlerdir.
//!
//! Denetlenen Risale-i Nur yöntemi değil, Zanistarast'ın
//! yöntemi doğru tanımlayıp doğru uygulamasıdır.
//!
//! Nihai karar Müdebbir'e aittir.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofPathKind {
    Quranic,
    Prophetic,
    RisaleMethod,
    CreationBook,
    Fitrah,
    Rational,
    Logical,
    Mathematical,
    Observational,
    Experimental,
    Historical,
    Comparative,
    ZanistarastSynthesis,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofStatus {
    Established,
    Strengthening,
    InProgress,
    NotYetAvailable,
    MethodInsufficient,
    RequiresReassessment,
    RejectedPath,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImprovementDirection {
    BetterObservation,
    BetterExperiment,
    BetterMeasurement,
    BetterMathematics,
    BetterLogic,
    BetterReasoning,
    BetterConceptDefinition,
    BetterQuranAnalysis,
    BetterRisaleAnalysis,
    BetterHadithVerification,
    BetterFitrahAnalysis,
    BetterCreationBookReading,
    BetterHistoricalResearch,
    BetterSourceComparison,
    BetterTechnology,
    BetterInterdisciplinarySynthesis,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofLimitationKind {
    InsufficientEvidence,
    InsufficientObservation,
    InsufficientMeasurement,
    InsufficientTechnology,
    ConceptualAmbiguity,
    LinguisticAmbiguity,
    LogicalGap,
    MathematicalGap,
    ExperimentalDesignProblem,
    ReproducibilityProblem,
    SourceVerificationProblem,
    InterpretationProblem,
    CategoryError,
    ScopeError,
    ConflictingHumanInterpretations,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvestigatedTruthStatus {
    QuranicallyEstablished,
    PropheticallyEstablished,

    /// Risale-i Nur'daki bağlayıcı kurucu yöntemle
    /// açıklanan veya ispat edilen iman hakikati.
    ///
    /// Bu statü Kur'an'ın vahiy statüsüyle eşit değildir.
    RisaleMethodologicallyEstablished,

    HumanInterpretationOfRevelation,
    HumanResearchConclusion,
    ZanistarastHypothesis,
    OpenQuestion,
}

impl InvestigatedTruthStatus {
    pub fn is_revelationally_established(self) -> bool {
        matches!(
            self,
            Self::QuranicallyEstablished
                | Self::PropheticallyEstablished
        )
    }

    pub fn is_risale_methodologically_established(self) -> bool {
        matches!(self, Self::RisaleMethodologicallyEstablished)
    }

    pub fn is_humanly_fallible(self) -> bool {
        matches!(
            self,
            Self::HumanInterpretationOfRevelation
                | Self::HumanResearchConclusion
                | Self::ZanistarastHypothesis
                | Self::OpenQuestion
        )
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofLimitation {
    pub kind: ProofLimitationKind,
    pub description: String,
    pub improvement_direction: ImprovementDirection,
}

impl ProofLimitation {
    pub fn new(
        kind: ProofLimitationKind,
        description: impl Into<String>,
        improvement_direction: ImprovementDirection,
    ) -> Self {
        Self {
            kind,
            description: description.into(),
            improvement_direction,
        }
    }

    pub fn is_complete(&self) -> bool {
        !self.description.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofPathEvidence {
    pub evidence_id: String,
    pub source_reference: String,
    pub source_statement: String,
    pub human_analysis: String,
    pub supports_path: bool,
    pub source_verified: bool,
    pub limitations: Vec<String>,
}

impl ProofPathEvidence {
    pub fn new(
        evidence_id: impl Into<String>,
        source_reference: impl Into<String>,
    ) -> Self {
        Self {
            evidence_id: evidence_id.into(),
            source_reference: source_reference.into(),
            source_statement: String::new(),
            human_analysis: String::new(),
            supports_path: false,
            source_verified: false,
            limitations: Vec::new(),
        }
    }

    pub fn with_source_statement(
        mut self,
        source_statement: impl Into<String>,
    ) -> Self {
        self.source_statement = source_statement.into();
        self
    }

    pub fn with_human_analysis(
        mut self,
        human_analysis: impl Into<String>,
    ) -> Self {
        self.human_analysis = human_analysis.into();
        self
    }

    pub fn with_support_status(
        mut self,
        supports_path: bool,
    ) -> Self {
        self.supports_path = supports_path;
        self
    }

    pub fn mark_source_verified(mut self) -> Self {
        self.source_verified = true;
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
        !self.evidence_id.trim().is_empty()
            && !self.source_reference.trim().is_empty()
            && !self.source_statement.trim().is_empty()
    }

    pub fn separates_source_from_analysis(&self) -> bool {
        self.human_analysis.trim().is_empty()
            || self.source_statement.trim()
                != self.human_analysis.trim()
    }

    pub fn can_support_established_path(&self) -> bool {
        self.is_complete()
            && self.separates_source_from_analysis()
            && self.supports_path
            && self.source_verified
            && self.limitations.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RisaleMethodBinding {
    /// `risale_method.rs` içindeki doğrulanmış yöntem
    /// kaydının kimliğidir.
    pub method_id: String,

    /// Kullanılan yöntemin insan tarafından yazılmış kısa
    /// açıklamasıdır; orijinal Risale metni değildir.
    pub application_description: String,

    /// Yöntem kimliğinin yöntem haritasında bulunduğunu
    /// gösterir.
    pub method_identity_verified: bool,

    /// İlgili orijinal Risale metinlerinin doğrulandığını
    /// gösterir.
    pub original_sources_verified: bool,

    /// Zanistarast uygulamasının orijinal yöntemden ayrı
    /// kaydedildiğini gösterir.
    pub application_separated: bool,

    /// Rasterast'ın yöntemi değil, yöntem bağlantısını ve
    /// Zanistarast uygulamasını doğruladığını gösterir.
    pub rasterast_verified: bool,
}

impl RisaleMethodBinding {
    pub fn new(method_id: impl Into<String>) -> Self {
        Self {
            method_id: method_id.into(),
            application_description: String::new(),
            method_identity_verified: false,
            original_sources_verified: false,
            application_separated: false,
            rasterast_verified: false,
        }
    }

    pub fn with_application_description(
        mut self,
        application_description: impl Into<String>,
    ) -> Self {
        self.application_description =
            application_description.into();
        self
    }

    pub fn mark_method_identity_verified(mut self) -> Self {
        self.method_identity_verified = true;
        self
    }

    pub fn mark_original_sources_verified(mut self) -> Self {
        self.original_sources_verified = true;
        self
    }

    pub fn mark_application_separated(mut self) -> Self {
        self.application_separated = true;
        self
    }

    pub fn mark_rasterast_verified(mut self) -> Self {
        self.rasterast_verified = true;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.method_id.trim().is_empty()
            && !self.application_description.trim().is_empty()
    }

    pub fn is_verified(&self) -> bool {
        self.is_complete()
            && self.method_identity_verified
            && self.original_sources_verified
            && self.application_separated
            && self.rasterast_verified
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofPath {
    pub proof_path_id: String,
    pub subject: String,
    pub investigated_truth_status: InvestigatedTruthStatus,
    pub kind: ProofPathKind,

    /// Yalnızca `ProofPathKind::RisaleMethod` için
    /// kullanılabilecek doğrulanmış yöntem bağlantısıdır.
    pub risale_method_binding: Option<RisaleMethodBinding>,

    pub status: ProofStatus,
    pub claim: String,
    pub reasoning_steps: Vec<String>,
    pub evidence: Vec<ProofPathEvidence>,
    pub counter_evidence: Vec<ProofPathEvidence>,
    pub limitations: Vec<ProofLimitation>,
    pub alternative_explanations: Vec<String>,
    pub improvement_directions: Vec<ImprovementDirection>,
    pub rasterast_verified: bool,
    pub requires_mudebbir_decision: bool,
}

impl ProofPath {
    pub fn new(
        proof_path_id: impl Into<String>,
        subject: impl Into<String>,
        investigated_truth_status: InvestigatedTruthStatus,
        kind: ProofPathKind,
        claim: impl Into<String>,
    ) -> Self {
        Self {
            proof_path_id: proof_path_id.into(),
            subject: subject.into(),
            investigated_truth_status,
            kind,
            risale_method_binding: None,
            status: ProofStatus::InProgress,
            claim: claim.into(),
            reasoning_steps: Vec::new(),
            evidence: Vec::new(),
            counter_evidence: Vec::new(),
            limitations: Vec::new(),
            alternative_explanations: Vec::new(),
            improvement_directions: Vec::new(),
            rasterast_verified: false,
            requires_mudebbir_decision: true,
        }
    }

    pub fn with_risale_method_binding(
        mut self,
        binding: RisaleMethodBinding,
    ) -> Self {
        self.risale_method_binding = Some(binding);
        self
    }

    pub fn with_status(
        mut self,
        status: ProofStatus,
    ) -> Self {
        self.status = status;
        self
    }

    pub fn with_reasoning_steps(
        mut self,
        reasoning_steps: Vec<String>,
    ) -> Self {
        self.reasoning_steps = reasoning_steps;
        self
    }

    pub fn with_evidence(
        mut self,
        evidence: Vec<ProofPathEvidence>,
    ) -> Self {
        self.evidence = evidence;
        self
    }

    pub fn with_counter_evidence(
        mut self,
        counter_evidence: Vec<ProofPathEvidence>,
    ) -> Self {
        self.counter_evidence = counter_evidence;
        self
    }

    pub fn with_limitations(
        mut self,
        limitations: Vec<ProofLimitation>,
    ) -> Self {
        self.limitations = limitations;
        self
    }

    pub fn with_alternative_explanations(
        mut self,
        alternative_explanations: Vec<String>,
    ) -> Self {
        self.alternative_explanations =
            alternative_explanations;
        self
    }

    pub fn with_improvement_directions(
        mut self,
        improvement_directions: Vec<ImprovementDirection>,
    ) -> Self {
        self.improvement_directions =
            improvement_directions;
        self
    }

    pub fn mark_rasterast_verified(mut self) -> Self {
        self.rasterast_verified = true;
        self
    }

    pub fn is_identity_complete(&self) -> bool {
        !self.proof_path_id.trim().is_empty()
            && !self.subject.trim().is_empty()
            && !self.claim.trim().is_empty()
    }

    pub fn evidence_is_valid(&self) -> bool {
        self.evidence.iter().all(|item| {
            item.is_complete()
                && item.separates_source_from_analysis()
        })
    }

    pub fn counter_evidence_is_valid(&self) -> bool {
        self.counter_evidence.iter().all(|item| {
            item.is_complete()
                && item.separates_source_from_analysis()
        })
    }

    pub fn limitations_are_valid(&self) -> bool {
        self.limitations
            .iter()
            .all(ProofLimitation::is_complete)
    }

    pub fn reasoning_steps_are_valid(&self) -> bool {
        self.reasoning_steps
            .iter()
            .all(|step| !step.trim().is_empty())
    }

    pub fn is_complete(&self) -> bool {
        self.is_identity_complete()
            && self.reasoning_steps_are_valid()
            && self.evidence_is_valid()
            && self.counter_evidence_is_valid()
            && self.limitations_are_valid()
    }

pub fn has_valid_risale_method_binding(&self) -> bool {
        match self.kind {
            ProofPathKind::RisaleMethod => self
                .risale_method_binding
                .as_ref()
                .map(RisaleMethodBinding::is_verified)
                .unwrap_or(false),

            _ => self.risale_method_binding.is_none(),
        }
    }

    pub fn preserves_risale_truth_status(&self) -> bool {
        if self.investigated_truth_status
            == InvestigatedTruthStatus::
                RisaleMethodologicallyEstablished
        {
            self.kind == ProofPathKind::RisaleMethod
                && self.has_valid_risale_method_binding()
        } else {
            true
        }
    }

    pub fn supporting_evidence_count(&self) -> usize {
        self.evidence
            .iter()
            .filter(|item| item.supports_path)
            .count()
    }

    pub fn verified_supporting_evidence_count(&self) -> usize {
        self.evidence
            .iter()
            .filter(|item| {
                item.can_support_established_path()
            })
            .count()
    }

    pub fn has_unresolved_items(&self) -> bool {
        !self.counter_evidence.is_empty()
            || !self.limitations.is_empty()
            || !self.alternative_explanations.is_empty()
    }

    pub fn proof_failure_does_not_negate_revelational_truth(
        &self,
    ) -> bool {
        self.investigated_truth_status
            .is_revelationally_established()
            && matches!(
                self.status,
                ProofStatus::NotYetAvailable
                    | ProofStatus::MethodInsufficient
                    | ProofStatus::RequiresReassessment
                    | ProofStatus::RejectedPath
            )
    }

    pub fn risale_application_failure_does_not_reject_method(
        &self,
    ) -> bool {
        self.kind == ProofPathKind::RisaleMethod
            && matches!(
                self.status,
                ProofStatus::MethodInsufficient
                    | ProofStatus::RequiresReassessment
                    | ProofStatus::RejectedPath
            )
    }

    pub fn human_model_remains_fallible(&self) -> bool {
        self.investigated_truth_status.is_humanly_fallible()
    }

    pub fn insufficient_method_has_improvement_direction(
        &self,
    ) -> bool {
        if matches!(
            self.status,
            ProofStatus::NotYetAvailable
                | ProofStatus::MethodInsufficient
                | ProofStatus::RequiresReassessment
        ) {
            !self.improvement_directions.is_empty()
                || self.limitations.iter().any(|item| {
                    item.improvement_direction
                        != ImprovementDirection::Unknown
                })
        } else {
            true
        }
    }

    pub fn can_be_treated_as_established_path(&self) -> bool {
        self.is_complete()
            && self.has_valid_risale_method_binding()
            && self.preserves_risale_truth_status()
            && self.status == ProofStatus::Established
            && self.verified_supporting_evidence_count() > 0
            && !self.has_unresolved_items()
            && self.rasterast_verified
            && self.requires_mudebbir_decision
    }

    pub fn is_constitutionally_valid(&self) -> bool {
        self.is_complete()
            && self.has_valid_risale_method_binding()
            && self.preserves_risale_truth_status()
            && self.insufficient_method_has_improvement_direction()
            && self.requires_mudebbir_decision
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofPathSet {
    pub subject: String,
    pub paths: Vec<ProofPath>,
}

impl ProofPathSet {
    pub fn new(subject: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
            paths: Vec::new(),
        }
    }

    pub fn with_paths(
        mut self,
        paths: Vec<ProofPath>,
    ) -> Self {
        self.paths = paths;
        self
    }

    pub fn add_path(&mut self, path: ProofPath) {
        self.paths.push(path);
    }

    pub fn has_path_kind(&self, kind: ProofPathKind) -> bool {
        self.paths.iter().any(|path| path.kind == kind)
    }

    pub fn established_path_count(&self) -> usize {
        self.paths
            .iter()
            .filter(|path| {
                path.can_be_treated_as_established_path()
            })
            .count()
    }

    pub fn paths_requiring_improvement(
        &self,
    ) -> Vec<&ProofPath> {
        self.paths
            .iter()
            .filter(|path| {
                matches!(
                    path.status,
                    ProofStatus::NotYetAvailable
                        | ProofStatus::MethodInsufficient
                        | ProofStatus::RequiresReassessment
                )
            })
            .collect()
    }

    pub fn risale_method_paths(&self) -> Vec<&ProofPath> {
        self.paths
            .iter()
            .filter(|path| {
                path.kind == ProofPathKind::RisaleMethod
            })
            .collect()
    }

    pub fn invalid_paths(&self) -> Vec<&ProofPath> {
        self.paths
            .iter()
            .filter(|path| {
                !path.is_constitutionally_valid()
            })
            .collect()
    }

    pub fn is_complete(&self) -> bool {
        !self.subject.trim().is_empty()
            && !self.paths.is_empty()
            && self.invalid_paths().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn supporting_evidence() -> ProofPathEvidence {
        ProofPathEvidence::new(
            "evidence-001",
            "Doğrulanmış örnek kaynak",
        )
        .with_source_statement(
            "Kaynağın kendi açık ifadesi.",
        )
        .with_human_analysis(
            "İfadenin Zanistarast açısından analizi.",
        )
        .with_support_status(true)
        .mark_source_verified()
    }

    fn verified_risale_binding() -> RisaleMethodBinding {
        RisaleMethodBinding::new(
            "risale-method-001",
        )
        .with_application_description(
            "Kâinat kitabı, düzen ve hikmet üzerinden okunur.",
        )
        .mark_method_identity_verified()
        .mark_original_sources_verified()
        .mark_application_separated()
        .mark_rasterast_verified()
    }

    #[test]
    fn missing_empirical_method_does_not_negate_quranic_truth() {
        let path = ProofPath::new(
            "proof-001",
            "Meleklerin varlığı",
            InvestigatedTruthStatus::QuranicallyEstablished,
            ProofPathKind::Experimental,
            "Meleklerin varlığının deneysel gösterim yolu.",
        )
        .with_status(ProofStatus::NotYetAvailable)
        .with_improvement_directions(vec![
            ImprovementDirection::BetterConceptDefinition,
            ImprovementDirection::BetterTechnology,
        ]);

        assert!(
            path.proof_failure_does_not_negate_revelational_truth()
        );
        assert!(
            path.insufficient_method_has_improvement_direction()
        );
        assert!(path.is_constitutionally_valid());
    }

    #[test]
    fn zanistarast_hypothesis_remains_fallible() {
        let path = ProofPath::new(
            "proof-002",
            "Altı boyutlu evren mimarisi",
            InvestigatedTruthStatus::ZanistarastHypothesis,
            ProofPathKind::ZanistarastSynthesis,
            "Evren için altı boyutlu mimari önerisi.",
        );

        assert!(path.human_model_remains_fallible());
        assert!(
            !path
                .proof_failure_does_not_negate_revelational_truth()
        );
    }

    #[test]
    fn established_path_requires_verified_evidence() {
        let path = ProofPath::new(
            "proof-003",
            "Örnek mantıksal hüküm",
            InvestigatedTruthStatus::HumanResearchConclusion,
            ProofPathKind::Logical,
            "Örnek sonuç geçerli çıkarımla desteklenmiştir.",
        )
        .with_status(ProofStatus::Established)
        .with_reasoning_steps(vec![
            "Birinci öncül.".to_string(),
            "İkinci öncül.".to_string(),
            "Sonuç.".to_string(),
        ])
        .with_evidence(vec![
            supporting_evidence(),
        ])
        .mark_rasterast_verified();

        assert!(path.is_complete());
        assert_eq!(
            path.verified_supporting_evidence_count(),
            1
        );
        assert!(
            path.can_be_treated_as_established_path()
        );
    }

    #[test]
    fn unresolved_limitation_blocks_established_path() {
        let limitation = ProofLimitation::new(
            ProofLimitationKind::LogicalGap,
            "İkinci öncülden sonuca geçiş açıklanmalıdır.",
            ImprovementDirection::BetterLogic,
        );

        let path = ProofPath::new(
            "proof-004",
            "Eksik mantıksal yol",
            InvestigatedTruthStatus::HumanResearchConclusion,
            ProofPathKind::Logical,
            "İddia mantıksal olarak kurulmaya çalışılıyor.",
        )
        .with_status(ProofStatus::Established)
        .with_evidence(vec![
            supporting_evidence(),
        ])
        .with_limitations(vec![
            limitation,
        ])
        .mark_rasterast_verified();

        assert!(path.has_unresolved_items());
        assert!(
            !path.can_be_treated_as_established_path()
        );
    }

    #[test]
    fn insufficient_method_requires_improvement_direction() {
        let path = ProofPath::new(
            "proof-005",
            "Araştırma konusu",
            InvestigatedTruthStatus::OpenQuestion,
            ProofPathKind::Observational,
            "Gözlemsel yol henüz yeterli değildir.",
        )
        .with_status(ProofStatus::MethodInsufficient);

        assert!(
            !path.insufficient_method_has_improvement_direction()
        );
        assert!(!path.is_constitutionally_valid());
    }

    #[test]
    fn source_and_human_analysis_remain_separate() {
        let evidence = ProofPathEvidence::new(
            "evidence-002",
            "Örnek kaynak",
        )
        .with_source_statement("Aynı ifade")
        .with_human_analysis("Aynı ifade")
        .with_support_status(true)
        .mark_source_verified();

        assert!(
            !evidence.separates_source_from_analysis()
        );
        assert!(
            !evidence.can_support_established_path()
        );
    }

  #[test]
    fn risale_path_requires_verified_method_binding() {
        let path = ProofPath::new(
            "proof-risale-001",
            "Tevhid ispat yöntemi",
            InvestigatedTruthStatus::
                RisaleMethodologicallyEstablished,
            ProofPathKind::RisaleMethod,
            "Risale-i Nur'un kurucu ispat yöntemi kullanılır.",
        )
        .with_evidence(vec![
            supporting_evidence(),
        ]);

        assert!(
            !path.has_valid_risale_method_binding()
        );
        assert!(!path.preserves_risale_truth_status());
        assert!(!path.is_constitutionally_valid());
    }

    #[test]
    fn verified_risale_binding_is_constitutionally_valid() {
        let path = ProofPath::new(
            "proof-risale-002",
            "Kâinat kitabından tevhid okuması",
            InvestigatedTruthStatus::
                RisaleMethodologicallyEstablished,
            ProofPathKind::RisaleMethod,
            "Düzen, hikmet ve birlik üzerinden tevhid okunur.",
        )
        .with_risale_method_binding(
            verified_risale_binding(),
        )
        .with_evidence(vec![
            supporting_evidence(),
        ]);

        assert!(path.has_valid_risale_method_binding());
        assert!(path.preserves_risale_truth_status());
        assert!(path.is_constitutionally_valid());
    }

    #[test]
    fn non_risale_path_cannot_hold_risale_binding() {
        let path = ProofPath::new(
            "proof-logical-001",
            "Mantıksal çıkarım",
            InvestigatedTruthStatus::HumanResearchConclusion,
            ProofPathKind::Logical,
            "Mantıksal bir ispat yolu.",
        )
        .with_risale_method_binding(
            verified_risale_binding(),
        )
        .with_evidence(vec![
            supporting_evidence(),
        ]);

        assert!(
            !path.has_valid_risale_method_binding()
        );
        assert!(!path.is_constitutionally_valid());
    }

    #[test]
    fn risale_application_failure_does_not_reject_method() {
        let path = ProofPath::new(
            "proof-risale-003",
            "Zanistarast yöntem uygulaması",
            InvestigatedTruthStatus::
                HumanInterpretationOfRevelation,
            ProofPathKind::RisaleMethod,
            "Risale yönteminin bilimsel alana uygulanması.",
        )
        .with_risale_method_binding(
            verified_risale_binding(),
        )
        .with_status(ProofStatus::RequiresReassessment)
        .with_improvement_directions(vec![
            ImprovementDirection::BetterRisaleAnalysis,
        ]);

        assert!(
            path.risale_application_failure_does_not_reject_method()
        );
        assert!(
            path.insufficient_method_has_improvement_direction()
        );
    }

    #[test]
    fn proof_path_set_combines_multiple_methods() {
        let quranic_path = ProofPath::new(
            "proof-006",
            "Meleklerin varlığı",
            InvestigatedTruthStatus::QuranicallyEstablished,
            ProofPathKind::Quranic,
            "Kur'an-ı Kerim meleklerin varlığını bildirir.",
        )
        .with_status(ProofStatus::Established)
        .with_evidence(vec![
            supporting_evidence(),
        ])
        .mark_rasterast_verified();

        let experimental_path = ProofPath::new(
            "proof-007",
            "Meleklerin varlığı",
            InvestigatedTruthStatus::QuranicallyEstablished,
            ProofPathKind::Experimental,
            "Deneysel gösterim yolu araştırılmaktadır.",
        )
        .with_status(ProofStatus::NotYetAvailable)
        .with_improvement_directions(vec![
            ImprovementDirection::BetterConceptDefinition,
            ImprovementDirection::BetterTechnology,
        ]);

        let set = ProofPathSet::new(
            "Meleklerin varlığı ve ispat yolları",
        )
        .with_paths(vec![
            quranic_path,
            experimental_path,
        ]);

        assert!(
            set.has_path_kind(ProofPathKind::Quranic)
        );
        assert!(
            set.has_path_kind(ProofPathKind::Experimental)
        );
        assert_eq!(set.established_path_count(), 1);
        assert_eq!(
            set.paths_requiring_improvement().len(),
            1
        );
        assert!(set.is_complete());
    }

    #[test]
    fn proof_path_set_finds_risale_paths() {
        let risale_path = ProofPath::new(
            "proof-risale-004",
            "Tevhid yöntemi",
            InvestigatedTruthStatus::
                RisaleMethodologicallyEstablished,
            ProofPathKind::RisaleMethod,
            "Kâinat kitabından tevhid okunur.",
        )
        .with_risale_method_binding(
            verified_risale_binding(),
        )
        .with_evidence(vec![
            supporting_evidence(),
        ]);

        let set = ProofPathSet::new(
            "Risale-i Nur ispat yolları",
        )
        .with_paths(vec![
            risale_path,
        ]);

        assert_eq!(set.risale_method_paths().len(), 1);
        assert!(set.is_complete());
    }

    #[test]
    fn rasterast_does_not_remove_mudebbir_gate() {
        let path = ProofPath::new(
            "proof-008",
            "Örnek hüküm",
            InvestigatedTruthStatus::HumanResearchConclusion,
            ProofPathKind::Rational,
            "Akli çıkarım örneği.",
        )
        .with_evidence(vec![
            supporting_evidence(),
        ])
        .mark_rasterast_verified();

        assert!(path.rasterast_verified);
        assert!(path.requires_mudebbir_decision);
    }
}


