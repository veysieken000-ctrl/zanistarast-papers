use crate::chat_interface::{
    MiraChatCommand,
    MiraChatInterface,
};
use crate::chat_session::ChatSession;
use crate::command_router::{
    CommandExecutionResult,
    CommandRouter,
};
use crate::MiraCore;
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;
use uuid::Uuid;

/// Tek bir yazılı Mira etkileşiminin sonucu.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatInteractionResult {
    pub session_id: Uuid,
    pub command: MiraChatCommand,
    pub created_task_id: Option<Uuid>,
    pub message: String,
    pub command_executed: bool,
    pub requires_mudebbir_approval: bool,
}

/// Sohbet oturumunu Mira komut ve yönlendirme katmanlarıyla birleştirir.
///
/// Bu katman:
/// - Müdebbirin mesajını sohbet geçmişine kaydeder,
/// - komutu MiraChatInterface ile tanır,
/// - yalnızca izin verilen salt okunur komutları çalıştırır,
/// - sonucu Mira mesajı olarak kaydeder,
/// - bilinmeyen komutları çalıştırmaz,
/// - dosya değiştirme, silme, taşıma, merge veya yayın yapmaz.
#[derive(Debug, Default)]
pub struct ChatOrchestrator {
    chat_interface: MiraChatInterface,
    command_router: CommandRouter,
}

impl ChatOrchestrator {
    pub fn new() -> Self {
        Self {
            chat_interface: MiraChatInterface::new(),
            command_router: CommandRouter::new(),
        }
    }

    /// Müdebbirin yazılı mesajını işler ve cevabı aynı oturuma kaydeder.
    pub fn process(
        &self,
        mira: &mut MiraCore,
        session: &mut ChatSession,
        input: &str,
        root: impl AsRef<Path>,
    ) -> io::Result<ChatInteractionResult> {
        session.add_mudebbir_message(input);

        let command = self.chat_interface.parse_command(input);
        let chat_response = self.chat_interface.handle(mira, input);

        let execution_result = self
            .command_router
            .execute(&command, root)?;

        let execution_summary =
            Self::execution_summary(&execution_result);

        let message = if execution_summary.is_empty() {
            chat_response.message.clone()
        } else {
            format!(
                "{}\n{}",
                chat_response.message,
                execution_summary
            )
        };

        session.add_mira_message(message.clone());

        Ok(ChatInteractionResult {
            session_id: session.session_id,
            command,
            created_task_id: chat_response.created_task_id,
            message,
            command_executed: !matches!(
                execution_result,
                CommandExecutionResult::NotExecutable { .. }
            ),
            requires_mudebbir_approval:
                chat_response.requires_mudebbir_approval,
        })
    }

    fn execution_summary(
        result: &CommandExecutionResult,
    ) -> String {
        match result {
            CommandExecutionResult::RepositoryScan(report) => {
                format!(
                    "Repository taraması tamamlandı. Dosya sayısı: {}. Klasör sayısı: {}. Toplam boyut: {} bayt.",
                    report.file_count(),
                    report.directory_count,
                    report.total_size_bytes
                )
            }

            CommandExecutionResult::WebsiteScan(report) => {
                format!(
                    "Website taraması tamamlandı. HTML sayfası: {}. Bağlantı: {}. Görsel: {}.",
                    report.page_count(),
                    report.total_link_count,
                    report.total_image_count
                )
            }

            CommandExecutionResult::ArticleInventory(report) => {
                format!(
                    "Makale envanteri oluşturuldu. Toplam makale adayı: {}. Repository kaynaklı: {}. Yalnızca website taramasında bulunan: {}.",
                    report.total_candidate_count(),
                    report.repository_candidate_count,
                    report.website_candidate_count
                )
            }

            CommandExecutionResult::NotExecutable {
                message,
            } => message.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat_session::ChatRole;
    use std::fs;
    use std::path::PathBuf;

    fn create_test_site() -> PathBuf {
        let test_root = std::env::temp_dir().join(format!(
            "zanistarast-mira-chat-orchestrator-{}",
            Uuid::new_v4()
        ));

        let articles = test_root.join("articles");

        fs::create_dir_all(&articles)
            .expect("test directories should be created");

        fs::write(
            test_root.join("README.md"),
            "Zanistarast test repository",
        )
        .expect("README should be written");

        fs::write(
            articles.join("hebun.html"),
            r#"
                <html>
                    <head>
                        <title>Hebûn Makalesi</title>
                    </head>
                    <body>
                        <a href="../index.html">Ana sayfa</a>
                    </body>
                </html>
            "#,
        )
        .expect("Hebûn test page should be written");

        test_root
    }

    #[test]
    fn repository_command_is_recorded_and_executed() {
        let test_root = create_test_site();
        let orchestrator = ChatOrchestrator::new();
        let mut mira = MiraCore::new();
        let mut session =
            ChatSession::new("Repository incelemesi");

        let result = orchestrator
            .process(
                &mut mira,
                &mut session,
                "depo tara",
                &test_root,
            )
            .expect("repository command should succeed");

        assert!(result.command_executed);
        assert!(result.created_task_id.is_some());
        assert_eq!(session.message_count(), 2);

        assert_eq!(
            session.messages()[0].role,
            ChatRole::Mudebbir
        );

        assert_eq!(
            session.messages()[1].role,
            ChatRole::Mira
        );

        assert!(
            result
                .message
                .contains("Dosya sayısı: 2")
        );

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }

    #[test]
    fn inventory_command_returns_candidate_count() {
        let test_root = create_test_site();
        let orchestrator = ChatOrchestrator::new();
        let mut mira = MiraCore::new();
        let mut session =
            ChatSession::new("Makale envanteri");

        let result = orchestrator
            .process(
                &mut mira,
                &mut session,
                "makale envanteri oluştur",
                &test_root,
            )
            .expect("inventory command should succeed");

        assert!(result.command_executed);
        assert!(
            result
                .message
                .contains("Toplam makale adayı: 2")
        );

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }

    #[test]
    fn unknown_command_is_recorded_but_not_executed() {
        let test_root = create_test_site();
        let orchestrator = ChatOrchestrator::new();
        let mut mira = MiraCore::new();
        let mut session =
            ChatSession::new("Güvenlik testi");

        let result = orchestrator
            .process(
                &mut mira,
                &mut session,
                "Bütün makaleleri hemen yayınla",
                &test_root,
            )
            .expect("unknown command should return safely");

        assert!(!result.command_executed);
        assert!(result.requires_mudebbir_approval);
        assert!(result.created_task_id.is_none());
        assert_eq!(session.message_count(), 2);
        assert!(mira.tasks().is_empty());

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }

    #[test]
    fn informational_command_is_saved_without_scan() {
        let test_root = create_test_site();
        let orchestrator = ChatOrchestrator::new();
        let mut mira = MiraCore::new();
        let mut session =
            ChatSession::new("Mira yardım");

        let result = orchestrator
            .process(
                &mut mira,
                &mut session,
                "yardım",
                &test_root,
            )
            .expect("help command should return safely");

        assert!(!result.command_executed);
        assert!(result.message.contains("Kullanılabilir komutlar"));
        assert_eq!(session.message_count(), 2);

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }

    #[test]
    fn interaction_keeps_session_identity() {
        let test_root = create_test_site();
        let orchestrator = ChatOrchestrator::new();
        let mut mira = MiraCore::new();
        let mut session =
            ChatSession::new("Kimlik doğrulama testi");

        let expected_session_id = session.session_id;

        let result = orchestrator
            .process(
                &mut mira,
                &mut session,
                "durum",
                &test_root,
            )
            .expect("status command should return safely");

        assert_eq!(
            result.session_id,
            expected_session_id
        );

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }
}


