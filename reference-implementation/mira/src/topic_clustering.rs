use crate::article_candidate_analysis::{
    ArticleCandidateAnalysisReport,
    ArticleMaturityLevel,
};
use crate::article_inventory::{
    ArticleInventoryReport,
    ZanistarastDomain,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::path::PathBuf;

/// Bir konu kümesindeki tek makale kaydı.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClusteredArticle {
    pub relative_path: PathBuf,
    pub title: Option<String>,
    pub readiness_score: u8,
    pub maturity_level: ArticleMaturityLevel,
}

/// Belirli bir Zanistarast alanına ait konu kümesi.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TopicCluster {
    pub domain: ZanistarastDomain,
    pub articles: Vec<ClusteredArticle>,
    pub total_readiness_score: usize,
}

impl TopicCluster {
    pub fn article_count(&self) -> usize {
        self.articles.len()
    }

    pub fn average_readiness_score(&self) -> u8 {
        if self.articles.is_empty() {
            return 0;
        }

        let average =
            self.total_readiness_score / self.articles.len();

        u8::try_from(average).unwrap_or(u8::MAX)
    }
}

/// Tüm konu kümelerinin salt okunur raporu.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TopicClusteringReport {
    pub clusters: Vec<TopicCluster>,
    pub total_clustered_articles: usize,
}

impl TopicClusteringReport {
    pub fn cluster_count(&self) -> usize {
        self.clusters.len()
    }

    pub fn cluster_for_domain(
        &self,
        domain: &ZanistarastDomain,
    ) -> Option<&TopicCluster> {
        self.clusters
            .iter()
            .find(|cluster| &cluster.domain == domain)
    }
}

/// Makale adaylarını Zanistarast konu alanlarına göre kümelendirir.
///
/// Bu işlem:
/// - orijinal dosyaları değiştirmez,
/// - içerik taşımaz,
/// - dosya silmez,
/// - yalnızca mevcut analiz sonuçlarını gruplandırır.
#[derive(Debug, Default)]
pub struct TopicClusterer;

impl TopicClusterer {
    pub fn new() -> Self {
        Self
    }

    pub fn cluster(
        &self,
        inventory: &ArticleInventoryReport,
        analysis: &ArticleCandidateAnalysisReport,
    ) -> TopicClusteringReport {
        let mut clusters =
            BTreeMap::<ZanistarastDomain, TopicCluster>::new();

        for candidate in &inventory.candidates {
            let Some(assessment) = analysis
                .assessments
                .iter()
                .find(|assessment| {
                    assessment.relative_path
                        == candidate.relative_path
                })
            else {
                continue;
            };

            for domain in &candidate.domains {
                let cluster =
                    clusters
                        .entry(domain.clone())
                        .or_insert_with(|| TopicCluster {
                            domain: domain.clone(),
                            articles: Vec::new(),
                            total_readiness_score: 0,
                        });

                cluster.total_readiness_score +=
                    usize::from(assessment.readiness_score);

                cluster.articles.push(ClusteredArticle {
                    relative_path:
                        candidate.relative_path.clone(),
                    title: candidate.title.clone(),
                    readiness_score:
                        assessment.readiness_score,
                    maturity_level:
                        assessment.maturity_level.clone(),
                });
            }
        }

        let mut clusters = clusters
            .into_values()
            .collect::<Vec<_>>();

        for cluster in &mut clusters {
            cluster.articles.sort_by(|left, right| {
                right
                    .readiness_score
                    .cmp(&left.readiness_score)
                    .then_with(|| {
                        left.relative_path
                            .cmp(&right.relative_path)
                    })
            });
        }

        let total_clustered_articles = clusters
            .iter()
            .map(TopicCluster::article_count)
            .sum();

        TopicClusteringReport {
            clusters,
            total_clustered_articles,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::article_candidate_analysis::{
        ArticleCandidateAssessment,
        ArticleCandidateAnalysisReport,
    };
    use crate::article_inventory::{
        ArticleCandidate,
        ArticleInventoryReport,
        ArticleSourceType,
    };

    #[test]
    fn articles_are_grouped_by_zanistarast_domain() {
        let inventory = ArticleInventoryReport {
            candidates: vec![
                ArticleCandidate {
                    relative_path: PathBuf::from(
                        "papers/hebun.md",
                    ),
                    title: Some(
                        "Hebûn Kuramsal Çerçevesi"
                            .to_string(),
                    ),
                    source_type:
                        ArticleSourceType::Markdown,
                    domains: vec![
                        ZanistarastDomain::Hebun,
                    ],
                    size_bytes: 8_000,
                },
                ArticleCandidate {
                    relative_path: PathBuf::from(
                        "papers/rabun.md",
                    ),
                    title: Some(
                        "Rabûn Yönetim Modeli".to_string(),
                    ),
                    source_type:
                        ArticleSourceType::Markdown,
                    domains: vec![
                        ZanistarastDomain::Rabun,
                    ],
                    size_bytes: 7_000,
                },
            ],
            repository_candidate_count: 2,
            website_candidate_count: 0,
        };

        let analysis = ArticleCandidateAnalysisReport {
            assessments: vec![
                ArticleCandidateAssessment {
                    relative_path: PathBuf::from(
                        "papers/hebun.md",
                    ),
                    title: Some(
                        "Hebûn Kuramsal Çerçevesi"
                            .to_string(),
                    ),
                    maturity_level:
                        ArticleMaturityLevel::StrongCandidate,
                    readiness_score: 90,
                    strengths: Vec::new(),
                    missing_elements: Vec::new(),
                    recommended_next_step:
                        "Review".to_string(),
                    requires_content_review: true,
                },
                ArticleCandidateAssessment {
                    relative_path: PathBuf::from(
                        "papers/rabun.md",
                    ),
                    title: Some(
                        "Rabûn Yönetim Modeli".to_string(),
                    ),
                    maturity_level:
                        ArticleMaturityLevel::DevelopingDraft,
                    readiness_score: 65,
                    strengths: Vec::new(),
                    missing_elements: Vec::new(),
                    recommended_next_step:
                        "Develop".to_string(),
                    requires_content_review: true,
                },
            ],
        };

        let clusterer = TopicClusterer::new();
        let report = clusterer.cluster(
            &inventory,
            &analysis,
        );

        assert_eq!(report.cluster_count(), 2);
        assert_eq!(report.total_clustered_articles, 2);

        let hebun_cluster = report
            .cluster_for_domain(
                &ZanistarastDomain::Hebun,
            )
            .expect("Hebûn cluster should exist");

        assert_eq!(hebun_cluster.article_count(), 1);
        assert_eq!(
            hebun_cluster.average_readiness_score(),
            90
        );
    }

    #[test]
    fn articles_inside_cluster_are_sorted_by_score() {
        let inventory = ArticleInventoryReport {
            candidates: vec![
                ArticleCandidate {
                    relative_path: PathBuf::from(
                        "papers/hebun-short.md",
                    ),
                    title: None,
                    source_type:
                        ArticleSourceType::Markdown,
                    domains: vec![
                        ZanistarastDomain::Hebun,
                    ],
                    size_bytes: 500,
                },
                ArticleCandidate {
                    relative_path: PathBuf::from(
                        "papers/hebun-main.md",
                    ),
                    title: Some(
                        "Hebûn Ana Makalesi".to_string(),
                    ),
                    source_type:
                        ArticleSourceType::Markdown,
                    domains: vec![
                        ZanistarastDomain::Hebun,
                    ],
                    size_bytes: 10_000,
                },
            ],
            repository_candidate_count: 2,
            website_candidate_count: 0,
        };

        let analysis = ArticleCandidateAnalysisReport {
            assessments: vec![
                ArticleCandidateAssessment {
                    relative_path: PathBuf::from(
                        "papers/hebun-short.md",
                    ),
                    title: None,
                    maturity_level:
                        ArticleMaturityLevel::Fragment,
                    readiness_score: 15,
                    strengths: Vec::new(),
                    missing_elements: Vec::new(),
                    recommended_next_step:
                        "Combine".to_string(),
                    requires_content_review: true,
                },
                ArticleCandidateAssessment {
                    relative_path: PathBuf::from(
                        "papers/hebun-main.md",
                    ),
                    title: Some(
                        "Hebûn Ana Makalesi".to_string(),
                    ),
                    maturity_level:
                        ArticleMaturityLevel::StrongCandidate,
                    readiness_score: 90,
                    strengths: Vec::new(),
                    missing_elements: Vec::new(),
                    recommended_next_step:
                        "Review".to_string(),
                    requires_content_review: true,
                },
            ],
        };

        let clusterer = TopicClusterer::new();
        let report = clusterer.cluster(
            &inventory,
            &analysis,
        );

        let cluster = report
            .cluster_for_domain(
                &ZanistarastDomain::Hebun,
            )
            .expect("Hebûn cluster should exist");

        assert_eq!(
            cluster.articles[0].relative_path,
            PathBuf::from("papers/hebun-main.md")
        );
        assert_eq!(
            cluster.articles[1].relative_path,
            PathBuf::from("papers/hebun-short.md")
        );
    }
}






