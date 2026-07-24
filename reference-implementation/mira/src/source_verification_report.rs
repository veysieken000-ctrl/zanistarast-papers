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
}

use crate::citation_reference_matcher::CitationReferenceMatchReport;
use crate::doi_validator::is_valid_doi;
use crate::url_validator::is_valid_url;

impl SourceVerificationReport {
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
}


#[test]
fn creates_report_from_citation_report() {
    use crate::citation_reference_matcher::CitationReferenceMatchReport;

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




