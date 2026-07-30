/// Risale-i Nur'un Zanistarast içindeki kurucu yöntem
/// konumunu temsil eden veri modelidir.
///
/// Anayasal ayrım:
///
/// - Kur'an-ı Kerim mutlak vahyî hakikatin kaynağıdır.
/// - Üstad Bediüzzaman Said-i Kürdî'nin Risale-i Nur'da
/// kullandığı aklî, mantıkî, temsilî ve ispatlayıcı
/// yöntemler Zanistarast için bağlayıcı ve mutlak doğru
/// yöntemlerdir.
/// - Zanistarast'ın bu yöntemleri anlama, sınıflandırma ve
/// uygulama biçimi insanîdir; bu nedenle Rasterast
/// denetimine açıktır.
/// - Denetlenen Risale-i Nur yöntemi değil, Zanistarast'ın
/// yöntemi doğru anlayıp uygulayıp uygulamadığıdır.

/// Risale-i Nur'daki kurucu yöntemlerin Zanistarast
/// açısından sahip olduğu bağlayıcılık alanıdır.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RisaleMethodAuthority {
    /// Kur'an hakikatlerini açıklama ve ispat etme yöntemi.
    QuranicProofMethod,

    /// Akıl yürütmede bağlayıcı yöntem.
    BindingReasoningMethod,

    /// Mantıksal çıkarımda bağlayıcı yöntem.
    BindingLogicalMethod,

    /// İman ile aklı birlikte çalıştıran bağlayıcı yöntem.
    BindingFaithReasonMethod,

    /// İnsanı öz, ruh, kalp, duygu, akıl, ahlak ve irade
    /// bütünlüğü içinde okuyan bağlayıcı yöntem.
    BindingHumanUnderstandingMethod,

    /// Kâinatı mana, düzen, hikmet, maksat ve esmâ
    /// üzerinden okuyan bağlayıcı yöntem.
    BindingCreationReadingMethod,

    /// Ahlak, vicdan ve fıtrat değerlendirmesinde
    /// bağlayıcı yöntem.
    BindingMoralFitrahMethod,
}

impl RisaleMethodAuthority {
    /// Risale-i Nur yöntemlerinin Zanistarast içinde
    /// isteğe bağlı bir araştırma tercihi olmadığını
    /// gösterir.
    pub fn is_binding(self) -> bool {
        true
    }

    /// Bu otoritenin Kur'an'ın vahiy statüsüyle aynı
    /// olmadığını açık biçimde korur.
    pub fn is_revelation(self) -> bool {
        false
    }
}

/// Risale-i Nur'da kullanılan temel düşünme, açıklama ve
/// ispat yöntemlerinin sınıflandırılmasıdır.
///
/// Bu sınıflandırma Risale-i Nur'un yerine geçmez.
/// Yalnızca Zanistarast'ın yöntem haritasını oluşturur.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RisaleMethodKind {
    /// Temsil yoluyla soyut veya derin bir hakikati
    /// anlaşılır hâle getirme.
    Representation,

    /// İki durum veya varlık düzeni arasında karşılaştırma.
    Comparison,

    /// Benzerliklerden hareketle açıklama ve çıkarım.
    Analogy,

    /// Verilen öncüllerden zorunlu sonuca ulaşma.
    NecessaryReasoning,

    /// Bir iddianın karşıtının doğurduğu çelişki veya
    /// imkânsızlığı gösterme.
    ContradictionAnalysis,

    /// Çok sayıdaki işaretin ortak sonucunu birlikte okuma.
    CollectiveEvidence,

    /// Bir tek eserden failine, sanatkârına veya kaynağına
    /// ulaşan çıkarım.
    FromWorkToMaker,

    /// Fiilden isme, isimden sıfata ve sıfattan zata doğru
    /// ilerleyen okuma.
    FromActToNameAndAttribute,

    /// Kâinattaki düzen, ölçü, uyum ve bağlantıları okuma.
    OrderReading,

    /// Varlıklardaki fayda, amaç, uygunluk ve hikmeti okuma.
    WisdomReading,

    /// Rahmet, inayet, rızık, koruma ve cevap verme
    /// ilişkilerini okuma.
    MercyAndProvidenceReading,

    /// Birlik içindeki çokluk ve çokluk içindeki birliği
    /// değerlendirme.
    UnityFromMultiplicity,

    /// Varlıklarda görünen isim ve sıfat tecellilerini okuma.
    DivineNamesReading,

    /// Kâinatı anlam taşıyan bir kitap olarak okuma.
    CreationBookReading,

    /// İnsanı kendisini ve varlığı okuyabilen merkezî bir
    /// muhatap olarak değerlendirme.
    HumanReading,

    /// İnsanın yaratılış yapısı, ihtiyaçları ve yönelimleri
    /// üzerinden delil kurma.
    FitrahReading,

    /// Vicdanın ihtiyaç, yönelim, sorumluluk ve şahitliğini
    /// değerlendirme.
    ConscienceReading,

    /// Kalbin iman, anlam, muhabbet, korku, ümit ve bağlılık
    /// yönlerini birlikte okuma.
    HeartReading,

    /// İnsanın aczini hakikate ulaşmada bir delil ve
    /// yöneliş kapısı olarak değerlendirme.
    ImpotenceMethod,

    /// İnsanın fakrını, ihtiyaçlarını ve bağımlılığını
    /// hakikate ulaşmada bir delil olarak değerlendirme.
    PovertyMethod,

    /// Şefkat üzerinden varlık, sorumluluk ve ahlak okuması.
    CompassionMethod,

    /// Tefekkür üzerinden insan, kâinat ve Kur'an
    /// ilişkisini kurma.
    ReflectionMethod,

    /// İman ile aklı çatıştırmadan birlikte çalıştırma.
    FaithReasonIntegration,

    /// Akıl ile kalbi aynı hakikat üzerinde birleştirme.
    HeartReasonIntegration,

    /// Duygu ile akıl arasındaki ilişkiyi hakikat ve ahlak
    /// ekseninde düzenleme.
    EmotionReasonIntegration,

    /// Ruh ile bedenin birbirinden koparılmadan okunması.
    SoulBodyIntegration,

    /// İnsan davranışından niyet, sorumluluk ve ahlak
    /// sonucuna ulaşma.
    MoralReasoning,

    /// Ölüm, fanilik ve geçicilik üzerinden baki anlamı
    /// araştırma.
    MortalityAndPermanenceReading,

    /// Haşir, adalet, hikmet, rahmet ve insan arzuları
    /// arasında bütünlüklü delil kurma.
    ResurrectionReasoning,

    /// Nübüvveti insanlık, vahiy, ahlak ve kâinat düzeni
    /// içinde değerlendirme.
    ProphethoodReasoning,

    /// Tevhidi bütün varlık alanlarını birleştiren ana
    /// açıklama ilkesi olarak okuma.
    UnityReasoning,

    /// Bir yöntemin birkaç akıl, kalp, fıtrat, kâinat ve
    /// vahiy yolunu birlikte kullanması.
    IntegratedProof,
}

/// Risale-i Nur yönteminin hitap ettiği veya değerlendirdiği
/// insan boyutlarıdır.
///
/// İnsan yalnızca biyolojik beden veya yalnızca akıl olarak
/// ele alınmaz.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HumanDimension {
    /// İnsanın varlıksal özü.
    Essence,

    /// Ruh.
    Spirit,

    /// Kalp.
    Heart,

    /// Vicdan.
    Conscience,

    /// Duyguların genel alanı.
    Emotion,

    /// Akıl ve kavrama gücü.
    Intellect,

    /// Muhakeme ve sonuç çıkarma gücü.
    Reason,

    /// Geçerli çıkarım ve çelişmezlik alanı.
    Logic,

    /// Ahlak ve değer alanı.
    Morality,

    /// Seçme ve yönelme gücü.
    Will,

    /// Niyet.
    Intention,

    /// İman.
    Faith,

    /// Muhabbet.
    Love,

    /// Korku.
    Fear,

    /// Ümit.
    Hope,

    /// Hayal.
    Imagination,

    /// Hafıza.
    Memory,

    /// Benlik ve ene.
    Selfhood,

    /// Beden.
    Body,

    /// Fiil ve davranış.
    Action,

    /// Sorumluluk.
    Responsibility,
}

/// Risale-i Nur yönteminin uygulandığı temel okuma alanıdır.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReadingDomain {
    /// Kur'an ayetlerinin okunması.
    Quran,

    /// Kâinat kitabının okunması.
    Creation,

    /// İnsan varlığının okunması.
    Human,

    /// Fıtratın okunması.
    Fitrah,

    /// Ruhun okunması.
    Spirit,

    /// Kalp ve vicdanın okunması.
    HeartAndConscience,

    /// Ahlakın okunması.
    Morality,

    /// Toplum ve medeniyetin okunması.
    Society,

    /// Tarihin okunması.
    History,

    /// Bilimsel bilgi ve modellerin okunması.
    Science,

    /// Hayat ve canlılığın okunması.
    Life,

    /// Ölüm, ahiret ve beka ilişkisinin okunması.
    Afterlife,

    /// İman hakikatlerinin bütüncül okunması.
    FaithTruths,

    /// Birden çok alanın birlikte okunması.
    Integrated,
}

/// Bir Risale-i Nur yönteminin kurduğu başlıca delil veya
/// ispat biçimidir.
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

/// Zanistarast'ın bir Risale-i Nur yöntemini değerlendirme
/// sürecindeki durumudur.
///
/// Bu durum yöntemin doğruluğunu değil, Zanistarast'ın
/// yöntemi anlama ve uygulama durumunu gösterir.
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

/// Risale-i Nur'un orijinal metnindeki kaynak kaydıdır.
///
/// Orijinal metin ile Zanistarast açıklaması ayrı tutulur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RisaleSourceReference {
    pub work_name: String,
    pub section_name: String,
    pub page_or_location: String,
    pub original_text: String,
    pub surrounding_context: String,
    pub edition_information: String,
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
        }
    }

    pub fn with_original_text(
        mut selfening,
    EmotionalBalance,
    FearHopeBalance,
    ResponsibilityAwareness,
    WorshipConsciousness,
    MeaningIntegration,
    IllusionRemoval,
    ContradictionRemoval,
}

/// Risale-i Nur'un orijinal metnindeki bir yöntem
/// dayanağını temsil eder.
///
/// Orijinal metin, Zanistarast açıklamasından ve bilimsel
/// uzantıdan ayrı tutulur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RisaleMethodReference {
    pub work_name: String,
    pub section_reference: String,
    pub original_text: String,
    pub method_observation: String,
    pub zanistarast_analysis: String,
}

impl RisaleMethodReference {
    pub fn new(
        work_name: impl Into<String>,
        section_reference: impl Into<String>,
    ) -> Self {
        Self {
            work_name: work_name.into(),
            section_reference: section_reference.into(),
            original_text: String::new(),
            method_observation: String::new(),
            zanistarast_analysis: String::new(),
        }
    }

    pub fn with_original_text(
        mut self,
        original_text: impl Into<String>,
    ) -> Self {
        self.original_text = original_text.into();
        self
    }

    pub fn with_method_observation(
        mut self,
        method_observation: impl Into<String>,
    ) -> Self {
        self.method_observation = method_observation.into();
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

    pub fn is_complete(&self) -> bool {
        !self.work_name.trim().is_empty()
            && !self.section_reference.trim().is_empty()
            && !self.original_text.trim().is_empty()
            && !self.method_observation.trim().is_empty()
    }

    /// Orijinal Risale metni ile Zanistarast yorumunun aynı
    /// içerik gibi kayaleProvenTruth {
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

/// Risale-i Nur yönteminin Zanistarast bilim paradigmasına
/// taşınan uygulamasıdır.
///
/// Bu kayıt yöntemin kendisi değildir. Zanistarast'ın
/// yöntemden hareketle yaptığı insanî uygulamadır ve
/// Rasterast denetimine açıktır.
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

/// Risale-i Nur'daki tek bir kurucu yöntemin Zanistarast
/// kayıt modelidir.
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

    /// Risale-i Nur yönteminin Kur'an'ın vahiy statüsüyle
    /// karıştırılmadığını denetler.
    pub fn preserves_revelation_distinction(&self) -> bool {
        !self.authority.is_revelation()
    }

    /// Risale-i Nur yöntemının Risale-i Nur'un
    /// orijinal metni gibi sunulup sunulmadığını denetler.
    pub fn separates_risale_from_zanistarast(&self) -> bool {
        self.source_references.iter().all(
            RisaleMethodReference::
                separates_original_from_zanistarast,
        )
    }

    /// Bilimsel uzantıların Risale-i Nur'un doğrudan hükmü
    /// olarak sunulmaması için kaynak temelinin varlığını
    /// denetler.
    pub fn scientific_extension_has_method_basis(&self) -> bool {
        self.scientific_extensions.is_empty()
            || self.has_source_basis()
    }

    /// Yöntemin insanı yalnız akıl veya beden boyutuna
    /// indirgemediğini denetler.
    pub fn has_integrated_human_scope(&self) -> bool {
        if self.reads_domain(ReadingDomain::Human) {
            let has_non_physical_dimension =
                self.addressed_dimensions.iter().any(|dimension| {
                    !matches!(dimension, HumanDimension::Body)
                });

            let has_inner_dimension =
                self.addressed_dimensions.iter().any(|dimension| {
                    matches!(
                        dimension,
                        HumanDimension::Essence
                            | HumanDimension::Spirit
                            | HumanDimension::Heart
                            | HumanDimension::Conscience
                            | HumanDimension::Emotion
                            | HumanDimension::Faith
                    )
                });

            has_non_physical_dimension && has_inner_dimension
        } else {
            true
        }
    }

    pub fn invalid_applications(
        &self,
    ) -> Vec<&RisaleMethodApplication> {
        self.applications
            .iter()
            .filter(|application| !applicationemin doğru
    /// sınıflandırıldığına ve doğru aktarıldığına karar
    /// verilmesi için kullanılan koşuldur.
    pub fn can_be_registered_as_verified_method(&self) -> bool {
        self.is_identity_complete()
            && self.preserves_revelation_distinction()
            && self.preserves_binding_method_authority()
            && self.has_verified_original_sources()
            && self.has_valid_reasoning_steps()
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
            && self
                .separates_source_method_from_zanistarast_additions()
            && self.requires_mudebbir_decision
    }
}

/// Birden fazla Risale-i Nur yöntemini birlikte tutan
/// yöntem haritasıdır.
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

    pub fn with_methods(mut self, methods: Vec<RisaleMethod>) -> Self {
        self.methods = methods;
        self
    }

    pub fn add_method(&mut self, method: RisaleMethod) {
        self.methods.push(method);
    }

    pub fn find_method(&self, method_id: &str) -> Option<&RisaleMethod> {
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
            .filter(|method| !method.is_constitutionally_valid())
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
    }

    fn source_reasoning_step() -> RisaleReasoningStep {
        RisaleReasoningStep::new(
            1,
            "Eserde görülen düzen ve ölçü tespit edilir.",
            "Bu adım Risale-i Nur'daki kâinat okuma yöntemine dayanır.",
        )
    }

    fn zanistarast_reasoning_step() -> RisaleReasoningStep {
        RisaleReasoningStep::new(
            2,
            "Yöntem çağdaş bir bilimsel modele uygulanır.",
            "Bu adım Zanistarast'ın yöntem uygulamasıdır.",
        )
        .mark_as_zanistarast_addition()
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
        .with_source_references(vec![source_reference()])
        .with_reasoning_steps(vec![
            source_reasoning_step(),
            zanistarast_reasoning_step(),
        ])
        .with_proven_truths(vec![proven_truth()])
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
            "Bu yöntem Zanistarast bilim paradigmasında fizik, hayat ve insan alanlarına uygulanacaktır.",
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
        .with_reasoning_steps(vec![source_reasoning_step()])
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
    fn reasoning_step_cannot_have_two_origins() {
        let mut step = source_reasoning_step();
        step.zanistarast_added = true;

        assert!(!step.has_valid_origin_marking());
    }

    #[test]
    fn complete_method_can_be_registered_after_rasterast() {
        let method = complete_method();

        assert!(method.is_constitutionally_valid());
        assert!(method.can_be_registered_as_verified_method());
        assert!(method.requires_mudebbir_decision);
    }

    #[test]
    fn unresolved_application_issue_blocks_registration() {
        let method = complete_method()
            .with_interpretation_risks(vec![
                "Yöntemin bilimsel alana aktarımı yeniden incelenmelidir."
                    .to_string(),
            ]);

        assert!(method.application_may_require_correction());
        assert!(!method.can_be_registered_as_verified_method());
    }

    #[test]
    fn scientific_application_remains_reviewable() {
        let application = ZanistarastMethodApplication::new(
            "application-001",
            "Biyoloji",
            "Canlılığı yalnızca maddi süreçlere indirgemeyen bütüncül model.",
        )
        .with_applied_method_description(
            "Kâinat kitabı, hikmet ve insan okuma yöntemleri biyolojiye uygulanır.",
        )
        .mark_source_method_preserved();

        assert!(application.is_complete());
        assert!(!application.can_be_approved());
    }

    #[test]
    fn scientific_application_requires_rasterast_and_mudebbir_gate() {
        let application = ZanistarastMethodApplication::new(
            "application-002",
            "İnsan bilimi",
            "Öz, ruh, duygu, akıl, ahlak ve beden bütünlüğü modeli.",
        )
        .with_applied_method_description(
            "İnsan okuma, fıtrat, vicdan ve iman-akıl bütünlüğü yöntemleri uygulanır.",
        )
        .mark_source_method_preserved()
        .mark_rasterast_verified();

        assert!(application.can_be_approved());
        assert!(application.requires_mudebbir_decision);
    }

    #[test]
    fn method_map_finds_methods_by_human_dimension() {
        let map = RisaleMethodMap::new(
            "risale-map-001",
            "Risale-i Nur Kurucu Yöntem Haritası",
        )
        .with_methods(vec![complete_method()]);

        let methods =
            map.methods_for_dimension(HumanDimension::Heart);

        assert_eq!(methods.len(), 1);
        assert!(map.is_complete());
        assert_eq!(map.verified_method_count(), 1);
    }

    #[test]
    fn rasterast_reviews_application_not_method_truth() {
        let method = complete_method()
            .with_application_contradictions(vec![
                "Zanistarast uygulamasında yöntem dışı bir çıkarım tespit edildi."
                    .to_string(),
            ]);

        assert!(method.preserves_binding_method_authority());
        assert!(method.application_may_require_correction());
        assert!(!method.can_be_registered_as_verified_method());
    }
}



    
