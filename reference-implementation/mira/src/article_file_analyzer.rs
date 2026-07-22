use crate::article_analysis_adapter::AcademicContentSignals;
use crate::article_inventory::ArticleSourceType;
use crate::content_signal_detector::detect_content_signals;
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub enum ArticleFileAnalysisError {
    ReadFailed(String),
}

/// Bir makale dosyasını salt okunur biçimde analiz eder.
///
/// Dosyada hiçbir değişiklik yapmaz.
/// Yalnızca içeriği okuyup akademik sinyalleri çıkarır.
pub fn analyze_article_file(
    path: &Path,
    source_type: &ArticleSourceType,
) -> Result<AcademicContentSignals, ArticleFileAnalysisError> {
    let content = fs::read_to_string(path).map_err(|error| {
        ArticleFileAnalysisError::ReadFailed(error.to_string())
    })?;

    Ok(detect_content_signals(source_type, &content))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_file_path(name: &str) -> std::path::PathBuf {
        let unique_id = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();

        std::env::temp_dir().join(format!(
            "zanistarast_mira_{name}_{}_{}.md",
            std::process::id(),
            unique_id
        ))
    }

    #[test]
    fn analyzer_reads_file_and_detects_signals() {
        let path = temporary_file_path("article");

        let content = r#"
# Abstract

Test abstract.

# Conclusion

Test conclusion.

# References

[1] Test reference.

$$
E = mc^2
$$

Experimental benchmark results.
"#;

        std::fs::write(&path, content)
            .expect("temporary article should be written");

        let signals = analyze_article_file(
            &path,
            &ArticleSourceType::Markdown,
        )
        .expect("article should be analyzed");

        assert!(signals.has_abstract);
        assert!(signals.has_references);
        assert!(signals.has_conclusion);
        assert!(signals.has_math);
        assert!(signals.has_experiments);

        std::fs::remove_file(&path)
            .expect("temporary article should be removed");
    }

    #[test]
    fn analyzer_returns_error_for_missing_file() {
        let path = temporary_file_path("missing");

        let result = analyze_article_file(
            &path,
            &ArticleSourceType::Markdown,
        );

        assert!(matches!(
            result,
            Err(ArticleFileAnalysisError::ReadFailed(_))
        ));
    }
}


