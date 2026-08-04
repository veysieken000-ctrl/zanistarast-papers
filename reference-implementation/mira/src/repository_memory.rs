use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Bir metin dosyasının salt okunur içerik tarama sonucudur.
#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
)]
pub struct RepositoryTextContent {
    pub relative_path: PathBuf,
    pub content: String,
    pub line_count: usize,
    pub character_count: usize,
}

impl RepositoryTextContent {
    /// İçerik kaydının zorunlu alanlarının eksiksiz
    /// olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.relative_path.as_os_str().is_empty()
    }

    /// Dosyanın boş olup olmadığını bildirir.
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
}

/// Proje hafızasına alınmış ve kaynak deposuyla
/// ilişkilendirilmiş tek bir metin belgesidir.
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
    pub text: RepositoryTextContent,
}

impl RepositoryMemoryDocument {
    /// Hafıza belgesinin zorunlu bilgilerinin
    /// eksiksiz olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.repository_name.trim().is_empty()
            && self.text.is_complete()
    }
}

/// Bir veya daha fazla depodan okunmuş metin içeriklerinin
/// ortak proje hafızasını temsil eder.
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

    /// Belirtilen depo kimliğine ait hafıza
    /// belgelerini döndürür.
    pub fn documents_for_repository(
        &self,
        repository_id: Uuid,
    ) -> Vec<&RepositoryMemoryDocument> {
        self.documents
            .iter()
            .filter(|document| {
                document.repository_id == repository_id
            })
            .collect()
    }

    /// Belirtilen depo adına ait hafıza
    /// belgelerini döndürür.
    pub fn documents_for_repository_name(
        &self,
        repository_name: &str,
    ) -> Vec<&RepositoryMemoryDocument> {
        self.documents
            .iter()
            .filter(|document| {
                document.repository_name == repository_name
            })
            .collect()
    }

    /// Depo kimliği ve göreli dosya yoluyla tek
    /// bir hafıza belgesi bulur.
    pub fn find_document(
        &self,
        repository_id: Uuid,
        relative_path: impl AsRef<Path>,
    ) -> Option<&RepositoryMemoryDocument> {
        self.documents.iter().find(|document| {
            document.repository_id == repository_id
                && document.text.relative_path
                    == relative_path.as_ref()
        })
    }

    /// Metin içeriğinde büyük-küçük harf duyarsız
    /// arama yapar.
    pub fn search_text(
        &self,
        query: &str,
    ) -> Vec<&RepositoryMemoryDocument> {
        let query = query.trim().to_lowercase();

        if query.is_empty() {
            return Vec::new();
        }

        self.documents
            .iter()
            .filter(|document| {
                document
                    .text
                    .content
                    .to_lowercase()
                    .contains(&query)
            })
            .collect()
    }

    /// Başka bir proje hafızasının belgelerini
    /// bu hafızaya ekler.
    pub fn merge(
        &mut self,
        other: RepositoryMemory,
    ) {
        self.documents.extend(other.documents);
    }

    /// Proje hafızasındaki bütün belgeleri temizler.
    pub fn clear(&mut self) {
        self.documents.clear();
    }

    /// Bir belge koleksiyonunu proje hafızasına ekler.
    pub fn extend<I>(
        &mut self,
        documents: I,
    )
    where
        I: IntoIterator<Item = RepositoryMemoryDocument>,
    {
        self.documents.extend(documents);
    }

    /// Hafıza belgeleri üzerinde salt okunur
    /// yineleyici oluşturur.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &RepositoryMemoryDocument> {
        self.documents.iter()
    }
}


