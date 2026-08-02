use std::path::PathBuf;
use std::time::SystemTime;

use uuid::Uuid;

/// Truth Log içinde kaydedilebilecek güvenlik ve
/// doğrulama olayının türünü belirtir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TruthLogEventKind {
    /// Orijinal dosyanın hash kaydı oluşturulmuştur.
    OriginalHashRecorded,

    /// Revize dosyanın hash kaydı oluşturulmuştur.
    RevisedHashRecorded,

    /// Orijinal dosyanın bütünlüğü doğrulanmıştır.
    FileIntegrityVerified,

    /// Orijinal dosyada değişiklik belirlenmiştir.
    FileModificationDetected,

    /// Orijinal ve revize sürüm eşleştirilmiştir.
    FileVersionPairCreated,

    /// Diff raporu oluşturulmuştur.
    DiffReportGenerated,

    /// Diff raporu güvenlik doğrulamasından geçmiştir.
    DiffSecurityVerified,

    /// Diff raporunda güvenlik uyuşmazlığı bulunmuştur.
    DiffSecurityRejected,

    /// Rasterast güvenlik doğrulaması tamamlanmıştır.
    RasterastVerified,

    /// Rasterast güvenlik doğrulaması başarısız olmuştur.
    RasterastRejected,

    /// Müdebbir bir güvenlik veya sürüm kararını onaylamıştır.
    MudebbirApproved,

    /// Müdebbir bir güvenlik veya sürüm kararını reddetmiştir.
    MudebbirRejected,
}

/// Truth Log olayının önem seviyesini belirtir.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TruthLogSeverity {
    Information,
    Warning,
    Critical,
}

/// Güvenlik ve doğrulama sürecinde gerçekleşen tek bir
/// değiştirilemez olay kaydını temsil eder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TruthLogEntry {
    pub id: Uuid,
    pub event_kind: TruthLogEventKind,
    pub severity: TruthLogSeverity,
    pub subject_id: Option<Uuid>,
    pub file_path: Option<PathBuf>,
    pub message: String,
    pub evidence: Vec<String>,
    pub created_at: SystemTime,
}

impl TruthLogEntry {
    /// Yeni bir Truth Log kaydı oluşturur.
    pub fn new(
        event_kind: TruthLogEventKind,
        severity: TruthLogSeverity,
        subject_id: Option<Uuid>,
        file_path: Option<PathBuf>,
        message: impl Into<String>,
        created_at: SystemTime,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            event_kind,
            severity,
            subject_id,
            file_path,
            message: message.into(),
            evidence: Vec::new(),
            created_at,
        }
    }

    /// Olay kaydına doğrulama kanıtları ekler.
    pub fn with_evidence(
        mut self,
        evidence: Vec<String>,
    ) -> Self {
        self.evidence = evidence;
        self
    }

    /// Truth Log kaydının zorunlu alanlarının eksiksiz
    /// olup olmadığını bildirir.
    pub fn is_complete(&self) -> bool {
        !self.message.trim().is_empty()
    }

    /// Kaydın belirtilen konu kimliğine bağlı olup
    /// olmadığını bildirir.
    pub fn belongs_to_subject(
        &self,
        subject_id: Uuid,
    ) -> bool {
        self.subject_id == Some(subject_id)
    }

    /// Kaydın belirtilen dosya yoluna ait olup
    /// olmadığını bildirir.
    pub fn belongs_to_file(
        &self,
        file_path: impl Into<PathBuf>,
    ) -> bool {
        self.file_path
            .as_ref()
            .is_some_and(|stored| {
                stored == &file_path.into()
            })
    }

    /// Kaydın kritik bir güvenlik olayı olup
    /// olmadığını bildirir.
    pub fn is_critical(&self) -> bool {
        self.severity == TruthLogSeverity::Critical
    }

    /// Kaydın Müdebbir kararını temsil edip
    /// etmediğini bildirir.
    pub fn is_mudebbir_decision(&self) -> bool {
        matches!(
            self.event_kind,
            TruthLogEventKind::MudebbirApproved
                | TruthLogEventKind::MudebbirRejected
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_complete_truth_log_entry() {
        let subject_id = Uuid::new_v4();

        let entry = TruthLogEntry::new(
            TruthLogEventKind::OriginalHashRecorded,
            TruthLogSeverity::Information,
            Some(subject_id),
            Some(PathBuf::from("articles/hebun.md")),
            "Orijinal dosya SHA-256 kaydı oluşturuldu.",
            SystemTime::now(),
        )
        .with_evidence(vec![
            "algorithm=SHA-256".to_string(),
            "digest=0123456789abcdef".to_string(),
        ]);

        assert!(entry.is_complete());
        assert!(entry.belongs_to_subject(subject_id));

        assert!(
            entry.belongs_to_file(
                "articles/hebun.md",
            )
        );

        assert_eq!(entry.evidence.len(), 2);
        assert!(!entry.is_critical());
        assert!(!entry.is_mudebbir_decision());
    }

    #[test]
    fn critical_security_event_is_reported() {
        let entry = TruthLogEntry::new(
            TruthLogEventKind::FileModificationDetected,
            TruthLogSeverity::Critical,
            None,
            Some(PathBuf::from(
                "articles/hebun.md",
            )),
            "Orijinal dosyanın değiştirildiği tespit edildi.",
            SystemTime::now(),
        );

        assert!(entry.is_complete());
        assert!(entry.is_critical());
    }

    #[test]
    fn empty_message_makes_entry_incomplete() {
        let entry = TruthLogEntry::new(
            TruthLogEventKind::DiffReportGenerated,
            TruthLogSeverity::Information,
            None,
            None,
            "",
            SystemTime::now(),
        );

        assert!(!entry.is_complete());
    }

    #[test]
    fn mudebbir_decision_event_is_recognized() {
        let approved = TruthLogEntry::new(
            TruthLogEventKind::MudebbirApproved,
            TruthLogSeverity::Information,
            Some(Uuid::new_v4()),
            None,
            "Müdebbir sürüm değişikliğini onayladı.",
            SystemTime::now(),
        );

        let rejected = TruthLogEntry::new(
            TruthLogEventKind::MudebbirRejected,
            TruthLogSeverity::Warning,
            Some(Uuid::new_v4()),
            None,
            "Müdebbir sürüm değişikliğini reddetti.",
            SystemTime::now(),
        );

        assert!(approved.is_mudebbir_decision());
        assert!(rejected.is_mudebbir_decision());
    }

    #[test]
    fn unrelated_subject_and_file_are_rejected() {
        let entry = TruthLogEntry::new(
            TruthLogEventKind::DiffSecurityVerified,
            TruthLogSeverity::Information,
            Some(Uuid::new_v4()),
            Some(PathBuf::from(
                "articles/rasterast.md",
            )),
            "Diff güvenlik doğrulamasından geçti.",
            SystemTime::now(),
        );

        assert!(
            !entry.belongs_to_subject(
                Uuid::new_v4(),
            )
        );

        assert!(
            !entry.belongs_to_file(
                "articles/hebun.md",
            )
        );
    }
}


