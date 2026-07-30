/// Zanistarast içinde kullanılan temel ispat ve araştırma
/// yollarını gösterir.
///
/// Bir hakikatin birden fazla ispat yolu olabilir.
/// Bu yollar birbirini dışlamak yerine karşılıklı olarak
/// destekleyebilir ve denetleyebilir.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProofPathKind {
    /// Kur'an-ı Kerim'in açık bildirimi ve ayetler arası
    /// bütünlük üzerinden kurulan yol.
    Quranic,

    /// Sıhhati doğrulanmış nebevî bildirimler üzerinden
    /// kurulan yol.
    Prophetic,

    /// Risale-i Nur'un orijinal metinlerinde kullanılan
    /// temsil, kıyas, çıkarım ve ispat yöntemleri.
    RisaleMethod,

    /// Kâinattaki düzen, yapı, yasa, ilişki ve oluşumların
    /// okunması üzerinden kurulan yol.
    CreationBook,

    /// Varlıkların yaratılış yapısı, yönelimi ve uygunluğu
    /// üzerinden kurulan fıtrat delili.
    Fitrah,

    /// Akli değerlendirme ve zorunlu sonuç çıkarma yolu.
    Rational,

    /// Çelişmezlik, geçerli çıkarım ve kavramsal tutarlılık
    /// üzerinden kurulan yol.
    Logical,

    /// Nicel ilişkilerin sembolik veya biçimsel olarak
    /// gösterilmesi.
    Mathematical,

    /// Doğrudan veya araçlı gözleme dayanan yol.
    Observational,

    /// Kontrollü sınama ve tekrarlanabilir deney yolu.
    Experimental,

    /// Tarihsel kayıt, belge, tanıklık ve olay incelemesi.
    Historical,

    /// Farklı kaynakların, modellerin ve açıklamaların
    /// karşılaştırılması.
    Comparative,

    /// Zanistarast'ın farklı bilgi ve ispat yollarını
    /// birleştirerek oluşturduğu insanî sentez.
    ZanistarastSynthesis,
}

/// Bir ispat yolunun mevcut çalışma durumudur.
///
/// Bu durum hakikatin ontolojik statüsünü değil,
/// insanın hakikati gösterme ve açıklama kapasitesini
/// ifade eder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProofStatus {
    /// İlgili ispat yolu kendi ölçütlerine göre kurulmuş
    /// ve Rasterast denetiminden geçirilmiştir.
    Established,

    /// İspat yolu güçlüdür; fakat yeni delil, açıklama veya
    /// karşılaştırmalarla geliştirilmektedir.
    Strengthening,

    /// Araştırma ve ispat çalışması devam etmektedir.
    InProgress,

    /// Mevcut insan bilgisi ve araçlarıyla yeterli yöntem
    /// henüz kurulmamıştır.
    ///
    /// Bu durum tek başına hakikatin yanlışlığını göstermez.
    NotYetAvailable,

    /// Kullanılan yöntemde önemli eksiklik belirlenmiştir.
    MethodInsufficient,

    /// Deliller veya çıkarımlar arasında çözümlenmemiş
    /// uyuşmazlık vardır.
    RequiresReassessment,

    /// Bu özel ispat yolu geçersiz bulunmuştur.
    ///
    /// Yolun reddedilmesi, araştırılan hakikatin otomatik
    /// olarak reddedilmesi anlamına gelmez.
    RejectedPath,
}

/// İspat yolunun geliştirilmesi gereken başlıca alanı
/// gösterir.
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

/// Bir ispat yolunun başarısızlık veya yetersizlik sebebidir.
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

/// Araştırılan hükmün Zanistarast içindeki temel statüsünü
/// belirtir.
///
/// Bu enum, insanın kurduğu ispat yoluyla hükmün kendisini
/// birbirinden ayırmak için kullanılır.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvestigatedTruthStatus {
    /// Kur'an-ı Kerim'in açık hükmüyle sabit kabul edilen
    /// vahyî hakikat.
    QuranicallyEstablished,

    /// Sıhhati doğrulanmış nebevî bildirimle desteklenen
    /// hakikat.
    PropheticallyEstablished,

    /// Kur'an veya sahih hadis hükmünün insan tarafından
    /// yapılan açıklaması.
    HumanInterpretationOfRevelation,

    /// Kâinat, fıtrat, akıl, mantık veya bilimsel inceleme
    /// sonucunda önerilen insanî açıklama.
    HumanResearchConclusion,

    /// Zanistarast tarafından önerilen ve yanlışlanabilir
    /// insanî hipotez veya sentez.
    ZanistarastHypothesis,

    /// Henüz hüküm verilemeyen araştırma alanı.
    OpenQuestion,
}

impl InvestigatedTruthStatus {
    /// İspat yolundaki eksiklik nedeniyle hakikat statüsünün
    /// düşürülemeyeceği vahyî durumları gösterir.
    pub fn is_revelationally_established(self) -> bool {
        matches!(
            self,
            Self::QuranicallyEstablished
                | Self::PropheticallyEstablished
        )
    }

    /// İnsan yorumunun, bilimsel modelin veya Zanistarast
    /// sentezinin yanlışlanabilir olup olmadığını gösterir.
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

/// Bir ispat yolunda belirlenen eksikliktir.
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

/// Bir ispat yolunun kaynak ve içerik bilgilerini taşır.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofPathEvidence {
    pub evidence_id: String,
    pub source_reference: String,
    pub source_statement: String,
    pub human_analysis: String,
    pub supports_path: bool,
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

    /// Kaynak metniyle insan analizinin aynı kayıt gibi
    /// gösterilmesini engeller.
    pub fn separates_source_from_analysis(&self) -> bool {
        self.human_analysis.trim().is_empty()
            || self.source_statement.trim()
                != self.human_analysis.trim()
    }
}

/// Tek bir ispat yolunun ayrıntılı kaydıdır.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProofPath {
    pub proof_path_id: String,
    pub subject: String,
    pub investigated_truth_status: InvestigatedTruthStatus,
    pub kind: ProofPathKind,
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

    pub fn is_complete(&self) -> bool {
        !self.proof_path_id.trim().is_empty()
            && !self.subject.trim().is_empty()
            && !self.claim.trim().is_empty()
            && self
                .evidence
                .iter()
                .all(ProofPathEvidence::is_complete)
            && self
                .counter_evidence
                .iter()
                .all(ProofPathEvidence::is_complete)
            && self
                .evidence
                .iter()
                .all(
                    ProofPathEvidence::
                        separates_source_from_analysis,
                )
            && self
                .counter_evidence
                .iter()
                .all(
                    ProofPathEvidence::
                        separates_source_from_analysis,
                )
            && self
                .limitations
                .iter()
                .all(ProofLimitation::is_complete)
    }

    pub fn supporting_evidence_count(&self) -> usize {
        self.evidence
            .iter()
            .filter(|item| item.supports_path)
            .count()
    }

    pub fn has_unresolved_items(&self) -> bool {
        !self.counter_evidence.is_empty()
            || !self.limitations.is_empty()
            || !self.alternative_explanations.is_empty()
    }

    /// Vahyî hakikat ile insanın kurduğu ispat yolunun
    /// durumunu birbirinden ayırır.
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

    /// İnsanî bir modelin sırf vahiy ile ilişkilendirildiği
    /// için yanlışlanamaz hâle getirilmesini engeller.
    pub fn human_model_remains_fallible(&self) -> bool {
        self.investigated_truth_status.is_humanly_fallible()
    }

    /// Yöntem yetersiz olduğunda geliştirme yönü
    /// gösterilmesini zorunlu tutar.
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
                || self
                    .limitations
                    .iter()
                    .any(|item| {
                        item.improvement_direction
                            != ImprovementDirection::Unknown
                    })
        } else {
            true
        }
    }

    /// Bir ispat yolunun kurulmuş sayılabilmesi için gereken
    /// temel Rasterast koşullarını denetler.
    pub fn can_be_treated_as_established_path(&self) -> bool {
        self.is_complete()
            && self.status == ProofStatus::Established
            && self.supporting_evidence_count() > 0
            && !self.has_unresolved_items()
            && self.rasterast_verified
    }

    pub fn is_constitutionally_valid(&self) -> bool {
        self.is_complete()
            && self.insufficient_method_has_improvement_direction()
            && self.requires_mudebbir_decision
    }
}

/// Aynı konuya ilişkin farklı ispat yollarını birlikte tutar.
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

    pub fn with_paths(mut self, paths: Vec<ProofPath>) -> Self {
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

    pub fn paths_requiring_improvement(&self) -> Vec<&ProofPath> {
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

    pub fn invalid_paths(&self) -> Vec<&ProofPath> {
        self.paths
            .iter()
            .filter(|path| !path.is_constitutionally_valid())
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
    fn established_path_requires_evidence_and_rasterast() {
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
        .with_evidence(vec![supporting_evidence()])
        .mark_rasterast_verified();

        assert!(path.is_complete());
        assert!(path.can_be_treated_as_established_path());
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
        .with_evidence(vec![supporting_evidence()])
        .with_limitations(vec![limitation])
        .mark_rasterast_verified();

        assert!(path.has_unresolved_items());
        assert!(!path.can_be_treated_as_established_path());
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
    fn source_and_human_analysis_must_remain_separate() {
        let evidence = ProofPathEvidence::new(
            "evidence-002",
            "Örnek kaynak",
        )
        .with_source_statement("Aynı ifade")
        .with_human_analysis("Aynı ifade")
        .with_support_status(true);

        assert!(!evidence.separates_source_from_analysis());
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
        .with_evidence(vec![supporting_evidence()])
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
        .with_paths(vec![quranic_path, experimental_path]);

        assert!(set.has_path_kind(ProofPathKind::Quranic));
        assert!(set.has_path_kind(ProofPathKind::Experimental));
        assert_eq!(set.established_path_count(), 1);
        assert_eq!(set.paths_requiring_improvement().len(), 1);
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
        .with_evidence(vec![supporting_evidence()])
        .mark_rasterast_verified();

        assert!(path.rasterast_verified);
        assert!(path.requires_mudebbir_decision);
    }
}



   
