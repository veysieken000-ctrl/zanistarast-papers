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

/// Bir bilgi düğümünün Zanistarast bilgi mimarisindeki
/// işlevsel katmanını belirtir.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub enum ZanistarastKnowledgeLayer {
    /// Değişmez çekirdek ilkeler, kavramlar,
    /// aksiyomlar, epistemik hükümler ve resmî kararlar.
    Dna,

    /// Görevler, süreçler, dönüşüm kuralları
    /// ve bilgi aktarım mekanizmaları.
    Rna,

    /// Makaleler, kod modülleri, raporlar,
    /// yayın paketleri ve diğer somut çıktılar.
    Protein,
}

/// Zanistarast DNA katmanındaki değişmez veya
/// otoriteye bağlı çekirdek bilgi türlerini belirtir.
#[derive(
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub enum ZanistarastDnaKind {
    /// Zanistarast’ın değişmez temel ilkesi.
    CorePrinciple,

    /// Tanımlanmış temel veya türetilmiş kavram.
    Concept,

    /// Biçimsel veya kurucu aksiyom.
    Axiom,

    /// Bilginin doğruluk ve kesinlik statüsüne
    /// ilişkin epistemik hüküm.
    EpistemicJudgment,

    /// Müdebbir tarafından verilmiş resmî karar.
    OfficialDecision,
}

/// Bir bilgi düğümünün Zanistarast DNA katmanındaki
/// ayrıntılı ve gerekçeli kaydıdır.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct DnaKnowledgeRecord {
    pub node_id: String,
    pub kind: ZanistarastDnaKind,
    pub rationale: String,
    pub immutable: bool,
}

impl DnaKnowledgeRecord {
    /// Yeni bir DNA bilgi kaydı oluşturur.
    pub fn new(
        node_id: impl Into<String>,
        kind: ZanistarastDnaKind,
        rationale: impl Into<String>,
        immutable: bool,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            kind,
            rationale: rationale.into(),
            immutable,
        }
    }

    /// DNA kaydının zorunlu bilgilerinin eksiksiz
    /// ve DNA ilkeleriyle uyumlu olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.node_id.trim().is_empty()
            && !self.rationale.trim().is_empty()
            && self.immutable
    }

    /// DNA kaydının belirtilen türde olup
    /// olmadığını bildirir.
    pub fn is_kind(
        &self,
        kind: ZanistarastDnaKind,
    ) -> bool {
        self.kind == kind
    }
}

/// Zanistarast DNA katmanındaki ayrıntılı
/// bilgi kayıtlarının koleksiyonudur.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct DnaKnowledgeMap {
    pub records: Vec<DnaKnowledgeRecord>,
}

impl DnaKnowledgeMap {
    /// Boş bir DNA bilgi haritası oluşturur.
    pub fn new() -> Self {
        Self::default()
    }

    /// Eksiksiz ve aynı düğüm için daha önce
    /// kaydedilmemiş bir DNA kaydı ekler.
    pub fn register(
        &mut self,
        record: DnaKnowledgeRecord,
    ) -> bool {
        if !record.is_complete() {
            return false;
        }

        if self.records.iter().any(|stored| {
            stored.node_id == record.node_id
        }) {
            return false;
        }

        self.records.push(record);
        true
    }

    /// Toplam DNA bilgi kaydı sayısını döndürür.
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    /// DNA bilgi haritasının boş olup
    /// olmadığını bildirir.
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }

    /// Belirtilen düğüme ait DNA kaydını döndürür.
    pub fn record_for_node(
        &self,
        node_id: &str,
    ) -> Option<&DnaKnowledgeRecord> {
        self.records.iter().find(|record| {
            record.node_id == node_id
        })
    }

    /// Belirtilen DNA türüne ait bütün
    /// kayıtları döndürür.
    pub fn records_of_kind(
        &self,
        kind: ZanistarastDnaKind,
    ) -> Vec<&DnaKnowledgeRecord> {
        self.records
            .iter()
            .filter(|record| record.is_kind(kind))
            .collect()
    }

    /// Bir düğümün DNA katmanında kayıtlı olup
    /// olmadığını bildirir.
    pub fn contains_node(
        &self,
        node_id: &str,
    ) -> bool {
        self.record_for_node(node_id).is_some()
    }
}

/// Mevcut bir bilgi düğümünün DNA–RNA–Protein
/// mimarisindeki katman atamasını temsil eder.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]

pub struct KnowledgeLayerAssignment {
    pub node_id: String,
    pub layer: ZanistarastKnowledgeLayer,
    pub rationale: String,
}

impl KnowledgeLayerAssignment {
    /// Yeni ve gerekçeli bir katman ataması oluşturur.
    pub fn new(
        node_id: impl Into<String>,
        layer: ZanistarastKnowledgeLayer,
        rationale: impl Into<String>,
    ) -> Self {
        Self {
            node_id: node_id.into(),
            layer,
            rationale: rationale.into(),
        }
    }

    /// Katman atamasının zorunlu bilgilerinin
    /// eksiksiz olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.node_id.trim().is_empty()
            && !self.rationale.trim().is_empty()
    }
}

/// Mevcut bilgi haritası düğümlerinin Zanistarast
/// DNA–RNA–Protein katman atamalarını taşır.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct KnowledgeLayerMap {
    pub assignments: Vec<KnowledgeLayerAssignment>,
}

impl KnowledgeLayerMap {
    /// Boş bir katman haritası oluşturur.
    pub fn new() -> Self {
        Self::default()
    }

    /// Eksiksiz ve daha önce aynı düğüm için
    /// kaydedilmemiş bir katman ataması ekler.
    pub fn assign(
        &mut self,
        assignment: KnowledgeLayerAssignment,
    ) -> bool {
        if !assignment.is_complete() {
            return false;
        }

        if self.assignments.iter().any(|stored| {
            stored.node_id == assignment.node_id
        }) {
            return false;
        }

        self.assignments.push(assignment);
        true
    }

    /// Belirtilen düğümün katman atamasını döndürür.
    pub fn assignment_for_node(
        &self,
        node_id: &str,
    ) -> Option<&KnowledgeLayerAssignment> {
        self.assignments
            .iter()
            .find(|assignment| {
                assignment.node_id == node_id
            })
    }

    /// Belirtilen katmana atanmış bütün düğüm
    /// kayıtlarını döndürür.
    pub fn assignments_for_layer(
        &self,
        layer: ZanistarastKnowledgeLayer,
    ) -> Vec<&KnowledgeLayerAssignment> {
        self.assignments
            .iter()
            .filter(|assignment| {
                assignment.layer == layer
            })
            .collect()
    }

    /// Toplam katman ataması sayısını döndürür.
    pub fn assignment_count(&self) -> usize {
        self.assignments.len()
    }

    /// Katman haritasının boş olup olmadığını bildirir.
    pub fn is_empty(&self) -> bool {
        self.assignments.is_empty()
    }
  /// Katman haritasındaki bütün atamaların mevcut
    /// bilgi haritası düğümlerine ait olup olmadığını
    /// doğrular.
    /// Katman atamaları ile mevcut bilgi haritası
/// düğümlerinin iki yönlü uyumluluğunu doğrular.
///
/// Bilgi haritasında bulunmayan atamalar ile henüz
/// katman atanmamış bilgi düğümleri ayrı raporlanır.
pub fn validate_against(
    &self,
    knowledge_map: &KnowledgeMapReport,
) -> KnowledgeLayerValidationReport {
    let mut knowledge_node_ids = knowledge_map
        .maps
        .iter()
        .flat_map(|map| map.nodes.iter())
        .map(|node| node.id.clone())
        .collect::<Vec<_>>();

    knowledge_node_ids.sort();
    knowledge_node_ids.dedup();

    let mut assigned_node_ids = self
        .assignments
        .iter()
        .map(|assignment| assignment.node_id.clone())
        .collect::<Vec<_>>();

    assigned_node_ids.sort();
    assigned_node_ids.dedup();

    let mut unknown_node_ids = assigned_node_ids
        .iter()
        .filter(|node_id| {
            !knowledge_node_ids.contains(node_id)
        })
        .cloned()
        .collect::<Vec<_>>();

    let mut unassigned_node_ids = knowledge_node_ids
        .iter()
        .filter(|node_id| {
            !assigned_node_ids.contains(node_id)
        })
        .cloned()
        .collect::<Vec<_>>();

    unknown_node_ids.sort();
    unknown_node_ids.dedup();

    unassigned_node_ids.sort();
    unassigned_node_ids.dedup();

    KnowledgeLayerValidationReport {
        assignment_count: self.assignments.len(),
        knowledge_node_count:
            knowledge_node_ids.len(),
        unknown_node_ids,
        unassigned_node_ids,
    }
}
}

       /// DNA–RNA–Protein katman atamalarının mevcut
/// bilgi haritasıyla iki yönlü uyumluluk sonucudur.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct KnowledgeLayerValidationReport {
    pub assignment_count: usize,
    pub knowledge_node_count: usize,
    pub unknown_node_ids: Vec<String>,
    pub unassigned_node_ids: Vec<String>,
}

impl KnowledgeLayerValidationReport {
    /// Bütün katman atamalarının mevcut düğümlere
    /// bağlı ve bütün bilgi düğümlerinin katman
    /// atanmış olup olmadığını bildirir.
    pub fn is_valid(&self) -> bool {
        self.unknown_node_ids.is_empty()
            && self.unassigned_node_ids.is_empty()
    }

    /// Bilgi haritasında bulunmayan düğüm kimliği
    /// sayısını döndürür.
    pub fn unknown_node_count(&self) -> usize {
        self.unknown_node_ids.len()
    }

    /// Henüz DNA, RNA veya Protein katmanına
    /// atanmamış düğüm sayısını döndürür.
    pub fn unassigned_node_count(&self) -> usize {
        self.unassigned_node_ids.len()
    }

    /// Katman ataması gerektiren herhangi bir bilgi
    /// düğümü bulunup bulunmadığını bildirir.
    pub fn has_unassigned_nodes(&self) -> bool {
        !self.unassigned_node_ids.is_empty()
    }
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

    #[test]
fn assigns_knowledge_nodes_to_dna_rna_protein_layers() {
    let mut layer_map = KnowledgeLayerMap::new();

    assert!(layer_map.assign(
        KnowledgeLayerAssignment::new(
            "hebun-core-principle",
            ZanistarastKnowledgeLayer::Dna,
            "Hebûn, Zanistarast çekirdek kavramıdır.",
        ),
    ));

    assert!(layer_map.assign(
        KnowledgeLayerAssignment::new(
            "article-production-process",
            ZanistarastKnowledgeLayer::Rna,
            "Makale üretim görevlerini çıktıya dönüştürür.",
        ),
    ));

    assert!(layer_map.assign(
        KnowledgeLayerAssignment::new(
            "rasterast-paper",
            ZanistarastKnowledgeLayer::Protein,
            "Akademik makale somut bir üretim çıktısıdır.",
        ),
    ));

    assert_eq!(layer_map.assignment_count(), 3);

    assert_eq!(
        layer_map
            .assignments_for_layer(
                ZanistarastKnowledgeLayer::Dna,
            )
            .len(),
        1,
    );

    assert_eq!(
        layer_map
            .assignment_for_node("rasterast-paper")
            .expect("protein assignment should exist")
            .layer,
        ZanistarastKnowledgeLayer::Protein,
    );
}

#[test]
fn rejects_incomplete_and_duplicate_layer_assignments() {
    let mut layer_map = KnowledgeLayerMap::new();

    assert!(!layer_map.assign(
        KnowledgeLayerAssignment::new(
            "",
            ZanistarastKnowledgeLayer::Dna,
            "Missing node identifier.",
        ),
    ));

    assert!(!layer_map.assign(
        KnowledgeLayerAssignment::new(
            "hebun",
            ZanistarastKnowledgeLayer::Dna,
            " ",
        ),
    ));

    assert!(layer_map.assign(
        KnowledgeLayerAssignment::new(
            "hebun",
            ZanistarastKnowledgeLayer::Dna,
            "Hebûn çekirdek kavramdır.",
        ),
    ));

    assert!(!layer_map.assign(
        KnowledgeLayerAssignment::new(
            "hebun",
            ZanistarastKnowledgeLayer::Protein,
            "The same node cannot receive a second layer.",
        ),
    ));

    assert_eq!(layer_map.assignment_count(), 1);
}
  #[test]
    fn validates_layer_assignments_against_knowledge_map() {
        let knowledge_map = KnowledgeMapReport {
            maps: vec![DomainKnowledgeMap {
                domain: ZanistarastDomain::Hebun,
                nodes: vec![
                    KnowledgeNode {
                        id: "hebun-main".to_string(),
                        relative_path:
                            PathBuf::from(
                                "papers/hebun-main.md",
                            ),
                        title:
                            Some("Hebûn".to_string()),
                        readiness_score: 90,
                        maturity_level:
                            ArticleMaturityLevel::StrongCandidate,
                    },
                ],
                relations: Vec::new(),
            }],
        };

        let mut layer_map =
            KnowledgeLayerMap::new();

        assert!(layer_map.assign(
            KnowledgeLayerAssignment::new(
                "hebun-main",
                ZanistarastKnowledgeLayer::Dna,
                "Hebûn çekirdek kavramdır.",
            ),
        ));

        let validation =
            layer_map.validate_against(
                &knowledge_map,
            );

        assert!(validation.is_valid());

        assert_eq!(
            validation.assignment_count,
            1,
        );

        assert_eq!(
            validation.unknown_node_count(),
            0,
        );
    assert_eq!(
    validation.knowledge_node_count,
    1,
);

assert_eq!(
    validation.unassigned_node_count(),
    0,
);

assert!(!validation.has_unassigned_nodes());

    }

    #[test]
    fn reports_layer_assignment_for_unknown_node() {
        let knowledge_map = KnowledgeMapReport {
            maps: Vec::new(),
        };

        let mut layer_map =
            KnowledgeLayerMap::new();

        assert!(layer_map.assign(
            KnowledgeLayerAssignment::new(
                "missing-node",
                ZanistarastKnowledgeLayer::Protein,
                "Makale çıktısı olduğu düşünülüyor.",
            ),
        ));

        let validation =
            layer_map.validate_against(
                &knowledge_map,
            );

        assert!(!validation.is_valid());

        assert_eq!(
            validation.unknown_node_count(),
            1,
        );

        assert_eq!(
            validation.unknown_node_ids,
            vec!["missing-node".to_string()],
        );

        assert_eq!(
    validation.knowledge_node_count,
    0,
);

assert_eq!(
    validation.unassigned_node_count(),
    0,
);
    }
#[test]
fn reports_knowledge_nodes_without_layer_assignment() {
    let knowledge_map = KnowledgeMapReport {
        maps: vec![DomainKnowledgeMap {
            domain: ZanistarastDomain::Hebun,
            nodes: vec![
                KnowledgeNode {
                    id: "hebun-core".to_string(),
                    relative_path:
                        PathBuf::from(
                            "papers/hebun-core.md",
                        ),
                    title:
                        Some("Hebûn Çekirdeği".to_string()),
                    readiness_score: 95,
                    maturity_level:
                        ArticleMaturityLevel::StrongCandidate,
                },
                KnowledgeNode {
                    id: "hebun-process".to_string(),
                    relative_path:
                        PathBuf::from(
                            "papers/hebun-process.md",
                        ),
                    title:
                        Some("Hebûn Süreci".to_string()),
                    readiness_score: 80,
                    maturity_level:
                        ArticleMaturityLevel::DevelopingDraft,
                },
            ],
            relations: Vec::new(),
        }],
    };

    let mut layer_map = KnowledgeLayerMap::new();

    assert!(layer_map.assign(
        KnowledgeLayerAssignment::new(
            "hebun-core",
            ZanistarastKnowledgeLayer::Dna,
            "Hebûn çekirdek kavramdır.",
        ),
    ));

    let validation =
        layer_map.validate_against(&knowledge_map);

    assert!(!validation.is_valid());

    assert_eq!(
        validation.assignment_count,
        1,
    );

    assert_eq!(
        validation.knowledge_node_count,
        2,
    );

    assert_eq!(
        validation.unknown_node_count(),
        0,
    );

    assert_eq!(
        validation.unassigned_node_count(),
        1,
    );

    assert!(validation.has_unassigned_nodes());

    assert_eq!(
        validation.unassigned_node_ids,
        vec!["hebun-process".to_string()],
    );
}
#[test]
fn registers_zanistarast_dna_knowledge_types() {
    let mut dna_map = DnaKnowledgeMap::new();

    assert!(dna_map.register(
        DnaKnowledgeRecord::new(
            "hebun-core",
            ZanistarastDnaKind::CorePrinciple,
            "Hebûn, Zanistarast’ın değişmez çekirdek ilkesidir.",
            true,
        ),
    ));

    assert!(dna_map.register(
        DnaKnowledgeRecord::new(
            "rasterast-concept",
            ZanistarastDnaKind::Concept,
            "Rasterast, doğrulama katmanının temel kavramıdır.",
            true,
        ),
    ));

    assert!(dna_map.register(
        DnaKnowledgeRecord::new(
            "heksa-nizam-axiom",
            ZanistarastDnaKind::Axiom,
            "Heksa Nizam için biçimsel bir kurucu aksiyomdur.",
            true,
        ),
    ));

    assert!(dna_map.register(
        DnaKnowledgeRecord::new(
            "revelation-judgment",
            ZanistarastDnaKind::EpistemicJudgment,
            "Vahiy, Zanistarast epistemik düzeninde en yüksek ölçüdür.",
            true,
        ),
    ));

    assert!(dna_map.register(
        DnaKnowledgeRecord::new(
            "mudebbir-decision",
            ZanistarastDnaKind::OfficialDecision,
            "Müdebbir tarafından verilmiş resmî karardır.",
            true,
        ),
    ));

    assert_eq!(dna_map.record_count(), 5);

    assert_eq!(
        dna_map
            .records_of_kind(
                ZanistarastDnaKind::Axiom,
            )
            .len(),
        1,
    );

    assert!(dna_map.contains_node("hebun-core"));

    assert_eq!(
        dna_map
            .record_for_node("mudebbir-decision")
            .expect("official decision should exist")
            .kind,
        ZanistarastDnaKind::OfficialDecision,
    );
}

#[test]
fn rejects_invalid_and_duplicate_dna_records() {
    let mut dna_map = DnaKnowledgeMap::new();

    assert!(!dna_map.register(
        DnaKnowledgeRecord::new(
            "",
            ZanistarastDnaKind::Concept,
            "Missing node identifier.",
            true,
        ),
    ));

    assert!(!dna_map.register(
        DnaKnowledgeRecord::new(
            "temporary-rule",
            ZanistarastDnaKind::CorePrinciple,
            "DNA kaydı değişmez olmalıdır.",
            false,
        ),
    ));

    assert!(!dna_map.register(
        DnaKnowledgeRecord::new(
            "empty-rationale",
            ZanistarastDnaKind::Axiom,
            " ",
            true,
        ),
    ));

    assert!(dna_map.register(
        DnaKnowledgeRecord::new(
            "hebun-core",
            ZanistarastDnaKind::CorePrinciple,
            "Hebûn çekirdek ilkedir.",
            true,
        ),
    ));

    assert!(!dna_map.register(
        DnaKnowledgeRecord::new(
            "hebun-core",
            ZanistarastDnaKind::Concept,
            "Aynı düğüm ikinci kez kaydedilemez.",
            true,
        ),
    ));

    assert_eq!(dna_map.record_count(), 1);
}

}





