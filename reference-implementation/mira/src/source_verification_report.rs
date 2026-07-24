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



