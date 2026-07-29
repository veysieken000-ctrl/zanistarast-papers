use std::time::SystemTime;

/// İki makale veya yazı arasındaki akademik ve tarihsel ilişkiyi
/// tanımlar.
///
/// Bu ilişkiler yalnızca başlık benzerliğine göre değil;
/// içerik kapsamı, bilgi haritası, doğrusal gelişim durakları
/// ve dairesel boyut ilişkileri dikkate alınarak belirlenmelidir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArticleRelationType {
    /// Yeni resmî makale eski içeriğin aktif yerini tamamen alır.
    Replaces,

    /// Yeni resmî makale birden fazla eski içeriği tek bir
    /// akademik çerçevede birleştirir.
    Merges,

    /// Yeni makale eski içeriğin yalnızca belirli bir bölümünü kapsar.
    PartiallyCovers,

    /// Yeni makale eski içeriği ortadan kaldırmadan geliştirir
    /// veya kapsamını genişletir.
    Extends,

    /// Bir makale diğer makaledeki sav, delil veya modeli destekler.
    Supports,

    /// İki çalışma arasında doğrulanması gereken bir çelişki vardır.
    Contradicts,

    /// Eski makalenin yerine daha güncel veya resmî bir makale geçmiştir.
    SupersededBy,
}

/// İlişkinin hangi doğruluk ve güven seviyesinde belirlendiğini gösterir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArticleRelationConfidence {
    Low,
    Medium,
    High,
    Verified,
}

/// Bir kaynak makale ile hedef makale arasındaki ilişki kaydıdır.
///
/// Örnek:
///
/// kaynak: resmî Duygu–Akıl makalesi
/// hedef: eski “Duygular ve Akıl” yazısı
/// ilişki: Replaces
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArticleRelation {
    pub source_article_id: String,
    pub target_article_id: String,
    pub relation_type: ArticleRelationType,
    pub rationale: String,
    pub covered_topics: Vec<String>,
    pub confidence: ArticleRelationConfidence,
    pub rasterast_verified: bool,
    pub requires_mudebbir_approval: bool,
    pub created_at: SystemTime,
}

impl ArticleRelation {
    pub fn new(
        source_article_id: impl Into<String>,
        target_article_id: impl Into<String>,
        relation_type: ArticleRelationType,
        rationale: impl Into<String>,
        created_at: SystemTime,
    ) -> Self {
        Self {
            source_article_id: source_article_id.into(),
            target_article_id: target_article_id.into(),
            relation_type,
            rationale: rationale.into(),
            covered_topics: Vec::new(),
            confidence: ArticleRelationConfidence::Low,
            rasterast_verified: false,
            requires_mudebbir_approval: true,
            created_at,
        }
    }

    pub fn with_covered_topics(
        mut self,
        covered_topics: Vec<String>,
    ) -> Self {
        self.covered_topics = covered_topics;
        self
    }

    pub fn with_confidence(
        mut self,
        confidence: ArticleRelationConfidence,
    ) -> Self {
        self.confidence = confidence;
        self
    }

    pub fn mark_rasterast_verified(mut self) -> Self {
        self.rasterast_verified = true;
        self.confidence = ArticleRelationConfidence::Verified;
        self
    }

    pub fn is_complete(&self) -> bool {
        !self.source_article_id.trim().is_empty()
            && !self.target_article_id.trim().is_empty()
            && self.source_article_id != self.target_article_id
            && !self.rationale.trim().is_empty()
    }

    pub fn can_propose_archival(&self) -> bool {
        self.is_complete()
            && self.rasterast_verified
            && matches!(
                self.relation_type,
                ArticleRelationType::Replaces
                    | ArticleRelationType::Merges
                    | ArticleRelationType::SupersededBy
            )
    }

    pub fn should_preserve_as_active_content(&self) -> bool {
        matches!(
            self.relation_type,
            ArticleRelationType::PartiallyCovers
                | ArticleRelationType::Extends
                | ArticleRelationType::Supports
                | ArticleRelationType::Contradicts
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn complete_relation(
        relation_type: ArticleRelationType,
    ) -> ArticleRelation {
        ArticleRelation::new(
            "official-article-001",
            "legacy-article-001",
            relation_type,
            "The official article fully covers the legacy article.",
            SystemTime::now(),
        )
        .with_covered_topics(vec![
            "Zanistarast".to_string(),
            "Newroza Kawa".to_string(),
        ])
        .with_confidence(ArticleRelationConfidence::High)
    }

    #[test]
    fn creates_complete_relation() {
        let relation =
            complete_relation(ArticleRelationType::Replaces);

        assert!(relation.is_complete());
        assert_eq!(
            relation.source_article_id,
            "official-article-001"
        );
        assert_eq!(
            relation.target_article_id,
            "legacy-article-001"
        );
        assert_eq!(
            relation.confidence,
            ArticleRelationConfidence::High
        );
        assert!(relation.requires_mudebbir_approval);
        assert!(!relation.rasterast_verified);
    }

    #[test]
    fn relation_cannot_target_itself() {
        let relation = ArticleRelation::new(
            "article-001",
            "article-001",
            ArticleRelationType::Replaces,
            "Invalid self relation.",
            SystemTime::now(),
        );

        assert!(!relation.is_complete());
    }

    #[test]
    fn verified_replacement_can_propose_archival() {
        let relation =
            complete_relation(ArticleRelationType::Replaces)
                .mark_rasterast_verified();

        assert!(relation.rasterast_verified);
        assert_eq!(
            relation.confidence,
            ArticleRelationConfidence::Verified
        );
        assert!(relation.can_propose_archival());
    }

    #[test]
    fn verified_merge_can_propose_archival() {
        let relation =
            complete_relation(ArticleRelationType::Merges)
                .mark_rasterast_verified();

        assert!(relation.can_propose_archival());
    }

    #[test]
    fn partial_coverage_does_not_propose_archival() {
        let relation =
            complete_relation(
                ArticleRelationType::PartiallyCovers,
            )
            .mark_rasterast_verified();

        assert!(!relation.can_propose_archival());
        assert!(relation.should_preserve_as_active_content());
    }

    #[test]
    fn extension_preserves_existing_content() {
        let relation =
            complete_relation(ArticleRelationType::Extends);

        assert!(relation.should_preserve_as_active_content());
        assert!(!relation.can_propose_archival());
    }

    #[test]
    fn contradiction_requires_preserving_both_records() {
        let relation =
            complete_relation(ArticleRelationType::Contradicts);

        assert!(relation.should_preserve_as_active_content());
        assert!(!relation.can_propose_archival());
    }

    #[test]
    fn empty_rationale_prevents_completeness() {
        let relation = ArticleRelation::new(
            "official-article-001",
            "legacy-article-001",
            ArticleRelationType::Replaces,
            "",
            SystemTime::now(),
        );

        assert!(!relation.is_complete());
    }

    #[test]
    fn rasterast_verification_sets_verified_confidence() {
        let relation =
            complete_relation(ArticleRelationType::Supports)
                .mark_rasterast_verified();

        assert!(relation.rasterast_verified);
        assert_eq!(
            relation.confidence,
            ArticleRelationConfidence::Verified
        );
    }
}


