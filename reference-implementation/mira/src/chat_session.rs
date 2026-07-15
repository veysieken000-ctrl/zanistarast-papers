use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Mira sohbetindeki mesajı gönderen taraf.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChatRole {
    Mudebbir,
    Mira,
    System,
}

/// Tek bir yazılı sohbet mesajı.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChatMessage {
    pub message_id: Uuid,
    pub role: ChatRole,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

impl ChatMessage {
    pub fn new(
        role: ChatRole,
        content: impl Into<String>,
    ) -> Self {
        Self {
            message_id: Uuid::new_v4(),
            role,
            content: content.into(),
            created_at: Utc::now(),
        }
    }
}

/// Müdebbir ile Mira arasındaki tek bir sohbet oturumu.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub session_id: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    messages: Vec<ChatMessage>,
}

impl ChatSession {
    /// Yeni bir sohbet oturumu oluşturur.
    pub fn new(title: impl Into<String>) -> Self {
        let now = Utc::now();

        Self {
            session_id: Uuid::new_v4(),
            title: title.into(),
            created_at: now,
            updated_at: now,
            messages: Vec::new(),
        }
    }

    /// Müdebbirin mesajını oturuma ekler.
    pub fn add_mudebbir_message(
        &mut self,
        content: impl Into<String>,
    ) -> Uuid {
        self.add_message(ChatRole::Mudebbir, content)
    }

    /// Mira'nın cevabını oturuma ekler.
    pub fn add_mira_message(
        &mut self,
        content: impl Into<String>,
    ) -> Uuid {
        self.add_message(ChatRole::Mira, content)
    }

    /// Sistem bilgilendirmesini oturuma ekler.
    pub fn add_system_message(
        &mut self,
        content: impl Into<String>,
    ) -> Uuid {
        self.add_message(ChatRole::System, content)
    }

    /// Oturumdaki bütün mesajları salt okunur döndürür.
    pub fn messages(&self) -> &[ChatMessage] {
        &self.messages
    }

    /// Oturumdaki mesaj sayısını döndürür.
    pub fn message_count(&self) -> usize {
        self.messages.len()
    }

    /// Son mesajı döndürür.
    pub fn last_message(&self) -> Option<&ChatMessage> {
        self.messages.last()
    }

    fn add_message(
        &mut self,
        role: ChatRole,
        content: impl Into<String>,
    ) -> Uuid {
        let message = ChatMessage::new(role, content);
        let message_id = message.message_id;

        self.messages.push(message);
        self.updated_at = Utc::now();

        message_id
    }
}

/// Mira sohbet oturumlarını çalışma belleğinde yönetir.
///
/// Bu ilk sürüm:
/// - mesajları yalnızca çalışma belleğinde tutar,
/// - dosyaya veya veritabanına yazmaz,
/// - mesajları kendiliğinden silmez,
/// - Müdebbir adına komut üretmez,
/// - yayın veya depo değişikliği yapmaz.
#[derive(Debug, Default)]
pub struct ChatSessionManager {
    sessions: Vec<ChatSession>,
}

impl ChatSessionManager {
    pub fn new() -> Self {
        Self {
            sessions: Vec::new(),
        }
    }

    /// Yeni sohbet oturumu açar.
    pub fn create_session(
        &mut self,
        title: impl Into<String>,
    ) -> Uuid {
        let session = ChatSession::new(title);
        let session_id = session.session_id;

        self.sessions.push(session);

        session_id
    }

    /// Bütün oturumları salt okunur döndürür.
    pub fn sessions(&self) -> &[ChatSession] {
        &self.sessions
    }

    /// Kimliğine göre sohbet oturumunu bulur.
    pub fn find_session(
        &self,
        session_id: Uuid,
    ) -> Option<&ChatSession> {
        self.sessions
            .iter()
            .find(|session| session.session_id == session_id)
    }

    /// Kimliğine göre sohbet oturumunu değiştirilebilir bulur.
    pub fn find_session_mut(
        &mut self,
        session_id: Uuid,
    ) -> Option<&mut ChatSession> {
        self.sessions
            .iter_mut()
            .find(|session| session.session_id == session_id)
    }

    /// Toplam oturum sayısını döndürür.
    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn manager_creates_chat_session() {
        let mut manager = ChatSessionManager::new();

        let session_id =
            manager.create_session("Mira akademik çalışma");

        assert_eq!(manager.session_count(), 1);
        assert!(manager.find_session(session_id).is_some());
    }

    #[test]
    fn session_records_mudebbir_and_mira_messages() {
        let mut session =
            ChatSession::new("Hebûn makale incelemesi");

        session.add_mudebbir_message(
            "Hebûn makalelerini sıraya koy.",
        );

        session.add_mira_message(
            "Salt okunur envanter görevi hazırlanıyor.",
        );

        assert_eq!(session.message_count(), 2);

        assert_eq!(
            session.messages()[0].role,
            ChatRole::Mudebbir
        );

        assert_eq!(
            session.messages()[1].role,
            ChatRole::Mira
        );
    }

    #[test]
    fn last_message_returns_latest_entry() {
        let mut session =
            ChatSession::new("Mira durum görüşmesi");

        session.add_system_message(
            "Oturum güvenli biçimde açıldı.",
        );

        session.add_mudebbir_message("Durum nedir?");

        let last_message = session
            .last_message()
            .expect("last message should exist");

        assert_eq!(last_message.role, ChatRole::Mudebbir);
        assert_eq!(last_message.content, "Durum nedir?");
    }

    #[test]
    fn unknown_session_is_not_created_implicitly() {
        let manager = ChatSessionManager::new();

        assert!(
            manager
                .find_session(Uuid::new_v4())
                .is_none()
        );

        assert_eq!(manager.session_count(), 0);
    }

    #[test]
    fn messages_are_returned_read_only() {
        let mut session =
            ChatSession::new("Rasterast değerlendirmesi");

        session.add_mira_message(
            "Rasterast raporu Müdebbir incelemesini bekliyor.",
        );

        let messages = session.messages();

        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].role, ChatRole::Mira);
    }
}


