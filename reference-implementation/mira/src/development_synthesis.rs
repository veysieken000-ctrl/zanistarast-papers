use std::time::SystemTime;

use crate::{
    ArticleCircularDevelopment,
    ArticleCorePeripheryDevelopment,
    ArticleLinearDevelopment,
};

/// Üç gelişim modelinin belirli bir konu üzerindeki
/// karşılıklı uyum durumudur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevelopmentAgreementStatus {
    /// Henüz yeterli inceleme yapılmamıştır.
    NotAssessed,

    /// Modeller birbirini kısmen desteklemektedir.
    PartiallyAligned,

    /// Modeller arasında güçlü yapısal uyum vardır.
    Aligned,

    /// Çözülmesi gereken farklılıklar bulunmaktadır.
    Divergent,

    /// Modeller arasında açık çelişki bulunmaktadır.
    Contradicted,

    /// Uyum Rasterast tarafından doğrulanmıştır.
    Verified,
}

/// Sentez raporunun ele aldığı çalışmanın temel bilgisidir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DevelopmentSynthesisMetadata {
    pub synthesis_id: String,
    pub article_id: String,
    pub domain: String,
    pub subject: String,
}

impl DevelopmentSynthesisMetadata {
    pub fn new(
        synthesis_id: impl Into<String>,
        article_id: impl Into<String>,
        domain: impl Into<String>,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            synthesis_id: synthesis_id.into(),
            article_id: article_id.into(),
            domain: domain.into(),
            subject: subject.into(),
        }
    }

    pub fn is_complete(&self) -> bool {
        !self.synthesis_id.trim().is_empty()
            && !self.article_id.trim().is_empty()
            && !self.domain.trim().is_empty()
            && !self.subject.trim().is_empty()
    }
}

/// İki veya daha fazla modelin aynı noktada birbirini
/// desteklediğini gösteren kayıttır.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisAgreement {
    pub subject: String,
    pub supporting_models: Vec<String>,
    pub rationale: String,
    pub evidence: Vec<String>,
}

impl SynthesisAgreement {
    pub fn new(
        subject: impl Into<String>,
        supporting_models: Vec<String>,
        rationale: impl Into<String>,
    ) -> Self {
        Self {
            subject: subject.into(),
            supporting_models,
            rationale: rationale.into(),
            evidence: Vec::new(),
        }
    }

    pub fn with_evidence(
        mut self,
        evidence: Vec<String>,
    ) -> Self {
        self.evidence = evidence;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.subject.trim().is_empty()
            && self.supporting_models.len() >= 2
            && !self.rationale.trim().is_empty()
    }
}

/// Modeller arasında aynı sonucu vermeyen fakat henüz
/// açık çelişki sayılmayan farklılığı kaydeder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisDifference {
    pub subject: String,
    pub model_positions: Vec<String>,
    pub explanation: String,
    pub requires_further_analysis: bool,
}

impl SynthesisDifference {
    pub fn new(
        subject: impl Into<String>,
        model_positions: Vec<String>,
        explanation: impl Into<String>,
    ) -> Self {
        Self {
            subject: subject.into(),
            model_positions,
            explanation: explanation.into(),
            requires_further_analysis: true,
        }
    }

    pub fn is_complete(&self) -> bool {
        !self.subject.trim().is_empty()
            && self.model_positions.len() >= 2
            && !self.explanation.trim().is_empty()
    }
}

/// Modeller arasında çözülmesi gereken açık çelişkiyi
/// görünür biçimde saklar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SynthesisContradiction {
    pub subject: String,
    pub claims: Vec<String>,
    pub risks: Vec<String>,
    pub proposed_resolution: String,
}

impl SynthesisContradiction {
    pub fn new(
        subject: impl Into<String>,
        claims: Vec<String>,
        proposed_resolution: impl Into<String>,
    ) -> Self {
        Self {
            subject: subject.into(),
            claims,
            risks: Vec::new(),
            proposed_resolution: proposed_resolution.into(),
        }
    }

    pub fn with_risks(
        mut self,
        risks: Vec<String>,
    ) -> Self {
        self.risks = risks;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.subject.trim().is_empty()
            && self.claims.len() >= 2
            && !self.proposed_resolution.trim().is_empty()
    }
}

/// Gelişim sentezinin Rasterast değerlendirmesidir.
///
/// Doğrulama yalnızca başarılı veya başarısız şeklinde
/// tutulmaz. Doğrulanan, doğrulanamayan ve riskli noktalar
/// ayrı ayrı korunur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DevelopmentRasterastAssessment {
    pub verified_items: Vec<String>,
    pub unverified_items: Vec<String>,
    pub contradictions: Vec<String>,
    pub risks: Vec<String>,
    pub verified: bool,
    pub requires_mudebbir_decision: bool,
}

impl DevelopmentRasterastAssessment {
    pub fn new() -> Self {
        Self {
            verified_items: Vec::new(),
            unverified_items: Vec::new(),
            contradictions: Vec::new(),
            risks: Vec::new(),
            verified: false,
            requires_mudebbir_decision: true,
        }
    }

    pub fn with_verified_items(
        mut self,
        verified_items: Vec<String>,
    ) -> Self {
        self.verified_items = verified_items;
        self
    }

    pub fn with_unverified_items(
        mut self,
        unverified_items: Vec<String>,
    ) -> Self {
        self.unverified_items = unverified_items;
        self
    }

    pub fn with_contradictions(
        mut self,
        contradictions: Vec<String>,
    ) -> Self {
        self.contradictions = contradictions;
        self
    }

    pub fn with_risks(
        mut self,
        risks: Vec<String>,
    ) -> Self {
        self.risks = risks;
        self
    }

    pub fn mark_verified(mut self) -> Self {
        self.verified = true;
        self
    }

    pub fn has_unresolved_items(&self) -> bool {
        !self.unverified_items.is_empty()
            || !self.contradictions.is_empty()
            || !self.risks.is_empty()
    }

    pub fn can_support_synthesis(&self) -> bool {
        self.verified
            && !self.verified_items.is_empty()
            && !self.has_unresolved_items()
    }
}

impl Default for DevelopmentRasterastAssessment {
    fn default() -> Self {
        Self::new()
    }
}

/// Doğrusal, dairesel ve merkez–çevre modellerinin
/// aynı çalışma üzerindeki birleşik sentez raporudur.
///
/// Bu yapı modelleri birbirine indirgemez. Her model kendi
/// kapsamını korur ve yalnızca aralarındaki uyum, fark,
/// çelişki ve eksik bağlantılar değerlendirilir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DevelopmentSynthesisReport {
    pub metadata: DevelopmentSynthesisMetadata,
    pub linear_development: ArticleLinearDevelopment,
    pub circular_development: ArticleCircularDevelopment,
    pub core_periphery_development:
        ArticleCorePeripheryDevelopment,
    pub agreements: Vec<SynthesisAgreement>,
    pub differences: Vec<SynthesisDifference>,
    pub contradictions: Vec<SynthesisContradiction>,
    pub missing_links: Vec<String>,
    pub status: DevelopmentAgreementStatus,
    pub rasterast_assessment:
        Option<DevelopmentRasterastAssessment>,
    pub requires_mudebbir_decision: bool,
    pub created_at: SystemTime,
}

impl DevelopmentSynthesisReport {
    pub fn new(
        metadata: DevelopmentSynthesisMetadata,
        linear_development: ArticleLinearDevelopment,
        circular_development: ArticleCircularDevelopment,
        core_periphery_development:
            ArticleCorePeripheryDevelopment,
        created_at: SystemTime,
    ) -> Self {
        Self {
            metadata,
            linear_development,
            circular_development,
            core_periphery_development,
            agreements: Vec::new(),
            differences: Vec::new(),
            contradictions: Vec::new(),
            missing_links: Vec::new(),
            status: DevelopmentAgreementStatus::NotAssessed,
            rasterast_assessment: None,
            requires_mudebbir_decision: true,
            created_at,
        }
    }

    pub fn with_agreements(
        mut self,
        agreements: Vec<SynthesisAgreement>,
    ) -> Self {
        self.agreements = agreements;
        self
    }

    pub fn with_differences(
        mut self,
        differences: Vec<SynthesisDifference>,
    ) -> Self {
        self.differences = differences;
        self
    }

    pub fn with_contradictions(
        mut self,
        contradictions: Vec<SynthesisContradiction>,
    ) -> Self {
        self.contradictions = contradictions;
        self
    }

    pub fn with_missing_links(
        mut self,
        missing_links: Vec<String>,
    ) -> Self {
        self.missing_links = missing_links;
        self
    }

    pub fn with_status(
        mut self,
        status: DevelopmentAgreementStatus,
    ) -> Self {
        self.status = status;
        self
    }

    pub fn with_rasterast_assessment(
        mut self,
        assessment: DevelopmentRasterastAssessment,
    ) -> Self {
        if assessment.can_support_synthesis() {
            self.status = DevelopmentAgreementStatus::Verified;
        }

        self.rasterast_assessment = Some(assessment);
        self
    }

    pub fn references_same_article(&self) -> bool {
        let article_id = &self.metadata.article_id;

        self.linear_development.article_id == *article_id
            && self.circular_development.article_id == *article_id
            && self.core_periphery_development.article_id
                == *article_id
    }

    pub fn is_complete(&self) -> bool {
        self.metadata.is_complete()
            && self.references_same_article()
            && self.linear_development.is_complete()
            && self.circular_development.is_complete()
            && self.core_periphery_development.is_complete()
            && self
                .agreements
                .iter()
                .all(SynthesisAgreement::is_complete)
            && self
                .differences
                .iter()
                .all(SynthesisDifference::is_complete)
            && self
                .contradictions
                .iter()
                .all(SynthesisContradiction::is_complete)
    }

    pub fn has_unresolved_conflicts(&self) -> bool {
        !self.differences.is_empty()
            || !self.contradictions.is_empty()
            || !self.missing_links.is_empty()
            || self.linear_development.has_unresolved_gaps()
            || self
                .circular_development
                .has_unresolved_relations()
            || self
                .core_periphery_development
                .has_unresolved_structure()
    }

    pub fn can_support_academic_synthesis(&self) -> bool {
        self.is_complete()
            && !self.agreements.is_empty()
            && !self.has_unresolved_conflicts()
            && self.linear_development.can_support_synthesis()
            && self.circular_development.can_support_synthesis()
            && self
                .core_periphery_development
                .can_support_synthesis()
            && self
                .rasterast_assessment
                .as_ref()
                .is_some_and(
                    DevelopmentRasterastAssessment::
                        can_support_synthesis,
                )
            && self.status
                == DevelopmentAgreementStatus::Verified
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        BidirectionalVerification,
        CircularDevelopmentMetadata,
        CorePeripheryLayer,
        CorePeripheryMetadata,
        CorePeripheryPhase,
        CorePeripheryRole,
        DevelopmentDimension,
        DevelopmentStage,
        DevelopmentStageStatus,
        DimensionRelation,
        DimensionRelationType,
        LayerRelation,
        LayerRelationType,
        LinearDevelopmentMetadata,
        ReasoningDirection,
        ReasoningResult,
    };

    fn linear_model(
        article_id: &str,
    ) -> ArticleLinearDevelopment {
        let metadata = LinearDevelopmentMetadata::new(
            "Örnek alan",
            "Başlangıç",
            "Bütünleşmiş gelişim",
        );

        let stages = vec![
            DevelopmentStage::new(
                "stage-1",
                "Başlangıç",
                1,
            )
            .with_status(
                DevelopmentStageStatus::Verified,
            ),
            DevelopmentStage::new(
                "stage-2",
                "Gelişim",
                2,
            )
            .with_status(
                DevelopmentStageStatus::Verified,
            ),
        ];

        ArticleLinearDevelopment::new(
            article_id,
            metadata,
            SystemTime::now(),
        )
        .with_stages(stages)
        .with_covered_stage_ids(vec![
            "stage-1".to_string(),
            "stage-2".to_string(),
        ])
        .mark_rasterast_verified()
    }

    fn circular_model(
        article_id: &str,
    ) -> ArticleCircularDevelopment {
        let metadata = CircularDevelopmentMetadata::new(
            "dimension-1",
            "Boyutlar arası yayılım",
            "Bütünleşmiş ilişki",
        );

        let dimensions = vec![
            DevelopmentDimension::new(
                "dimension-1",
                "Birinci boyut",
            ),
            DevelopmentDimension::new(
                "dimension-2",
                "İkinci boyut",
            ),
        ];

        let relations = vec![
            DimensionRelation::new(
                "dimension-1",
                "dimension-2",
                DimensionRelationType::Influences,
                "Birinci boyut ikinci boyutu etkiler.",
            )
            .mark_rasterast_verified(),
        ];

        ArticleCircularDevelopment::new(
            article_id,
            metadata,
            SystemTime::now(),
        )
        .with_dimensions(dimensions)
        .with_relations(relations)
        .mark_rasterast_verified()
    }

    fn core_periphery_model(
        article_id: &str,
    ) -> ArticleCorePeripheryDevelopment {
        let metadata = CorePeripheryMetadata::new(
            "Örnek alan",
            "İşlevsel merkez",
            "Katmanlarıyla bütün yapı",
        );

        let layers = vec![
            CorePeripheryLayer::new(
                "core",
                "Merkez",
                1,
                CorePeripheryPhase::CoreFormation,
                CorePeripheryRole::Core,
            ),
            CorePeripheryLayer::new(
                "outer",
                "Çevre",
                2,
                CorePeripheryPhase::IntegratedWhole,
                CorePeripheryRole::IntegratingLayer,
            ),
        ];

        let relations = vec![
            LayerRelation::new(
                "core",
                "outer",
                LayerRelationType::Grounds,
                "Merkez çevresel yapıya temel sağlar.",
            )
            .mark_rasterast_verified(),
        ];

        let deductive = ReasoningResult::new(
            ReasoningDirection::Deductive,
            "Merkezden çevreye bütünlük destekleniyor.",
        );

        let inductive = ReasoningResult::new(
            ReasoningDirection::Inductive,
            "Çevreden merkeze bütünlük destekleniyor.",
        );

        let verification =
            BidirectionalVerification::new(
                deductive,
                inductive,
            )
            .with_agreements(vec![
                "İki yön yapısal olarak uyumludur."
                    .to_string(),
            ])
            .mark_rasterast_verified();

        ArticleCorePeripheryDevelopment::new(
            article_id,
            metadata,
            SystemTime::now(),
        )
        .with_layers(layers)
        .with_relations(relations)
        .with_bidirectional_verification(verification)
        .mark_rasterast_verified()
    }

    fn complete_report() -> DevelopmentSynthesisReport {
        let article_id = "article-001";

        let metadata = DevelopmentSynthesisMetadata::new(
            "synthesis-001",
            article_id,
            "Örnek alan",
            "Üç gelişim modelinin karşılıklı doğrulanması",
        );

        let agreement = SynthesisAgreement::new(
            "Bütünsel gelişim",
            vec![
                "linear".to_string(),
                "circular".to_string(),
                "core-periphery".to_string(),
            ],
            "Üç model aynı gelişim bütünlüğünü destekler.",
        )
        .with_evidence(vec![
            "Yapısal uyum kaydedildi.".to_string(),
        ]);

        DevelopmentSynthesisReport::new(
            metadata,
            linear_model(article_id),
            circular_model(article_id),
            core_periphery_model(article_id),
            SystemTime::now(),
        )
        .with_agreements(vec![agreement])
        .with_status(
            DevelopmentAgreementStatus::Aligned,
        )
    }

    #[test]
    fn creates_complete_synthesis_report() {
        let report = complete_report();

        assert!(report.is_complete());
        assert!(report.references_same_article());
        assert_eq!(report.agreements.len(), 1);
        assert!(report.requires_mudebbir_decision);
    }

    #[test]
    fn different_article_identifiers_prevent_completeness() {
        let metadata = DevelopmentSynthesisMetadata::new(
            "synthesis-002",
            "article-001",
            "Örnek alan",
            "Uyumsuz makale kimlikleri",
        );

        let report = DevelopmentSynthesisReport::new(
            metadata,
            linear_model("article-001"),
            circular_model("article-002"),
            core_periphery_model("article-001"),
            SystemTime::now(),
        );

        assert!(!report.references_same_article());
        assert!(!report.is_complete());
    }

    #[test]
    fn difference_blocks_academic_synthesis() {
        let difference = SynthesisDifference::new(
            "Gelişim sırası",
            vec![
                "Doğrusal model: A → B".to_string(),
                "Dairesel model: B → A".to_string(),
            ],
            "İki model ilişkinin yönünü farklı gösteriyor.",
        );

        let report = complete_report()
            .with_differences(vec![difference]);

        assert!(report.has_unresolved_conflicts());
        assert!(!report.can_support_academic_synthesis());
    }

    #[test]
    fn contradiction_blocks_academic_synthesis() {
        let contradiction =
            SynthesisContradiction::new(
                "Merkez ilişkisi",
                vec![
                    "Birinci model A unsurunu merkez kabul eder."
                        .to_string(),
                    "İkinci model B unsurunu merkez kabul eder."
                        .to_string(),
                ],
                "Kaynaklar yeniden doğrulanmalıdır.",
            )
            .with_risks(vec![
                "Yanlış merkez belirleme riski."
                    .to_string(),
            ]);

        let report = complete_report()
            .with_contradictions(vec![contradiction]);

        assert!(report.has_unresolved_conflicts());
        assert!(!report.can_support_academic_synthesis());
    }

    #[test]
    fn missing_link_blocks_academic_synthesis() {
        let report = complete_report()
            .with_missing_links(vec![
                "Doğrusal durak ile çevresel katman arasında bağ eksik."
                    .to_string(),
            ]);

        assert!(report.has_unresolved_conflicts());
        assert!(!report.can_support_academic_synthesis());
    }

    #[test]
    fn rasterast_assessment_requires_no_unresolved_items() {
        let assessment =
            DevelopmentRasterastAssessment::new()
                .with_verified_items(vec![
                    "Üç modelin temel uyumu."
                        .to_string(),
                ])
                .with_unverified_items(vec![
                    "Bir ilişki henüz doğrulanmadı."
                        .to_string(),
                ])
                .mark_verified();

        assert!(assessment.has_unresolved_items());
        assert!(!assessment.can_support_synthesis());
    }

    #[test]
    fn verified_report_supports_academic_synthesis() {
        let assessment =
            DevelopmentRasterastAssessment::new()
                .with_verified_items(vec![
                    "Doğrusal gelişim doğrulandı."
                        .to_string(),
                    "Dairesel ilişkiler doğrulandı."
                        .to_string(),
                    "Merkez–çevre yapısı doğrulandı."
                        .to_string(),
                    "Tümevarım ve tümdengelim uyumlu."
                        .to_string(),
                ])
                .mark_verified();

        let report = complete_report()
            .with_rasterast_assessment(assessment);

        assert_eq!(
            report.status,
            DevelopmentAgreementStatus::Verified
        );
        assert!(!report.has_unresolved_conflicts());
        assert!(report.can_support_academic_synthesis());
    }

    #[test]
    fn rasterast_verification_does_not_remove_mudebbir_gate() {
        let assessment =
            DevelopmentRasterastAssessment::new()
                .with_verified_items(vec![
                    "Sentez doğrulandı.".to_string(),
                ])
                .mark_verified();

        let report = complete_report()
            .with_rasterast_assessment(assessment);

        assert!(report.can_support_academic_synthesis());
        assert!(report.requires_mudebbir_decision);
    }
}




