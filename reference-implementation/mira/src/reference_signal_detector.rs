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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_reference_signals() {
        let content = r#"
# References

doi:10.1000/xyz123

https://doi.org/10.1000/xyz123

@article{
example2025,
title={Example}
}
"#;

        let signals = detect_reference_signals(content);

        assert!(signals.has_references_section);
        assert!(signals.has_doi);
        assert!(signals.has_url);
        assert!(signals.has_bibtex_entry);
    }

    #[test]
    fn detects_missing_reference_signals() {
        let content = r#"
This is an ordinary text.

No bibliography.
No DOI.
No URL.
"#;

        let signals = detect_reference_signals(content);

        assert!(!signals.has_references_section);
        assert!(!signals.has_doi);
        assert!(!signals.has_url);
        assert!(!signals.has_bibtex_entry);
    }
}


