use uuid::Uuid;

/// Depolar arasında belirlenen ilişkinin türü.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RepositoryRelationKind {
    References,
    DependsOn,
    Extends,
    SharesConcepts,
}

/// İki depo arasında belirlenen tek bir ilişkinin kaydı.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryRelation {
    pub source_repository: Uuid,
    pub target_repository: Uuid,
    pub kind: RepositoryRelationKind,
    pub source_line: usize,
    pub evidence: String,
}

impl RepositoryRelation {
    /// İki depo arasında yeni bir ilişki kaydı oluşturur.
    pub fn new(
        source_repository: Uuid,
        target_repository: Uuid,
        kind: RepositoryRelationKind,
        source_line: usize,
        evidence: impl Into<String>,
    ) -> Self {
        Self {
            source_repository,
            target_repository,
            kind,
            source_line,
            evidence: evidence.into(),
        }
    }

    /// İlişki kaydının zorunlu alanlarının eksiksiz
    /// ve geçerli olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        self.source_repository != self.target_repository
            && self.source_line > 0
            && !self.evidence.trim().is_empty()
    }

    /// İlişkinin belirtilen depodan başlayıp
    /// başlamadığını bildirir.
    pub fn originates_from(
        &self,
        repository_id: Uuid,
    ) -> bool {
        self.source_repository == repository_id
    }

    /// İlişkinin belirtilen depoyu hedefleyip
    /// hedeflemediğini bildirir.
    pub fn targets(
        &self,
        repository_id: Uuid,
    ) -> bool {
        self.target_repository == repository_id
    }

    /// İlişkinin belirtilen türde olup
    /// olmadığını bildirir.
    pub fn is_kind(
        &self,
        kind: RepositoryRelationKind,
    ) -> bool {
        self.kind == kind
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_complete_repository_relation() {
        let source_repository = Uuid::new_v4();
        let target_repository = Uuid::new_v4();

        let relation = RepositoryRelation::new(
            source_repository,
            target_repository,
            RepositoryRelationKind::References,
            42,
            "See the referenced repository.",
        );

        assert!(relation.is_complete());

        assert!(
            relation.originates_from(source_repository),
        );

        assert!(
            relation.targets(target_repository),
        );

        assert!(
            relation.is_kind(
                RepositoryRelationKind::References,
            ),
        );
    }

    #[test]
    fn rejects_relation_to_same_repository() {
        let repository_id = Uuid::new_v4();

        let relation = RepositoryRelation::new(
            repository_id,
            repository_id,
            RepositoryRelationKind::DependsOn,
            12,
            "Self dependency.",
        );

        assert!(!relation.is_complete());
    }

    #[test]
    fn rejects_relation_without_source_line() {
        let relation = RepositoryRelation::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            RepositoryRelationKind::Extends,
            0,
            "Extension evidence.",
        );

        assert!(!relation.is_complete());
    }

    #[test]
    fn rejects_relation_without_evidence() {
        let relation = RepositoryRelation::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            RepositoryRelationKind::SharesConcepts,
            8,
            " ",
        );

        assert!(!relation.is_complete());
    }
}

