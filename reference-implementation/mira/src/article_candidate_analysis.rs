use crate::article_inventory::{
    ArticleCandidate,
    ArticleInventoryReport,
    ZanistarastDomain,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Bir makale adayının mevcut akademik olgunluk düzeyi.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ArticleMaturityLevel {
    Fragment,
    EarlyDraft,
    DevelopingDraft,
    StrongCandidate,
}

/// Makale adayına ilişkin salt okunur bilimsel değerlendirme.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArticleCandidateAssessment {
    pub relative_path: PathBuf,
    pub title: Option<String>,
    pub maturity_level: ArticleMaturityLevel,
    pub readiness_score: u8,
    pub strengths: Vec<String>,
    pub missing_elements: Vec<String>,
    pub recommended_next_step: String,
    pub requires_content_review: bool,
}

/// Tüm makale adaylarının analiz raporu.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArticleCandidateAnalysisReport {
    pub assessments: Vec<ArticleCandidateAssessment>,
}

impl ArticleCandidateAnalysisReport {
    pub fn assessment_count(&self) -> usize {
        self.assessments.len()
    }

    pub fn strong_candidate_count(&self) -> usize {
        self.assessments
            .iter()
            .filter(|assessment| {
                assessment.maturity_level
                    == ArticleMaturityLevel::StrongCandidate
            })
            .count()
    }
}

/// Makale envanterindeki adayları salt okunur biçimde değerlendirir.
///
/// Bu analiz:
/// - orijinal dosyaları değiştirmez,
/// - içerik hakkında doğrulanmamış hüküm vermez,
/// - yalnızca mevcut metadata üzerinden ön değerlendirme yapar,
/// - kesin akademik karar için içerik incelemesi gerektiğini belirtir.
#[derive(Debug, Default)]
pub struct ArticleCandidateAnalyzer;

impl ArticleCandidateAnalyzer {
    pub fn new() -> Self {
        Self
    }

    /// Makale envanterindeki tüm adaylar için ön analiz üretir.
    pub fn analyze(
        &self,
        inventory: &ArticleInventoryReport,
    ) -> ArticleCandidateAnalysisReport {
        let mut assessments = inventory
            .candidates
            .iter()
            .map(Self::assess_candidate)
            .collect::<Vec<_>>();

        assessments.sort_by(|left, right| {
            right
                .readiness_score
                .cmp(&left.readiness_score)
                .then_with(|| left.relative_path.cmp(&right.relative_path))
        });

        ArticleCandidateAnalysisReport { assessments }
    }

    fn assess_candidate(
        candidate: &ArticleCandidate,
    ) -> ArticleCandidateAssessment {
        let mut readiness_score = 0_u8;
        let mut strengths = Vec::new();
        let mut missing_elements = Vec::new();

        if candidate.title.is_some() {
            readiness_score += 20;
            strengths.push(
                "Açık bir başlık tespit edildi.".to_string(),
            );
        } else {
            missing_elements.push(
                "Akademik başlık doğrulanmalıdır.".to_string(),
            );
        }

        if candidate.size_bytes >= 8_000 {
            readiness_score += 35;
            strengths.push(
                "Dosya boyutu kapsamlı bir çalışma ihtimalini destekliyor."
                    .to_string(),
            );
        } else if candidate.size_bytes >= 3_000 {
            readiness_score += 25;
            strengths.push(
                "Dosya boyutu gelişmekte olan bir taslağa uygundur."
                    .to_string(),
            );
        } else if candidate.size_bytes >= 1_000 {
            readiness_score += 15;
            missing_elements.push(
                "Metnin kapsamı akademik makale için genişletilebilir."
                    .to_string(),
            );
        } else {
            readiness_score += 5;
            missing_elements.push(
                "Metin kısa görünüyor; bağımsız makale yerine parça veya not olabilir."
                    .to_string(),
            );
        }

        if Self::has_classified_domain(candidate) {
            readiness_score += 25;
            strengths.push(
                "Zanistarast konu alanı tespit edildi.".to_string(),
            );
        } else {
            missing_elements.push(
                "Zanistarast konu sınıflandırması henüz doğrulanmadı."
                    .to_string(),
            );
        }

        readiness_score += 10;

        missing_elements.push(
            "Özet ve anahtar kelimeler içerik incelemesiyle doğrulanmalıdır."
                .to_string(),
        );
        missing_elements.push(
            "Yöntem veya kuramsal çerçeve içerik incelemesiyle belirlenmelidir."
                .to_string(),
        );
        missing_elements.push(
            "Kaynaklar ve atıflar doğrulanmalıdır.".to_string(),
        );

        let maturity_level =
            Self::maturity_level_from_score(readiness_score);

        let recommended_next_step = match maturity_level {
            ArticleMaturityLevel::Fragment => {
                "İlgili konu kümesindeki diğer metinlerle bağlantısı incelenmeli."
                    .to_string()
            }
            ArticleMaturityLevel::EarlyDraft => {
                "İçerik okunmalı ve makale türü belirlenmeli."
                    .to_string()
            }
            ArticleMaturityLevel::DevelopingDraft => {
                "Akademik yapı, kaynak ve yöntem eksikleri analiz edilmeli."
                    .to_string()
            }
            ArticleMaturityLevel::StrongCandidate => {
                "Müdebbir incelemesi için ayrıntılı akademik dönüşüm raporu hazırlanmalı."
                    .to_string()
            }
        };

        ArticleCandidateAssessment {
            relative_path: candidate.relative_path.clone(),
            title: candidate.title.clone(),
            maturity_level,
            readiness_score,
            strengths,
            missing_elements,
            recommended_next_step,
            requires_content_review: true,
        }
    }

    fn has_classified_domain(candidate: &ArticleCandidate) -> bool {
        candidate
            .domains
            .iter()
            .any(|domain| domain != &ZanistarastDomain::Unclassified)
    }

    fn maturity_level_from_score(
        readiness_score: u8,
    ) -> ArticleMaturityLevel {
        match readiness_score {
            0..=29 => ArticleMaturityLevel::Fragment,
            30..=49 => ArticleMaturityLevel::EarlyDraft,
            50..=69 => ArticleMaturityLevel::DevelopingDraft,
            _ => ArticleMaturityLevel::StrongCandidate,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::article_inventory::{
        ArticleSourceType,
        ZanistarastDomain,
    };
    use std::path::PathBuf;

    #[test]
    fn strong_candidate_receives_high_score() {
        let inventory = ArticleInventoryReport {
            candidates: vec![ArticleCandidate {
                relative_path: PathBuf::from(
                    "papers/hebun-theory.html",
                ),
                title: Some(
                    "Hebûn Kuramsal Çerçevesi".to_string(),
                ),
                source_type: ArticleSourceType::Html,
                domains: vec![ZanistarastDomain::Hebun],
                size_bytes: 12_000,
            }],
            repository_candidate_count: 1,
            website_candidate_count: 0,
        };

        let analyzer = ArticleCandidateAnalyzer::new();
        let report = analyzer.analyze(&inventory);

        assert_eq!(report.assessment_count(), 1);
        assert_eq!(report.strong_candidate_count(), 1);
        assert_eq!(
            report.assessments[0].maturity_level,
            ArticleMaturityLevel::StrongCandidate
        );
        assert!(report.assessments[0].requires_content_review);
    }

    #[test]
    fn short_unclassified_file_is_marked_as_fragment() {
        let inventory = ArticleInventoryReport {
            candidates: vec![ArticleCandidate {
                relative_path: PathBuf::from("notes/general.md"),
                title: None,
                source_type: ArticleSourceType::Markdown,
                domains: vec![ZanistarastDomain::Unclassified],
                size_bytes: 300,
            }],
            repository_candidate_count: 1,
            website_candidate_count: 0,
        };

        let analyzer = ArticleCandidateAnalyzer::new();
        let report = analyzer.analyze(&inventory);

        assert_eq!(
            report.assessments[0].maturity_level,
            ArticleMaturityLevel::Fragment
        );
        assert!(
            report.assessments[0].readiness_score < 30
        );
    }

    #[test]
    fn assessments_are_sorted_by_readiness_score() {
        let inventory = ArticleInventoryReport {
            candidates: vec![
                ArticleCandidate {
                    relative_path: PathBuf::from("notes/short.md"),
                    title: None,
                    source_type: ArticleSourceType::Markdown,
                    domains: vec![ZanistarastDomain::Unclassified],
                    size_bytes: 200,
                },
                ArticleCandidate {
                    relative_path: PathBuf::from(
                        "papers/rasterast.html",
                    ),
                    title: Some(
                        "Rasterast Doğrulama Sistemi".to_string(),
                    ),
                    source_type: ArticleSourceType::Html,
                    domains: vec![
                        ZanistarastDomain::Rasterast,
                    ],
                    size_bytes: 10_000,
                },
            ],
            repository_candidate_count: 2,
            website_candidate_count: 0,
        };

        let analyzer = ArticleCandidateAnalyzer::new();
        let report = analyzer.analyze(&inventory);

        assert!(
            report.assessments[0].readiness_score
                > report.assessments[1].readiness_score
        );
    }
}




