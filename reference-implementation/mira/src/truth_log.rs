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

    /// Geçersiz dosya sürüm çifti reddedilmiştir.
    FileVersionPairRejected,
    
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
    pub previous_chain_digest: Option<String>,
    pub chain_digest: String,
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
            previous_chain_digest: None,
            chain_digest: String::new(),
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

/// Güvenlik ve doğrulama olaylarının sıralı
/// Truth Log koleksiyonunu temsil eder.
#[derive(Debug, Clone, Default)]
pub struct TruthLog {
    entries: Vec<TruthLogEntry>,
}

impl TruthLog {
    /// Boş bir Truth Log oluşturur.
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
     
    /// Eksiksiz ve daha önce kaydedilmemiş bir
    /// Truth Log olayını koleksiyona ekler.
    /// Truth Log kaydının zincir özetini SHA-256 ile hesaplar.
fn calculate_chain_digest(
    entry: &TruthLogEntry,
    previous_chain_digest: Option<&str>,
) -> String {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();

    hasher.update(entry.id.as_bytes());
    hasher.update(format!("{:?}", entry.event_kind));
    hasher.update(format!("{:?}", entry.severity));

    if let Some(subject_id) = entry.subject_id {
        hasher.update(subject_id.as_bytes());
    }

    if let Some(file_path) = &entry.file_path {
        hasher.update(
            file_path.to_string_lossy().as_bytes(),
        );
    }

    hasher.update(entry.message.as_bytes());

    for evidence in &entry.evidence {
        hasher.update(evidence.as_bytes());
    }

    if let Some(previous_digest) = previous_chain_digest {
        hasher.update(previous_digest.as_bytes());
    }

    format!("{:x}", hasher.finalize())
}
   /// Eksiksiz ve daha önce kaydedilmemiş bir
/// Truth Log olayını zincire ekler.
pub fn append(
    &mut self,
    mut entry: TruthLogEntry,
) -> bool {
    if !entry.is_complete() {
        return false;
    }

    if self
        .entries
        .iter()
        .any(|stored| stored.id == entry.id)
    {
        return false;
    }

    let previous_chain_digest = self
        .entries
        .last()
        .map(|stored| stored.chain_digest.clone());

    entry.previous_chain_digest =
        previous_chain_digest.clone();

    entry.chain_digest = Self::calculate_chain_digest(
        &entry,
        previous_chain_digest.as_deref(),
    );

    self.entries.push(entry);
    true
}
      /// Eksiksiz ve daha önce kaydedilmemiş bir
/// Truth Log olayını zincire ekler.
pub fn append(
    &mut self,
    mut entry: TruthLogEntry,
) -> bool {
    if !entry.is_complete() {
        return false;
    }

    if self
        .entries
        .iter()
        .any(|stored| stored.id == entry.id)
    {
        return false;
    }

    let previous_chain_digest = self
        .entries
        .last()
        .map(|stored| stored.chain_digest.clone());

    entry.previous_chain_digest =
        previous_chain_digest.clone();

    entry.chain_digest = Self::calculate_chain_digest(
        &entry,
        previous_chain_digest.as_deref(),
    );

    self.entries.push(entry);
    true
}



    /// Truth Log kayıt zincirinin baştan sona
/// değiştirilmeden korunduğunu doğrular.
pub fn verify_chain(&self) -> bool {
    let mut expected_previous_digest:
        Option<&str> = None;

    for entry in &self.entries {
        if entry.previous_chain_digest.as_deref()
            != expected_previous_digest
        {
            return false;
        }

        let expected_digest =
            Self::calculate_chain_digest(
                entry,
                expected_previous_digest,
            );

        if entry.chain_digest != expected_digest {
            return false;
        }

        expected_previous_digest =
            Some(entry.chain_digest.as_str());
    }

    true
}

    /// Truth Log zincirinin son kayıt özetini döndürür.
pub fn latest_chain_digest(
    &self,
) -> Option<&str> {
    self.entries
        .last()
        .map(|entry| entry.chain_digest.as_str())
}

    /// Bütün kayıtları salt okunur biçimde döndürür.
    pub fn entries(&self) -> &[TruthLogEntry] {
        &self.entries
    }

    /// Kayıt sayısını döndürür.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Truth Log’un boş olup olmadığını bildirir.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Kimliğine göre bir kayıt bulur.
    pub fn find(
        &self,
        entry_id: Uuid,
    ) -> Option<&TruthLogEntry> {
        self.entries
            .iter()
            .find(|entry| entry.id == entry_id)
    }

    /// Konu kimliğine bağlı kayıtları döndürür.
    pub fn entries_for_subject(
        &self,
        subject_id: Uuid,
    ) -> Vec<&TruthLogEntry> {
        self.entries
            .iter()
            .filter(|entry| {
                entry.belongs_to_subject(subject_id)
            })
            .collect()
    }

    /// Dosya yoluna bağlı kayıtları döndürür.
    pub fn entries_for_file(
        &self,
        file_path: impl Into<PathBuf>,
    ) -> Vec<&TruthLogEntry> {
        let file_path = file_path.into();

        self.entries
            .iter()
            .filter(|entry| {
                entry
                    .file_path
                    .as_ref()
                    .is_some_and(|stored| {
                        stored == &file_path
                    })
            })
            .collect()
    }

    /// Kritik güvenlik kayıtlarını döndürür.
    pub fn critical_entries(
        &self,
    ) -> Vec<&TruthLogEntry> {
        self.entries
            .iter()
            .filter(|entry| entry.is_critical())
            .collect()
    }
/// Bir dosya hash kaydını Truth Log olayına dönüştürerek
/// koleksiyona ekler.
///
/// Eksik veya geçersiz hash kayıtları kabul edilmez.
pub fn record_file_hash(
    &mut self,
    record: &crate::FileHashRecord,
    subject_id: Option<Uuid>,
    created_at: SystemTime,
) -> bool {
    if !record.is_complete() {
        return false;
    }

    let event_kind = match record.role {
        crate::FileHashRole::Original => {
            TruthLogEventKind::OriginalHashRecorded
        }
        crate::FileHashRole::Revised => {
            TruthLogEventKind::RevisedHashRecorded
        }
    };

    let message = match record.role {
        crate::FileHashRole::Original => {
            "Orijinal dosya hash kaydı oluşturuldu."
        }
        crate::FileHashRole::Revised => {
            "Revize dosya hash kaydı oluşturuldu."
        }
    };

    let entry = TruthLogEntry::new(
        event_kind,
        TruthLogSeverity::Information,
        subject_id,
        Some(record.path.clone()),
        message,
        created_at,
    )
    .with_evidence(vec![
        format!("algorithm={}", record.algorithm),
        format!("digest={}", record.digest),
    ]);

    self.append(entry)
}

   /// Dosya bütünlük doğrulama sonucunu Truth Log olayına
/// dönüştürerek koleksiyona ekler.
pub fn record_file_integrity(
    &mut self,
    report: &crate::FileIntegrityReport,
    subject_id: Option<Uuid>,
    created_at: SystemTime,
) -> bool {
    let (
        event_kind,
        severity,
        message,
    ) = match report.status {
        crate::FileIntegrityStatus::Intact => (
            TruthLogEventKind::FileIntegrityVerified,
            TruthLogSeverity::Information,
            "Orijinal dosyanın bütünlüğü doğrulandı.",
        ),

        crate::FileIntegrityStatus::Modified => (
            TruthLogEventKind::FileModificationDetected,
            TruthLogSeverity::Critical,
            "Orijinal dosyada içerik değişikliği tespit edildi.",
        ),

        crate::FileIntegrityStatus::AlgorithmMismatch => (
            TruthLogEventKind::FileModificationDetected,
            TruthLogSeverity::Warning,
            "Hash algoritmaları arasında uyuşmazlık tespit edildi.",
        ),

        crate::FileIntegrityStatus::InvalidRecord => (
            TruthLogEventKind::FileModificationDetected,
            TruthLogSeverity::Critical,
            "Dosya bütünlük doğrulamasında geçersiz hash kaydı bulundu.",
        ),
    };

    let entry = TruthLogEntry::new(
        event_kind,
        severity,
        subject_id,
        Some(report.original.path.clone()),
        message,
        created_at,
    )
    .with_evidence(vec![
        format!("status={:?}", report.status),
        format!(
            "original_algorithm={}",
            report.original.algorithm,
        ),
        format!(
            "original_digest={}",
            report.original.digest,
        ),
        format!(
            "current_algorithm={}",
            report.current.algorithm,
        ),
        format!(
            "current_digest={}",
            report.current.digest,
        ),
    ]);

    self.append(entry)
}

    /// Diff raporunun sürüm çiftiyle güvenlik doğrulama
/// sonucunu Truth Log olayına dönüştürerek kaydeder.
pub fn record_diff_security(
    &mut self,
    report: &crate::FileDiffReport,
    pair: &crate::FileVersionPair,
    subject_id: Option<Uuid>,
    created_at: SystemTime,
) -> bool {
    let status = report.security_status(pair);

    let (
        event_kind,
        severity,
        message,
    ) = match status {
        crate::FileDiffSecurityStatus::Verified => (
            TruthLogEventKind::DiffSecurityVerified,
            TruthLogSeverity::Information,
            "Diff raporu güvenlik doğrulamasından geçti.",
        ),

        crate::FileDiffSecurityStatus::InvalidVersionPair => (
            TruthLogEventKind::DiffSecurityRejected,
            TruthLogSeverity::Warning,
            "Diff raporu geçersiz sürüm çifti nedeniyle reddedildi.",
        ),

        crate::FileDiffSecurityStatus::PathMismatch => (
            TruthLogEventKind::DiffSecurityRejected,
            TruthLogSeverity::Critical,
            "Diff raporu ile sürüm çifti dosya yolları eşleşmedi.",
        ),

        crate::FileDiffSecurityStatus::InconsistentDiff => (
            TruthLogEventKind::DiffSecurityRejected,
            TruthLogSeverity::Critical,
            "Diff raporunun satır değişikliği kayıtları tutarsızdır.",
        ),

        crate::FileDiffSecurityStatus::HashDiffMismatch => (
            TruthLogEventKind::DiffSecurityRejected,
            TruthLogSeverity::Critical,
            "Hash sonucu ile diff değişiklik sonucu çelişmektedir.",
        ),
    };

    let entry = TruthLogEntry::new(
        event_kind,
        severity,
        subject_id,
        Some(report.original_path.clone()),
        message,
        created_at,
    )
    .with_evidence(vec![
        format!("security_status={status:?}"),
        format!(
            "original_path={}",
            report.original_path.display(),
        ),
        format!(
            "revised_path={}",
            report.revised_path.display(),
        ),
        format!(
            "added_count={}",
            report.added_count(),
        ),
        format!(
            "removed_count={}",
            report.removed_count(),
        ),
        format!(
            "modified_count={}",
            report.modified_count(),
        ),
        format!(
            "unchanged_count={}",
            report.unchanged_count(),
        ),
    ]);

    self.append(entry)
}

    /// Dosya sürüm çiftinin doğrulama sonucunu
/// Truth Log olayına dönüştürerek kaydeder.
pub fn record_file_version_pair(
    &mut self,
    pair: &crate::FileVersionPair,
    subject_id: Option<Uuid>,
    created_at: SystemTime,
) -> bool {
    let (
        event_kind,
        severity,
        message,
    ) = if pair.is_matched() {
        (
            TruthLogEventKind::FileVersionPairCreated,
            TruthLogSeverity::Information,
            "Doğrulanmış dosya sürüm çifti oluşturuldu.",
        )
    } else {
        (
            TruthLogEventKind::FileVersionPairRejected,
            TruthLogSeverity::Warning,
            "Geçersiz dosya sürüm çifti oluşturulmaya çalışıldı.",
        )
    };

    let entry = TruthLogEntry::new(
        event_kind,
        severity,
        subject_id,
        Some(pair.original.path.clone()),
        message,
        created_at,
    )
    .with_evidence(vec![
        format!(
            "original_path={}",
            pair.original.path.display(),
        ),
        format!(
            "revised_path={}",
            pair.revised.path.display(),
        ),
        format!("matched={}", pair.is_matched()),
        format!(
            "original_algorithm={}",
            pair.original.algorithm,
        ),
        format!(
            "revised_algorithm={}",
            pair.revised.algorithm,
        ),
    ]);

    self.append(entry)
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
   
#[test]
fn truth_log_stores_complete_entry() {
    let mut truth_log = TruthLog::new();

    let entry = TruthLogEntry::new(
        TruthLogEventKind::OriginalHashRecorded,
        TruthLogSeverity::Information,
        None,
        Some(PathBuf::from(
            "articles/hebun.md",
        )),
        "Orijinal hash kaydı oluşturuldu.",
        SystemTime::now(),
    );

    let entry_id = entry.id;

    assert!(truth_log.append(entry));
    assert_eq!(truth_log.len(), 1);
    assert!(!truth_log.is_empty());

    assert!(
        truth_log.find(entry_id).is_some()
    );
}
}
#[test]
fn truth_log_rejects_incomplete_entry() {
    let mut truth_log = TruthLog::new();

    let entry = TruthLogEntry::new(
        TruthLogEventKind::DiffReportGenerated,
        TruthLogSeverity::Information,
        None,
        None,
        "",
        SystemTime::now(),
    );

    assert!(!truth_log.append(entry));
    assert!(truth_log.is_empty());
}

#[test]
fn truth_log_rejects_duplicate_entry_id() {
    let mut truth_log = TruthLog::new();

    let entry = TruthLogEntry::new(
        TruthLogEventKind::FileIntegrityVerified,
        TruthLogSeverity::Information,
        None,
        Some(PathBuf::from(
            "articles/hebun.md",
        )),
        "Dosya bütünlüğü doğrulandı.",
        SystemTime::now(),
    );

    let duplicate = entry.clone();

    assert!(truth_log.append(entry));
    assert!(!truth_log.append(duplicate));
    assert_eq!(truth_log.len(), 1);
}

#[test]
fn truth_log_filters_entries_by_subject_and_file() {
    let mut truth_log = TruthLog::new();
    let subject_id = Uuid::new_v4();

    let related = TruthLogEntry::new(
        TruthLogEventKind::DiffSecurityVerified,
        TruthLogSeverity::Information,
        Some(subject_id),
        Some(PathBuf::from(
            "articles/hebun.md",
        )),
        "Diff güvenlik doğrulamasından geçti.",
        SystemTime::now(),
    );

    let unrelated = TruthLogEntry::new(
        TruthLogEventKind::OriginalHashRecorded,
        TruthLogSeverity::Information,
        Some(Uuid::new_v4()),
        Some(PathBuf::from(
            "articles/rasterast.md",
        )),
        "Başka dosyanın hash kaydı oluşturuldu.",
        SystemTime::now(),
    );

    assert!(truth_log.append(related));
    assert!(truth_log.append(unrelated));

    assert_eq!(
        truth_log
            .entries_for_subject(subject_id)
            .len(),
        1,
    );

    assert_eq!(
        truth_log
            .entries_for_file(
                "articles/hebun.md",
            )
            .len(),
        1,
    );
}

#[test]
fn truth_log_returns_critical_security_entries() {
    let mut truth_log = TruthLog::new();

    let critical = TruthLogEntry::new(
        TruthLogEventKind::FileModificationDetected,
        TruthLogSeverity::Critical,
        None,
        Some(PathBuf::from(
            "articles/hebun.md",
        )),
        "Orijinal dosyada değişiklik tespit edildi.",
        SystemTime::now(),
    );

    let information = TruthLogEntry::new(
        TruthLogEventKind::OriginalHashRecorded,
        TruthLogSeverity::Information,
        None,
        Some(PathBuf::from(
            "articles/hebun.md",
        )),
        "Hash kaydı oluşturuldu.",
        SystemTime::now(),
    );

    assert!(truth_log.append(critical));
    assert!(truth_log.append(information));

    assert_eq!(
        truth_log.critical_entries().len(),
        1,
    );
 }

#[test]
fn truth_log_records_revised_hash_event() {
    let mut truth_log = TruthLog::new();

    let record = crate::FileHashRecord::new(
        "articles/hebun-v2.md",
        crate::FileHashRole::Revised,
        "SHA-256",
        "fedcba9876543210",
        SystemTime::now(),
    );

    assert!(truth_log.record_file_hash(
        &record,
        None,
        SystemTime::now(),
    ));

    let entry = truth_log
        .entries()
        .first()
        .expect("revised hash event should be recorded");

    assert_eq!(
        entry.event_kind,
        TruthLogEventKind::RevisedHashRecorded,
    );

    assert!(
        entry.belongs_to_file(
            "articles/hebun-v2.md",
        )
    );
}

    
#[test]
fn truth_log_rejects_incomplete_hash_record() {
    let mut truth_log = TruthLog::new();

    let record = crate::FileHashRecord::new(
        "",
        crate::FileHashRole::Original,
        "",
        "",
        SystemTime::now(),
    );

    assert!(!truth_log.record_file_hash(
        &record,
        None,
        SystemTime::now(),
    ));

    assert!(truth_log.is_empty());
}

#[test]
fn truth_log_records_verified_file_integrity() {
    let mut truth_log = TruthLog::new();
    let subject_id = Uuid::new_v4();

    let original = crate::FileHashRecord::new(
        "articles/hebun.md",
        crate::FileHashRole::Original,
        "SHA-256",
        "same-digest",
        SystemTime::now(),
    );

    let current = crate::FileHashRecord::new(
        "articles/hebun.md",
        crate::FileHashRole::Revised,
        "sha-256",
        "same-digest",
        SystemTime::now(),
    );

    let report = crate::FileIntegrityReport::verify(
        original,
        current,
        SystemTime::now(),
    );

    assert!(truth_log.record_file_integrity(
        &report,
        Some(subject_id),
        SystemTime::now(),
    ));

    assert_eq!(truth_log.len(), 1);

    let entry = truth_log
        .entries()
        .first()
        .expect("integrity event should be recorded");

    assert_eq!(
        entry.event_kind,
        TruthLogEventKind::FileIntegrityVerified,
    );

    assert_eq!(
        entry.severity,
        TruthLogSeverity::Information,
    );

    assert!(entry.belongs_to_subject(subject_id));
    assert!(entry.belongs_to_file("articles/hebun.md"));
    assert!(!entry.is_critical());
    assert_eq!(entry.evidence.len(), 5);
}

#[test]
fn truth_log_records_critical_file_modification() {
    let mut truth_log = TruthLog::new();

    let original = crate::FileHashRecord::new(
        "articles/hebun.md",
        crate::FileHashRole::Original,
        "SHA-256",
        "original-digest",
        SystemTime::now(),
    );

    let current = crate::FileHashRecord::new(
        "articles/hebun.md",
        crate::FileHashRole::Revised,
        "SHA-256",
        "changed-digest",
        SystemTime::now(),
    );

    let report = crate::FileIntegrityReport::verify(
        original,
        current,
        SystemTime::now(),
    );

    assert!(truth_log.record_file_integrity(
        &report,
        None,
        SystemTime::now(),
    ));

    let entry = truth_log
        .entries()
        .first()
        .expect("modification event should be recorded");

    assert_eq!(
        entry.event_kind,
        TruthLogEventKind::FileModificationDetected,
    );

    assert_eq!(
        entry.severity,
        TruthLogSeverity::Critical,
    );

    assert!(entry.is_critical());

    assert_eq!(
        truth_log.critical_entries().len(),
        1,
    );
}

#[test]
fn truth_log_records_hash_algorithm_mismatch() {
    let mut truth_log = TruthLog::new();

    let original = crate::FileHashRecord::new(
        "articles/hebun.md",
        crate::FileHashRole::Original,
        "SHA-256",
        "same-digest",
        SystemTime::now(),
    );

    let current = crate::FileHashRecord::new(
        "articles/hebun.md",
        crate::FileHashRole::Revised,
        "SHA-512",
        "same-digest",
        SystemTime::now(),
    );

    let report = crate::FileIntegrityReport::verify(
        original,
        current,
        SystemTime::now(),
    );

    assert!(truth_log.record_file_integrity(
        &report,
        None,
        SystemTime::now(),
    ));

    let entry = truth_log
        .entries()
        .first()
        .expect("algorithm mismatch should be recorded");

    assert_eq!(
        entry.event_kind,
        TruthLogEventKind::FileModificationDetected,
    );

    assert_eq!(
        entry.severity,
        TruthLogSeverity::Warning,
    );

    assert!(!entry.is_critical());
}

#[test]
fn truth_log_records_verified_diff_security() {
    use std::fs;

    let mut truth_log = TruthLog::new();
    let subject_id = Uuid::new_v4();

    let original_path = std::env::temp_dir().join(
        format!(
            "truth-log-diff-original-{}.txt",
            std::process::id(),
        ),
    );

    let revised_path = std::env::temp_dir().join(
        format!(
            "truth-log-diff-revised-{}.txt",
            std::process::id(),
        ),
    );

    fs::write(
        &original_path,
        "Hebun\nRasterast\n",
    )
    .expect("original file should be created");

    fs::write(
        &revised_path,
        "Hebun\nNew line\nRasterast\n",
    )
    .expect("revised file should be created");

    let pair = crate::FileVersionPair::from_files_sha256(
        &original_path,
        &revised_path,
        SystemTime::now(),
    )
    .expect("version pair should be created");

    let report = crate::FileDiffReport::from_version_pair_lcs(
        &pair,
        SystemTime::now(),
    )
    .expect("diff report should be created");

    assert!(truth_log.record_diff_security(
        &report,
        &pair,
        Some(subject_id),
        SystemTime::now(),
    ));

    let entry = truth_log
        .entries()
        .first()
        .expect("diff security event should be recorded");

    assert_eq!(
        entry.event_kind,
        TruthLogEventKind::DiffSecurityVerified,
    );

    assert_eq!(
        entry.severity,
        TruthLogSeverity::Information,
    );

    assert!(entry.belongs_to_subject(subject_id));
    assert!(!entry.is_critical());
    assert_eq!(entry.evidence.len(), 7);

    fs::remove_file(&original_path)
        .expect("original file should be removed");

    fs::remove_file(&revised_path)
        .expect("revised file should be removed");
}

#[test]
fn truth_log_records_critical_diff_path_mismatch() {
    let mut truth_log = TruthLog::new();

    let original = crate::FileHashRecord::new(
        "articles/hebun.md",
        crate::FileHashRole::Original,
        "SHA-256",
        "original-digest",
        SystemTime::now(),
    );

    let revised = crate::FileHashRecord::new(
        "articles/hebun-v2.md",
        crate::FileHashRole::Revised,
        "SHA-256",
        "revised-digest",
        SystemTime::now(),
    );

    let pair = crate::FileVersionPair::new(
        original,
        revised,
        SystemTime::now(),
    );

    let report = crate::FileDiffReport::new(
        "articles/other.md",
        "articles/other-v2.md",
        vec![
            crate::FileLineChange::new(
                crate::FileLineChangeKind::Modified,
                Some(1),
                Some(1),
                Some("Old".to_string()),
                Some("New".to_string()),
            ),
        ],
        SystemTime::now(),
    );

    assert!(truth_log.record_diff_security(
        &report,
        &pair,
        None,
        SystemTime::now(),
    ));

    let entry = truth_log
        .entries()
        .first()
        .expect("path mismatch event should be recorded");

    assert_eq!(
        entry.event_kind,
        TruthLogEventKind::DiffSecurityRejected,
    );

    assert_eq!(
        entry.severity,
        TruthLogSeverity::Critical,
    );

    assert!(entry.is_critical());
}

#[test]
fn truth_log_records_invalid_version_pair_warning() {
    let mut truth_log = TruthLog::new();

    let invalid_original = crate::FileHashRecord::new(
        "articles/hebun.md",
        crate::FileHashRole::Revised,
        "SHA-256",
        "original-digest",
        SystemTime::now(),
    );

    let revised = crate::FileHashRecord::new(
        "articles/hebun-v2.md",
        crate::FileHashRole::Revised,
        "SHA-256",
        "revised-digest",
        SystemTime::now(),
    );

    let pair = crate::FileVersionPair::new(
        invalid_original,
        revised,
        SystemTime::now(),
    );

    let report = crate::FileDiffReport::new(
        "articles/hebun.md",
        "articles/hebun-v2.md",
        Vec::new(),
        SystemTime::now(),
    );

    assert!(truth_log.record_diff_security(
        &report,
        &pair,
        None,
        SystemTime::now(),
    ));

    let entry = truth_log
        .entries()
        .first()
        .expect("invalid pair event should be recorded");

    assert_eq!(
        entry.event_kind,
        TruthLogEventKind::DiffSecurityRejected,
    );

    assert_eq!(
        entry.severity,
        TruthLogSeverity::Warning,
    );

    assert!(!entry.is_critical());
}

#[test]
fn truth_log_records_valid_file_version_pair() {
    let mut truth_log = TruthLog::new();
    let subject_id = Uuid::new_v4();

    let original = crate::FileHashRecord::new(
        "articles/hebun.md",
        crate::FileHashRole::Original,
        "SHA-256",
        "original-digest",
        SystemTime::now(),
    );

    let revised = crate::FileHashRecord::new(
        "articles/hebun-v2.md",
        crate::FileHashRole::Revised,
        "SHA-256",
        "revised-digest",
        SystemTime::now(),
    );

    let pair = crate::FileVersionPair::new(
        original,
        revised,
        SystemTime::now(),
    );

    assert!(pair.is_matched());

    assert!(truth_log.record_file_version_pair(
        &pair,
        Some(subject_id),
        SystemTime::now(),
    ));

    assert_eq!(truth_log.len(), 1);

    let entry = truth_log
        .entries()
        .first()
        .expect("version pair event should be recorded");

    assert_eq!(
        entry.event_kind,
        TruthLogEventKind::FileVersionPairCreated,
    );

    assert_eq!(
        entry.severity,
        TruthLogSeverity::Information,
    );

    assert!(entry.belongs_to_subject(subject_id));
    assert!(entry.belongs_to_file("articles/hebun.md"));
    assert_eq!(entry.evidence.len(), 5);
}

#[test]
fn truth_log_records_invalid_file_version_pair() {
    let mut truth_log = TruthLog::new();

    let invalid_original = crate::FileHashRecord::new(
        "articles/hebun.md",
        crate::FileHashRole::Revised,
        "SHA-256",
        "original-digest",
        SystemTime::now(),
    );

    let revised = crate::FileHashRecord::new(
        "articles/hebun-v2.md",
        crate::FileHashRole::Revised,
        "SHA-256",
        "revised-digest",
        SystemTime::now(),
    );

    let pair = crate::FileVersionPair::new(
        invalid_original,
        revised,
        SystemTime::now(),
    );

    assert!(!pair.is_matched());

    assert!(truth_log.record_file_version_pair(
        &pair,
        None,
        SystemTime::now(),
    ));

    let entry = truth_log
        .entries()
        .first()
        .expect("invalid version pair event should be recorded");

    assert_eq!(
        entry.event_kind,
        TruthLogEventKind::FileVersionPairRejected,
    );

    assert_eq!(
        entry.severity,
        TruthLogSeverity::Warning,
    );

    assert!(!entry.is_critical());
    assert_eq!(entry.evidence.len(), 5);
}

#[test]
fn truth_log_builds_valid_immutable_chain() {
    let mut truth_log = TruthLog::new();

    let first = TruthLogEntry::new(
        TruthLogEventKind::OriginalHashRecorded,
        TruthLogSeverity::Information,
        None,
        Some(PathBuf::from(
            "articles/hebun.md",
        )),
        "Orijinal hash kaydı oluşturuldu.",
        SystemTime::now(),
    );

    let second = TruthLogEntry::new(
        TruthLogEventKind::DiffReportGenerated,
        TruthLogSeverity::Information,
        None,
        Some(PathBuf::from(
            "articles/hebun.md",
        )),
        "Diff raporu oluşturuldu.",
        SystemTime::now(),
    );

    assert!(truth_log.append(first));
    assert!(truth_log.append(second));

    assert!(truth_log.verify_chain());

    let entries = truth_log.entries();

    assert_eq!(entries.len(), 2);
    assert!(entries[0].previous_chain_digest.is_none());

    assert_eq!(
        entries[1].previous_chain_digest.as_deref(),
        Some(entries[0].chain_digest.as_str()),
    );

    assert_eq!(entries[0].chain_digest.len(), 64);
    assert_eq!(entries[1].chain_digest.len(), 64);

    assert_eq!(
        truth_log.latest_chain_digest(),
        Some(entries[1].chain_digest.as_str()),
    );
}

#[test]
fn truth_log_detects_modified_entry() {
    let mut truth_log = TruthLog::new();

    let first = TruthLogEntry::new(
        TruthLogEventKind::OriginalHashRecorded,
        TruthLogSeverity::Information,
        None,
        Some(PathBuf::from(
            "articles/hebun.md",
        )),
        "Orijinal hash kaydı oluşturuldu.",
        SystemTime::now(),
    );

    let second = TruthLogEntry::new(
        TruthLogEventKind::DiffSecurityVerified,
        TruthLogSeverity::Information,
        None,
        Some(PathBuf::from(
            "articles/hebun.md",
        )),
        "Diff güvenlik doğrulamasından geçti.",
        SystemTime::now(),
    );

    assert!(truth_log.append(first));
    assert!(truth_log.append(second));
    assert!(truth_log.verify_chain());

    truth_log.entries[0].message =
        "Sonradan değiştirilmiş kayıt.".to_string();

    assert!(!truth_log.verify_chain());
}



