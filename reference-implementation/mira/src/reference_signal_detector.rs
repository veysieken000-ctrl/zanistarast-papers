#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceSignals {
    pub has_references_section: bool,
    pub has_doi: bool,
    pub has_url: bool,
    pub has_bibtex_entry: bool,
}

/// Akademik metindeki temel kaynak sinyallerini salt okunur biçimde tespit eder.
///
/// Bu fonksiyon kaynakların doğru olduğunu henüz iddia etmez.
/// Yalnızca doğrulama aşamasında kullanılacak işaretleri çıkarır.
pub fn detect_reference_signals(content: &str) -> ReferenceSignals {
    let normalized = content.to_lowercase();

    ReferenceSignals {
        has_references_section: normalized.contains("# references")
            || normalized.contains("## references")
            || normalized.contains("# kaynakça")
            || normalized.contains("## kaynakça")
            || normalized.contains("\\section{references}")
            || normalized.contains("\\section{kaynakça}"),

        has_doi: normalized.contains("doi:")
            || normalized.contains("doi.org/"),

        has_url: normalized.contains("http://")
            || normalized.contains("https://"),

        has_bibtex_entry: normalized.contains("@article{")
            || normalized.contains("@book{")
            || normalized.contains("@inproceedings{")
            || normalized.contains("@misc{"),
    }
}

