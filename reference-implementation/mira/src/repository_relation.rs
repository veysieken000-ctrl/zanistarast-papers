/// Depolar arasında belirlenen ilişkinin türü.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryRelationKind {
    References,
    DependsOn,
    Extends,
    SharesConcepts,
}

/// İki depo arasında belirlenen tek bir ilişkinin kaydı.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryRelation {
    pub source_repository: uuid::Uuid,
    pub target_repository: uuid::Uuid,
    pub kind: RepositoryRelationKind,
    pub source_line: usize,
    pub evidence: String,
}

