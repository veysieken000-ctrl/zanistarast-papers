use crate::article_candidate_analysis::ArticleMaturityLevel;
use crate::article_inventory::ZanistarastDomain;
use crate::topic_clustering::{
    ClusteredArticle,
    TopicClusteringReport,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Bilgi haritasındaki tek bir bilimsel içerik düğümü.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnowledgeNode {
    pub id: String,
    pub relative_path: PathBuf,
    pub title: Option<String>,
    pub readiness_score: u8,
    pub maturity_level: ArticleMaturityLevel,
}

/// Aynı bilgi alanındaki iki düğüm arasındaki ilişki.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnowledgeRelation {
    pub source_id: String,
    pub target_id: String,
    pub relation_type: KnowledgeRelationType,
}

/// Bilgi haritasında kullanılabilecek ilişki türleri.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KnowledgeRelationType {
    SameDomain,
    PossibleContinuation,
    RequiresReview,
}

/// Tek bir Zanistarast alanına ait bilgi haritası.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DomainKnowledgeMap {
    pub domain: ZanistarastDomain,
    pub nodes: Vec<KnowledgeNode>,
    pub relations: Vec<KnowledgeRelation>,
}

impl DomainKnowledgeMap {
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn relation_count(&self) -> usize {
        self.relations.len()
    }
}

/// Tüm Zanistarast bilgi haritalarının birleşik raporu.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct KnowledgeMapReport {
    pub maps: Vec<DomainKnowledgeMap>,
}

impl KnowledgeMapReport {
    pub fn map_count(&self) -> usize {
        self.maps.len()
    }

    pub fn map_for_domain(
        &self,
        domain: &ZanistarastDomain,
    ) -> Option<&DomainKnowledgeMap> {
        self.maps
            .iter()
            .find(|map| &map.domain == domain)
    }
}

/// Konu kümelerinden salt okunur bilgi haritaları üretir.
///
/// Bu işlem:
/// - orijinal metinleri değiştirmez,
/// - dosya taşımaz,
/// - dosya silmez,
/// - yalnızca mevcut kümeleri düğüm ve ilişkilere dönüştürür.
#[derive(Debug, Default)]
pub struct KnowledgeMapBuilder;

impl KnowledgeMapBuilder {
    pub fn new() -> Self {
        Self
    }

    pub fn build(
        &self,
        clustering_report: &TopicClusteringReport,
    ) -> KnowledgeMapReport {
        let mut maps = clustering_report
            .clusters
            .iter()
            .filter(|cluster| {
                cluster.domain != ZanistarastDomain::Unclassified
            })
            .map(|cluster| {
                let nodes = cluster
                    .articles
                    .iter()
                    .map(Self::node_from_article)
                    .collect::<Vec<_>>();

                let relations = Self::build_relations(&nodes);

                DomainKnowledgeMap {
                    domain: cluster.domain.clone(),
                    nodes,
                    relations,
                }
            })
            .collect::<Vec<_>>();

        maps.sort_by(|left, right| left.domain.cmp(&right.domain));

        KnowledgeMapReport { maps }
    }

    fn node_from_article(
        article: &ClusteredArticle,
    ) -> KnowledgeNode {
        KnowledgeNode {
            id: Self::node_id(&article.relative_path),
            relative_path: article.relative_path.clone(),
            title: article.title.clone(),
            readiness_score: article.readiness_score,
            maturity_level: article.maturity_level.clone(),
        }
    }

    fn node_id(path: &std::path::Path) -> String {
        path.to_string_lossy()
            .chars()
            .map(|character| {
                if character.is_alphanumeric() {
                    character.to_ascii_lowercase()
                } else {
                    '-'
                }
            })
            .collect::<String>()
            .split('-')
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }

    fn build_relations(
        nodes: &[KnowledgeNode],
    ) -> Vec<KnowledgeRelation> {
        let mut relations = Vec::new();

        for (index, source) in nodes.iter().enumerate() {
            for target in nodes.iter().skip(index + 1) {
                relations.push(KnowledgeRelation {
                    source_id: source.id.clone(),
                    target_id: target.id.clone(),
                    relation_type: Self::relation_type(
                        source,
                        target,
                    ),
                });
            }
        }

        relations
    }

    fn relation_type(
        source: &KnowledgeNode,
        target: &KnowledgeNode,
    ) -> KnowledgeRelationType {
        if source.maturity_level == ArticleMaturityLevel::Fragment
            || target.maturity_level
                == ArticleMaturityLevel::Fragment
        {
            return KnowledgeRelationType::RequiresReview;
        }

        let score_difference = source
            .readiness_score
            .abs_diff(target.readiness_score);

        if score_difference <= 15 {
            KnowledgeRelationType::PossibleContinuation
        } else {
            KnowledgeRelationType::SameDomain
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::topic_clustering::{
        TopicCluster,
        TopicClusteringReport,
    };

    #[test]
    fn builder_creates_domain_knowledge_maps() {
        let clustering_report = TopicClusteringReport {
            clusters: vec![
                TopicCluster {
                    domain: ZanistarastDomain::Hebun,
                    articles: vec![
                        ClusteredArticle {
                            relative_path: PathBuf::from(
                                "papers/hebun-main.md",
                            ),
                            title: Some(
                                "Hebûn Ana Makalesi".to_string(),
                            ),
                            readiness_score: 90,
                            maturity_level:
                                ArticleMaturityLevel::StrongCandidate,
                        },
                        ClusteredArticle {
                            relative_path: PathBuf::from(
                                "papers/hebun-method.md",
                            ),
                            title: Some(
                                "Hebûn Yöntemi".to_string(),
                            ),
                            readiness_score: 80,
                            maturity_level:
                                ArticleMaturityLevel::DevelopingDraft,
                        },
                    ],
                    total_readiness_score: 170,
                },
                TopicCluster {
                    domain: ZanistarastDomain::Rabun,
                    articles: vec![
                        ClusteredArticle {
                            relative_path: PathBuf::from(
                                "papers/rabun.md",
                            ),
                            title: Some(
                                "Rabûn Yönetim Modeli".to_string(),
                            ),
                            readiness_score: 75,
                            maturity_level:
                                ArticleMaturityLevel::DevelopingDraft,
                        },
                    ],
                    total_readiness_score: 75,
                },
            ],
            total_clustered_articles: 3,
        };

        let builder = KnowledgeMapBuilder::new();
        let report = builder.build(&clustering_report);

        assert_eq!(report.map_count(), 2);

        let hebun_map = report
            .map_for_domain(&ZanistarastDomain::Hebun)
            .expect("Hebûn knowledge map should exist");

        assert_eq!(hebun_map.node_count(), 2);
        assert_eq!(hebun_map.relation_count(), 1);
        assert_eq!(
            hebun_map.relations[0].relation_type,
            KnowledgeRelationType::PossibleContinuation
        );
    }

    #[test]
    fn unclassified_cluster_is_not_mapped() {
        let clustering_report = TopicClusteringReport {
            clusters: vec![TopicCluster {
                domain: ZanistarastDomain::Unclassified,
                articles: vec![ClusteredArticle {
                    relative_path: PathBuf::from(
                        "notes/general.md",
                    ),
                    title: None,
                    readiness_score: 10,
                    maturity_level:
                        ArticleMaturityLevel::Fragment,
                }],
                total_readiness_score: 10,
            }],
            total_clustered_articles: 1,
        };

        let builder = KnowledgeMapBuilder::new();
        let report = builder.build(&clustering_report);

        assert_eq!(report.map_count(), 0);
    }

    #[test]
    fn fragment_relation_requires_review() {
        let nodes = vec![
            KnowledgeNode {
                id: "hebun-main".to_string(),
                relative_path: PathBuf::from(
                    "papers/hebun-main.md",
                ),
                title: Some("Hebûn".to_string()),
                readiness_score: 90,
                maturity_level:
                    ArticleMaturityLevel::StrongCandidate,
            },
            KnowledgeNode {
                id: "hebun-note".to_string(),
                relative_path: PathBuf::from(
                    "notes/hebun-note.md",
                ),
                title: None,
                readiness_score: 15,
                maturity_level:
                    ArticleMaturityLevel::Fragment,
            },
        ];

        let relations =
            KnowledgeMapBuilder::build_relations(&nodes);

        assert_eq!(relations.len(), 1);
        assert_eq!(
            relations[0].relation_type,
            KnowledgeRelationType::RequiresReview
        );
    }
}





