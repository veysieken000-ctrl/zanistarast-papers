use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::publication_package::{
    PublicationRequest,
    PublicationTarget,
};

/// Müdebbir tarafından verilen yayın kararını temsil eder.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PublicationApprovalDecision {
    Approved,
    Rejected,
}

impl PublicationApprovalDecision {
    /// Kararın yayınlamaya izin verip vermediğini bildirir.
    pub fn permits_publication(self) -> bool {
        matches!(self, Self::Approved)
    }

    /// Kararın sabit metinsel adını döndürür.
    pub fn name(self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::Rejected => "rejected",
        }
    }
}

/// Müdebbir kararının gerekçesini temsil eder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalReason {
    AcademicQualityVerified,
    MetadataVerified,
    RightsAndLicenseVerified,
    SecurityVerified,
    RasterastVerified,
    InsufficientEvidence,
    MetadataIncomplete,
    RightsOrLicenseUnclear,
    SecurityRisk,
    RasterastRejected,
    Other(String),
}

impl ApprovalReason {
    /// Karar gerekçesinin sabit veya açıklamalı metnini döndürür.
    pub fn description(&self) -> String {
        match self {
            Self::AcademicQualityVerified => {
                "Akademik kalite doğrulandı.".to_string()
            }
            Self::MetadataVerified => {
                "Yayın metadata bilgileri doğrulandı.".to_string()
            }
            Self::RightsAndLicenseVerified => {
                "Telif ve lisans koşulları doğrulandı.".to_string()
            }
            Self::SecurityVerified => {
                "Güvenlik kontrolleri tamamlandı.".to_string()
            }
            Self::RasterastVerified => {
                "Rasterast doğrulaması başarıyla tamamlandı.".to_string()
            }
            Self::InsufficientEvidence => {
                "Yayın için yeterli kanıt bulunamadı.".to_string()
            }
            Self::MetadataIncomplete => {
                "Yayın metadata bilgileri eksik.".to_string()
            }
            Self::RightsOrLicenseUnclear => {
                "Telif veya lisans koşulları açık değil.".to_string()
            }
            Self::SecurityRisk => {
                "Yayın işlemi güvenlik riski taşıyor.".to_string()
            }
            Self::RasterastRejected => {
                "Rasterast doğrulaması yayını reddetti.".to_string()
            }
            Self::Other(reason) => reason.clone(),
        }
    }
}

/// Müdebbir tarafından verilen yayın kararının kayıt altına
/// alınmış hâlini temsil eder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationApprovalRecord {
    pub id: Uuid,
    pub request_id: Uuid,
    pub target: PublicationTarget,
    pub decision: PublicationApprovalDecision,
    pub reasons: Vec<ApprovalReason>,
    pub decided_by: String,
    pub decided_at: DateTime<Utc>,
    pub note: Option<String>,
}

impl PublicationApprovalRecord {
    /// Yeni bir yayın karar kaydı oluşturur.
    pub fn new(
        request_id: Uuid,
        target: PublicationTarget,
        decision: PublicationApprovalDecision,
        reasons: Vec<ApprovalReason>,
        decided_by: impl Into<String>,
        note: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            request_id,
            target,
            decision,
            reasons,
            decided_by: decided_by.into(),
            decided_at: Utc::now(),
            note,
        }
    }

    /// Kararın onay olup olmadığını bildirir.
    pub fn is_approved(&self) -> bool {
        self.decision.permits_publication()
    }

    /// Kararın ret olup olmadığını bildirir.
    pub fn is_rejected(&self) -> bool {
        matches!(
            self.decision,
            PublicationApprovalDecision::Rejected
        )
    }

    /// Karar kaydında en az bir gerekçe bulunup
    /// bulunmadığını bildirir.
    pub fn has_reasons(&self) -> bool {
        !self.reasons.is_empty()
    }

    /// Kararı veren tarafın geçerli olup olmadığını bildirir.
    pub fn has_valid_decider(&self) -> bool {
        !self.decided_by.trim().is_empty()
    }

    /// Karar kaydının temel alanlarının geçerli olup
    /// olmadığını bildirir.
    pub fn is_valid(&self) -> bool {
        self.has_reasons()
            && self.has_valid_decider()
    }
}
/// Yayın onay sürecinde oluşabilecek doğrulama hataları.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublicationApprovalError {
    InvalidRequest,
    TargetMismatch,
    MissingMudebbir,
    MissingReasons,
    InvalidDecision,
}

/// Yayın onay doğrulama sonucunu temsil eder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PublicationApprovalValidation {
    pub valid: bool,
    pub error: Option<PublicationApprovalError>,
}

impl PublicationApprovalValidation {
    /// Başarılı doğrulama sonucu oluşturur.
    pub fn success() -> Self {
        Self {
            valid: true,
            error: None,
        }
    }

    /// Başarısız doğrulama sonucu oluşturur.
    pub fn failure(
        error: PublicationApprovalError,
    ) -> Self {
        Self {
            valid: false,
            error: Some(error),
        }
    }
}

impl PublicationApprovalRecord {
    /// Karar kaydını doğrular.
    pub fn validate(
        &self,
        request: &PublicationRequest,
    ) -> PublicationApprovalValidation {
        if !request.is_ready() {
            return PublicationApprovalValidation::failure(
                PublicationApprovalError::InvalidRequest,
            );
        }

        if self.request_id != request.id {
        return PublicationApprovalValidation::failure(
        PublicationApprovalError::InvalidRequest,
    );
}
       
     if self.target != request.target {
    return PublicationApprovalValidation::failure(
        PublicationApprovalError::TargetMismatch,
    );
}
   
        
        if !self.has_valid_decider() {
            return PublicationApprovalValidation::failure(
                PublicationApprovalError::MissingMudebbir,
            );
        }

        if !self.has_reasons() {
            return PublicationApprovalValidation::failure(
                PublicationApprovalError::MissingReasons,
            );
        }

        PublicationApprovalValidation::success()
    }
}
/// Yayın onay servislerini sağlayıcıdan bağımsız hâle getiren arayüz.
pub trait PublicationApprovalService {
    fn approve(
        &self,
        request: &PublicationRequest,
        record: PublicationApprovalRecord,
    ) -> Result<
        PublicationApprovalRecord,
        PublicationApprovalError,
    >;
}

/// Varsayılan Müdebbir yayın onay servisi.
#[derive(Debug, Default)]
pub struct DefaultPublicationApprovalService;

impl PublicationApprovalService
    for DefaultPublicationApprovalService
{
    fn approve(
        &self,
        request: &PublicationRequest,
        record: PublicationApprovalRecord,
    ) -> Result<
        PublicationApprovalRecord,
        PublicationApprovalError,
    > {
        let validation = record.validate(request);

        if !validation.valid {
            return Err(
                validation
                    .error
                    .unwrap_or(
                        PublicationApprovalError::InvalidDecision,
                    ),
            );
        }

        Ok(record)
    }
}

impl PublicationApprovalRecord {
    /// Kararın yayınlamaya izin verip vermediğini bildirir.
    pub fn permits_publication(&self) -> bool {
        self.is_approved()
    }

    /// Kararın reddedilmiş olduğunu bildirir.
    pub fn denies_publication(&self) -> bool {
        self.is_rejected()
    }

    /// Karar gerekçelerini metin listesi olarak döndürür.
    pub fn reason_descriptions(
        &self,
    ) -> Vec<String> {
        self.reasons
            .iter()
            .map(ApprovalReason::description)
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    use crate::publication_package::{
        PublicationMetadata,
        PublicationPackage,
        PublicationRequest,
        PublicationTarget,
    };

    fn complete_package() -> PublicationPackage {
        PublicationPackage {
            title: "Rasterast Verification".to_string(),
            latex_source:
                "\\documentclass{article}".to_string(),
            pdf_bytes: b"%PDF-1.7\n".to_vec(),
            bibtex_source: Some(
                "@article{rasterast2026}".to_string(),
            ),
        }
    }

    fn complete_metadata() -> PublicationMetadata {
        PublicationMetadata::new(
            "Rasterast Verification",
            vec![
                "Veysi yê MALA SAF".to_string(),
            ],
            "Deterministic verification.",
            vec![
                "Rasterast".to_string(),
                "Zanistarast".to_string(),
            ],
            "tr",
            "CC-BY-4.0",
            "1.0.0",
        )
    }

    fn test_request_id() -> Uuid {
    Uuid::from_u128(
        0x12345678_1234_5678_1234_567812345678,
    )
}

fn approved_request() -> PublicationRequest {
    let mut request = PublicationRequest::new(
        PublicationTarget::Zenodo,
        complete_package(),
        complete_metadata(),
    );

    request.id = test_request_id();
    request.approve_by_mudebbir();

    request
}

fn approval_record(
    decision: PublicationApprovalDecision,
) -> PublicationApprovalRecord {
    PublicationApprovalRecord::new(
        test_request_id(),
        PublicationTarget::Zenodo,
        decision,
        vec![
            ApprovalReason::AcademicQualityVerified,
            ApprovalReason::RasterastVerified,
        ],
        "Müdebbir",
        None,
    )
}


    #[test]
    fn approval_record_is_valid() {
        let record = approval_record(
            PublicationApprovalDecision::Approved,
        );

        assert!(record.has_reasons());
        assert!(record.has_valid_decider());
        assert!(record.is_valid());
    }

    #[test]
    fn approval_record_reports_approved() {
        let record = approval_record(
            PublicationApprovalDecision::Approved,
        );

        assert!(record.is_approved());
        assert!(!record.is_rejected());
        assert!(record.permits_publication());
        assert!(!record.denies_publication());
    }

    #[test]
    fn rejected_record_reports_rejected() {
        let record = approval_record(
            PublicationApprovalDecision::Rejected,
        );

        assert!(!record.is_approved());
        assert!(record.is_rejected());
        assert!(!record.permits_publication());
        assert!(record.denies_publication());
    }

    #[test]
    fn approval_record_validation_succeeds() {
        let request = approved_request();

        let record = approval_record(
            PublicationApprovalDecision::Approved,
        );

        let validation =
            record.validate(&request);

        assert!(validation.valid);
        assert_eq!(validation.error, None);
    }

    #[test]
fn validation_rejects_mismatched_publication_target() {
    let request = approved_request();

    let record = PublicationApprovalRecord::new(
        request.id,
        PublicationTarget::Arxiv,
        PublicationApprovalDecision::Approved,
        vec![
            ApprovalReason::AcademicQualityVerified,
            ApprovalReason::RasterastVerified,
        ],
        "Müdebbir",
        None,
    );

    let validation = record.validate(&request);

    assert!(!validation.valid);

    assert_eq!(
        validation.error,
        Some(
            PublicationApprovalError::TargetMismatch,
        ),
    );
}
    
    #[test]
    fn validation_fails_for_invalid_request() {
        let request = PublicationRequest::new(
            PublicationTarget::Zenodo,
            complete_package(),
            complete_metadata(),
        );

        let record = approval_record(
            PublicationApprovalDecision::Approved,
        );

        let validation = record.validate(&request);

        assert!(!validation.valid);

        assert_eq!(
            validation.error,
            Some(
                PublicationApprovalError::InvalidRequest,
            ),
        );
    }

    #[test]
    fn validation_fails_without_reasons() {
       let request = approved_request();

let record = PublicationApprovalRecord::new(
    request.id,

            PublicationTarget::Zenodo,
            PublicationApprovalDecision::Approved,
            Vec::new(),
            "Müdebbir",
            None,
        );

        let validation = record.validate(&request);

        assert!(!validation.valid);

        assert_eq!(
            validation.error,
            Some(
                PublicationApprovalError::MissingReasons,
            ),
        );
    }

    #[test]
    fn validation_fails_without_decider() {
        let request = approved_request();

        let record = PublicationApprovalRecord::new(
    request.id,
            PublicationTarget::Zenodo,
            PublicationApprovalDecision::Approved,
            vec![
                ApprovalReason::AcademicQualityVerified,
            ],
            "",
            None,
        );

        let validation = record.validate(&request);

        assert!(!validation.valid);

        assert_eq!(
            validation.error,
            Some(
                PublicationApprovalError::MissingMudebbir,
            ),
        );
    }

    #[test]
    fn default_service_accepts_valid_record() {
        let request = approved_request();

        let record = approval_record(
            PublicationApprovalDecision::Approved,
        );

        let service =
    DefaultPublicationApprovalService;

        let result =
            service.approve(&request, record.clone());

        assert!(result.is_ok());

        let approved =
            result.expect("approval should succeed");

        assert_eq!(
            approved.decision,
            PublicationApprovalDecision::Approved,
        );

        assert!(approved.permits_publication());
    }

    #[test]
    fn default_service_rejects_invalid_record() {
        let request = approved_request();

        let record = PublicationApprovalRecord::new(
    request.id,
            PublicationTarget::Zenodo,
            PublicationApprovalDecision::Approved,
            Vec::new(),
            "",
            None,
        );

       let service =
    DefaultPublicationApprovalService; 

        let result =
            service.approve(&request, record);

        assert!(result.is_err());

        assert_eq!(
            result.err(),
            Some(
                PublicationApprovalError::MissingMudebbir,
            ),
        );
    }
#[test]
    fn approval_reason_descriptions_are_not_empty() {
        let reasons = vec![
            ApprovalReason::AcademicQualityVerified,
            ApprovalReason::MetadataVerified,
            ApprovalReason::RightsAndLicenseVerified,
            ApprovalReason::SecurityVerified,
            ApprovalReason::RasterastVerified,
        ];

        for reason in reasons {
            assert!(
                !reason.description().trim().is_empty()
            );
        }
    }

    #[test]
    fn other_reason_preserves_custom_text() {
        let reason = ApprovalReason::Other(
            "Özel değerlendirme".to_string(),
        );

        assert_eq!(
            reason.description(),
            "Özel değerlendirme"
        );
    }

    #[test]
    fn decision_names_are_stable() {
        assert_eq!(
            PublicationApprovalDecision::Approved.name(),
            "approved",
        );

        assert_eq!(
            PublicationApprovalDecision::Rejected.name(),
            "rejected",
        );
    }

    #[test]
    fn approved_decision_permits_publication() {
        assert!(
            PublicationApprovalDecision::Approved
                .permits_publication()
        );

        assert!(
            !PublicationApprovalDecision::Rejected
                .permits_publication()
        );
    }

    #[test]
    fn reason_descriptions_are_returned() {
        let record = approval_record(
            PublicationApprovalDecision::Approved,
        );

        let descriptions =
            record.reason_descriptions();

        assert_eq!(descriptions.len(), 2);

        assert!(
            descriptions[0].contains("Akademik")
        );

        assert!(
            descriptions[1].contains("Rasterast")
        );
    }

    #[test]
    fn approval_record_contains_note() {
        let record = PublicationApprovalRecord::new(
            Uuid::new_v4(),
            PublicationTarget::Zenodo,
            PublicationApprovalDecision::Approved,
            vec![
                ApprovalReason::AcademicQualityVerified,
            ],
            "Müdebbir",
            Some(
                "Yayınlanması uygundur."
                    .to_string(),
            ),
        );

        assert_eq!(
            record.note.as_deref(),
            Some("Yayınlanması uygundur.")
        );
    }
 #[test]
    fn approval_record_has_unique_identifier() {
        let first_record = approval_record(
            PublicationApprovalDecision::Approved,
        );

        let second_record = approval_record(
            PublicationApprovalDecision::Approved,
        );

        assert_ne!(
            first_record.id,
            second_record.id,
        );
    }

    #[test]
    fn approval_record_preserves_request_identifier() {
        let request_id = Uuid::new_v4();

        let record = PublicationApprovalRecord::new(
            request_id,
            PublicationTarget::Zenodo,
            PublicationApprovalDecision::Approved,
            vec![
                ApprovalReason::RasterastVerified,
            ],
            "Müdebbir",
            None,
        );

        assert_eq!(
            record.request_id,
            request_id,
        );
    }

    #[test]
    fn approval_record_preserves_publication_target() {
        let record = PublicationApprovalRecord::new(
            Uuid::new_v4(),
            PublicationTarget::Arxiv,
            PublicationApprovalDecision::Approved,
            vec![
                ApprovalReason::AcademicQualityVerified,
            ],
            "Müdebbir",
            None,
        );

        assert_eq!(
            record.target,
            PublicationTarget::Arxiv,
        );
    }

    #[test]
    fn approval_record_preserves_decider() {
        let record = PublicationApprovalRecord::new(
            Uuid::new_v4(),
            PublicationTarget::Zenodo,
            PublicationApprovalDecision::Approved,
            vec![
                ApprovalReason::SecurityVerified,
            ],
            "Veysi yê MALA SAF",
            None,
        );

        assert_eq!(
            record.decided_by,
            "Veysi yê MALA SAF",
        );
    }

    #[test]
    fn rejection_reasons_have_descriptions() {
        let reasons = vec![
            ApprovalReason::InsufficientEvidence,
            ApprovalReason::MetadataIncomplete,
            ApprovalReason::RightsOrLicenseUnclear,
            ApprovalReason::SecurityRisk,
            ApprovalReason::RasterastRejected,
        ];

        for reason in reasons {
            assert!(
                !reason.description().trim().is_empty()
            );
        }
    }

    #[test]
    fn validation_success_has_no_error() {
        let validation =
            PublicationApprovalValidation::success();

        assert!(validation.valid);
        assert_eq!(validation.error, None);
    }

    #[test]
    fn validation_failure_contains_error() {
        let validation =
            PublicationApprovalValidation::failure(
                PublicationApprovalError::InvalidRequest,
            );

        assert!(!validation.valid);

        assert_eq!(
            validation.error,
            Some(
                PublicationApprovalError::InvalidRequest,
            ),
        );
    }

    #[test]
    fn rejected_record_can_be_valid() {
        let request = approved_request();

      let record = PublicationApprovalRecord::new(
    request.id, 
            PublicationTarget::Zenodo,
            PublicationApprovalDecision::Rejected,
            vec![
                ApprovalReason::InsufficientEvidence,
                ApprovalReason::RasterastRejected,
            ],
            "Müdebbir",
            Some(
                "Kanıtlar güçlendirilmeden yayınlanamaz."
                    .to_string(),
            ),
        );

        let validation =
            record.validate(&request);

        assert!(record.is_valid());
        assert!(validation.valid);
        assert!(record.denies_publication());
        assert!(!record.permits_publication());
    }
}

