use uuid::Uuid;

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
    /// Boş bir repository ilişki grafı oluşturur.
    pub fn new() -> Self {
        Self::default()
    }

    /// Graf içindeki toplam ilişki sayısını döndürür.
    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }

    /// Grafın boş olup olmadığını bildirir.
    pub fn is_empty(&self) -> bool {
        self.relations.is_empty()
    }

    /// Bütün ilişkileri salt okunur biçimde döndürür.
    pub fn relations(&self) -> &[RepositoryRelation] {
        &self.relations
    }

    /// Eksiksiz ve daha önce kaydedilmemiş bir
    /// repository ilişkisini grafa ekler.
    pub fn add_relation(
        &mut self,
        relation: RepositoryRelation,
    ) -> bool {
        if !relation.is_complete() {
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

    /// Proje hafızasındaki metinlerde başka repository
    /// adlarının geçmesini kanıt olarak kullanarak
    /// repository ilişkileri çıkarır.
    pub fn infer_from_memory(
        &mut self,
        memory: &RepositoryMemory,
    ) -> usize {
        let mut repositories: Vec<(Uuid, String)> =
            Vec::new();

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
                    target_repository_name
                        .trim()
                        .to_lowercase();

                if normalized_target_name.is_empty() {
                    continue;
                }

                let source_line = source_document
                    .text
                    .content
                    .lines()
                    .position(|line| {
                        line.to_lowercase().contains(
                            &normalized_target_name,
                        )
                    })
                    .map(|index| index + 1);

                let Some(source_line) = source_line else {
                    continue;
                };

                let evidence = format!(
                    "{}:{} references repository {}",
                    source_document.repository_name,
                    source_document
                        .text
                        .relative_path
                        .display(),
                    target_repository_name,
                );

                let relation = RepositoryRelation::new(
                    source_document.repository_id,
                    *target_repository_id,
                    RepositoryRelationKind::References,
                    source_line,
                    evidence,
                );

                if self.add_relation(relation) {
                    added_count += 1;
                }
            }
        }

        added_count
    }

    /// Belirtilen repository’den çıkan ilişkileri döndürür.
    pub fn relations_from(
        &self,
        source_repository: Uuid,
    ) -> Vec<&RepositoryRelation> {
        self.relations
            .iter()
            .filter(|relation| {
                relation.originates_from(
                    source_repository,
                )
            })
            .collect()
    }

    /// Belirtilen repository’ye yönelen ilişkileri döndürür.
    pub fn relations_to(
        &self,
        target_repository: Uuid,
    ) -> Vec<&RepositoryRelation> {
        self.relations
            .iter()
            .filter(|relation| {
                relation.targets(target_repository)
            })
            .collect()
    }

    /// Belirtilen türdeki repository ilişkilerini döndürür.
    pub fn relations_of_kind(
        &self,
        kind: RepositoryRelationKind,
    ) -> Vec<&RepositoryRelation> {
        self.relations
            .iter()
            .filter(|relation| relation.is_kind(kind))
            .collect()
    }

    /// Kaynak, hedef ve ilişki türüne göre tek bir
    /// repository ilişkisi bulur.
    pub fn find_relation(
        &self,
        source_repository: Uuid,
        target_repository: Uuid,
        kind: RepositoryRelationKind,
    ) -> Option<&RepositoryRelation> {
        self.relations.iter().find(|relation| {
            relation.originates_from(
                source_repository,
            ) && relation.targets(target_repository)
                && relation.is_kind(kind)
        })
    }

    /// Kaynak, hedef ve tür bilgileriyle eşleşen
    /// bir ilişkinin bulunup bulunmadığını bildirir.
    pub fn has_relation(
        &self,
        source_repository: Uuid,
        target_repository: Uuid,
        kind: RepositoryRelationKind,
    ) -> bool {
        self.find_relation(
            source_repository,
            target_repository,
            kind,
        )
        .is_some()
    }

    /// Kanıt metninde büyük-küçük harf duyarsız
    /// arama yapar.
    ///
    /// Boş sorgular sonuç üretmez.
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use crate::repository_memory::{
    RepositoryMemoryDocument,
    RepositoryTextContent,
};

    #[test]
    fn graph_adds_complete_repository_relation() {
        let source_repository = Uuid::new_v4();
        let target_repository = Uuid::new_v4();

        let relation = RepositoryRelation::new(
            source_repository,
            target_repository,
            RepositoryRelationKind::References,
            4,
            "README.md references target repository.",
        );

        let mut graph = RepositoryGraph::new();

        assert!(graph.add_relation(relation));
        assert_eq!(graph.relation_count(), 1);
        assert!(!graph.is_empty());
        assert_eq!(graph.relations().len(), 1);
    }

    #[test]
    fn graph_rejects_incomplete_repository_relation() {
        let relation = RepositoryRelation::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            RepositoryRelationKind::DependsOn,
            0,
            "Missing source line.",
        );

        let mut graph = RepositoryGraph::new();

        assert!(!graph.add_relation(relation));
        assert!(graph.is_empty());
    }

    #[test]
    fn graph_rejects_duplicate_repository_relation() {
        let source_repository = Uuid::new_v4();
        let target_repository = Uuid::new_v4();

        let first = RepositoryRelation::new(
            source_repository,
            target_repository,
            RepositoryRelationKind::References,
            3,
            "First evidence.",
        );

        let duplicate = RepositoryRelation::new(
            source_repository,
            target_repository,
            RepositoryRelationKind::References,
            8,
            "Different evidence for the same relation.",
        );

        let mut graph = RepositoryGraph::new();

        assert!(graph.add_relation(first));
        assert!(!graph.add_relation(duplicate));
        assert_eq!(graph.relation_count(), 1);
    }

    #[test]
    fn graph_infers_repository_reference_from_memory() {
        let source_repository = Uuid::new_v4();
        let target_repository = Uuid::new_v4();

        let memory = RepositoryMemory {
            documents: vec![
                RepositoryMemoryDocument {
                    repository_id: source_repository,
                    repository_name:
                        "zanistarast-papers".to_string(),
                    text: RepositoryTextContent {
                        relative_path:
                            PathBuf::from("README.md"),
                        content: concat!(
                            "Project overview.\n",
                            "This repository uses ",
                            "zanistarast-ontology."
                        )
                        .to_string(),
                        line_count: 2,
                        character_count: 59,
                    },
                },
                RepositoryMemoryDocument {
                    repository_id: target_repository,
                    repository_name:
                        "zanistarast-ontology"
                            .to_string(),
                    text: RepositoryTextContent {
                        relative_path:
                            PathBuf::from("README.md"),
                        content:
                            "Ontology definitions."
                                .to_string(),
                        line_count: 1,
                        character_count: 21,
                    },
                },
            ],
        };

        let mut graph = RepositoryGraph::new();

        assert_eq!(
            graph.infer_from_memory(&memory),
            1,
        );

        assert_eq!(graph.relation_count(), 1);

        let relation = graph
            .find_relation(
                source_repository,
                target_repository,
                RepositoryRelationKind::References,
            )
            .expect("inferred relation should exist");

        assert_eq!(relation.source_line, 2);

        assert!(
            relation
                .evidence
                .contains("README.md"),
        );

        assert_eq!(
            graph.infer_from_memory(&memory),
            0,
        );

        assert_eq!(graph.relation_count(), 1);
    }

    #[test]
    fn graph_relations_can_be_queried() {
        let first_repository = Uuid::new_v4();
        let second_repository = Uuid::new_v4();
        let third_repository = Uuid::new_v4();

        let mut graph = RepositoryGraph::new();

        assert!(graph.add_relation(
            RepositoryRelation::new(
                first_repository,
                second_repository,
                RepositoryRelationKind::References,
                5,
                "README.md references second repository.",
            ),
        ));

        assert!(graph.add_relation(
            RepositoryRelation::new(
                first_repository,
                third_repository,
                RepositoryRelationKind::DependsOn,
                7,
                "Cargo.toml depends on third repository.",
            ),
        ));

        assert_eq!(
            graph.relations_from(first_repository).len(),
            2,
        );

        assert_eq!(
            graph.relations_to(second_repository).len(),
            1,
        );

        assert_eq!(
            graph
                .relations_of_kind(
                    RepositoryRelationKind::References,
                )
                .len(),
            1,
        );

        assert!(graph.has_relation(
            first_repository,
            third_repository,
            RepositoryRelationKind::DependsOn,
        ));

        assert!(!graph.has_relation(
            second_repository,
            first_repository,
            RepositoryRelationKind::Extends,
        ));
    }

    #[test]
    fn graph_searches_relation_evidence() {
        let mut graph = RepositoryGraph::new();

        assert!(graph.add_relation(
            RepositoryRelation::new(
                Uuid::new_v4(),
                Uuid::new_v4(),
                RepositoryRelationKind::References,
                11,
                "README.md references Rasterast.",
            ),
        ));

        assert_eq!(
            graph
                .relations_with_evidence("RASTERAST")
                .len(),
            1,
        );

        assert!(
            graph
                .relations_with_evidence(" ")
                .is_empty(),
        );
    }
}

