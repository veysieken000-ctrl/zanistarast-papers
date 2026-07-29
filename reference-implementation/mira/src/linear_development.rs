use std::time::SystemTime;

/// Doğrusal gelişim zincirindeki tek bir durağın durumudur.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevelopmentStageStatus {
    Proposed,
    Supported,
    Verified,
    Incomplete,
    Contradicted,
}

/// Bir boyutun doğrusal gelişim çizgisindeki tek bir durağıdır.
///
/// Örnek:
/// atom → molekül → hücre → organizma → insan
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DevelopmentStage {
    pub stage_id: String,
    pub name: String,
    pub description: String,
    pub sequence: u32,
    pub status: DevelopmentStageStatus,
    pub evidence: Vec<String>,
    pub uncertainties: Vec<String>,
}

impl DevelopmentStage {
    pub fn new(
        stage_id: impl Into<String>,
        name: impl Into<String>,
        sequence: u32,
    ) -> Self {
        Self {
            stage_id: stage_id.into(),
            name: name.into(),
            description: String::new(),
            sequence,
            status: DevelopmentStageStatus::Proposed,
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

    pub fn with_status(
        mut self,
        status: DevelopmentStageStatus,
    ) -> Self {
        self.status = status;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.stage_id.trim().is_empty()
            && !self.name.trim().is_empty()
    }

    pub fn is_verified(&self) -> bool {
        self.status == DevelopmentStageStatus::Verified
    }
}

/// Bir doğrusal gelişim zincirinin temel tanımıdır.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinearDevelopmentMetadata {
    pub dimension: String,
    pub origin: String,
    pub proposed_maturity: String,
}

impl LinearDevelopmentMetadata {
    pub fn new(
        dimension: impl Into<String>,
        origin: impl Into<String>,
        proposed_maturity: impl Into<String>,
    ) -> Self {
        Self {
            dimension: dimension.into(),
            origin: origin.into(),
            proposed_maturity: proposed_maturity.into(),
        }
    }

    pub fn is_complete(&self) -> bool {
        !self.dimension.trim().is_empty()
            && !self.origin.trim().is_empty()
            && !self.proposed_maturity.trim().is_empty()
    }
}

/// Bir makalenin Zanistarast doğrusal gelişim çizgisindeki
/// kapsamını ve yerini kaydeder.
///
/// Bu model kemal hakkında nihai hüküm vermez.
/// Yalnızca makalenin önerdiği gelişim çizgisini,
/// durakları ve doğrulama durumunu kaydeder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArticleLinearDevelopment {
    pub article_id: String,
    pub metadata: LinearDevelopmentMetadata,
    pub stages: Vec<DevelopmentStage>,
    pub covered_stage_ids: Vec<String>,
    pub missing_stage_ids: Vec<String>,
    pub rasterast_verified: bool,
    pub requires_mudebbir_decision: bool,
    pub created_at: SystemTime,
}

impl ArticleLinearDevelopment {
    pub fn new(
        article_id: impl Into<String>,
        metadata: LinearDevelopmentMetadata,
        created_at: SystemTime,
    ) -> Self {
        Self {
            article_id: article_id.into(),
            metadata,
            stages: Vec::new(),
            covered_stage_ids: Vec::new(),
            missing_stage_ids: Vec::new(),
            rasterast_verified: false,
            requires_mudebbir_decision: true,
            created_at,
        }
    }

    pub fn with_stages(
        mut self,
        mut stages: Vec<DevelopmentStage>,
    ) -> Self {
        stages.sort_by_key(|stage| stage.sequence);
        self.stages = stages;
        self
    }

    pub fn with_covered_stage_ids(
        mut self,
        covered_stage_ids: Vec<String>,
    ) -> Self {
        self.covered_stage_ids = covered_stage_ids;
        self
    }

    pub fn with_missing_stage_ids(
        mut self,
        missing_stage_ids: Vec<String>,
    ) -> Self {
        self.missing_stage_ids = missing_stage_ids;
        self
    }

    pub fn mark_rasterast_verified(mut self) -> Self {
        self.rasterast_verified = true;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.article_id.trim().is_empty()
            && self.metadata.is_complete()
            && !self.stages.is_empty()
            && self
                .stages
                .iter()
                .all(DevelopmentStage::is_complete)
            && self.has_unique_stage_ids()
            && self.has_unique_sequences()
    }

    pub fn has_unresolved_gaps(&self) -> bool {
        !self.missing_stage_ids.is_empty()
            || self.stages.iter().any(|stage| {
                matches!(
                    stage.status,
                    DevelopmentStageStatus::Incomplete
                        | DevelopmentStageStatus::Contradicted
                )
            })
    }

    pub fn can_support_synthesis(&self) -> bool {
        self.is_complete()
            && self.rasterast_verified
            && !self.has_unresolved_gaps()
    }

    pub fn verified_stage_count(&self) -> usize {
        self.stages
            .iter()
            .filter(|stage| stage.is_verified())
            .count()
    }

    fn has_unique_stage_ids(&self) -> bool {
        let mut stage_ids: Vec<&str> = self
            .stages
            .iter()
            .map(|stage| stage.stage_id.as_str())
            .collect();

        stage_ids.sort_unstable();
        stage_ids.dedup();

        stage_ids.len() == self.stages.len()
    }

    fn has_unique_sequences(&self) -> bool {
        let mut sequences: Vec<u32> = self
            .stages
            .iter()
            .map(|stage| stage.sequence)
            .collect();

        sequences.sort_unstable();
        sequences.dedup();

        sequences.len() == self.stages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn physics_development() -> ArticleLinearDevelopment {
        let metadata = LinearDevelopmentMetadata::new(
            "Fizik",
            "Atom",
            "İnsan bedeninde fiziksel bütünlük",
        );

        let stages = vec![
            DevelopmentStage::new(
                "physics-atom",
                "Atom",
                1,
            )
            .with_description(
                "Fiziksel maddenin temel gelişim durağı.",
            )
            .with_status(
                DevelopmentStageStatus::Verified,
            ),
            DevelopmentStage::new(
                "physics-molecule",
                "Molekül",
                2,
            )
            .with_status(
                DevelopmentStageStatus::Verified,
            ),
            DevelopmentStage::new(
                "physics-organic-structure",
                "Organik yapı",
                3,
            )
            .with_status(
                DevelopmentStageStatus::Verified,
            ),
            DevelopmentStage::new(
                "physics-human-body",
                "İnsan bedeni",
                4,
            )
            .with_status(
                DevelopmentStageStatus::Verified,
            ),
        ];

        ArticleLinearDevelopment::new(
            "physics-official-001",
            metadata,
            SystemTime::now(),
        )
        .with_stages(stages)
        .with_covered_stage_ids(vec![
            "physics-atom".to_string(),
            "physics-molecule".to_string(),
            "physics-organic-structure".to_string(),
            "physics-human-body".to_string(),
        ])
    }

    #[test]
    fn creates_complete_linear_development() {
        let development = physics_development();

        assert!(development.is_complete());
        assert_eq!(
            development.metadata.dimension,
            "Fizik"
        );
        assert_eq!(development.stages.len(), 4);
        assert_eq!(development.verified_stage_count(), 4);
        assert!(development.requires_mudebbir_decision);
    }

    #[test]
    fn stages_are_sorted_by_sequence() {
        let metadata = LinearDevelopmentMetadata::new(
            "Biyoloji",
            "Tek hücreli canlı",
            "İnsan biyolojisi",
        );

        let stages = vec![
            DevelopmentStage::new(
                "human-biology",
                "İnsan biyolojisi",
                3,
            ),
            DevelopmentStage::new(
                "single-cell",
                "Tek hücreli canlı",
                1,
            ),
            DevelopmentStage::new(
                "multi-cell",
                "Çok hücreli canlı",
                2,
            ),
        ];

        let development =
            ArticleLinearDevelopment::new(
                "biology-official-001",
                metadata,
                SystemTime::now(),
            )
            .with_stages(stages);

        assert_eq!(
            development.stages[0].stage_id,
            "single-cell"
        );
        assert_eq!(
            development.stages[1].stage_id,
            "multi-cell"
        );
        assert_eq!(
            development.stages[2].stage_id,
            "human-biology"
        );
    }

    #[test]
    fn duplicate_stage_identifiers_prevent_completeness() {
        let metadata = LinearDevelopmentMetadata::new(
            "Zihin",
            "Algı",
            "Bütünsel idrak",
        );

        let stages = vec![
            DevelopmentStage::new(
                "perception",
                "Algı",
                1,
            ),
            DevelopmentStage::new(
                "perception",
                "Tekrarlanan algı",
                2,
            ),
        ];

        let development =
            ArticleLinearDevelopment::new(
                "mind-official-001",
                metadata,
                SystemTime::now(),
            )
            .with_stages(stages);

        assert!(!development.is_complete());
    }

    #[test]
    fn duplicate_sequences_prevent_completeness() {
        let metadata = LinearDevelopmentMetadata::new(
            "Ahlak",
            "Ahlaki farkındalık",
            "Ahlaki kemal",
        );

        let stages = vec![
            DevelopmentStage::new(
                "awareness",
                "Farkındalık",
                1,
            ),
            DevelopmentStage::new(
                "responsibility",
                "Sorumluluk",
                1,
            ),
        ];

        let development =
            ArticleLinearDevelopment::new(
                "ethics-official-001",
                metadata,
                SystemTime::now(),
            )
            .with_stages(stages);

        assert!(!development.is_complete());
    }

    #[test]
    fn missing_stage_creates_unresolved_gap() {
        let development = physics_development()
            .with_missing_stage_ids(vec![
                "physics-living-system".to_string(),
            ]);

        assert!(development.has_unresolved_gaps());
        assert!(!development.can_support_synthesis());
    }

    #[test]
    fn contradiction_creates_unresolved_gap() {
        let metadata = LinearDevelopmentMetadata::new(
            "Tarih",
            "İlk toplumsal hafıza",
            "Uygarlık bilinci",
        );

        let stages = vec![
            DevelopmentStage::new(
                "historical-memory",
                "Tarihsel hafıza",
                1,
            )
            .with_status(
                DevelopmentStageStatus::Contradicted,
            ),
        ];

        let development =
            ArticleLinearDevelopment::new(
                "history-official-001",
                metadata,
                SystemTime::now(),
            )
            .with_stages(stages);

        assert!(development.has_unresolved_gaps());
        assert!(!development.can_support_synthesis());
    }

    #[test]
    fn rasterast_verified_complete_chain_supports_synthesis() {
        let development =
            physics_development().mark_rasterast_verified();

        assert!(development.rasterast_verified);
        assert!(!development.has_unresolved_gaps());
        assert!(development.can_support_synthesis());
    }

    #[test]
    fn unverified_chain_cannot_support_synthesis() {
        let development = physics_development();

        assert!(development.is_complete());
        assert!(!development.can_support_synthesis());
    }

    #[test]
    fn empty_dimension_prevents_completeness() {
        let metadata = LinearDevelopmentMetadata::new(
            "",
            "Başlangıç",
            "Kemal yönü",
        );

        let development =
            ArticleLinearDevelopment::new(
                "article-001",
                metadata,
                SystemTime::now(),
            )
            .with_stages(vec![
                DevelopmentStage::new(
                    "stage-001",
                    "Birinci durak",
                    1,
                ),
            ]);

        assert!(!development.is_complete());
    }
}




