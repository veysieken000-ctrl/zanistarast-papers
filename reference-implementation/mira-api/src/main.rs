use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};
use uuid::Uuid;
use zanistarast_mira::chat_service::MiraChatService;

/// Mira API tarafından paylaşılan uygulama durumu.
#[derive(Clone)]
struct AppState {
    chat_service: Arc<Mutex<MiraChatService>>,
}

/// Sağlık kontrolü cevabı.
#[derive(Debug, Serialize)]
struct HealthResponse {
    service: &'static str,
    status: &'static str,
}

/// Yeni sohbet oturumu oluşturma isteği.
#[derive(Debug, Deserialize)]
struct CreateSessionRequest {
    title: String,
}

/// Yeni sohbet oturumu oluşturma cevabı.
#[derive(Debug, Serialize)]
struct CreateSessionResponse {
    session_id: Uuid,
    title: String,
    status: &'static str,
}

/// Standart API hata cevabı.
#[derive(Debug, Serialize)]
struct ApiError {
    error: String,
}

/// API servisinin çalıştığını doğrular.
async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        service: "zanistarast-mira-api",
        status: "ok",
    })
}

/// Müdebbir için yeni bir Mira sohbet oturumu oluşturur.
async fn create_session(
    State(state): State<AppState>,
    Json(request): Json<CreateSessionRequest>,
) -> Result<
    (StatusCode, Json<CreateSessionResponse>),
    (StatusCode, Json<ApiError>),
> {
    let title = request.title.trim();

    if title.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "Sohbet başlığı boş olamaz.".to_string(),
            }),
        ));
    }

    let mut service = state.chat_service.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                error: "Mira sohbet servisine erişilemedi.".to_string(),
            }),
        )
    })?;

    let session_id = service.create_session(title);

    Ok((
        StatusCode::CREATED,
        Json(CreateSessionResponse {
            session_id,
            title: title.to_string(),
            status: "created",
        }),
    ))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState {
        chat_service: Arc::new(Mutex::new(
            MiraChatService::new(),
        )),
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/sessions", post(create_session))
        .with_state(state);

    let address =
        SocketAddr::from(([127, 0, 0, 1], 3000));

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Mira API listener should bind");

    tracing::info!(
        "Mira API listening on http://{address}"
    );

    axum::serve(listener, app)
        .await
        .expect("Mira API server should run");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_state() -> AppState {
        AppState {
            chat_service: Arc::new(Mutex::new(
                MiraChatService::new(),
            )),
        }
    }

    #[tokio::test]
    async fn health_response_reports_ok() {
        let response = health().await;

        assert_eq!(
            response.0.service,
            "zanistarast-mira-api"
        );
        assert_eq!(response.0.status, "ok");
    }

    #[tokio::test]
    async fn session_endpoint_creates_chat_session() {
        let state = test_state();

        let result = create_session(
            State(state.clone()),
            Json(CreateSessionRequest {
                title: "Hebûn akademik çalışması"
                    .to_string(),
            }),
        )
        .await
        .expect("session creation should succeed");

        assert_eq!(result.0, StatusCode::CREATED);
        assert_eq!(
            result.1.0.title,
            "Hebûn akademik çalışması"
        );
        assert_eq!(result.1.0.status, "created");

        let service = state
            .chat_service
            .lock()
            .expect("chat service lock should succeed");

        assert_eq!(service.session_count(), 1);
        assert!(
            service
                .session(result.1.0.session_id)
                .is_some()
        );
    }

    #[tokio::test]
    async fn empty_session_title_is_rejected() {
        let state = test_state();

        let error = create_session(
            State(state),
            Json(CreateSessionRequest {
                title: " ".to_string(),
            }),
        )
        .await
        .expect_err("empty title should fail");

        assert_eq!(error.0, StatusCode::BAD_REQUEST);
        assert_eq!(
            error.1.0.error,
            "Sohbet başlığı boş olamaz."
        );
    }
}


