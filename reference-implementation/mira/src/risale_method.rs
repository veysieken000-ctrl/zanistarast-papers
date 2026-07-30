/// Risale-i Nur'un Zanistarast içindeki kurucu yöntem
/// konumunu temsil eden veri modelidir.
///
/// Anayasal ayrım:
///
/// - Kur'an-ı Kerim mutlak vahyî hakikatin kaynağıdır.
/// - Üstad Bediüzzaman Said-i Kürdî'nin Risale-i Nur'da
/// kullandığı aklî, mantıkî ve ispatlayıcı yöntemler
/// Zanistarast için bağlayıcı kurucu yöntemlerdir.
/// - Risale-i Nur'un vahiy statüsü yoktur ve Kur'an-ı
/// Kerim ile aynı ontolojik konuma yerleştirilemez.
/// - Denetlenen Risale-i Nur'un yöntemi değil,
/// Zanistarast'ın yöntemi anlayış ve uygulama biçimidir.
/// - Nihai karar Müdebbir'e aittir.

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RisaleMethodAuthority {
    QuranicProofMethod,
    BindingReasoningMethod,
    BindingLogicalMethod,
    BindingFaithReasonMethod,
    BindingHumanUnderstandingMethod,
    BindingCreationReadingMethod,
    BindingMoralFitrahMethod,
}

impl RisaleMethodAuthority {
    pub fn is_binding(self) -> bool {
        true
    }

    pub fn is_revelation(self) -> bool {
        false
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RisaleMethodKind {
    Representation,
    Comparison,
    Analogy,
    NecessaryReasoning,
    ContradictionAnalysis,
    CollectiveEvidence,
    FromWorkToMaker,
    FromActToNameAndAttribute,
    OrderReading,
    WisdomReading,
    MercyAndProvidenceReading,
    UnityFromMultiplicity,
    DivineNamesReading,
    CreationBookReading,
    HumanReading,
    FitrahReading,
    ConscienceReading,
    HeartReading,
    ImpotenceMethod,
    PovertyMethod,
    CompassionMethod,
    ReflectionMethod,
    FaithReasonIntegration,
    HeartReasonIntegration,
    EmotionReasonIntegration,
    SoulBodyIntegration,
    MoralReasoning,
    MortalityAndPermanenceReading,
    ResurrectionReasoning,
    ProphethoodReasoning,
    UnityReasoning,
    IntegratedProof,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HumanDimension {
    Essence,
    Spirit,
    Heart,
    Conscience,
    Emotion,
    Intellect,
    Reason,
    Logic,
    Morality,
    Will,
    Intention,
    Faith,
    Love,
    Fear,
    Hope,
    Imagination,
    Memory,
    Selfhood,
    Body,
    Action,
    Responsibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReadingDomain {
    Quran,
    Creation,
    Human,
    Fitrah,
    Spirit,
    HeartAndConscience,
    Morality,
    Society,
    History,
    Science,
    Life,
    Afterlife,
    FaithTruths,
    Integrated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RisaleProofType {
    Quranic,
    Rational,
    Logical,
    Fitrah,
    Conscience,
    Moral,
    Spiritual,
    Experiential,
    Observational,
    CreationBased,
    HumanBased,
    Historical,
    Comparative,
    Representative,
    NecessaryInference,
    Integrated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RisaleMethodReviewStatus {
    NotStarted,
    SourceLocated,
    OriginalTextVerified,
    MethodIdentificationInProgress,
    RequiresContextReview,
    RequiresQuranRelationReview,
    RequiresLogicReview,
    RequiresHumanDimensionReview,
    RequiresCreationRelationReview,
    RequiresRasterastReview,
    AwaitingMudebbirDecision,
    ApprovedForUse,
    ApplicationRequiresCorrection,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RisaleReasoningOrigin {
    OriginalRisaleMethod,
    ZanistarastApplication,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RisaleSourceReference {
    pub work_name: String,
    pub section_name: String,
    pub page_or_location: String,
    pub original_text: String,
    pub surrounding_context: String,
    pub edition_information: String,
    pub original_text_verified: bool,
}

impl RisaleSourceReference {
    pub fn new(
        work_name: impl Into<String>,
        section_name: impl Into<String>,
        page_or_location: impl Into<String>,
    ) -> Self {
        Self {
            work_name: work_name.into(),
            section_name: section_name.into(),
            page_or_location: page_or_location.into(),
            original_text: String::new(),
            surrounding_context: String::new(),
            edition_information: String::new(),
            original_text_verified: false,
        }
    }

    pub fn with_original_text(
        mut self,
        original_text: impl Into<String>,
    ) -> Self {
        self.original_text = original_text.into();
        self
    }

    pub fn with_surrounding_context(
        mut self,
        surrounding_context: impl Into<String>,
    ) -> Self {
        self.surrounding_context = surrounding_context.into();
        self
    }

    pub fn with_edition_information(
        mut self,
        edition_information: impl Into<String>,
    ) -> Self {
        self.edition_information = edition_information.into();
        self
    }

    pub fn mark_original_text_verified(mut self) -> Self {
        self.original_text_verified = true;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.work_name.trim().is_empty()
            && !self.section_name.trim().is_empty()
            && !self.page_or_location.trim().is_empty()
            && !self.original_text.trim().is_empty()
            && !self.surrounding_context.trim().is_empty()
            && !self.edition_information.trim().is_empty()
    }

    pub fn is_verified(&self) -> bool {
        self.is_complete() && self.original_text_verified
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RisaleReasoningStep {
    pub order: usize,
    pub statement: String,
    pub explanation: String,
    pub origin: RisaleReasoningOrigin,
    pub source_reference_index: Option<usize>,
}

impl RisaleReasoningStep {
    pub fn new(
        order: usize,
        statement: impl Into<String>,
        explanation: impl Into<String>,
        origin: RisaleReasoningOrigin,
    ) -> Self {
        Self {
            order,
            statement: statement.into(),
            explanation: explanation.into(),
            origin,
            source_reference_index: None,
        }
    }

    pub fn with_source_reference_index(
        mut self,
        source_reference_index: usize,
    ) -> Self {
        self.source_reference_index = Some(source_reference_index);
        self
    }

    pub fn is_complete(&self) -> bool {
        self.order > 0
            && !self.statement.trim().is_empty()
            && !self.explanation.trim().is_empty()
    }

    pub fn is_original_risale_step(&self) -> bool {
        self.origin == RisaleReasoningOrigin::OriginalRisaleMethod
    }

    pub fn is_zanistarast_application_step(&self) -> bool {
        self.origin == RisaleReasoningOrigin::ZanistarastApplication
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RisaleProvenTruth {
    pub name: String,
    pub description: String,
    pub related_quran_topics: Vec<String>,
    pub related_human_dimensions: Vec<HumanDimension>,
    pub related_creation_signs: Vec<String>,
}

impl RisaleProvenTruth {
    pub fn new(
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            related_quran_topics: Vec::new(),
            related_human_dimensions: Vec::new(),
            related_creation_signs: Vec::new(),
        }
    }

    pub fn with_related_quran_topics(
        mut self,
        topics: Vec<String>,
    ) -> Self {
        self.related_quran_topics = topics;
        self
    }

    pub fn with_related_human_dimensions(
        mut self,
        dimensions: Vec<HumanDimension>,
    ) -> Self {
        self.related_human_dimensions = dimensions;
        self
    }

    pub fn with_related_creation_signs(
        mut self,
        signs: Vec<String>,
    ) -> Self {
        self.related_creation_signs = signs;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.name.trim().is_empty()
            && !self.description.trim().is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZanistarastMethodApplication {
    pub application_id: String,
    pub scientific_domain: String,
    pub proposed_model: String,
    pub applied_method_description: String,
    pub expected_benefits: Vec<String>,
    pub risks: Vec<String>,
    pub uncertainties: Vec<String>,
    pub source_method_preserved: bool,
    pub rasterast_verified: bool,
    pub requires_mudebbir_decision: bool,
}

impl ZanistarastMethodApplication {
    pub fn new(
        application_id: impl Into<String>,
        scientific_domain: impl Into<String>,
        proposed_model: impl Into<String>,
    ) -> Self {
        Self {
            application_id: application_id.into(),
            scientific_domain: scientific_domain.into(),
            proposed_model: proposed_model.into(),
            applied_method_description: String::new(),
            expected_benefits: Vec::new(),
            risks: Vec::new(),
            uncertainties: Vec::new(),
            source_method_preserved: false,
            rasterast_verified: false,
            requires_mudebbir_decision: true,
        }
    }

    pub fn with_applied_method_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.applied_method_description = description.into();
        self
    }

    pub fn with_expected_benefits(
        mut self,
        benefits: Vec<String>,
    ) -> Self {
        self.expected_benefits = benefits;
        self
    }

    pub fn with_risks(mut self, risks: Vec<String>) -> Self {
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

    pub fn mark_source_method_preserved(mut self) -> Self {
        self.source_method_preserved = true;
        self
    }

    pub fn mark_rasterast_verified(mut self) -> Self {
        self.rasterast_verified = true;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.application_id.trim().is_empty()
            && !self.scientific_domain.trim().is_empty()
            && !self.proposed_model.trim().is_empty()
            && !self.applied_method_description.trim().is_empty()
    }

    pub fn can_be_approved(&self) -> bool {
        self.is_complete()
            && self.source_method_preserved
            && self.rasterast_verified
            && self.requires_mudebbir_decision
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RisaleMethod {
    pub method_id: String,
    pub name: String,
    pub description: String,

    pub authority: RisaleMethodAuthority,
    pub method_kind: RisaleMethodKind,
    pub reading_domain: ReadingDomain,
    pub proof_type: RisaleProofType,
    pub review_status: RisaleMethodReviewStatus,

    pub addressed_dimensions: Vec<HumanDimension>,
    pub source_references: Vec<RisaleSourceReference>,
    pub reasoning_steps: Vec<RisaleReasoningStep>,
    pub proven_truths: Vec<RisaleProvenTruth>,

    pub related_quran_topics: Vec<String>,
    pub creation_book_correspondences: Vec<String>,
    pub fitrah_correspondences: Vec<String>,
    pub moral_correspondences: Vec<String>,

    pub zanistarast_interpretation: String,
    pub scientific_applications: Vec<ZanistarastMethodApplication>,

    pub interpretation_risks: Vec<String>,
    pub unresolved_questions: Vec<String>,
    pub contradictions_in_application: Vec<String>,

    pub original_method_preserved: bool,
    pub rasterast_verified: bool,
    pub requires_mudebbir_decision: bool,
}

impl RisaleMethod {
    pub fn new(
        method_id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
        authority: RisaleMethodAuthority,
        method_kind: RisaleMethodKind,
        reading_domain: ReadingDomain,
        proof_type: RisaleProofType,
    ) -> Self {
        Self {
            method_id: method_id.into(),
            name: name.into(),
            description: description.into(),

            authority,
            method_kind,
            reading_domain,
            proof_type,
            review_status: RisaleMethodReviewStatus::NotStarted,

            addressed_dimensions: Vec::new(),
            source_references: Vec::new(),
            reasoning_steps: Vec::new(),
            proven_truths: Vec::new(),

            related_quran_topics: Vec::new(),
            creation_book_correspondences: Vec::new(),
            fitrah_correspondences: Vec::new(),
            moral_correspondences: Vec::new(),

            zanistarast_interpretation: String::new(),
            scientific_applications: Vec::new(),

            interpretation_risks: Vec::new(),
            unresolved_questions: Vec::new(),
            contradictions_in_application: Vec::new(),

            original_method_preserved: false,
            rasterast_verified: false,
            requires_mudebbir_decision: true,
        }
    }

    pub fn with_review_status(
        mut self,
        status: RisaleMethodReviewStatus,
    ) -> Self {
        self.review_status = status;
        self
    }

    pub fn with_addressed_dimensions(
        mut self,
        dimensions: Vec<HumanDimension>,
    ) -> Self {
        self.addressed_dimensions = dimensions;
        self
    }

    pub fn with_source_references(
        mut self,
        references: Vec<RisaleSourceReference>,
    ) -> Self {
        self.source_references = references;
        self
    }

    pub fn with_reasoning_steps(
        mut self,
        steps: Vec<RisaleReasoningStep>,
    ) -> Self {
        self.reasoning_steps = steps;
        self
    }

    pub fn with_proven_truths(
        mut self,
        truths: Vec<RisaleProvenTruth>,
    ) -> Self {
        self.proven_truths = truths;
        self
    }

    pub fn with_related_quran_topics(
        mut self,
        topics: Vec<String>,
    ) -> Self {
        self.related_quran_topics = topics;
        self
    }

    pub fn with_creation_book_correspondences(
        mut self,
        correspondences: Vec<String>,
    ) -> Self {
        self.creation_book_correspondences = correspondences;
        self
    }

    pub fn with_fitrah_correspondences(
        mut self,
        correspondences: Vec<String>,
    ) -> Self {
        self.fitrah_correspondences = correspondences;
        self
    }

    pub fn with_moral_correspondences(
        mut self,
        correspondences: Vec<String>,
    ) -> Self {
        self.moral_correspondences = correspondences;
        self
    }

    pub fn with_zanistarast_interpretation(
        mut self,
        interpretation: impl Into<String>,
    ) -> Self {
        self.zanistarast_interpretation = interpretation.into();
        self
    }

    pub fn with_scientific_applications(
        mut self,
        applications: Vec<ZanistarastMethodApplication>,
    ) -> Self {
        self.scientific_applications = applications;
        self
    }

    pub fn with_interpretation_risks(
        mut self,
        risks: Vec<String>,
    ) -> Self {
        self.interpretation_risks = risks;
        self
    }

    pub fn with_unresolved_questions(
        mut self,
        questions: Vec<String>,
    ) -> Self {
        self.unresolved_questions = questions;
        self
    }

    pub fn with_application_contradictions(
        mut self,
        contradictions: Vec<String>,
    ) -> Self {
        self.contradictions_in_application = contradictions;
        self
    }

    pub fn mark_original_method_preserved(mut self) -> Self {
        self.original_method_preserved = true;
        self
    }

    pub fn mark_rasterast_verified(mut self) -> Self {
        self.rasterast_verified = true;
        self
    }

    pub fn add_scientific_application(
        &mut self,
        application: ZanistarastMethodApplication,
    ) {
        self.scientific_applications.push(application);
    }

    pub fn is_identity_complete(&self) -> bool {
        !self.method_id.trim().is_empty()
            && !self.name.trim().is_empty()
            && !self.description.trim().is_empty()
    }

    pub fn preserves_revelation_distinction(&self) -> bool {
        !self.authority.is_revelation()
    }

    pub fn preserves_binding_method_authority(&self) -> bool {
        self.authority.is_binding()
    }

    pub fn has_verified_original_sources(&self) -> bool {
        !self.source_references.is_empty()
            && self
                .source_references
                .iter()
                .all(RisaleSourceReference::is_verified)
    }

    pub fn has_valid_reasoning_steps(&self) -> bool {
        if self.reasoning_steps.is_empty() {
            return false;
        }

        if !self
            .reasoning_steps
            .iter()
            .all(RisaleReasoningStep::is_complete)
        {
            return false;
        }

        let mut orders: Vec<usize> = self
            .reasoning_steps
            .iter()
            .map(|step| step.order)
            .collect();

        orders.sort_unstable();
        orders.dedup();

        orders.len() == self.reasoning_steps.len()
    }

pub fn has_original_risale_reasoning_step(&self) -> bool {
        self.reasoning_steps
            .iter()
            .any(RisaleReasoningStep::is_original_risale_step)
    }

    pub fn separates_source_method_from_zanistarast_additions(
        &self,
    ) -> bool {
        self.reasoning_steps.iter().all(|step| match step.origin {
            RisaleReasoningOrigin::OriginalRisaleMethod => {
                step.source_reference_index
                    .map(|index| index < self.source_references.len())
                    .unwrap_or(false)
            }
            RisaleReasoningOrigin::ZanistarastApplication => true,
        })
    }

    pub fn has_unresolved_application_issues(&self) -> bool {
        !self.interpretation_risks.is_empty()
            || !self.unresolved_questions.is_empty()
            || !self.contradictions_in_application.is_empty()
    }

    pub fn application_may_require_correction(&self) -> bool {
        self.has_unresolved_application_issues()
            || self
                .scientific_applications
                .iter()
                .any(|application| {
                    !application.source_method_preserved
                        || !application.rasterast_verified
                })
    }

    pub fn can_be_registered_as_verified_method(&self) -> bool {
        self.is_identity_complete()
            && self.preserves_revelation_distinction()
            && self.preserves_binding_method_authority()
            && self.has_verified_original_sources()
            && self.has_valid_reasoning_steps()
            && self.has_original_risale_reasoning_step()
            && self
                .separates_source_method_from_zanistarast_additions()
            && !self.has_unresolved_application_issues()
            && self.original_method_preserved
            && self.rasterast_verified
            && self.requires_mudebbir_decision
    }

    pub fn is_constitutionally_valid(&self) -> bool {
        self.is_identity_complete()
            && self.preserves_revelation_distinction()
            && self.preserves_binding_method_authority()
            && self.requires_mudebbir_decision
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RisaleMethodMap {
    pub map_id: String,
    pub title: String,
    pub methods: Vec<RisaleMethod>,
}

impl RisaleMethodMap {
    pub fn new(
        map_id: impl Into<String>,
        title: impl Into<String>,
    ) -> Self {
        Self {
            map_id: map_id.into(),
            title: title.into(),
            methods: Vec::new(),
        }
    }

    pub fn with_methods(
        mut self,
        methods: Vec<RisaleMethod>,
    ) -> Self {
        self.methods = methods;
        self
    }

    pub fn add_method(&mut self, method: RisaleMethod) {
        self.methods.push(method);
    }

    pub fn find_method(
        &self,
        method_id: &str,
    ) -> Option<&RisaleMethod> {
        self.methods
            .iter()
            .find(|method| method.method_id == method_id)
    }

    pub fn methods_by_kind(
        &self,
        kind: RisaleMethodKind,
    ) -> Vec<&RisaleMethod> {
        self.methods
            .iter()
            .filter(|method| method.method_kind == kind)
            .collect()
    }

    pub fn methods_for_dimension(
        &self,
        dimension: HumanDimension,
    ) -> Vec<&RisaleMethod> {
        self.methods
            .iter()
            .filter(|method| {
                method.addressed_dimensions.contains(&dimension)
            })
            .collect()
    }

    pub fn methods_by_reading_domain(
        &self,
        reading_domain: ReadingDomain,
    ) -> Vec<&RisaleMethod> {
        self.methods
            .iter()
            .filter(|method| {
                method.reading_domain == reading_domain
            })
            .collect()
    }

    pub fn verified_method_count(&self) -> usize {
        self.methods
            .iter()
            .filter(|method| {
                method.can_be_registered_as_verified_method()
            })
            .count()
    }

    pub fn methods_requiring_correction(
        &self,
    ) -> Vec<&RisaleMethod> {
        self.methods
            .iter()
            .filter(|method| {
                method.application_may_require_correction()
            })
            .collect()
    }

    pub fn invalid_methods(&self) -> Vec<&RisaleMethod> {
        self.methods
            .iter()
            .filter(|method| {
                !method.is_constitutionally_valid()
            })
            .collect()
    }

    pub fn is_complete(&self) -> bool {
        !self.map_id.trim().is_empty()
            && !self.title.trim().is_empty()
            && !self.methods.is_empty()
            && self.invalid_methods().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn source_reference() -> RisaleSourceReference {
        RisaleSourceReference::new(
            "Sözler",
            "Örnek bölüm",
            "Örnek konum",
        )
        .with_original_text(
            "Risale-i Nur'un doğrulanmış orijinal metni.",
        )
        .with_surrounding_context(
            "Metnin önceki ve sonraki bağlamı.",
        )
        .with_edition_information(
            "Doğrulanmış orijinal metin bilgisi.",
        )
        .mark_original_text_verified()
    }

    fn source_reasoning_step() -> RisaleReasoningStep {
        RisaleReasoningStep::new(
            1,
            "Eserde görülen düzen ve ölçü tespit edilir.",
            "Bu adım Risale-i Nur'daki kâinat okuma yöntemine dayanır.",
            RisaleReasoningOrigin::OriginalRisaleMethod,
        )
        .with_source_reference_index(0)
    }

    fn zanistarast_reasoning_step() -> RisaleReasoningStep {
        RisaleReasoningStep::new(
            2,
            "Yöntem çağdaş bir bilimsel modele uygulanır.",
            "Bu adım Zanistarast'ın yöntem uygulamasıdır.",
            RisaleReasoningOrigin::ZanistarastApplication,
        )
    }

    fn proven_truth() -> RisaleProvenTruth {
        RisaleProvenTruth::new(
            "Tevhid",
            "Varlıklardaki düzen ve birlik üzerinden tevhid hakikatinin okunması.",
        )
        .with_related_quran_topics(vec![
            "Tevhid".to_string(),
            "Yaratılış".to_string(),
        ])
        .with_related_human_dimensions(vec![
            HumanDimension::Faith,
            HumanDimension::Intellect,
            HumanDimension::Heart,
        ])
        .with_related_creation_signs(vec![
            "Düzen".to_string(),
            "Ölçü".to_string(),
            "Birlik".to_string(),
        ])
    }

    fn complete_method() -> RisaleMethod {
        RisaleMethod::new(
            "risale-method-001",
            "Kâinat kitabını düzen ve hikmet üzerinden okuma",
            "Varlıklardaki düzen, ölçü, hikmet ve birlikten hareket eden kurucu ispat yöntemi.",
            RisaleMethodAuthority::BindingCreationReadingMethod,
            RisaleMethodKind::CreationBookReading,
            ReadingDomain::Integrated,
            RisaleProofType::Integrated,
        )
        .with_review_status(
            RisaleMethodReviewStatus::RequiresRasterastReview,
        )
        .with_addressed_dimensions(vec![
            HumanDimension::Faith,
            HumanDimension::Intellect,
            HumanDimension::Reason,
            HumanDimension::Logic,
            HumanDimension::Heart,
        ])
        .with_source_references(vec![
            source_reference(),
        ])
        .with_reasoning_steps(vec![
            source_reasoning_step(),
            zanistarast_reasoning_step(),
        ])
        .with_proven_truths(vec![
            proven_truth(),
        ])
        .with_related_quran_topics(vec![
            "Tevhid".to_string(),
            "Hikmet".to_string(),
            "Yaratılış".to_string(),
        ])
        .with_creation_book_correspondences(vec![
            "Varlıklar arasındaki ölçülü ilişki.".to_string(),
        ])
        .with_fitrah_correspondences(vec![
            "İnsanın anlam ve birlik arayışı.".to_string(),
        ])
        .with_moral_correspondences(vec![
            "Varlığı emanet ve sorumluluk içinde okuma."
                .to_string(),
        ])
        .with_zanistarast_interpretation(
            "Bu yöntem fizik, hayat ve insan alanlarına uygulanacaktır.",
        )
        .mark_original_method_preserved()
        .mark_rasterast_verified()
    }

    #[test]
    fn risale_method_is_binding_but_not_revelation() {
        let method = complete_method();

        assert!(method.preserves_binding_method_authority());
        assert!(method.preserves_revelation_distinction());
        assert!(!method.authority.is_revelation());
    }

    #[test]
    fn verified_method_requires_original_source() {
        let method = RisaleMethod::new(
            "risale-method-002",
            "Temsil yöntemi",
            "Hakikati temsil yoluyla açıklayan yöntem.",
            RisaleMethodAuthority::QuranicProofMethod,
            RisaleMethodKind::Representation,
            ReadingDomain::FaithTruths,
            RisaleProofType::Representative,
        )
        .with_reasoning_steps(vec![
            source_reasoning_step(),
        ])
        .mark_original_method_preserved()
        .mark_rasterast_verified();

        assert!(!method.has_verified_original_sources());
        assert!(!method.can_be_registered_as_verified_method());
    }

    #[test]
    fn source_and_zanistarast_steps_are_separated() {
        let method = complete_method();

        assert!(
            method
                .separates_source_method_from_zanistarast_additions()
        );
    }

    #[test]
    fn source_step_requires_valid_reference_index() {
        let mut method = complete_method();

        method.reasoning_steps[0].source_reference_index =
            Some(99);

        assert!(
            !method
                .separates_source_method_from_zanistarast_additions()
        );
    }

    #[test]
    fn complete_method_can_be_registered_after_rasterast() {
        let method = complete_method();

        assert!(method.is_constitutionally_valid());
        assert!(
            method.can_be_registered_as_verified_method()
        );
        assert!(method.requires_mudebbir_decision);
    }

    #[test]
    fn unresolved_issue_blocks_registration() {
        let method = complete_method()
            .with_interpretation_risks(vec![
                "Bilimsel aktarım yeniden incelenmelidir."
                    .to_string(),
            ]);

        assert!(method.application_may_require_correction());
        assert!(
            !method.can_be_registered_as_verified_method()
        );
    }

    #[test]
    fn scientific_application_remains_reviewable() {
        let application =
            ZanistarastMethodApplication::new(
                "application-001",
                "Biyoloji",
                "Canlılığı yalnızca maddi süreçlere indirgemeyen model.",
            )
            .with_applied_method_description(
                "Kâinat kitabı ve hikmet okuma yöntemi biyolojiye uygulanır.",
            )
            .mark_source_method_preserved();

        assert!(application.is_complete());
        assert!(!application.can_be_approved());
    }

    #[test]
    fn scientific_application_requires_rasterast() {
        let application =
            ZanistarastMethodApplication::new(
                "application-002",
                "İnsan bilimi",
                "Öz, ruh, duygu, akıl ve beden bütünlüğü modeli.",
            )
            .with_applied_method_description(
                "İnsan, fıtrat ve vicdan okuma yöntemleri uygulanır.",
            )
            .mark_source_method_preserved()
            .mark_rasterast_verified();

        assert!(application.can_be_approved());
        assert!(application.requires_mudebbir_decision);
    }

    #[test]
    fn method_map_finds_method_by_dimension() {
        let map = RisaleMethodMap::new(
            "risale-map-001",
            "Risale-i Nur Kurucu Yöntem Haritası",
        )
        .with_methods(vec![
            complete_method(),
        ]);

        let methods =
            map.methods_for_dimension(HumanDimension::Heart);

        assert_eq!(methods.len(), 1);
        assert!(map.is_complete());
        assert_eq!(map.verified_method_count(), 1);
    }

    #[test]
    fn application_problem_does_not_remove_method_authority() {
        let method = complete_method()
            .with_application_contradictions(vec![
                "Zanistarast uygulamasında yöntem dışı çıkarım."
                    .to_string(),
            ]);

        assert!(method.preserves_binding_method_authority());
        assert!(method.application_may_require_correction());
        assert!(
            !method.can_be_registered_as_verified_method()
        );
    }
}





   
   



    
