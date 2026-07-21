use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use uuid::Uuid;

/// Tek Müdebbir hesabı için kısa ömürlü sunucu oturumlarını yönetir.
///
/// Güvenlik ilkeleri:
/// - Oturum kimliği tahmin edilemez ve rastgele üretilir.
/// - Oturum bilgisi tarayıcıda kalıcı olarak saklanmaz.
/// - Süresi dolan oturumlar geçersiz sayılır.
/// - Oturum sunucu tarafından iptal edilebilir.
/// - Gerçek Müdebbir erişim anahtarı oturum kaydına yazılmaz.
#[derive(Debug, Clone)]
pub struct MudebbirSessionStore {
    sessions: Arc<Mutex<HashMap<String, Instant>>>,
    session_ttl: Duration,
}

impl MudebbirSessionStore {
    /// Belirtilen yaşam süresine sahip boş bir oturum deposu oluşturur.
    pub fn new(session_ttl: Duration) -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            session_ttl,
        }
    }

    /// Yeni ve kısa ömürlü bir Müdebbir oturumu oluşturur.
    pub fn create_session(&self) -> Result<String, String> {
        let session_id = Uuid::new_v4().simple().to_string();
        let expires_at = Instant::now() + self.session_ttl;

        let mut sessions = self
            .sessions
            .lock()
            .map_err(|_| "Müdebbir oturum deposuna erişilemedi.".to_string())?;

        Self::remove_expired_sessions(&mut sessions);
        sessions.insert(session_id.clone(), expires_at);

        Ok(session_id)
    }

    /// Verilen oturum kimliğinin mevcut ve geçerli olup olmadığını denetler.
    pub fn is_valid(&self, session_id: &str) -> Result<bool, String> {
        if session_id.trim().is_empty() {
            return Ok(false);
        }

        let mut sessions = self
            .sessions
            .lock()
            .map_err(|_| "Müdebbir oturum deposuna erişilemedi.".to_string())?;

        let Some(expires_at) = sessions.get(session_id).copied() else {
            return Ok(false);
        };

        if expires_at <= Instant::now() {
            sessions.remove(session_id);
            return Ok(false);
        }

        Ok(true)
    }

    /// Belirtilen Müdebbir oturumunu geçersiz kılar.
    pub fn revoke_session(&self, session_id: &str) -> Result<bool, String> {
        let mut sessions = self
            .sessions
            .lock()
            .map_err(|_| "Müdebbir oturum deposuna erişilemedi.".to_string())?;

        Ok(sessions.remove(session_id).is_some())
    }

    /// Süresi dolan oturumları depodan temizler.
    fn remove_expired_sessions(sessions: &mut HashMap<String, Instant>) {
        let now = Instant::now();
        sessions.retain(|_, expires_at| *expires_at > now);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn created_session_is_valid() {
        let store =
            MudebbirSessionStore::new(Duration::from_secs(30 * 60));

        let session_id = store
            .create_session()
            .expect("session should be created");

        assert!(!session_id.is_empty());
        assert!(
            store
                .is_valid(&session_id)
                .expect("session should be checked")
        );
    }

    #[test]
    fn revoked_session_is_rejected() {
        let store =
            MudebbirSessionStore::new(Duration::from_secs(30 * 60));

        let session_id = store
            .create_session()
            .expect("session should be created");

        assert!(
            store
                .revoke_session(&session_id)
                .expect("session should be revoked")
        );

        assert!(
            !store
                .is_valid(&session_id)
                .expect("session should be checked")
        );
    }

    #[test]
    fn unknown_session_is_rejected() {
        let store =
            MudebbirSessionStore::new(Duration::from_secs(30 * 60));

        assert!(
            !store
                .is_valid("unknown-mudebbir-session")
                .expect("session should be checked")
        );
    }

   #[test]
fn expired_session_is_rejected() {
    let store =
        MudebbirSessionStore::new(Duration::from_millis(1));

    let session_id = store
        .create_session()
        .expect("session should be created");

    std::thread::sleep(Duration::from_millis(5));

    assert!(
        !store
            .is_valid(&session_id)
            .expect("expired session should be checked")
    );
}

    
    #[test]
    fn cloned_store_uses_shared_sessions() {
        let store =
            MudebbirSessionStore::new(Duration::from_secs(30 * 60));
        let cloned_store = store.clone();

        let session_id = store
            .create_session()
            .expect("session should be created");

        assert!(
            cloned_store
                .is_valid(&session_id)
                .expect("shared session should be checked")
        );
    }
}


