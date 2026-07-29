use std::time::SystemTime;

/// Merkez–çevre yapısında bir katmanın temel rolünü belirtir.
///
/// Roller belirli bir varlık türüne sabitlenmez.
/// Aynı veya benzer yapı fizik, biyoloji, insan, toplum,
/// bilgi ve uygarlık alanlarında farklı biçimlerde görülebilir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorePeripheryRole {
    /// İncelenen yapının özsel veya işlevsel merkezidir.
    Core,

    /// Merkeze en yakın iç katmandır.
    InnerLayer,

    /// Önceki katmanları kuşatan ve kapsamını genişleten katmandır.
    SurroundingLayer,

    /// İç katmanlar arasındaki ilişkiyi düzenleyen katmandır.
    RegulatingLayer,

    /// Katmanları daha geniş bir bütünlükte birleştiren katmandır.
    IntegratingLayer,

    /// Bütün yapının dış çevreyle ilişki kurduğu katmandır.
    InterfaceLayer,
}

/// Merkez–çevre gelişiminin genel evresidir.
///
/// Evreler belirli bir örneğe bağlı değildir.
/// Her varlık alanında farklı sayıda katman ve durak bulunabilir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorePeripheryPhase {
    /// Özsel veya işlevsel merkezin belirlendiği evre.
    CoreFormation,

    /// Merkezin çevresel katmanlarda açıldığı evre.
    LayeredExpansion,

    /// Katmanların birbirini kuşattığı ve düzenlediği evre.
    EnclosureAndRegulation,

    /// Merkez ve çevrenin bütünsel bir yapı oluşturduğu evre.
    IntegratedWhole,
}

/// Merkez–çevre modelindeki çıkarım yönüdür.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReasoningDirection {
    /// Merkezden, ilkeden veya bütünden katmanlara ilerler.
    Deductive,

    /// Katmanlardan, gözlemlerden veya parçalardan merkeze ilerler.
    Inductive,
}

/// Katmanlar arasındaki ilişkinin türüdür.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayerRelationType {
    /// Kaynak katman hedef katmanı kuşatır.
    Encloses,

    /// Kaynak katman hedef katmana temel veya imkân sağlar.
    Grounds,

    /// Kaynak katman hedef katmanı düzenler.
    Regulates,

    /// Kaynak katman hedef katmanın kapsamını genişletir.
    Expands,

    /// İki katman karşılıklı etkileşim içindedir.
    MutualInfluence,

    /// Kaynak katman hedef katmandan geri besleme alır.
    ReceivesFeedback,

    /// Katmanlar daha geniş bir bütünlükte birleşir.
    IntegratesWith,

    /// Katmanlar arasında incelenmesi gereken gerilim vardır.
    Tension,

    /// Katmanlar arasında doğrulanması gereken çelişki vardır.
    Contradiction,
}

/// Bir ilişkinin doğrulama güven seviyesidir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorePeripheryConfidence {
    Low,
    Medium,
    High,
    Verified,
}

/// Merkez–çevre yapısındaki tek bir katmandır.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorePeripheryLayer {
    pub layer_id: String,
    pub name: String,
    pub description: String,
    pub sequence: u32,
    pub phase: CorePeripheryPhase,
    pub role: CorePeripheryRole,
    pub evidence: Vec<String>,
    pub uncertainties: Vec<String>,
}

impl CorePeripheryLayer {
    pub fn new(
        layer_id: impl Into<String>,
        name: impl Into<String>,
        sequence: u32,
        phase: CorePeripheryPhase,
        role: CorePeripheryRole,
    ) -> Self {
        Self {
            layer_id: layer_id.into(),
            name: name.into(),
            description: String::new(),
            sequence,
            phase,
            role,
            evidence: Vec::new(),
            uncertainties: Vec::new(),
        }
    }

    pub fn with_description(
        mut self,
        description: impl Into<String>,
    ) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_evidence(
        mut self,
        evidence: Vec<String>,
    ) -> Self {
        self.evidence = evidence;
        self
    }

    pub fn with_uncertainties(
        mut self,
        uncertainties: Vec<String>,
    ) -> Self {
        self.uncertainties = uncertainties;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.layer_id.trim().is_empty()
            && !self.name.trim().is_empty()
    }

    pub fn has_uncertainty(&self) -> bool {
        !self.uncertainties.is_empty()
    }
}

/// İki katman arasındaki merkez–çevre ilişkisini kaydeder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayerRelation {
    pub source_layer_id: String,
    pub target_layer_id: String,
    pub relation_type: LayerRelationType,
    pub rationale: String,
    pub evidence: Vec<String>,
    pub uncertainties: Vec<String>,
    pub confidence: CorePeripheryConfidence,
    pub rasterast_verified: bool,
}

impl LayerRelation {
    pub fn new(
        source_layer_id: impl Into<String>,
        target_layer_id: impl Into<String>,
        relation_type: LayerRelationType,
        rationale: impl Into<String>,
    ) -> Self {
        Self {
            source_layer_id: source_layer_id.into(),
            target_layer_id: target_layer_id.into(),
            relation_type,
            rationale: rationale.into(),
            evidence: Vec::new(),
            uncertainties: Vec::new(),
            confidence: CorePeripheryConfidence::Low,
            rasterast_verified: false,
        }
    }

    pub fn with_evidence(
        mut self,
        evidence: Vec<String>,
    ) -> Self {
        self.evidence = evidence;
        self
    }

    pub fn with_uncertainties(
        mut self,
        uncertainties: Vec<String>,
    ) -> Self {
        self.uncertainties = uncertainties;
        self
    }

    pub fn with_confidence(
        mut self,
        confidence: CorePeripheryConfidence,
    ) -> Self {
        self.confidence = confidence;
        self
    }

    pub fn mark_rasterast_verified(mut self) -> Self {
        self.rasterast_verified = true;
        self.confidence = CorePeripheryConfidence::Verified;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.source_layer_id.trim().is_empty()
            && !self.target_layer_id.trim().is_empty()
            && self.source_layer_id != self.target_layer_id
            && !self.rationale.trim().is_empty()
    }

    pub fn has_unresolved_risk(&self) -> bool {
        !self.uncertainties.is_empty()
            || matches!(
                self.relation_type,
                LayerRelationType::Tension
                    | LayerRelationType::Contradiction
            )
    }
}

/// Tümevarım veya tümdengelim yönünde elde edilen tek bir
/// değerlendirme sonucudur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReasoningResult {
    pub direction: ReasoningDirection,
    pub conclusion: String,
    pub supported_items: Vec<String>,
    pub unsupported_items: Vec<String>,
    pub contradictions: Vec<String>,
}

impl ReasoningResult {
    pub fn new(
        direction: ReasoningDirection,
        conclusion: impl Into<String>,
    ) -> Self {
        Self {
            direction,
            conclusion: conclusion.into(),
            supported_items: Vec::new(),
            unsupported_items: Vec::new(),
            contradictions: Vec::new(),
        }
    }

    pub fn with_supported_items(
        mut self,
        supported_items: Vec<String>,
    ) -> Self {
        self.supported_items = supported_items;
        self
    }

    pub fn with_unsupported_items(
        mut self,
        unsupported_items: Vec<String>,
    ) -> Self {
        self.unsupported_items = unsupported_items;
        self
    }

    pub fn with_contradictions(
        mut self,
        contradictions: Vec<String>,
    ) -> Self {
        self.contradictions = contradictions;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.conclusion.trim().is_empty()
    }

    pub fn has_unresolved_items(&self) -> bool {
        !self.unsupported_items.is_empty()
            || !self.contradictions.is_empty()
    }
}

/// Tümevarım ile tümdengelim sonuçlarının karşılıklı
/// doğrulama kaydıdır.
///
/// Uyum yalnızca bir boolean değerle temsil edilmez.
/// Desteklenen, eksik ve çelişkili noktalar ayrı tutulur.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BidirectionalVerification {
    pub deductive_result: ReasoningResult,
    pub inductive_result: ReasoningResult,
    pub agreements: Vec<String>,
    pub differences: Vec<String>,
    pub contradictions: Vec<String>,
    pub missing_links: Vec<String>,
    pub rasterast_verified: bool,
    pub requires_mudebbir_decision: bool,
}

impl BidirectionalVerification {
    pub fn new(
        deductive_result: ReasoningResult,
        inductive_result: ReasoningResult,
    ) -> Self {
        Self {
            deductive_result,
            inductive_result,
            agreements: Vec::new(),
            differences: Vec::new(),
            contradictions: Vec::new(),
            missing_links: Vec::new(),
            rasterast_verified: false,
            requires_mudebbir_decision: true,
        }
    }

    pub fn with_agreements(
        mut self,
        agreements: Vec<String>,
    ) -> Self {
        self.agreements = agreements;
        self
    }

    pub fn with_differences(
        mut self,
        differences: Vec<String>,
    ) -> Self {
        self.differences = differences;
        self
    }

    pub fn with_contradictions(
        mut self,
        contradictions: Vec<String>,
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

    pub fn mark_rasterast_verified(mut self) -> Self {
        self.rasterast_verified = true;
        self
    }

    pub fn is_complete(&self) -> bool {
        self.deductive_result.is_complete()
            && self.inductive_result.is_complete()
    }

    pub fn has_unresolved_conflict(&self) -> bool {
        self.deductive_result.has_unresolved_items()
            || self.inductive_result.has_unresolved_items()
            || !self.contradictions.is_empty()
            || !self.missing_links.is_empty()
    }

    pub fn can_support_conclusion(&self) -> bool {
        self.is_complete()
            && self.rasterast_verified
            && !self.agreements.is_empty()
            && !self.has_unresolved_conflict()
    }
}

/// Merkez–çevre modelinin genel üst bilgisidir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CorePeripheryMetadata {
    pub domain: String,
    pub core_principle: String,
    pub integrated_whole: String,
}

impl CorePeripheryMetadata {
    pub fn new(
        domain: impl Into<String>,
        core_principle: impl Into<String>,
        integrated_whole: impl Into<String>,
    ) -> Self {
        Self {
            domain: domain.into(),
            core_principle: core_principle.into(),
            integrated_whole: integrated_whole.into(),
        }
    }

    pub fn is_complete(&self) -> bool {
        !self.domain.trim().is_empty()
            && !self.core_principle.trim().is_empty()
            && !self.integrated_whole.trim().is_empty()
    }
}

/// Bir makalenin veya bilgi çalışmasının merkez–çevre,
/// katmanlı kuşatma ve çift yönlü doğrulama kapsamıdır.
///
/// Model belirli bir insan örneğine sabit değildir.
/// Doğada, bilgide, toplumda ve diğer varlık alanlarında
/// tespit edilen aynı veya benzer örüntüleri kaydedebilir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArticleCorePeripheryDevelopment {
    pub article_id: String,
    pub metadata: CorePeripheryMetadata,
    pub layers: Vec<CorePeripheryLayer>,
    pub relations: Vec<LayerRelation>,
    pub bidirectional_verification:
        Option<BidirectionalVerification>,
    pub rasterast_verified: bool,
    pub requires_mudebbir_decision: bool,
    pub created_at: SystemTime,
}

impl ArticleCorePeripheryDevelopment {
    pub fn new(
        article_id: impl Into<String>,
        metadata: CorePeripheryMetadata,
        created_at: SystemTime,
    ) -> Self {
        Self {
            article_id: article_id.into(),
            metadata,
            layers: Vec::new(),
            relations: Vec::new(),
            bidirectional_verification: None,
            rasterast_verified: false,
            requires_mudebbir_decision: true,
            created_at,
        }
    }

    pub fn with_layers(
        mut self,
        mut layers: Vec<CorePeripheryLayer>,
    ) -> Self {
        layers.sort_by_key(|layer| layer.sequence);
        self.layers = layers;
        self
    }

    pub fn with_relations(
        mut self,
        relations: Vec<LayerRelation>,
    ) -> Self {
        self.relations = relations;
        self
    }

    pub fn with_bidirectional_verification(
        mut self,
        verification: BidirectionalVerification,
    ) -> Self {
        self.bidirectional_verification = Some(verification);
        self
    }

    pub fn mark_rasterast_verified(mut self) -> Self {
        self.rasterast_verified = true;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.article_id.trim().is_empty()
            && self.metadata.is_complete()
            && !self.layers.is_empty()
            && self
                .layers
                .iter()
                .all(CorePeripheryLayer::is_complete)
            && !self.relations.is_empty()
            && self
                .relations
                .iter()
                .all(LayerRelation::is_complete)
            && self.has_unique_layer_ids()
            && self.has_unique_sequences()
            && self.has_core_layer()
            && self.relations_reference_known_layers()
    }

    pub fn has_unresolved_structure(&self) -> bool {
        self.layers
            .iter()
            .any(CorePeripheryLayer::has_uncertainty)
            || self
                .relations
                .iter()
                .any(LayerRelation::has_unresolved_risk)
    }

    pub fn can_support_synthesis(&self) -> bool {
        self.is_complete()
            && self.rasterast_verified
            && !self.has_unresolved_structure()
            && self
                .relations
                .iter()
                .all(|relation| relation.rasterast_verified)
            && self
                .bidirectional_verification
                .as_ref()
                .is_some_and(
                    BidirectionalVerification::can_support_conclusion,
                )
    }

    fn has_unique_layer_ids(&self) -> bool {
        let mut layer_ids: Vec<&str> = self
            .layers
            .iter()
            .map(|layer| layer.layer_id.as_str())
            .collect();

        layer_ids.sort_unstable();
        layer_ids.dedup();

        layer_ids.len() == self.layers.len()
    }

    fn has_unique_sequences(&self) -> bool {
        let mut sequences: Vec<u32> = self
            .layers
            .iter()
            .map(|layer| layer.sequence)
            .collect();

        sequences.sort_unstable();
        sequences.dedup();

        sequences.len() == self.layers.len()
    }

    fn has_core_layer(&self) -> bool {
        self.layers.iter().any(|layer| {
            layer.role == CorePeripheryRole::Core
        })
    }

    fn relations_reference_known_layers(&self) -> bool {
        self.relations.iter().all(|relation| {
            self.has_layer(&relation.source_layer_id)
                && self.has_layer(
                    &relation.target_layer_id,
                )
        })
    }

    fn has_layer(&self, layer_id: &str) -> bool {
        self.layers
            .iter()
            .any(|layer| layer.layer_id == layer_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example_development(
    ) -> ArticleCorePeripheryDevelopment {
        let metadata = CorePeripheryMetadata::new(
            "Örnek varlık sistemi",
            "İşlevsel öz merkez",
            "Katmanlarıyla bütün sistem",
        );

        let layers = vec![
            CorePeripheryLayer::new(
                "core",
                "Öz merkez",
                1,
                CorePeripheryPhase::CoreFormation,
                CorePeripheryRole::Core,
            ),
            CorePeripheryLayer::new(
                "inner",
                "İç katman",
                2,
                CorePeripheryPhase::LayeredExpansion,
                CorePeripheryRole::InnerLayer,
            ),
            CorePeripheryLayer::new(
                "surrounding",
                "Kuşatan katman",
                3,
                CorePeripheryPhase::EnclosureAndRegulation,
                CorePeripheryRole::SurroundingLayer,
            ),
            CorePeripheryLayer::new(
                "whole",
                "Bütünleştirici katman",
                4,
                CorePeripheryPhase::IntegratedWhole,
                CorePeripheryRole::IntegratingLayer,
            ),
        ];

        let relations = vec![
            LayerRelation::new(
                "core",
                "inner",
                LayerRelationType::Grounds,
                "Öz merkez iç katmana temel sağlar.",
            )
            .mark_rasterast_verified(),
            LayerRelation::new(
                "surrounding",
                "inner",
                LayerRelationType::Encloses,
                "Kuşatan katman iç katmanı kapsar.",
            )
            .mark_rasterast_verified(),
            LayerRelation::new(
                "whole",
                "surrounding",
                LayerRelationType::IntegratesWith,
                "Bütünleştirici katman yapıyı birleştirir.",
            )
            .mark_rasterast_verified(),
        ];

        let deductive = ReasoningResult::new(
            ReasoningDirection::Deductive,
            "Merkezden çevreye katmanlı bütünlük görülür.",
        )
        .with_supported_items(vec![
            "Katman sırası destekleniyor.".to_string(),
        ]);

        let inductive = ReasoningResult::new(
            ReasoningDirection::Inductive,
            "Çevresel gözlemler merkez ilkesini destekler.",
        )
        .with_supported_items(vec![
            "Gözlemler merkezle uyumludur.".to_string(),
        ]);

        let verification =
            BidirectionalVerification::new(
                deductive,
                inductive,
            )
            .with_agreements(vec![
                "İki yön aynı yapısal bütünlüğü destekliyor."
                    .to_string(),
            ])
            .mark_rasterast_verified();

        ArticleCorePeripheryDevelopment::new(
            "article-example-001",
            metadata,
            SystemTime::now(),
        )
        .with_layers(layers)
        .with_relations(relations)
        .with_bidirectional_verification(verification)
    }

    #[test]
    fn creates_complete_core_periphery_development() {
        let development = example_development();

        assert!(development.is_complete());
        assert_eq!(development.layers.len(), 4);
        assert_eq!(development.relations.len(), 3);
        assert!(
            development.requires_mudebbir_decision
        );
    }

    #[test]
    fn layers_are_sorted_by_sequence() {
        let metadata = CorePeripheryMetadata::new(
            "Örnek alan",
            "Öz",
            "Bütün",
        );

        let layers = vec![
            CorePeripheryLayer::new(
                "outer",
                "Dış katman",
                2,
                CorePeripheryPhase::LayeredExpansion,
                CorePeripheryRole::SurroundingLayer,
            ),
            CorePeripheryLayer::new(
                "core",
                "Merkez",
                1,
                CorePeripheryPhase::CoreFormation,
                CorePeripheryRole::Core,
            ),
        ];

        let development =
            ArticleCorePeripheryDevelopment::new(
                "article-001",
                metadata,
                SystemTime::now(),
            )
            .with_layers(layers);

        assert_eq!(
            development.layers[


