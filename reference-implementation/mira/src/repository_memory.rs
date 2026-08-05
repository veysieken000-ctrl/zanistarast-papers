use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::repository_change_tracker::{
    RepositoryChangeKind,
    RepositoryFileChange,
};

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

    /// Hafızadaki belge sayısını döndürür.
pub fn len(&self) -> usize {
    self.documents.len()
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

    /// Depo değişikliklerini güncel metin içerikleriyle
/// birlikte proje hafızasına uygular.
pub fn apply_changes(
    &mut self,
    repository_id: Uuid,
    repository_name: &str,
    changes: &[RepositoryFileChange],
    current_contents: &[RepositoryTextContent],
) {
    for change in changes {
        match change.kind {
            RepositoryChangeKind::Added
            | RepositoryChangeKind::Modified => {
                let Some(current_path) =
                    change.current_path.as_deref()
                else {
                    continue;
                };

                let Some(current_text) =
                    current_contents.iter().find(|text| {
                        text.relative_path == current_path
                    })
                else {
                    continue;
                };

                if let Some(document) =
                    self.documents.iter_mut().find(|document| {
                        document.repository_id == repository_id
                            && document.text.relative_path
                                == current_path
                    })
                {
                    document.repository_name =
                        repository_name.to_string();
                    document.text = current_text.clone();
                } else {
                    self.documents.push(
                        RepositoryMemoryDocument {
                            repository_id,
                            repository_name:
                                repository_name.to_string(),
                            text: current_text.clone(),
                        },
                    );
                }
            }

            RepositoryChangeKind::Removed => {
                let Some(previous_path) =
                    change.previous_path.as_deref()
                else {
                    continue;
                };

                self.documents.retain(|document| {
                    document.repository_id != repository_id
                        || document.text.relative_path
                            != previous_path
                });
            }

            RepositoryChangeKind::Moved => {
                let (
                    Some(previous_path),
                    Some(current_path),
                ) = (
                    change.previous_path.as_deref(),
                    change.current_path.as_deref(),
                )
                else {
                    continue;
                };

                let current_text =
                    current_contents.iter().find(|text| {
                        text.relative_path == current_path
                    });

                if let Some(document) =
                    self.documents.iter_mut().find(|document| {
                        document.repository_id == repository_id
                            && document.text.relative_path
                                == previous_path
                    })
                {
                    document.repository_name =
                        repository_name.to_string();

                    if let Some(current_text) = current_text {
                        document.text = current_text.clone();
                    } else {
                        document.text.relative_path =
                            current_path.to_path_buf();
                    }
                }
            }
        }
    }
}
    
    pub fn contains(
    &self,
    repository_id: Uuid,
    relative_path: impl AsRef<Path>,
) -> bool {
    self.find_document(repository_id, relative_path)
        .is_some()
}

    /// Hafıza belgeleri üzerinde salt okunur
    /// yineleyici oluşturur.
    pub fn iter(
        &self,
    ) -> impl Iterator<Item = &RepositoryMemoryDocument> {
        self.documents.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repository_change_tracker::RepositoryFileChange;

    #[test]
    fn applies_repository_changes_to_project_memory() {
        let repository_id = Uuid::new_v4();

        let mut memory = RepositoryMemory {
            documents: vec![
                RepositoryMemoryDocument {
                    repository_id,
                    repository_name:
                        "zanistarast-papers".to_string(),
                    text: RepositoryTextContent {
                        relative_path:
                            PathBuf::from("removed.md"),
                        content:
                            "Old removed content".to_string(),
                        line_count: 1,
                        character_count: 19,
                    },
                },
                RepositoryMemoryDocument {
                    repository_id,
                    repository_name:
                        "zanistarast-papers".to_string(),
                    text: RepositoryTextContent {
                        relative_path:
                            PathBuf::from("modified.md"),
                        content:
                            "Old content".to_string(),
                        line_count: 1,
                        character_count: 11,
                    },
                },
                RepositoryMemoryDocument {
                    repository_id,
                    repository_name:
                        "zanistarast-papers".to_string(),
                    text: RepositoryTextContent {
                        relative_path:
                            PathBuf::from("old-name.md"),
                        content:
                            "Moved content".to_string(),
                        line_count: 1,
                        character_count: 13,
                    },
                },
            ],
        };

        let changes = vec![
            RepositoryFileChange::added("added.md"),
            RepositoryFileChange::modified("modified.md"),
            RepositoryFileChange::removed("removed.md"),
            RepositoryFileChange::moved(
                "old-name.md",
                "new-name.md",
            ),
        ];

        let current_contents = vec![
            RepositoryTextContent {
                relative_path:
                    PathBuf::from("added.md"),
                content:
                    "Added content".to_string(),
                line_count: 1,
                character_count: 13,
            },
            RepositoryTextContent {
                relative_path:
                    PathBuf::from("modified.md"),
                content:
                    "New content".to_string(),
                line_count: 1,
                character_count: 11,
            },
            RepositoryTextContent {
                relative_path:
                    PathBuf::from("new-name.md"),
                content:
                    "Moved content".to_string(),
                line_count: 1,
                character_count: 13,
            },
        ];

        memory.apply_changes(
            repository_id,
            "zanistarast-papers",
            &changes,
            &current_contents,
        );

        assert_eq!(memory.document_count(), 3);

        assert!(
            memory
                .find_document(
                    repository_id,
                    Path::new("removed.md"),
                )
                .is_none(),
        );

        assert_eq!(
            memory
                .find_document(
                    repository_id,
                    Path::new("added.md"),
                )
                .expect("added document should exist")
                .text
                .content,
            "Added content",
        );

        assert_eq!(
            memory
                .find_document(
                    repository_id,
                    Path::new("modified.md"),
                )
                .expect("modified document should exist")
                .text
                .content,
            "New content",
        );

        assert!(
            memory
                .find_document(
                    repository_id,
                    Path::new("old-name.md"),
                )
                .is_none(),
        );

        assert!(
            memory
                .find_document(
                    repository_id,
                    Path::new("new-name.md"),
                )
                .is_some(),
        );
    }
#[test]
fn repository_memory_len_reports_document_count() {
    let mut memory = RepositoryMemory::default();

    assert_eq!(memory.len(), 0);

    let document = RepositoryMemoryDocument {
        repository_id: Uuid::new_v4(),
        repository_name: "demo".to_string(),
        text: RepositoryTextContent {
            relative_path: PathBuf::from("sample.md"),
            content: "sample".to_string(),
            line_count: 1,
            character_count: 6,
        },
    };

    memory.extend(std::iter::once(document));


    assert_eq!(memory.len(), 1);
}
#[test]
fn repository_memory_contains_document() {
    let repository_id = Uuid::new_v4();

    let mut memory = RepositoryMemory::default();

    let document = RepositoryMemoryDocument {
        repository_id,
        repository_name: "demo".to_string(),
        text: RepositoryTextContent {
            relative_path: PathBuf::from("paper.md"),
            content: "sample".to_string(),
            line_count: 1,
            character_count: 6,
        },
    };

    memory.extend(std::iter::once(document));

    assert!(memory.contains(
        repository_id,
        Path::new("paper.md"),
    ));

    assert!(!memory.contains(
        repository_id,
        Path::new("missing.md"),
    ));
}

}




