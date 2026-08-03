use std::path::{Path, PathBuf};

use uuid::Uuid;

/// Mira tarafından salt okunur biçimde incelenecek
/// tek bir kaynak deponun kayıt modelidir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryRoot {
    pub id: Uuid,
    pub name: String,
    pub root_path: PathBuf,
    pub read_only: bool,
}

impl RepositoryRoot {
    /// Yeni bir salt okunur depo kaydı oluşturur.
    pub fn new(
        name: impl Into<String>,
        root_path: impl Into<PathBuf>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            root_path: root_path.into(),
            read_only: true,
        }
    }

    /// Depo kaydının zorunlu alanlarının eksiksiz
    /// olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.name.trim().is_empty()
            && !self.root_path.as_os_str().is_empty()
            && self.read_only
    }

    /// Kayıtlı kök yolun dosya sisteminde bir dizin
    /// olarak bulunup bulunmadığını bildirir.
    pub fn exists(&self) -> bool {
        self.root_path.is_dir()
    }

    /// Verilen yolun bu deponun kök yoluyla aynı
    /// olup olmadığını bildirir.
    pub fn matches_root(
        &self,
        path: impl AsRef<Path>,
    ) -> bool {
        self.root_path == path.as_ref()
    }
}

/// Mira’nın erişebildiği salt okunur kaynak depoların
/// düzenli envanterini temsil eder.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RepositoryInventory {
    repositories: Vec<RepositoryRoot>,
}

impl RepositoryInventory {
    /// Boş bir depo envanteri oluşturur.
    pub fn new() -> Self {
        Self {
            repositories: Vec::new(),
        }
    }

    /// Eksiksiz ve daha önce kaydedilmemiş bir
    /// salt okunur depoyu envantere ekler.
    pub fn register(
        &mut self,
        repository: RepositoryRoot,
    ) -> bool {
        if !repository.is_complete() {
            return false;
        }

        if self.repositories.iter().any(|stored| {
            stored.root_path == repository.root_path
        }) {
            return false;
        }

        self.repositories.push(repository);
        true
    }

    /// Kayıtlı depoları salt okunur biçimde döndürür.
    pub fn repositories(&self) -> &[RepositoryRoot] {
        &self.repositories
    }

    /// Kayıtlı depo sayısını döndürür.
    pub fn len(&self) -> usize {
        self.repositories.len()
    }

    /// Envanterin boş olup olmadığını bildirir.
    pub fn is_empty(&self) -> bool {
        self.repositories.is_empty()
    }

    /// Kimliğine göre depo kaydı bulur.
    pub fn find(
        &self,
        repository_id: Uuid,
    ) -> Option<&RepositoryRoot> {
        self.repositories
            .iter()
            .find(|repository| {
                repository.id == repository_id
            })
    }

    /// Kök yoluna göre depo kaydı bulur.
    pub fn find_by_root(
        &self,
        root_path: impl AsRef<Path>,
    ) -> Option<&RepositoryRoot> {
        self.repositories
            .iter()
            .find(|repository| {
                repository.root_path
                    == root_path.as_ref()
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_complete_read_only_repository_root() {
        let repository = RepositoryRoot::new(
            "zanistarast-papers",
            "/projects/zanistarast-papers",
        );

        assert!(repository.is_complete());
        assert!(repository.read_only);

        assert!(repository.matches_root(
            "/projects/zanistarast-papers",
        ));
    }

    #[test]
    fn rejects_incomplete_repository_root() {
        let repository = RepositoryRoot::new(
            "",
            "",
        );

        assert!(!repository.is_complete());
    }

    #[test]
    fn registers_multiple_read_only_repositories() {
        let mut inventory = RepositoryInventory::new();

        let papers = RepositoryRoot::new(
            "zanistarast-papers",
            "/projects/zanistarast-papers",
        );

        let website = RepositoryRoot::new(
            "zanistarast-site",
            "/projects/zanistarast-site",
        );

        let papers_id = papers.id;

        assert!(inventory.register(papers));
        assert!(inventory.register(website));

        assert_eq!(inventory.len(), 2);
        assert!(!inventory.is_empty());
        assert!(inventory.find(papers_id).is_some());

        assert!(
            inventory
                .find_by_root(
                    "/projects/zanistarast-site",
                )
                .is_some()
        );
    }

    #[test]
    fn rejects_duplicate_repository_root() {
        let mut inventory = RepositoryInventory::new();

        assert!(inventory.register(
            RepositoryRoot::new(
                "primary",
                "/projects/zanistarast",
            ),
        ));

        assert!(!inventory.register(
            RepositoryRoot::new(
                "duplicate",
                "/projects/zanistarast",
            ),
        ));

        assert_eq!(inventory.len(), 1);
    }

    #[test]
    fn does_not_expose_mutable_repository_collection() {
        let mut inventory = RepositoryInventory::new();

        assert!(inventory.register(
            RepositoryRoot::new(
                "zanistarast-papers",
                "/projects/zanistarast-papers",
            ),
        ));

        let repositories = inventory.repositories();

        assert_eq!(repositories.len(), 1);
        assert!(repositories[0].read_only);
    }
}



