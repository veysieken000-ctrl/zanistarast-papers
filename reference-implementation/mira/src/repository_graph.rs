use crate::repository_memory::RepositoryMemory;
use crate::repository_relation::{
    RepositoryRelation,
    RepositoryRelationKind,
};

/// Depolar arasında belirlenen ilişkilerin ortak grafıdır.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RepositoryGraph {
    pub relations: Vec<RepositoryRelation>,
}

impl RepositoryGraph {
    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }

    pub fn is_empty(&self) -> bool {
        self.relations.is_empty()
    }

    /// Eksiksiz ve daha önce kaydedilmemiş bir
    /// depo ilişkisini grafa ekler.
    pub fn add_relation(
        &mut self,
        relation: RepositoryRelation,
    ) -> bool {
        if relation.source_repository
            == relation.target_repository
            || relation.evidence.trim().is_empty()
        {
            return false;
        }

        if self.relations.iter().any(|stored| {
            stored.source_repository
                == relation.source_repository
                && stored.target_repository
                    == relation.target_repository
                && stored.kind == relation.kind
        }) {
            return false;
        }

        self.relations.push(relation);
        true
    }

    /// Proje hafızasındaki metinlerde başka depo adlarının
    /// geçmesini kanıt olarak kullanarak ilişkiler çıkarır.
    pub fn infer_from_memory(
        &mut self,
        memory: &RepositoryMemory,
    ) -> usize {
        let mut repositories:
            Vec<(uuid::Uuid, String)> = Vec::new();

        for document in memory.iter() {
            if !repositories.iter().any(
                |(repository_id, _)| {
                    *repository_id == document.repository_id
                },
            ) {
                repositories.push((
                    document.repository_id,
                    document.repository_name.clone(),
                ));
            }
        }

        let mut added_count = 0;

        for source_document in memory.iter() {
            let content =
                source_document.text.content.to_lowercase();

            for (
                target_repository_id,
                target_repository_name,
            ) in &repositories
            {
                if source_document.repository_id
                    == *target_repository_id
                {
                    continue;
                }

                let normalized_target_name =
                    target_repository_name.to_lowercase();

                if normalized_target_name.is_empty()
                    || !content.contains(
                        &normalized_target_name,
                    )
                {
                    continue;
                }

                let source_line = source_document
                    .text
                    .content
                    .lines()
                    .position(|line| {
                        line.to_lowercase()
                            .contains(&normalized_target_name)
                    })
                    .map(|index| index + 1)
                    .unwrap_or(0);

                let evidence = format!(
                    "{}:{} references repository {}",
                    source_document.repository_name,
                    source_document
                        .text
                        .relative_path
                        .display(),
                    target_repository_name,
                );

                if self.add_relation(
                    RepositoryRelation {
                        source_repository:
                            source_document.repository_id,
                        target_repository:
                            *target_repository_id,
                        kind:
                            RepositoryRelationKind::References,
                        source_line,
                        evidence,
                    },
                ) {
                    added_count += 1;
                }
            }
        }

        added_count
    }

    /// Belirtilen depodan çıkan ilişkileri döndürür.
    pub fn relations_from(
        &self,
        source_repository: uuid::Uuid,
    ) -> Vec<&RepositoryRelation> {
        self.relations
            .iter()
            .filter(|relation| {
                relation.source_repository
                    == source_repository
            })
            .collect()
    }

    /// Belirtilen depoya yönelen ilişkileri döndürür.
    pub fn relations_to(
        &self,
        target_repository: uuid::Uuid,
    ) -> Vec<&RepositoryRelation> {
        self.relations
            .iter()
            .filter(|relation| {
                relation.target_repository
                    == target_repository
            })
            .collect()
    }

    /// Belirtilen türdeki depo ilişkilerini döndürür.
    pub fn relations_of_kind(
        &self,
        kind: RepositoryRelationKind,
    ) -> Vec<&RepositoryRelation> {
        self.relations
            .iter()
            .filter(|relation| relation.kind == kind)
            .collect()
    }

    /// Kaynak, hedef ve ilişki türüne göre tek bir
    /// depo ilişkisi bulur.
    pub fn find_relation(
        &self,
        source_repository: uuid::Uuid,
        target_repository: uuid::Uuid,
        kind: RepositoryRelationKind,
    ) -> Option<&RepositoryRelation> {
        self.relations.iter().find(|relation| {
            relation.source_repository
                == source_repository
                && relation.target_repository
                    == target_repository
                && relation.kind == kind
        })
    }

    pub fn has_relation(
        &self,
        source_repository: uuid::Uuid,
        target_repository: uuid::Uuid,
        kind: RepositoryRelationKind,
    ) -> bool {
        self.find_relation(
            source_repository,
            target_repository,
            kind,
        )
        .is_some()
    }

    pub fn relations_with_evidence(
        &self,
        query: &str,
    ) -> Vec<&RepositoryRelation> {
        let query = query.trim().to_lowercase();

        if query.is_empty() {
            return Vec::new();
        }

        self.relations
            .iter()
            .filter(|relation| {
                relation
                    .evidence
                    .to_lowercase()
                    .contains(&query)
            })
            .collect()
    }
}



