use crate::citation_reference_matcher::CitationReferenceMatchReport;
use crate::doi_validator::is_valid_doi;
use crate::url_validator::is_valid_url;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceVerificationReport {
    pub doi_count: usize,
    pub valid_doi_count: usize,
    pub invalid_doi_count: usize,

    pub url_count: usize,
    pub valid_url_count: usize,
    pub invalid_url_count: usize,

    pub citation_count: usize,
    pub reference_count: usize,

    pub missing_references: Vec<u32>,
    pub unused_references: Vec<u32>,
}

impl SourceVerificationReport {
    pub fn is_verified(&self) -> bool {
        self.invalid_doi_count == 0
            && self.invalid_url_count == 0
            && self.missing_references.is_empty()
            && self.unused_references.is_empty()
    }

    pub fn from_citation_report(
        report: &CitationReferenceMatchReport,
    ) -> Self {
        Self {
            doi_count: 0,
            valid_doi_count: 0,
            invalid_doi_count: 0,

            url_count: 0,
            valid_url_count: 0,
            invalid_url_count: 0,

            citation_count: report.citation_numbers.len(),
            reference_count: report.reference_numbers.len(),

            missing_references: report.missing_references.clone(),
            unused_references: report.unused_references.clone(),
        }
    }

    pub fn from_validation_results(
        doi_count: usize,
        valid_doi_count: usize,
        url_count: usize,
        valid_url_count: usize,
        citation_report: &CitationReferenceMatchReport,
    ) -> Self {
        Self {
            doi_count,
            valid_doi_count,
            invalid_doi_count: doi_count.saturating_sub(valid_doi_count),

            url_count,
            valid_url_count,
            invalid_url_count: url_count.saturating_sub(valid_url_count),

            citation_count: citation_report.citation_numbers.len(),
            reference_count: citation_report.reference_numbers.len(),

            missing_references: citation_report.missing_references.clone(),
            unused_references: citation_report.unused_references.clone(),
        }
    }

    pub fn from_candidates(
        doi_candidates: &[&str],
        url_candidates: &[&str],
        citation_report: &CitationReferenceMatchReport,
    ) -> Self {
        let valid_doi_count = doi_candidates
            .iter()
            .filter(|candidate| is_valid_doi(candidate))
            .count();

        let valid_url_count = url_candidates
            .iter()
            .filter(|candidate| is_valid_url(candidate))
            .count();

        Self::from_validation_results(
            doi_candidates.len(),
            valid_doi_count,
            url_candidates.len(),
            valid_url_count,
            citation_report,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verification_state_depends_on_validation_results() {
        let report = SourceVerificationReport {
            doi_count: 2,
            valid_doi_count: 2,
            invalid_doi_count: 0,

            url_count: 1,
            valid_url_count: 1,
            invalid_url_count: 0,

            citation_count: 3,
            reference_count: 3,

            missing_references: Vec::new(),
            unused_references: Vec::new(),
        };

        assert!(report.is_verified());

        let report = SourceVerificationReport {
            invalid_doi_count: 1,
            ..report
        };

        assert!(!report.is_verified());
    }

    #[test]
    fn creates_report_from_citation_report() {
        let citation_report = CitationReferenceMatchReport {
            citation_numbers: vec![1, 2],
            reference_numbers: vec![1],
            missing_references: vec![2],
            unused_references: Vec::new(),
        };

        let report =
            SourceVerificationReport::from_citation_report(&citation_report);

        assert_eq!(report.citation_count, 2);
        assert_eq!(report.reference_count, 1);
        assert_eq!(report.missing_references, vec![2]);
        assert!(!report.is_verified());
    }

    #[test]
    fn creates_report_from_all_validation_results() {
        let citation_report = CitationReferenceMatchReport {
            citation_numbers: vec![1, 2],
            reference_numbers: vec![1, 2],
            missing_references: Vec::new(),
            unused_references: Vec::new(),
        };

        let report = SourceVerificationReport::from_validation_results(
            2,
            1,
            3,
            2,
            &citation_report,
        );

        assert_eq!(report.doi_count, 2);
        assert_eq!(report.valid_doi_count, 1);
        assert_eq!(report.invalid_doi_count, 1);

        assert_eq!(report.url_count, 3);
        assert_eq!(report.valid_url_count, 2);
        assert_eq!(report.invalid_url_count, 1);

        assert_eq!(report.citation_count, 2);
        assert_eq!(report.reference_count, 2);
        assert!(!report.is_verified());
    }

    #[test]
    fn validates_doi_and_url_candidates() {
        let citation_report = CitationReferenceMatchReport {
            citation_numbers: vec![1],
            reference_numbers: vec![1],
            missing_references: Vec::new(),
            unused_references: Vec::new(),
        };

        let doi_candidates = [
            "10.1000/xyz123",
            "invalid-doi",
        ];

        let url_candidates = [
            "https://example.org",
            "invalid-url",
        ];

        let report = SourceVerificationReport::from_candidates(
            &doi_candidates,
            &url_candidates,
            &citation_report,
        );

        assert_eq!(report.doi_count, 2);
        assert_eq!(report.valid_doi_count, 1);
        assert_eq!(report.invalid_doi_count, 1);

        assert_eq!(report.url_count, 2);
        assert_eq!(report.valid_url_count, 1);
        assert_eq!(report.invalid_url_count, 1);

        assert!(!report.is_verified());
    }
}
