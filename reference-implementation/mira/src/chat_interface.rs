use crate::{
    MiraCore,
    MiraRiskLevel,
    MiraTask,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Müdebbirin yazılı olarak verebileceği temel Mira komutları.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MiraChatCommand {
    Help,
    Status,
    ListTasks,
    ScanRepository,
    ScanWebsite,
    BuildArticleInventory,
    Unknown(String),
}

/// Mira'nın yazılı komuta verdiği standart cevap.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MiraChatResponse {
    pub message: String,
    pub created_task_id: Option<Uuid>,
    pub requires_mudebbir_approval: bool,
}

impl MiraChatResponse {
    fn informational(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            created_task_id: None,
            requires_mudebbir_approval: false,
        }
    }

    fn task_created(
        message: impl Into<String>,
        task_id: Uuid,
        requires_mudebbir_approval: bool,
    ) -> Self {
        Self {
            message: message.into(),
            created_task_id: Some(task_id),
            requires_mudebbir_approval,
        }
    }
}

/// Müdebbirin yazılı komutları ile Mira çekirdeği arasındaki arayüz.
///
/// Bu katman:
/// - doğal dildeki temel komutları tanır,
/// - salt okunur işlemleri görev kuyruğuna ekler,
/// - bilinmeyen komutları kendiliğinden çalıştırmaz,
/// - yayın, merge, silme veya orijinal metin değişikliği yapmaz,
/// - kritik işlemleri otomatik olarak onaylamaz.
#[derive(Debug, Default)]
pub struct MiraChatInterface;

impl MiraChatInterface {
    pub fn new() -> Self {
        Self
    }

    /// Kullanıcının yazdığı metni temel Mira komutuna dönüştürür.
    pub fn parse_command(&self, input: &str) -> MiraChatCommand {
        let normalized = input
            .trim()
            .to_lowercase()
            .replace('û', "u")
            .replace('ı', "i")
            .replace('ş', "s")
            .replace('ğ', "g")
            .replace('ü', "u")
            .replace('ö', "o")
            .replace('ç', "c");

        match normalized.as_str() {
            "yardim" | "help" => MiraChatCommand::Help,

            "durum"
            | "sistem durumu"
            | "status" => MiraChatCommand::Status,

            "gorevleri listele"
            | "gorev listesi"
            | "list tasks" => MiraChatCommand::ListTasks,

            "depo tara"
            | "repository tara"
            | "repository scan" => {
                MiraChatCommand::ScanRepository
            }

            "site tara"
            | "web sitesi tara"
            | "website tara"
            | "website scan" => {
                MiraChatCommand::ScanWebsite
            }

            "makale envanteri olustur"
            | "makaleleri say"
            | "makale envanteri"
            | "build article inventory" => {
                MiraChatCommand::BuildArticleInventory
            }

            _ => MiraChatCommand::Unknown(
                input.trim().to_string(),
            ),
        }
    }

    /// Yazılı komutu işler ve güvenli Mira cevabı üretir.
    pub fn handle(
        &self,
        mira: &mut MiraCore,
        input: &str,
    ) -> MiraChatResponse {
        match self.parse_command(input) {
            MiraChatCommand::Help => {
                MiraChatResponse::informational(
                    "Kullanılabilir komutlar: yardım, durum, görevleri listele, depo tara, site tara, makale envanteri oluştur.",
                )
            }

            MiraChatCommand::Status => {
                MiraChatResponse::informational(format!(
                    "Mira çalışıyor. Kayıtlı görev sayısı: {}.",
                    mira.tasks().len()
                ))
            }

            MiraChatCommand::ListTasks => {
                if mira.tasks().is_empty() {
                    return MiraChatResponse::informational(
                        "Henüz kayıtlı Mira görevi bulunmuyor.",
                    );
                }

                let task_lines = mira
                    .tasks()
                    .iter()
                    .map(|task| {
                        format!(
                            "- {} | durum: {:?} | risk: {:?}",
                            task.title,
                            task.status,
                            task.risk_level
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                MiraChatResponse::informational(format!(
                    "Kayıtlı görevler:\n{task_lines}"
                ))
            }

            MiraChatCommand::ScanRepository => {
                let task = MiraTask::new(
                    "Salt okunur repository taraması",
                    "Repository Scanner ile depoyu yalnızca oku ve dosya envanteri oluştur. Hiçbir dosyayı değiştirme, taşıma veya silme.",
                    MiraRiskLevel::Low,
                    false,
                );

                let task_id = mira.register_task(task);

                MiraChatResponse::task_created(
                    "Salt okunur depo tarama görevi oluşturuldu.",
                    task_id,
                    false,
                )
            }

            MiraChatCommand::ScanWebsite => {
                let task = MiraTask::new(
                    "Salt okunur website taraması",
                    "Website Scanner ile yerel HTML sayfalarını yalnızca oku; başlık, bağlantı ve görselleri raporla. Hiçbir dosyayı değiştirme.",
                    MiraRiskLevel::Low,
                    false,
                );

                let task_id = mira.register_task(task);

                MiraChatResponse::task_created(
                    "Salt okunur site tarama görevi oluşturuldu.",
                    task_id,
                    false,
                )
            }

            MiraChatCommand::BuildArticleInventory => {
                let task = MiraTask::new(
                    "Makale envanteri oluşturma",
                    "Repository ve website tarama sonuçlarından salt okunur bilimsel makale adayı envanteri oluştur. Orijinal metinleri değiştirme.",
                    MiraRiskLevel::Low,
                    false,
                );

                let task_id = mira.register_task(task);

                MiraChatResponse::task_created(
                    "Makale envanteri görevi oluşturuldu.",
                    task_id,
                    false,
                )
            }

            MiraChatCommand::Unknown(command) => {
                MiraChatResponse {
                    message: format!(
                        "Bu komut henüz tanımlı değil: \"{command}\". Güvenlik nedeniyle kendiliğinden işlem yapılmadı. Kullanılabilir komutları görmek için \"yardım\" yaz.",
                    ),
                    created_task_id: None,
                    requires_mudebbir_approval: true,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn turkish_repository_command_is_recognized() {
        let interface = MiraChatInterface::new();

        assert_eq!(
            interface.parse_command("Depo tara"),
            MiraChatCommand::ScanRepository
        );
    }

    #[test]
    fn inventory_command_creates_read_only_task() {
        let interface = MiraChatInterface::new();
        let mut mira = MiraCore::new();

        let response = interface.handle(
            &mut mira,
            "makale envanteri oluştur",
        );

        assert!(response.created_task_id.is_some());
        assert!(!response.requires_mudebbir_approval);
        assert_eq!(mira.tasks().len(), 1);

        let task = &mira.tasks()[0];

        assert_eq!(task.risk_level, MiraRiskLevel::Low);
        assert!(!task.requires_mudebbir_approval);
    }

    #[test]
    fn status_reports_registered_task_count() {
        let interface = MiraChatInterface::new();
        let mut mira = MiraCore::new();

        interface.handle(&mut mira, "depo tara");

        let response = interface.handle(
            &mut mira,
            "durum",
        );

        assert!(
            response
                .message
                .contains("Kayıtlı görev sayısı: 1")
        );
    }

    #[test]
    fn unknown_command_does_not_create_task() {
        let interface = MiraChatInterface::new();
        let mut mira = MiraCore::new();

        let response = interface.handle(
            &mut mira,
            "Bütün makaleleri hemen yayınla",
        );

        assert!(response.created_task_id.is_none());
        assert!(response.requires_mudebbir_approval);
        assert!(mira.tasks().is_empty());
    }

    #[test]
    fn task_list_contains_registered_task_title() {
        let interface = MiraChatInterface::new();
        let mut mira = MiraCore::new();

        interface.handle(&mut mira, "site tara");

        let response = interface.handle(
            &mut mira,
            "görevleri listele",
        );

        assert!(
            response
                .message
                .contains("Salt okunur website taraması")
        );
    }
}

