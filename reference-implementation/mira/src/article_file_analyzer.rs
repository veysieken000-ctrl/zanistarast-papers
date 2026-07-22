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




