use crate::academic_output::AcademicOutput;
use uuid::Uuid;

/// Akademik çalışmanın yayınlanabilir dosyalarını taşır.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationPackage {
    pub title: String,
    pub latex_source: String,
    pub pdf_bytes: Vec<u8>,
    pub bibtex_source: Option<String>,
}

/// Dosya sisteminde güvenle kullanılabilecek temel dosya adı üretir.
pub fn sanitize_base_name(value: &str) -> String {
    let sanitized: String = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric()
                || character == '-'
                || character == '_'
            {
                character
            } else {
                '_'
            }
        })
        .collect();

    let sanitized = sanitized.trim_matches('_');

    if sanitized.is_empty() {
        "publication".to_string()
    } else {
        sanitized.to_string()
    }
}

/// Mira tarafından desteklenen yayın hedefleri.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationTarget {
    Zenodo,
    Arxiv,
    HuggingFace,
    PapersWithCode,
}

impl PublicationTarget {
    /// Yayın hedefinin sabit adını döndürür.
    pub fn name(self) -> &'static str {
        match self {
            Self::Zenodo => "Zenodo",
            Self::Arxiv => "arXiv",
            Self::HuggingFace => "Hugging Face",
            Self::PapersWithCode => "Papers With Code",
        }
    }
}
/// Yayın hedefine gönderilecek akademik metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationMetadata {
    pub title: String,
    pub authors: Vec<String>,
    pub abstract_text: String,
    pub keywords: Vec<String>,
    pub language: String,
    pub license: String,
    pub version: String,
}

impl PublicationMetadata {
    pub fn new(
        title: impl Into<String>,
        authors: Vec<String>,
        abstract_text: impl Into<String>,
        keywords: Vec<String>,
        language: impl Into<String>,
        license: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            authors,
            abstract_text: abstract_text.into(),
            keywords,
            language: language.into(),
            license: license.into(),
            version: version.into(),
        }
    }

    /// Zorunlu metadata alanlarının tamamlanıp
    /// tamamlanmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.title.trim().is_empty()
            && !self.authors.is_empty()
            && self
                .authors
                .iter()
                .all(|author| !author.trim().is_empty())
            && !self.abstract_text.trim().is_empty()
            && !self.language.trim().is_empty()
            && !self.license.trim().is_empty()
            && !self.version.trim().is_empty()
    }
}
/// Zanistarast yayın metadata bilgisinin
/// Zenodo için hazırlanmış temsilidir.
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
)]
pub struct ZenodoMetadata {
    pub title: String,
    pub creators: Vec<String>,
    pub description: String,
    pub keywords: Vec<String>,
    pub language: String,
    pub license: String,
    pub version: String,
}

impl ZenodoMetadata {
    /// Genel yayın metadata bilgisini
    /// Zenodo metadata modeline dönüştürür.
    pub fn from_publication_metadata(
        metadata: &PublicationMetadata,
    ) -> Self {
        Self {
            title: metadata.title.clone(),
            creators: metadata.authors.clone(),
            description: metadata.abstract_text.clone(),
            keywords: metadata.keywords.clone(),
            language: metadata.language.clone(),
            license: metadata.license.clone(),
            version: metadata.version.clone(),
        }
    }

    /// Zenodo metadata kaydının zorunlu
    /// bilgilerinin eksiksiz olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.title.trim().is_empty()
            && !self.creators.is_empty()
            && self
                .creators
                .iter()
                .all(|creator| !creator.trim().is_empty())
            && !self.description.trim().is_empty()
            && !self.language.trim().is_empty()
            && !self.license.trim().is_empty()
            && !self.version.trim().is_empty()
    }
}
/// Yayın işleminin başarısızlık nedenleri.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicationError {
    IncompletePackage,
    IncompleteMetadata,
    MissingMudebbirApproval,
    ProviderFailure(String),
}
/// Belirli bir yayın hedefine gönderilecek yayın isteği.
///
/// Akademik üretim onayı, dış dünyaya yayınlama izni değildir.
/// Gerçek yayınlama işlemi ayrıca açık Müdebbir onayı gerektirir.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationRequest {
    pub id: Uuid,
    pub target: PublicationTarget,
    pub package: PublicationPackage,
    pub metadata: PublicationMetadata,
    mudebbir_approved: bool,
}

impl PublicationRequest {
    pub fn new(
        target: PublicationTarget,
        package: PublicationPackage,
        metadata: PublicationMetadata,
    ) -> Self {
        Self {
    id: Uuid::new_v4(),
    target,
    package,
    metadata,
    mudebbir_approved: false,
}
    }

    /// Bu yayın isteğini Müdebbir onaylar.
    pub fn approve_by_mudebbir(&mut self) {
        self.mudebbir_approved = true;
    }

    /// Müdebbir onayı alınıp alınmadığını bildirir.
    pub fn is_approved_by_mudebbir(&self) -> bool {
        self.mudebbir_approved
    }

    /// Yayın isteğinin neden hazır olmadığını bildirir.
    /// Hazırsa None döndürür.
    pub fn validation_error(
        &self,
    ) -> Option<PublicationError> {
        if !self.package.is_ready_for_publication() {
            return Some(
                PublicationError::IncompletePackage,
            );
        }

        if !self.metadata.is_complete() {
            return Some(
                PublicationError::IncompleteMetadata,
            );
        }

        if !self.is_approved_by_mudebbir() {
            return Some(
                PublicationError::MissingMudebbirApproval,
            );
        }

        None
    }

    /// Yayın isteğinin tüm koşulları sağlayıp
    /// sağlamadığını bildirir.
    pub fn is_ready(&self) -> bool {
        self.validation_error().is_none()
    }
}
/// Bir yayın işleminin sonucunu temsil eder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationResult {
    pub target: PublicationTarget,
    pub success: bool,
    pub identifier: Option<String>,
    pub error: Option<PublicationError>,
}

impl PublicationResult {
    /// Başarılı yayın sonucu oluşturur.
    pub fn success(
        target: PublicationTarget,
        identifier: impl Into<String>,
    ) -> Self {
        Self {
            target,
            success: true,
            identifier: Some(identifier.into()),
            error: None,
        }
    }

    /// Başarısız yayın sonucu oluşturur.
    pub fn failure(
        target: PublicationTarget,
        error: PublicationError,
    ) -> Self {
        Self {
            target,
            success: false,
            identifier: None,
            error: Some(error),
        }
    }
}

/// Yayın servislerini sağlayıcıdan bağımsız hale getiren arayüz.
pub trait PublicationService {
    fn publish(
        &self,
        request: &PublicationRequest,
    ) -> PublicationResult;
}
/// Akademik çıktıdan nihai yayın paketi oluşturur.
pub fn build_publication_package(
    title: impl Into<String>,
    academic_output: &AcademicOutput,
    bibtex_source: Option<String>,
) -> PublicationPackage {
    PublicationPackage::from_academic_output(
        title,
        academic_output,
        bibtex_source,
    )
}

impl PublicationPackage {
    /// Akademik çıktıdan yayın paketi oluşturur.
    pub fn from_academic_output(
        title: impl Into<String>,
        output: &AcademicOutput,
        bibtex_source: Option<String>,
    ) -> Self {
        Self {
            title: title.into(),
            latex_source: output.latex_source.clone(),
            pdf_bytes: output.pdf_bytes.clone(),
            bibtex_source,
        }
    }

    /// Pakete sonradan BibTeX kaynağı ekler.
    pub fn with_bibtex(
        mut self,
        bibtex: impl Into<String>,
    ) -> Self {
        self.bibtex_source = Some(bibtex.into());
        self
    }

    /// Temel yayın dosyalarının eksiksiz olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.title.trim().is_empty()
            && !self.latex_source.trim().is_empty()
            && !self.pdf_bytes.is_empty()
    }

    /// Boş olmayan bir BibTeX kaynağı bulunup bulunmadığını bildirir.
    pub fn has_bibliography(&self) -> bool {
        self.bibtex_source
            .as_ref()
            .is_some_and(|value| !value.trim().is_empty())
    }

    /// Paketin teknik olarak yayınlanmaya hazır olup olmadığını bildirir.
    pub fn is_ready_for_publication(&self) -> bool {
        self.is_complete() && self.has_bibliography()
    }
}
/// Yayın paketini LaTeX, PDF ve varsa BibTeX dosyaları
/// olarak belirtilen dizine yazar.
pub fn export_publication_package(
    package: &PublicationPackage,
    output_directory: &str,
    base_name: &str,
) -> std::io::Result<Vec<std::path::PathBuf>> {
    let directory = std::path::Path::new(output_directory);
    let base_name = sanitize_base_name(base_name);

    std::fs::create_dir_all(directory)?;

    let tex_path = directory.join(
        format!("{base_name}.tex"),
    );

    let pdf_path = directory.join(
        format!("{base_name}.pdf"),
    );

    std::fs::write(
        &tex_path,
        &package.latex_source,
    )?;

    std::fs::write(
        &pdf_path,
        &package.pdf_bytes,
    )?;

    let mut written_files = vec![
        tex_path,
        pdf_path,
    ];

    if let Some(bibtex_source) =
        &package.bibtex_source
    {
        let bib_path = directory.join(
            format!("{base_name}.bib"),
        );

        std::fs::write(
            &bib_path,
            bibtex_source,
        )?;

        written_files.push(bib_path);
    }

    Ok(written_files)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::academic_output::{
        generate_academic_output,
        AcademicOutputInput,
    };

    struct MockPublicationService;

    impl PublicationService for MockPublicationService {
        fn publish(
            &self,
            request: &PublicationRequest,
        ) -> PublicationResult {
            if let Some(error) =
                request.validation_error()
            {
                PublicationResult::failure(
                    request.target,
                    error,
                )
            } else {
                PublicationResult::success(
                    request.target,
                    "mock-publication-id",
                )
            }
        }
    }

    fn complete_metadata() -> PublicationMetadata {
        PublicationMetadata::new(
            "Rasterast Verification",
            vec![
                "Veysi yê MALA SAF".to_string(),
            ],
            "Deterministic verification for academic publication.",
            vec![
                "Rasterast".to_string(),
                "Zanistarast".to_string(),
            ],
            "tr",
            "CC-BY-4.0",
            "1.0.0",
        )
    }

    fn complete_package() -> PublicationPackage {
        PublicationPackage {
            title:
                "Rasterast Verification".to_string(),
            latex_source:
                "\\documentclass{article}\n\
                 \\begin{document}\n\
                 Zanistarast.\n\
                 \\end{document}"
                    .to_string(),
            pdf_bytes: b"%PDF-1.7\n".to_vec(),
            bibtex_source: Some(
                "@article{rasterast2026}"
                    .to_string(),
            ),
        }
    }
#[test]
    fn complete_publication_package_is_ready() {
        let package = complete_package();

        assert!(package.is_complete());
        assert!(package.has_bibliography());
        assert!(package.is_ready_for_publication());
    }

    #[test]
    fn package_without_bibliography_is_not_ready() {
        let package = PublicationPackage {
            title: "Zanistarast".to_string(),
            latex_source:
                "\\documentclass{article}"
                    .to_string(),
            pdf_bytes: b"%PDF-1.7\n".to_vec(),
            bibtex_source: None,
        };

        assert!(package.is_complete());
        assert!(!package.has_bibliography());
        assert!(!package.is_ready_for_publication());
    }

    #[test]
    fn empty_pdf_prevents_package_completeness() {
        let package = PublicationPackage {
            title: "Zanistarast".to_string(),
            latex_source:
                "\\documentclass{article}"
                    .to_string(),
            pdf_bytes: Vec::new(),
            bibtex_source: None,
        };

        assert!(!package.is_complete());
        assert!(!package.is_ready_for_publication());
    }

    #[test]
    fn builds_package_from_academic_output() {
        let output = generate_academic_output(
            AcademicOutputInput {
                title: "Rasterast".to_string(),
                author:
                    "Veysi yê MALA SAF"
                        .to_string(),
                abstract_text:
                    "Deterministic verification."
                        .to_string(),
                body:
                    "\\section{Intro}\nContent."
                        .to_string(),
                bibliography: None,
            },
        );

        let package =
            PublicationPackage::from_academic_output(
                "Rasterast",
                &output,
                Some(
                    "@article{rasterast2026}"
                        .to_string(),
                ),
            );

        assert_eq!(package.title, "Rasterast");

        assert_eq!(
            package.latex_source,
            output.latex_source,
        );

        assert_eq!(
            package.pdf_bytes,
            output.pdf_bytes,
        );

        assert!(package.is_ready_for_publication());
    }

    #[test]
    fn can_attach_bibtex_after_creation() {
        let output = generate_academic_output(
            AcademicOutputInput {
                title: "Rasterast".to_string(),
                author:
                    "Veysi yê MALA SAF"
                        .to_string(),
                abstract_text: String::new(),
                body: String::new(),
                bibliography: None,
            },
        );

        let package =
            PublicationPackage::from_academic_output(
                "Rasterast",
                &output,
                None,
            )
            .with_bibtex(
                "@article{rasterast2026}",
            );

        assert!(package.has_bibliography());

        assert_eq!(
            package.bibtex_source.as_deref(),
            Some("@article{rasterast2026}"),
        );
    }

    #[test]
    fn unapproved_request_reports_approval_error() {
        let request = PublicationRequest::new(
            PublicationTarget::Zenodo,
            complete_package(),
            complete_metadata(),
        );

        assert!(!request.is_ready());

        assert_eq!(
            request.validation_error(),
            Some(
                PublicationError::
                    MissingMudebbirApproval,
            ),
        );
    }

    #[test]
    fn approved_complete_request_is_ready() {
        let mut request = PublicationRequest::new(
            PublicationTarget::Zenodo,
            complete_package(),
            complete_metadata(),
        );

        request.approve_by_mudebbir();

        assert!(
            request.is_approved_by_mudebbir()
        );

        assert!(request.is_ready());

        assert_eq!(
            request.validation_error(),
            None,
        );
    }

    #[test]
    fn incomplete_package_reports_package_error() {
        let package = PublicationPackage {
            title:
                "Rasterast Verification".to_string(),
            latex_source:
                "\\documentclass{article}"
                    .to_string(),
            pdf_bytes: b"%PDF-1.7\n".to_vec(),
            bibtex_source: None,
        };

        let mut request = PublicationRequest::new(
            PublicationTarget::Zenodo,
            package,
            complete_metadata(),
        );

        request.approve_by_mudebbir();

        assert!(!request.is_ready());

        assert_eq!(
            request.validation_error(),
            Some(
                PublicationError::IncompletePackage,
            ),
        );
    }

    #[test]
    fn incomplete_metadata_reports_metadata_error() {
        let metadata = PublicationMetadata::new(
            "",
            Vec::new(),
            "",
            Vec::new(),
            "",
            "",
            "",
        );

        let mut request = PublicationRequest::new(
            PublicationTarget::Zenodo,
            complete_package(),
            metadata,
        );

        request.approve_by_mudebbir();

        assert!(!request.is_ready());

        assert_eq!(
            request.validation_error(),
            Some(
                PublicationError::IncompleteMetadata,
            ),
        );
    }
 #[test]
    fn mock_service_publishes_approved_request() {
        let mut request = PublicationRequest::new(
            PublicationTarget::Zenodo,
            complete_package(),
            complete_metadata(),
        );

        request.approve_by_mudebbir();

        let service = MockPublicationService;
        let result = service.publish(&request);

        assert!(result.success);

        assert_eq!(
            result.target,
            PublicationTarget::Zenodo,
        );

        assert_eq!(
            result.identifier.as_deref(),
            Some("mock-publication-id"),
        );

        assert_eq!(result.error, None);
    }

    #[test]
    fn mock_service_rejects_unapproved_request() {
        let request = PublicationRequest::new(
            PublicationTarget::Zenodo,
            complete_package(),
            complete_metadata(),
        );

        let service = MockPublicationService;
        let result = service.publish(&request);

        assert!(!result.success);
        assert_eq!(result.identifier, None);

        assert_eq!(
            result.error,
            Some(
                PublicationError::MissingMudebbirApproval,
            ),
        );
    }

    #[test]
    fn publication_result_represents_success() {
        let result = PublicationResult::success(
            PublicationTarget::Zenodo,
            "10.5281/zenodo.1234567",
        );

        assert!(result.success);

        assert_eq!(
            result.identifier.as_deref(),
            Some("10.5281/zenodo.1234567"),
        );

        assert_eq!(result.error, None);
    }

#[test]
fn publication_requests_have_unique_identifiers() {
    let first_request = PublicationRequest::new(
        PublicationTarget::Zenodo,
        complete_package(),
        complete_metadata(),
    );

    let second_request = PublicationRequest::new(
        PublicationTarget::Zenodo,
        complete_package(),
        complete_metadata(),
    );

    assert_ne!(
        first_request.id,
        second_request.id,
    );
}
    
    #[test]
    fn publication_result_represents_failure() {
        let result = PublicationResult::failure(
            PublicationTarget::Zenodo,
            PublicationError::MissingMudebbirApproval,
        );

        assert!(!result.success);
        assert_eq!(result.identifier, None);

        assert_eq!(
            result.error,
            Some(
                PublicationError::MissingMudebbirApproval,
            ),
        );
    }

    #[test]
    fn provider_failure_can_carry_message() {
        let result = PublicationResult::failure(
            PublicationTarget::Arxiv,
            PublicationError::ProviderFailure(
                "Sağlayıcı yanıt vermedi.".to_string(),
            ),
        );

        assert!(!result.success);

        assert_eq!(
            result.error,
            Some(
                PublicationError::ProviderFailure(
                    "Sağlayıcı yanıt vermedi.".to_string(),
                ),
            ),
        );
    }

    #[test]
    fn publication_metadata_reports_complete_state() {
        let metadata = complete_metadata();

        assert!(metadata.is_complete());

        assert_eq!(
            metadata.title,
            "Rasterast Verification",
        );

        assert_eq!(
            metadata.authors,
            vec![
                "Veysi yê MALA SAF".to_string(),
            ],
        );
    }

    #[test]
    fn publication_metadata_rejects_missing_fields() {
        let metadata = PublicationMetadata::new(
            "",
            Vec::new(),
            "",
            Vec::new(),
            "",
            "",
            "",
        );

        assert!(!metadata.is_complete());
    }

    #[test]
    fn publication_targets_have_stable_names() {
        assert_eq!(
            PublicationTarget::Zenodo.name(),
            "Zenodo",
        );

        assert_eq!(
            PublicationTarget::Arxiv.name(),
            "arXiv",
        );

        assert_eq!(
            PublicationTarget::HuggingFace.name(),
            "Hugging Face",
        );

        assert_eq!(
            PublicationTarget::PapersWithCode.name(),
            "Papers With Code",
        );
    }

    #[test]
    fn sanitizes_publication_file_name() {
        assert_eq!(
            sanitize_base_name(
                "Rasterast Verification/2026",
            ),
            "Rasterast_Verification_2026",
        );

        assert_eq!(
            sanitize_base_name(
                "zanistarast-paper_v1",
            ),
            "zanistarast-paper_v1",
        );

        assert_eq!(
            sanitize_base_name("///"),
            "publication",
        );
    }

    #[test]
    fn builds_final_publication_package() {
        let academic_output = generate_academic_output(
            AcademicOutputInput {
                title: "Rasterast Verification".to_string(),
                author: "Veysi yê MALA SAF".to_string(),
                abstract_text:
                    "Deterministic verification.".to_string(),
                body:
                    "\\section{Introduction}\nZanistarast."
                        .to_string(),
                bibliography: Some(
                    "references".to_string(),
                ),
            },
        );

        let package = build_publication_package(
            "Rasterast Verification",
            &academic_output,
            Some(
                "@article{rasterast2026}".to_string(),
            ),
        );

        assert_eq!(
            package.title,
            "Rasterast Verification",
        );

        assert_eq!(
            package.latex_source,
            academic_output.latex_source,
        );

        assert_eq!(
            package.pdf_bytes,
            academic_output.pdf_bytes,
        );

        assert!(package.is_ready_for_publication());
    }

    #[test]
    fn exports_publication_package_files() {
        let package = complete_package();

        let output_directory =
            std::env::temp_dir().join(format!(
                "mira-publication-package-{}",
                std::process::id(),
            ));

        let output_directory_text =
            output_directory
                .to_str()
                .expect(
                    "Temporary directory must be valid UTF-8.",
                );

        let written_files =
            export_publication_package(
                &package,
                output_directory_text,
                "Rasterast Verification/2026",
            )
            .expect(
                "Publication package must be exported.",
            );

        assert_eq!(written_files.len(), 3);

        assert!(
            output_directory
                .join(
                    "Rasterast_Verification_2026.tex",
                )
                .exists()
        );

        assert!(
            output_directory
                .join(
                    "Rasterast_Verification_2026.pdf",
                )
                .exists()
        );

        assert!(
            output_directory
                .join(
                    "Rasterast_Verification_2026.bib",
                )
                .exists()
        );

        std::fs::remove_dir_all(&output_directory)
            .expect(
                "Temporary directory must be removed.",
            );
    }
#[test]
    fn prepares_complete_zenodo_metadata() {
        let metadata = PublicationMetadata::new(
            "Rasterast Verification",
            vec![
                "Veysi yê MALA SAF".to_string(),
            ],
            "Deterministic verification for academic publication.",
            vec![
                "Rasterast".to_string(),
                "Zanistarast".to_string(),
            ],
            "tr",
            "CC-BY-4.0",
            "1.0.0",
        );

        let zenodo =
            ZenodoMetadata::from_publication_metadata(
                &metadata,
            );

        assert!(zenodo.is_complete());

        assert_eq!(
            zenodo.title,
            "Rasterast Verification",
        );

        assert_eq!(
            zenodo.creators,
            vec![
                "Veysi yê MALA SAF".to_string(),
            ],
        );

        assert_eq!(zenodo.language, "tr");
        assert_eq!(zenodo.license, "CC-BY-4.0");
        assert_eq!(zenodo.version, "1.0.0");
    }
#[test]
    fn incomplete_zenodo_metadata_is_rejected() {
        let metadata = PublicationMetadata::new(
            "",
            Vec::new(),
            "",
            Vec::new(),
            "",
            "",
            "",
        );

        let zenodo =
            ZenodoMetadata::from_publication_metadata(
                &metadata,
            );

        assert!(!zenodo.is_complete());
    }

}




