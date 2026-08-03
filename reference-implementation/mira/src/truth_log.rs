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
    pub fn append(
        &mut self,
        entry: TruthLogEntry,
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

        self.entries.push(entry);
        true
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

}

#[cfg(test)]
mod tests {
    use super::*;

#[test]
fn truth_log_records_original_hash_event() {
    let mut truth_log = TruthLog::new();
    let subject_id = Uuid::new_v4();

    let record = crate::FileHashRecord::new(
        "articles/hebun.md",
        crate::FileHashRole::Original,
        "SHA-256",
        "0123456789abcdef",
        SystemTime::now(),
    );

    assert!(truth_log.record_file_hash(
        &record,
        Some(subject_id),
        SystemTime::now(),
    ));

    assert_eq!(truth_log.len(), 1);

    let entry = truth_log
        .entries()
        .first()
        .expect("hash event should be recorded");

    assert_eq!(
        entry.event_kind,
        TruthLogEventKind::OriginalHashRecorded,
    );

    assert_eq!(
        entry.severity,
        TruthLogSeverity::Information,
    );

    assert!(entry.belongs_to_subject(subject_id));
    assert!(entry.belongs_to_file("articles/hebun.md"));
    assert_eq!(entry.evidence.len(), 2);
}

    
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





