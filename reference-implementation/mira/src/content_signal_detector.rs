use crate::article_analysis_adapter::AcademicContentSignals;
use crate::article_inventory::ArticleSourceType;

/// Makale metninden temel akademik içerik sinyallerini çıkarır.
///
/// Bu dedektör salt okunurdur.
/// Metni değiştirmez ve belirsiz bir özelliği doğru kabul etmez.
pub fn detect_content_signals(
   source_type: &ArticleSourceType,
    content: &str,
) -> AcademicContentSignals {
    let normalized = content.to_lowercase();

  AcademicContentSignals {
    has_abstract: contains_abstract(source_type, &normalized),
    has_references: contains_references(source_type, &normalized),
    has_conclusion: contains_conclusion(source_type, &normalized),
    has_math: contains_math(source_type, content, &normalized),
    has_experiments: contains_experiments(&normalized),
    } 
} 

fn contains_abstract(
    source_type: &ArticleSourceType,
    normalized: &str,
) -> bool {
    match source_type {
        ArticleSourceType::Markdown => {
            normalized.contains("# abstract")
                || normalized.contains("## abstract")
                || normalized.contains("# özet")
                || normalized.contains("## özet")
        }

        ArticleSourceType::Html => {
            normalized.contains(">abstract<")
                || normalized.contains(">özet<")
                || normalized.contains("id=\"abstract\"")
        }

        ArticleSourceType::Latex => {
            normalized.contains("\\begin{abstract}")
        }
    }
}

fn contains_references(
    source_type: &ArticleSourceType,
    normalized: &str,
) -> bool {
    match source_type {
        ArticleSourceType::Markdown => {
            normalized.contains("# references")
                || normalized.contains("## references")
                || normalized.contains("# kaynakça")
                || normalized.contains("## kaynakça")
        }

        ArticleSourceType::Html => {
            normalized.contains(">references<")
                || normalized.contains(">kaynakça<")
                || normalized.contains("id=\"references\"")
        }

        ArticleSourceType::Latex => {
            normalized.contains("\\bibliography{")
                || normalized.contains("\\begin{thebibliography}")
        }
    }
}

fn contains_conclusion(
    source_type: &ArticleSourceType,
    normalized: &str,
) -> bool {
    match source_type {
        ArticleSourceType::Markdown => {
            normalized.contains("# conclusion")
                || normalized.contains("## conclusion")
                || normalized.contains("# sonuç")
                || normalized.contains("## sonuç")
        }

        ArticleSourceType::Html => {
            normalized.contains(">conclusion<")
                || normalized.contains(">sonuç<")
                || normalized.contains("id=\"conclusion\"")
        }

        ArticleSourceType::Latex => {
            normalized.contains("\\section{conclusion}")
                || normalized.contains("\\section{sonuç}")
        }
    }
}

fn contains_math(
    source_type: &ArticleSourceType,
    content: &str,
    normalized: &str,
) -> bool {
    match source_type {
        ArticleSourceType::Markdown => {
            content.contains("$$")
                || normalized.contains("\\begin{equation}")
        }

        ArticleSourceType::Html => {
            normalized.contains("<math")
                || normalized.contains("class=\"math")
        }

        ArticleSourceType::Latex => {
            normalized.contains("\\begin{equation}")
                || normalized.contains("\\[")
                || normalized.contains("\\(")
        }
    }
}

fn contains_experiments(normalized: &str) -> bool {
    normalized.contains("experiment")
        || normalized.contains("experimental")
        || normalized.contains("deney")
        || normalized.contains("benchmark")
}

use zanistarast_mira::article_inventory::ArticleSourceType;
use zanistarast_mira::content_signal_detector::detect_content_signals;

#[test]
fn markdown_detects_all_basic_sections() {
    let content = r#"
# Abstract

Some text.

# Conclusion

Done.

# References

[1] Test

$$
E = mc^2
$$

Experiment benchmark
"#;

    let signals = detect_content_signals(
        &ArticleSourceType::Markdown,
        content,
    );

    assert!(signals.has_abstract);
    assert!(signals.has_references);
    assert!(signals.has_conclusion);
    assert!(signals.has_math);
    assert!(signals.has_experiments);
}


