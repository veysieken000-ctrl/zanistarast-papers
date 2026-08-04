use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Repository içinden salt okunur biçimde alınan
/// tek bir metin belgesini temsil eder.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct RepositoryMemoryDocument {
    pub repository_id: Uuid,
    pub repository_name: String,
    pub relative_path: PathBuf,
    pub content: String,
    pub line_count: usize,
    pub character_count: usize,
}

/// Bir veya daha fazla repository içinden okunan
/// metin belgelerinin ortak proje hafızasıdır.
#[derive(
    Debug,
    Clone,
    Default,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct RepositoryMemory {
    pub documents: Vec<RepositoryMemoryDocument>,
}

impl RepositoryMemory {
    /// Hafızadaki belge sayısını döndürür.
    pub fn document_count(&self) -> usize {
        self.documents.len()
    }

    /// Hafızanın boş olup olmadığını bildirir.
    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }

    /// Hafızadaki belgeleri salt okunur biçimde döndürür.
    pub fn documents(
        &self,
    ) -> &[RepositoryMemoryDocument] {
        &self.documents
    }

    /// Hafıza belgeleri üzerinde salt okunur yineleyici
    /// oluşturur.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &RepositoryMemoryDocument> {
        self.documents.iter()
    }
}

