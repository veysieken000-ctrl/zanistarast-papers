use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::{
    RepositoryEntryKind,
    RepositoryFileInventory,
    RepositoryFileRecord,
    RepositoryRoot,
};

/// Repository taramasında bulunan tek bir dosyanın kaydı.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepositoryFile {
    pub relative_path: PathBuf,
    pub extension: Option<String>,
    pub size_bytes: u64,
}

/// Bir metin dosyasının salt okunur içerik tarama sonucudur.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
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
    pub repository_id: uuid::Uuid,
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

// Bir veya daha fazla depodan okunmuş metin içeriklerinin
/// ortak proje hafızasını temsil eder.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepositoryMemory {
    pub documents: Vec<RepositoryMemoryDocument>,
}

impl RepositoryMemory {
    pub fn document_count(&self) -> usize {
        self.documents.len()
    }

    pub fn is_empty(&self) -> bool {
        self.documents.is_empty()
    }
/// Belirtilen depo kimliğine ait hafıza belgelerini döndürür.
    pub fn documents_for_repository(
        &self,
        repository_id: uuid::Uuid,
    ) -> Vec<&RepositoryMemoryDocument> {
        self.documents
            .iter()
            .filter(|document| {
                document.repository_id == repository_id
            })
            .collect()
    }

    /// Belirtilen depo adına ait hafıza belgelerini döndürür.
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

    /// Depo kimliği ve göreli dosya yoluyla tek bir
    /// hafıza belgesi bulur.
    pub fn find_document(
        &self,
        repository_id: uuid::Uuid,
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
    ///
    /// Boş sorgular sonuç üretmez.
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
pub fn merge(&mut self, other: RepositoryMemory) {
    self.documents.extend(other.documents);
}
pub fn clear(&mut self) {
    self.documents.clear();
}
pub fn extend<I>(&mut self, documents: I)
where
    I: IntoIterator<Item = RepositoryMemoryDocument>,
{
    self.documents.extend(documents);
}
pub fn iter(
    &self,
) -> impl Iterator<Item = &RepositoryMemoryDocument> {
    self.documents.iter()
}
    
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryRelationKind {
    References,
    DependsOn,
    Extends,
    SharesConcepts,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryRelation {
    pub source_repository: uuid::Uuid,
    pub target_repository: uuid::Uuid,
    pub kind: RepositoryRelationKind,
    pub evidence: String,
}

/// Salt okunur repository taramasının sonucu.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RepositoryScanReport {
    pub root: PathBuf,
    pub files: Vec<RepositoryFile>,
    pub directory_count: usize,
    pub total_size_bytes: u64,
}

impl RepositoryScanReport {
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RepositoryRelationKind {
    References,
    DependsOn,
    Extends,
    SharesConcepts,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepositoryRelation {
    pub source_repository: uuid::Uuid,
    pub target_repository: uuid::Uuid,
    pub kind: RepositoryRelationKind,
    pub evidence: String,
}

/// Depoyu yalnızca okuyarak dosya envanteri çıkarır.
///
/// Bu tarayıcı:
/// - dosya oluşturmaz,
/// - dosya değiştirmez,
/// - dosya silmez,
/// - bulunan içerikleri yalnızca raporlar.
#[derive(Debug, Default)]
pub struct RepositoryScanner;

impl RepositoryScanner {
    pub fn new() -> Self {
        Self
    }

   /// Depo kökü içindeki tek bir UTF-8 metin dosyasını
/// salt okunur biçimde içerik düzeyinde tarar.
pub fn read_text_content(
    &self,
    repository: &RepositoryRoot,
    relative_path: impl AsRef<Path>,
) -> io::Result<RepositoryTextContent> {
    if !repository.is_complete()
        || !repository.read_only
    {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "repository must be complete and read-only",
        ));
    }

    let root = repository.root_path.canonicalize()?;
    let relative_path = relative_path.as_ref();

    if relative_path.as_os_str().is_empty()
        || relative_path.is_absolute()
    {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "relative file path is required",
        ));
    }

    let file_path = root.join(relative_path);
    let canonical_file_path = file_path.canonicalize()?;

    if !canonical_file_path.starts_with(&root) {
        return Err(io::Error::new(
            io::ErrorKind::PermissionDenied,
            "file path is outside repository root",
        ));
    }

    if !canonical_file_path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "repository entry is not a file",
        ));
    }

    let bytes = fs::read(&canonical_file_path)?;

    let content = String::from_utf8(bytes).map_err(|_| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            "repository file is not valid UTF-8 text",
        )
    })?;

    let line_count = content.lines().count();
    let character_count = content.chars().count();

    Ok(RepositoryTextContent {
        relative_path: relative_path.to_path_buf(),
        content,
        line_count,
        character_count,
    })
}

   /// Depodaki desteklenen bütün metin dosyalarını
/// salt okunur biçimde içerik düzeyinde tarar.
///
/// Geçerli metin uzantıları ile README, LICENSE,
/// Dockerfile ve Makefile gibi uzantısız metin dosyaları
/// okunur. Diğer dosya türleri atlanır.
pub fn read_all_text_contents(
    &self,
    repository: &RepositoryRoot,
) -> io::Result<Vec<RepositoryTextContent>> {
    let inventory = self.scan_inventory(repository)?;
    let mut contents = Vec::new();

    for record in inventory.records() {
        if !record.is_file()
            || !Self::is_supported_text_path(
                &record.relative_path,
            )
        {
            continue;
        }

        let content = self.read_text_content(
            repository,
            &record.relative_path,
        )?;

        contents.push(content);
    }

    Ok(contents)
}
/// Birden fazla deponun desteklenen metinlerini,
/// kaynak depo kimliklerini koruyarak ortak hafızada toplar.
pub fn build_memory(
    &self,
    repositories: &[RepositoryRoot],
) -> io::Result<RepositoryMemory> {
    let mut memory = RepositoryMemory::default();

    for repository in repositories {
        let contents =
            self.read_all_text_contents(repository)?;

        for text in contents {
            memory.documents.push(
                RepositoryMemoryDocument {
                    repository_id: repository.id,
                    repository_name:
                        repository.name.clone(),
                    text,
                },
            );
        }
    }

    Ok(memory)
}

/// Bir dosya yolunun proje hafızasına alınabilecek
/// desteklenen bir metin türü olup olmadığını bildirir.
fn is_supported_text_path(
    path: &Path,
) -> bool {
    let file_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default();

    if matches!(
        file_name,
        "README"
            | "LICENSE"
            | "Dockerfile"
            | "Makefile"
            | "CITATION"
            | "CHANGELOG"
    ) {
        return true;
    }

    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(str::to_ascii_lowercase);

    matches!(
        extension.as_deref(),
        Some(
            "md"
                | "txt"
                | "rs"
                | "toml"
                | "json"
                | "yaml"
                | "yml"
                | "xml"
                | "csv"
                | "tsv"
                | "html"
                | "css"
                | "js"
                | "ts"
                | "py"
                | "sh"
                | "tex"
                | "bib"
                | "org"
                | "rst"
                | "sql"
        )
    )
}
 
    /// Verilen kök klasörü salt okunur biçimde tarar.
    pub fn scan(&self, root: impl AsRef<Path>) -> io::Result<RepositoryScanReport> {
        let root = root.as_ref().canonicalize()?;
        let mut files = Vec::new();
        let mut directory_count = 0;

        Self::scan_directory(
            &root,
            &root,
            &mut files,
            &mut directory_count,
        )?;

        files.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));

        let total_size_bytes = files.iter().map(|file| file.size_bytes).sum();

        Ok(RepositoryScanReport {
            root,
            files,
            directory_count,
            total_size_bytes,
        })
    }

    fn scan_directory(
        root: &Path,
        current: &Path,
        files: &mut Vec<RepositoryFile>,
        directory_count: &mut usize,
    ) -> io::Result<()> {
        *directory_count += 1;

        let mut entries = fs::read_dir(current)?
            .collect::<Result<Vec<_>, io::Error>>()?;

        entries.sort_by_key(|entry| entry.path());

        for entry in entries {
            let path = entry.path();
            let file_type = entry.file_type()?;

            if file_type.is_symlink() {
                continue;
            }

            if file_type.is_dir() {
                Self::scan_directory(
                    root,
                    &path,
                    files,
                    directory_count,
                )?;
                continue;
            }

            if !file_type.is_file() {
                continue;
            }

            let metadata = entry.metadata()?;
            let relative_path = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_path_buf();

            let extension = path
                .extension()
                .and_then(|value| value.to_str())
                .map(str::to_owned);

            files.push(RepositoryFile {
                relative_path,
                extension,
                size_bytes: metadata.len(),
            });
        }

        Ok(())
    }
    
    /// Kayıtlı bir depo kökünü salt okunur biçimde tarayarak
    /// dosya ve dizin kayıtlarını RepositoryFileInventory
    /// modeline aktarır.
    ///
    /// Normal dosyalar için SHA-256 özeti hesaplanır.
    /// Sembolik bağlantılar takip edilmez.
    pub fn scan_inventory(
        &self,
        repository: &RepositoryRoot,
    ) -> io::Result<RepositoryFileInventory> {
        if !repository.is_complete()
            || !repository.read_only
        {
            return Err(io::Error::new(
                io::ErrorKind::PermissionDenied,
                "repository must be complete and read-only",
            ));
        }

        if !repository.root_path.is_dir() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "repository root directory was not found",
            ));
        }

        let root = repository.root_path.canonicalize()?;
        let mut inventory = RepositoryFileInventory::new();

        Self::scan_inventory_directory(
            repository,
            &root,
            &root,
            &mut inventory,
        )?;

        Ok(inventory)
    }

    fn scan_inventory_directory(
        repository: &RepositoryRoot,
        root: &Path,
        current: &Path,
        inventory: &mut RepositoryFileInventory,
    ) -> io::Result<()> {
        let mut entries = fs::read_dir(current)?
            .collect::<Result<Vec<_>, io::Error>>()?;

        entries.sort_by_key(|entry| entry.path());

        for entry in entries {
            let path = entry.path();
            let file_type = entry.file_type()?;

            if file_type.is_symlink() {
                continue;
            }

            let relative_path = path
                .strip_prefix(root)
                .map_err(|_| {
                    io::Error::new(
                        io::ErrorKind::InvalidData,
                        "repository entry is outside root",
                    )
                })?
                .to_path_buf();

            let metadata = entry.metadata()?;

            if file_type.is_dir() {
                let record = RepositoryFileRecord::new(
                    repository.id,
                    relative_path,
                    path.clone(),
                    RepositoryEntryKind::Directory,
                    0,
                    metadata.modified().ok(),
                );

                Self::register_inventory_record(
                    inventory,
                    record,
                )?;

                Self::scan_inventory_directory(
                    repository,
                    root,
                    &path,
                    inventory,
                )?;

                continue;
            }

            if !file_type.is_file() {
                continue;
            }

            let digest = Self::calculate_sha256(&path)?;

            let record = RepositoryFileRecord::new(
                repository.id,
                relative_path,
                path,
                RepositoryEntryKind::File,
                metadata.len(),
                metadata.modified().ok(),
            )
            .with_sha256(digest);

            Self::register_inventory_record(
                inventory,
                record,
            )?;
        }

        Ok(())
    }

    fn calculate_sha256(
        path: &Path,
    ) -> io::Result<String> {
        let bytes = fs::read(path)?;

        let mut hasher = Sha256::new();
        hasher.update(bytes);

        Ok(format!("{:x}", hasher.finalize()))
    }

    fn register_inventory_record(
        inventory: &mut RepositoryFileInventory,
        record: RepositoryFileRecord,
    ) -> io::Result<()> {
        if inventory.register(record) {
            Ok(())
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "repository inventory record was rejected",
            ))
        }
    }

    
    /// Birden fazla depoyu salt okunur biçimde tarar.
///
/// Başarısız olan depo diğerlerini durdurmaz.
/// Başarılı taramalar döndürülür.
pub fn scan_multiple(
    &self,
    repositories: &[RepositoryRoot],
) -> Vec<RepositoryFileInventory> {
    let mut inventories = Vec::new();

    for repository in repositories {
        if let Ok(inventory) =
            self.scan_inventory(repository)
        {
            inventories.push(inventory);
        }
    }

    inventories
}
pub fn scan_with_filter<F>(
    &self,
    repository: &RepositoryRoot,
    filter: F,
) -> io::Result<RepositoryFileInventory>
where
    F: Fn(&RepositoryFileRecord) -> bool,
{
    let inventory = self.scan_inventory(repository)?;
    let mut filtered_inventory =
        RepositoryFileInventory::new();

    for record in inventory.records() {
        if filter(record)
            && !filtered_inventory.register(record.clone())
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "filtered repository record was rejected",
            ));
        }
    }

    Ok(filtered_inventory)
}

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn scanner_creates_read_only_inventory() {
        let test_root = std::env::temp_dir().join(format!(
            "zanistarast-mira-scanner-{}",
            uuid::Uuid::new_v4()
        ));

        let nested = test_root.join("papers");
        fs::create_dir_all(&nested).expect("test directory should be created");
        fs::write(test_root.join("README.md"), "Zanistarast")
            .expect("test README should be written");
        fs::write(nested.join("hebun.md"), "Hebûn")
            .expect("test article should be written");

        let scanner = RepositoryScanner::new();
        let report = scanner
            .scan(&test_root)
            .expect("repository scan should succeed");

        assert_eq!(report.file_count(), 2);
        assert_eq!(report.directory_count, 2);
        assert!(report.total_size_bytes > 0);

        fs::remove_dir_all(&test_root)
            .expect("test directory should be removed");
    }
#[test]
    fn scanner_builds_hashed_repository_file_inventory() {
        let test_root = std::env::temp_dir().join(
            format!(
                "zanistarast-mira-inventory-{}",
                uuid::Uuid::new_v4(),
            ),
        );

        let nested = test_root.join("src");
        let source_path = nested.join("lib.rs");
        let readme_path = test_root.join("README.md");

        fs::create_dir_all(&nested)
            .expect("test directory should be created");

        fs::write(
            &source_path,
            b"pub fn hebun() {}",
        )
        .expect("source file should be written");

        fs::write(
            &readme_path,
            b"# Zanistarast",
        )
        .expect("README should be written");

        let original_source = fs::read(&source_path)
            .expect("source file should be readable");

        let repository = RepositoryRoot::new(
            "zanistarast-test",
            &test_root,
        );

        let scanner = RepositoryScanner::new();

        let inventory = scanner
            .scan_inventory(&repository)
            .expect("repository inventory scan should succeed");

        assert_eq!(inventory.file_count(), 2);
        assert_eq!(inventory.directory_count(), 1);
        assert_eq!(inventory.len(), 3);

        let source_record = inventory
            .find_by_relative_path(
                repository.id,
                "src/lib.rs",
            )
            .expect("source file should be inventoried");

        assert!(source_record.is_file());
        assert!(source_record.has_sha256());

        assert_eq!(
            source_record
                .sha256_digest
                .as_ref()
                .expect("SHA-256 digest should exist")
                .len(),
            64,
        );

        let directory_record = inventory
            .find_by_relative_path(
                repository.id,
                "src",
            )
            .expect("source directory should be inventoried");

        assert!(directory_record.is_directory());

        assert_eq!(
            fs::read(&source_path)
                .expect("source file should remain readable"),
            original_source,
        );

        fs::remove_dir_all(&test_root)
            .expect("test directory should be removed");
    }
#[test]
fn scanner_scans_multiple_repositories() {
    let scanner = RepositoryScanner::new();

    let repositories: Vec<RepositoryRoot> = Vec::new();

    let inventories =
        scanner.scan_multiple(&repositories);

    assert!(inventories.is_empty());
}
#[test]
fn scanner_reads_utf8_file_content_without_modification() {
    let test_root = std::env::temp_dir().join(
        format!(
            "zanistarast-mira-content-{}",
            uuid::Uuid::new_v4(),
        ),
    );

    fs::create_dir_all(&test_root)
        .expect("test repository should be created");

    let file_path = test_root.join("hebun.md");
    let original_content = "Hebûn\nZanabûn\nRasterast";

    fs::write(
        &file_path,
        original_content.as_bytes(),
    )
    .expect("test text file should be written");

    let repository = RepositoryRoot::new(
        "zanistarast-content-test",
        &test_root,
    );

    let scanner = RepositoryScanner::new();

    let scanned = scanner
        .read_text_content(
            &repository,
            "hebun.md",
        )
        .expect("UTF-8 content should be scanned");

    assert!(scanned.is_complete());
    assert!(!scanned.is_empty());
    assert_eq!(scanned.relative_path, PathBuf::from("hebun.md"));
    assert_eq!(scanned.content, original_content);
    assert_eq!(scanned.line_count, 3);
    assert_eq!(
        scanned.character_count,
        original_content.chars().count(),
    );

    assert_eq!(
        fs::read_to_string(&file_path)
            .expect("source file should remain readable"),
        original_content,
    );

    fs::remove_dir_all(&test_root)
        .expect("test repository should be removed");
}
#[test]
fn scanner_reads_all_supported_text_files() {
    let test_root = std::env::temp_dir().join(
        format!(
            "zanistarast-mira-all-content-{}",
            uuid::Uuid::new_v4(),
        ),
    );

    let source_directory = test_root.join("src");

    fs::create_dir_all(&source_directory)
        .expect("test repository should be created");

    fs::write(
        test_root.join("README.md"),
        "# Zanistarast",
    )
    .expect("README should be written");

    fs::write(
        source_directory.join("lib.rs"),
        "pub fn hebun() {}",
    )
    .expect("Rust source should be written");

    fs::write(
        test_root.join("image.png"),
        [0_u8, 159, 146, 150],
    )
    .expect("binary file should be written");

    let repository = RepositoryRoot::new(
        "zanistarast-content-inventory",
        &test_root,
    );

    let scanner = RepositoryScanner::new();

    let contents = scanner
        .read_all_text_contents(&repository)
        .expect("supported text files should be read");

    assert_eq!(contents.len(), 2);

    assert!(contents.iter().any(|content| {
        content.relative_path
            == Path::new("README.md")
            && content.content == "# Zanistarast"
    }));

    assert!(contents.iter().any(|content| {
        content.relative_path
            == Path::new("src/lib.rs")
            && content.content == "pub fn hebun() {}"
    }));

    assert!(!contents.iter().any(|content| {
        content.relative_path
            == Path::new("image.png")
    }));

    assert_eq!(
        fs::read_to_string(
            test_root.join("README.md"),
        )
        .expect("README should remain readable"),
        "# Zanistarast",
    );

    fs::remove_dir_all(&test_root)
        .expect("test repository should be removed");
}
#[test]
fn scanner_builds_repository_memory_with_source_identity() {
    let test_root = std::env::temp_dir().join(
        format!(
            "zanistarast-mira-memory-{}",
            uuid::Uuid::new_v4(),
        ),
    );

    fs::create_dir_all(&test_root)
        .expect("test repository should be created");

    fs::write(
        test_root.join("README.md"),
        "# Zanistarast Memory",
    )
    .expect("test document should be written");

    let repository = RepositoryRoot::new(
        "zanistarast-memory-test",
        &test_root,
    );

    let repository_id = repository.id;

    let scanner = RepositoryScanner::new();

    let memory = scanner
        .build_memory(&[repository])
        .expect("repository memory should be built");

    assert_eq!(memory.document_count(), 1);
    assert!(!memory.is_empty());

    let document = &memory.documents[0];

    assert!(document.is_complete());

    assert_eq!(
        document.repository_id,
        repository_id,
    );

    assert_eq!(
        document.repository_name,
        "zanistarast-memory-test",
    );

    assert_eq!(
        document.text.relative_path,
        PathBuf::from("README.md"),
    );

    assert_eq!(
        document.text.content,
        "# Zanistarast Memory",
    );

    fs::remove_dir_all(&test_root)
        .expect("test repository should be removed");
}
#[test]
    fn repository_memory_can_be_queried_by_source_path_and_text() {
        let first_root = std::env::temp_dir().join(
            format!(
                "zanistarast-memory-first-{}",
                uuid::Uuid::new_v4(),
            ),
        );

        let second_root = std::env::temp_dir().join(
            format!(
                "zanistarast-memory-second-{}",
                uuid::Uuid::new_v4(),
            ),
        );

        fs::create_dir_all(&first_root)
            .expect("first repository should be created");

        fs::create_dir_all(&second_root)
            .expect("second repository should be created");

        fs::write(
            first_root.join("README.md"),
            "Hebûn ve Zanabûn araştırması",
        )
        .expect("first document should be written");

        fs::write(
            second_root.join("README.md"),
            "Rasterast doğrulama sistemi",
        )
        .expect("second document should be written");

        let first_repository = RepositoryRoot::new(
            "hebun-zanabun",
            &first_root,
        );

        let second_repository = RepositoryRoot::new(
            "rasterast",
            &second_root,
        );

        let first_repository_id = first_repository.id;
        let second_repository_id = second_repository.id;

        let scanner = RepositoryScanner::new();

        let memory = scanner
            .build_memory(&[
                first_repository,
                second_repository,
            ])
            .expect("project memory should be built");

        assert_eq!(memory.document_count(), 2);

        assert_eq!(
            memory
                .documents_for_repository(
                    first_repository_id,
                )
                .len(),
            1,
        );

        assert_eq!(
            memory
                .documents_for_repository_name(
                    "rasterast",
                )
                .len(),
            1,
        );

        let first_document = memory
            .find_document(
                first_repository_id,
                Path::new("README.md"),
            )
            .expect("first document should be found");

        assert_eq!(
            first_document.repository_name,
            "hebun-zanabun",
        );

        assert_eq!(
            first_document.text.content,
            "Hebûn ve Zanabûn araştırması",
        );

        let second_document = memory
            .find_document(
                second_repository_id,
                Path::new("README.md"),
            )
            .expect("second document should be found");

        assert_eq!(
            second_document.text.content,
            "Rasterast doğrulama sistemi",
        );

        let search_results =
            memory.search_text("RASTERAST");

        assert_eq!(search_results.len(), 1);

        assert_eq!(
            search_results[0].repository_name,
            "rasterast",
        );

        assert!(memory.search_text(" ").is_empty());

        fs::remove_dir_all(first_root)
            .expect("first repository should be removed");

        fs::remove_dir_all(second_root)
            .expect("second repository should be removed");
    }

}


