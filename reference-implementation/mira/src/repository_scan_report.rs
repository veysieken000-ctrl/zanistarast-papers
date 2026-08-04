use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Repository taramasında bulunan tek bir dosyanın kaydı.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct RepositoryFile {
    pub relative_path: PathBuf,
    pub extension: Option<String>,
    pub size_bytes: u64,
}

/// Salt okunur repository taramasının sonucu.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct RepositoryScanReport {
    pub root: PathBuf,
    pub files: Vec<RepositoryFile>,
    pub directory_count: usize,
    pub total_size_bytes: u64,
}

impl RepositoryScanReport {
    /// Tarama raporundaki dosya sayısını döndürür.
    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}


