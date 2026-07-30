/// Zanistarast bilimsel sentezinde kullanılan temel kaynak
/// ve delil alanlarıdır.
///
/// Bu sıralama, bütün kaynakların aynı ontolojik veya
/// epistemik statüde olduğu anlamına gelmez.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TruthFoundationKind {
    /// Zanistarast'ın en üst ve tartışılmaz vahyî hakikat
    /// kaynağıdır.
    Quran,

    /// Sıhhati doğrulanmış nebevî aktarımlardır.
    AuthenticHadith,

    /// Kur'an'ın okunması, iman hakikatlerinin açıklanması
    /// ve ispat yolları bakımından başlıca yorum ve yöntem
    /// referansıdır.
    RisaleNurOriginalText,

    /// Yaratılmış varlıkların, düzenlerin, yasaların ve
    /// ilişkilerin gözlem ve araştırma alanıdır.
    CreationBook,

    /// Yaratılışa yerleştirilmiş yapı, yönelim ve uygunluk
    /// delillerinin araştırma alanıdır.
    FitrahEvidence,

    /// Kavramları ayırma, ilişkileri kurma, hüküm ve sonuç
    /// çıkarma aracıdır.
    Reason,

    /// Çelişmezlik, geçerli çıkarım ve kavramsal tutarlılık
    /// denetimidir.
    Logic,

    /// Varlık ve olayların doğrudan veya araçlı izlenmesidir.
    Observation,

    /// Kontrollü şartlarda sınama ve tekrar araştırmasıdır.
    Experiment,

    /// Nicel veya nitel özelliklerin belirli yöntemlerle
    /// kaydedilmesidir.
    Measurement,

    /// İlişkilerin sembolik ve biçimsel olarak modellenmesidir.
    Mathematics,

    /// Kaynakları, yöntemleri ve sonuçları açık bilimsel
    /// araştırmalardır.
    ScientificResearch,

    /// Zanistarast tarafından oluşturulan yorum, hipotez,
    /// açıklama veya sentezdir.
    ZanistarastInterpretation,
}

/// Bir kaynağın Zanistarast içindeki temel otorite ve
/// hakikat statüsüdür.
///
/// Bu statü, insanın kaynağı yorumlama başarısıyla aynı şey
/// değildir. Kaynak kesin olabilirken insan yorumu hatalı
/// veya eksik olabilir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationAuthority {
    /// Kur'an-ı Kerim'in Zanistarast içindeki tartışılmaz
    /// vahyî hakikat statüsüdür.
    AbsoluteRevelatoryTruth,

    /// Sıhhati doğrulanmış nebevî bildirimin otoritesidir.
    AuthenticPropheticAuthority,

    /// Kur'an değildir ve vahiy seviyesinde değildir; fakat
    /// Zanistarast için başlıca yorum ve ispat yöntemi
    /// referansıdır.
    PrimaryInterpretiveReference,

    /// Kâinat ve fıtratta gözlenen yaratılmış düzenin delil
    /// değeridir.
    CreatedOrderEvidence,

    /// Akıl, mantık, deney, gözlem, ölçüm ve matematik gibi
    /// insanın hakikati araştırma yöntemleridir.
    InvestigativeMethod,

    /// Zanistarast tarafından üretilen ve Rasterast
    /// denetimine açık insan yorumudur.
    HumanInterpretation,
}

/// Bir temel kaynağın belirli bir çalışmada nasıl
/// kullanıldığını gösterir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationUse {
    /// Doğrudan hüküm veya bildirim kaynağıdır.
    DirectStatement,

    /// Bir kavramı açıklamak veya yorumlamak için kullanılır.
    Interpretation,

    /// Akli veya mantıki ispat yolu sağlar.
    ProofMethod,

    /// Gözlemsel veya deneysel delil sağlar.
    EmpiricalEvidence,

    /// Matematiksel veya biçimsel model sağlar.
    FormalModel,

    /// Kaynaklar arasında karşılaştırma yapılmasını sağlar.
    ComparativeAnalysis,

    /// Yeni bir Zanistarast sentezinin kurulmasına katkı verir.
    Synthesis,
}

/// Bir kaynağın insan tarafından kullanımında uygulanacak
/// temel doğrulama durumudur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FoundationReviewStatus {
    NotReviewed,
    UnderReview,
    Reviewed,
    RequiresReinterpretation,
    Conflicted,
    RejectedUse,
}

/// Bir hakikat temelinin tekil kaydıdır.
///
/// `authority`, kaynağın Zanistarast içindeki statüsünü;
/// `review_status` ise insanın o kaynağı belirli çalışmada
/// kullanma ve yorumlama durumunu gösterir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TruthFoundation {
    pub foundation_id: String,
    pub kind: TruthFoundationKind,
    pub authority: FoundationAuthority,
    pub use_type: FoundationUse,
    pub reference: String,
    pub original_text: String,
    pub human_interpretation: String,
    pub review_status: FoundationReviewStatus,
    pub limitations: Vec<String>,
    pub open_questions: Vec<String>,
    pub requires_rasterast_review: bool,
    pub requires_mudebbir_decision: bool,
}

impl TruthFoundation {
    pub fn new(
        foundation_id: impl Into<String>,
        kind: TruthFoundationKind,
        authority: FoundationAuthority,
        use_type: FoundationUse,
        reference: impl Into<String>,
    ) -> Self {
        Self {
            foundation_id: foundation_id.into(),
            kind,
            authority,
            use_type,
            reference: reference.into(),
            original_text: String::new(),
            human_interpretation: String::new(),
            review_status: FoundationReviewStatus::NotReviewed,
            limitations: Vec::new(),
            open_questions: Vec::new(),
            requires_rasterast_review: true,
            requires_mudebbir_decision: true,
        }
    }

    pub fn with_original_text(
        mut self,
        original_text: impl Into<String>,
    ) -> Self {
        self.original_text = original_text.into();
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

    pub fn with_review_status(
        mut self,
        review_status: FoundationReviewStatus,
    ) -> Self {
        self.review_status = review_status;
        self
    }

    pub fn with_limitations(
        mut self,
        limitations: Vec<String>,
    ) -> Self {
        self.limitations = limitations;
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
        !self.foundation_id.trim().is_empty()
            && !self.reference.trim().is_empty()
    }

    /// Kur'an-ı Kerim kaydının doğru otorite statüsüyle
    /// oluşturulup oluşturulmadığını denetler.
    pub fn has_valid_quranic_authority(&self) -> bool {
        self.kind != TruthFoundationKind::Quran
            || self.authority
                == FoundationAuthority::AbsoluteRevelatoryTruth
    }

    /// Risale-i Nur'un Kur'an veya vahiy seviyesine
    /// çıkarılmasını engeller.
    pub fn has_valid_risale_authority(&self) -> bool {
        self.kind
            != TruthFoundationKind::RisaleNurOriginalText
            || self.authority
                == FoundationAuthority::
                    PrimaryInterpretiveReference
    }

    /// Zanistarast yorumunun mutlak veya nebevî otorite
    /// olarak kaydedilmesini engeller.
    pub fn has_valid_zanistarast_authority(&self) -> bool {
        self.kind
            != TruthFoundationKind::ZanistarastInterpretation
            || self.authority
                == FoundationAuthority::HumanInterpretation
    }

    /// Kaynağın kendi metni ile insan yorumunun birbirine
    /// karıştırılmasını engeller.
    pub fn separates_source_from_interpretation(&self) -> bool {
        self.human_interpretation.trim().is_empty()
            || self.original_text.trim().is_empty()
            || self.original_text.trim()
                != self.human_interpretation.trim()
    }

    /// Kur'anî hakikatin deneysel ispat bulunmadığı için
    /// düşük statüye indirilemeyeceğini belirtir.
    pub fn cannot_be_downgraded_by_empirical_absence(
        &self,
    ) -> bool {
        self.kind == TruthFoundationKind::Quran
            && self.authority
                == FoundationAuthority::AbsoluteRevelatoryTruth
    }

    /// Kaynak hakikati kesin olsa bile insan yorumunun
    /// Rasterast denetiminden geçmesi gerekir.
    pub fn interpretation_requires_review(&self) -> bool {
        !self.human_interpretation.trim().is_empty()
            && self.requires_rasterast_review
    }

    /// Kaynak kullanımının anayasal temel şartlara uyup
    /// uymadığını gösterir.
    pub fn is_constitutionally_valid(&self) -> bool {
        self.is_complete()
            && self.has_valid_quranic_authority()
            && self.has_valid_risale_authority()
            && self.has_valid_zanistarast_authority()
            && self.separates_source_from_interpretation()
    }
}

/// Zanistarast'ın bir çalışmada kullandığı birden fazla
/// hakikat temelini birlikte tutar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TruthFoundationSet {
    pub subject: String,
    pub foundations: Vec<TruthFoundation>,
}

impl TruthFoundationSet {
    pub fn new(
        subject: impl Into<String>,
    ) -> Self {
        Self {
            subject: subject.into(),
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

    pub fn has_quranic_foundation(&self) -> bool {
        self.foundations.iter().any(|foundation| {
            foundation.kind == TruthFoundationKind::Quran
                && foundation.authority
                    == FoundationAuthority::
                        AbsoluteRevelatoryTruth
        })
    }

    pub fn has_risale_reference(&self) -> bool {
        self.foundations.iter().any(|foundation| {
            foundation.kind
                == TruthFoundationKind::
                    RisaleNurOriginalText
                && foundation.authority
                    == FoundationAuthority::
                        PrimaryInterpretiveReference
        })
    }

    pub fn has_creation_book_evidence(&self) -> bool {
        self.foundations.iter().any(|foundation| {
            foundation.kind
                == TruthFoundationKind::CreationBook
        })
    }

    pub fn has_fitrah_evidence(&self) -> bool {
        self.foundations.iter().any(|foundation| {
            foundation.kind
                == TruthFoundationKind::FitrahEvidence
        })
    }

    pub fn has_rational_method(&self) -> bool {
        self.foundations.iter().any(|foundation| {
            matches!(
                foundation.kind,
                TruthFoundationKind::Reason
                    | TruthFoundationKind::Logic
                    | TruthFoundationKind::Mathematics
            )
        })
    }

    pub fn has_empirical_method(&self) -> bool {
        self.foundations.iter().any(|foundation| {
            matches!(
                foundation.kind,
                TruthFoundationKind::Observation
                    | TruthFoundationKind::Experiment
                    | TruthFoundationKind::Measurement
                    | TruthFoundationKind::ScientificResearch
            )
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

    pub fn is_complete(&self) -> bool {
        !self.subject.trim().is_empty()
            && !self.foundations.is_empty()
            && self.invalid_foundations().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn quran_foundation() -> TruthFoundation {
        TruthFoundation::new(
            "foundation-quran-001",
            TruthFoundationKind::Quran,
            FoundationAuthority::AbsoluteRevelatoryTruth,
            FoundationUse::DirectStatement,
            "Kur'an-ı Kerim ayet kaydı",
        )
        .with_original_text(
            "Doğrulanmış ayet metni burada korunur.",
        )
        .with_human_interpretation(
            "Ayetin Zanistarast açısından değerlendirmesi.",
        )
    }

    #[test]
    fn quran_has_absolute_revelatory_authority() {
        let foundation = quran_foundation();

        assert!(foundation.has_valid_quranic_authority());
        assert!(
            foundation
                .cannot_be_downgraded_by_empirical_absence()
        );
        assert!(foundation.is_constitutionally_valid());
    }

    #[test]
    fn quran_cannot_be_registered_as_human_interpretation() {
        let foundation = TruthFoundation::new(
            "foundation-quran-002",
            TruthFoundationKind::Quran,
            FoundationAuthority::HumanInterpretation,
            FoundationUse::DirectStatement,
            "Kur'an-ı Kerim ayet kaydı",
        );

        assert!(!foundation.has_valid_quranic_authority());
        assert!(!foundation.is_constitutionally_valid());
    }

    #[test]
    fn risale_is_primary_interpretive_reference() {
        let foundation = TruthFoundation::new(
            "foundation-risale-001",
            TruthFoundationKind::RisaleNurOriginalText,
            FoundationAuthority::
                PrimaryInterpretiveReference,
            FoundationUse::ProofMethod,
            "Risale-i Nur orijinal metin kaydı",
        )
        .with_original_text(
            "Orijinal metin bölümü.",
        )
        .with_human_interpretation(
            "Metindeki ispat yolunun Zanistarast analizi.",
        );

        assert!(foundation.has_valid_risale_authority());
        assert!(foundation.is_constitutionally_valid());
    }

    #[test]
    fn risale_cannot_be_assigned_quranic_authority() {
        let foundation = TruthFoundation::new(
            "foundation-risale-002",
            TruthFoundationKind::RisaleNurOriginalText,
            FoundationAuthority::
                AbsoluteRevelatoryTruth,
            FoundationUse::Interpretation,
            "Risale-i Nur orijinal metin kaydı",
        );

        assert!(!foundation.has_valid_risale_authority());
        assert!(!foundation.is_constitutionally_valid());
    }

    #[test]
    fn zanistarast_interpretation_remains_human_interpretation() {
        let foundation = TruthFoundation::new(
            "foundation-zanistarast-001",
            TruthFoundationKind::ZanistarastInterpretation,
            FoundationAuthority::HumanInterpretation,
            FoundationUse::Synthesis,
            "Zanistarast sentez kaydı",
        );

        assert!(
            foundation.has_valid_zanistarast_authority()
        );
        assert!(foundation.requires_mudebbir_decision);
        assert!(foundation.is_constitutionally_valid());
    }

    #[test]
    fn source_and_interpretation_must_remain_separate() {
        let foundation = TruthFoundation::new(
            "foundation-logic-001",
            TruthFoundationKind::Logic,
            FoundationAuthority::InvestigativeMethod,
            FoundationUse::ProofMethod,
            "Mantıksal çıkarım kaydı",
        )
        .with_original_text("Aynı ifade")
        .with_human_interpretation("Aynı ifade");

        assert!(
            !foundation
                .separates_source_from_interpretation()
        );
        assert!(!foundation.is_constitutionally_valid());
    }

    #[test]
    fn foundation_set_combines_distinct_proof_paths() {
        let risale = TruthFoundation::new(
            "foundation-risale-003",
            TruthFoundationKind::RisaleNurOriginalText,
            FoundationAuthority::
                PrimaryInterpretiveReference,
            FoundationUse::ProofMethod,
            "Risale-i Nur orijinal metin kaydı",
        );

        let creation_book = TruthFoundation::new(
            "foundation-creation-001",
            TruthFoundationKind::CreationBook,
            FoundationAuthority::CreatedOrderEvidence,
            FoundationUse::EmpiricalEvidence,
            "Kâinat kitabı gözlem kaydı",
        );

        let fitrah = TruthFoundation::new(
            "foundation-fitrah-001",
            TruthFoundationKind::FitrahEvidence,
            FoundationAuthority::CreatedOrderEvidence,
            FoundationUse::ProofMethod,
            "Fıtrat delili kaydı",
        );

        let logic = TruthFoundation::new(
            "foundation-logic-002",
            TruthFoundationKind::Logic,
            FoundationAuthority::InvestigativeMethod,
            FoundationUse::ProofMethod,
            "Mantıksal inceleme kaydı",
        );

        let observation = TruthFoundation::new(
            "foundation-observation-001",
            TruthFoundationKind::Observation,
            FoundationAuthority::InvestigativeMethod,
            FoundationUse::EmpiricalEvidence,
            "Gözlem kaydı",
        );

        let set = TruthFoundationSet::new(
            "Meleklerin varlığı ve ispat yolları",
        )
        .with_foundations(vec![
            quran_foundation(),
            risale,
            creation_book,
            fitrah,
            logic,
            observation,
        ]);

        assert!(set.has_quranic_foundation());
        assert!(set.has_risale_reference());
        assert!(set.has_creation_book_evidence());
        assert!(set.has_fitrah_evidence());
        assert!(set.has_rational_method());
        assert!(set.has_empirical_method());
        assert!(set.is_complete());
    }

    #[test]
    fn rasterast_review_applies_to_human_interpretation() {
        let foundation = quran_foundation();

        assert!(foundation.interpretation_requires_review());
        assert!(foundation.requires_rasterast_review);
        assert!(foundation.requires_mudebbir_decision);
    }
}


