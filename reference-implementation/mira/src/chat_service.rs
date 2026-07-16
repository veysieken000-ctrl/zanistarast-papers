use crate::chat_orchestrator::{
    ChatInteractionResult,
    ChatOrchestrator,
};
use crate::chat_session::{
    ChatSession,
    ChatSessionManager,
};
use crate::{
    MiraCore,
    MiraTask,
};
use std::io;
use std::path::Path;
use uuid::Uuid;

/// Mira'nın yazılı sohbet işlemlerini tek merkezden yöneten servis.
///
/// Bu katman:
/// - Mira çekirdeğini yönetir,
/// - sohbet oturumlarını açar,
/// - Müdebbir mesajlarını işler,
/// - komutları güvenli orkestratöre yönlendirir,
/// - bilinmeyen oturumları kendiliğinden oluşturmaz,
/// - yayın, silme, merge veya orijinal metin değişikliği yapmaz.
#[derive(Debug, Default)]
pub struct MiraChatService {
    mira: MiraCore,
    sessions: ChatSessionManager,
    orchestrator: ChatOrchestrator,
}

impl MiraChatService {
    pub fn new() -> Self {
        Self {
            mira: MiraCore::new(),
            sessions: ChatSessionManager::new(),
            orchestrator: ChatOrchestrator::new(),
        }
    }

    /// Müdebbir için yeni bir yazılı sohbet oturumu açar.
    pub fn create_session(
        &mut self,
        title: impl Into<String>,
    ) -> Uuid {
        self.sessions.create_session(title)
    }

    /// Belirtilen oturuma Müdebbir mesajı gönderir.
    pub fn send_message(
        &mut self,
        session_id: Uuid,
        message: &str,
        root: impl AsRef<Path>,
    ) -> io::Result<ChatInteractionResult> {
        let session = self
            .sessions
            .find_session_mut(session_id)
            .ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    format!(
                        "Mira sohbet oturumu bulunamadı: {session_id}"
                    ),
                )
            })?;

        self.orchestrator.process(
            &mut self.mira,
            session,
            message,
            root,
        )
    }

    /// Kimliğine göre sohbet oturumunu salt okunur döndürür.
    pub fn session(
        &self,
        session_id: Uuid,
    ) -> Option<&ChatSession> {
        self.sessions.find_session(session_id)
    }

    /// Bütün sohbet oturumlarını salt okunur döndürür.
    pub fn sessions(&self) -> &[ChatSession] {
        self.sessions.sessions()
    }

    /// Mira'da kayıtlı görevleri salt okunur döndürür.
    pub fn tasks(&self) -> &[MiraTask] {
        self.mira.tasks()
    }

    /// Toplam sohbet oturumu sayısını döndürür.
    pub fn session_count(&self) -> usize {
        self.sessions.session_count()
    }

    /// Toplam Mira görev sayısını döndürür.
    pub fn task_count(&self) -> usize {
        self.mira.tasks().len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chat_session::ChatRole;
    use std::fs;
    use std::path::PathBuf;

    fn create_test_repository() -> PathBuf {
        let test_root = std::env::temp_dir().join(format!(
            "zanistarast-mira-chat-service-{}",
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
    fn service_creates_mudebbir_chat_session() {
        let mut service = MiraChatService::new();

        let session_id =
            service.create_session("Mira akademik çalışma");

        assert_eq!(service.session_count(), 1);
        assert!(service.session(session_id).is_some());
    }

    #[test]
    fn service_processes_repository_scan_message() {
        let test_root = create_test_repository();
        let mut service = MiraChatService::new();

        let session_id =
            service.create_session("Repository taraması");

        let result = service
            .send_message(
                session_id,
                "depo tara",
                &test_root,
            )
            .expect("repository scan message should succeed");

        assert!(result.command_executed);
        assert!(result.created_task_id.is_some());
        assert_eq!(service.task_count(), 1);

        let session = service
            .session(session_id)
            .expect("session should exist");

        assert_eq!(session.message_count(), 2);
        assert_eq!(
            session.messages()[0].role,
            ChatRole::Mudebbir
        );
        assert_eq!(
            session.messages()[1].role,
            ChatRole::Mira
        );

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }

    #[test]
    fn unknown_session_returns_not_found_error() {
        let test_root = create_test_repository();
        let mut service = MiraChatService::new();

        let error = service
            .send_message(
                Uuid::new_v4(),
                "durum",
                &test_root,
            )
            .expect_err("unknown session should fail");

        assert_eq!(error.kind(), io::ErrorKind::NotFound);
        assert_eq!(service.session_count(), 0);

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }

    #[test]
    fn service_preserves_multiple_chat_sessions() {
        let mut service = MiraChatService::new();

        let first =
            service.create_session("Hebûn çalışması");
        let second =
            service.create_session("Rasterast çalışması");

        assert_ne!(first, second);
        assert_eq!(service.session_count(), 2);
        assert!(service.session(first).is_some());
        assert!(service.session(second).is_some());
    }

    #[test]
    fn unsafe_unknown_command_does_not_create_task() {
        let test_root = create_test_repository();
        let mut service = MiraChatService::new();

        let session_id =
            service.create_session("Güvenlik denetimi");

        let result = service
            .send_message(
                session_id,
                "Bütün dosyaları sil",
                &test_root,
            )
            .expect("unknown command should return safely");

        assert!(!result.command_executed);
        assert!(result.requires_mudebbir_approval);
        assert_eq!(service.task_count(), 0);

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }
}

