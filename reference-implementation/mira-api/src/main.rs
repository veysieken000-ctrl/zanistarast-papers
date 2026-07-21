mod auth;
pub mod session;

use auth::MudebbirAuth;
use session::MudebbirSessionStore;
use axum::{
    extract::{Path, Request, State},
http::{
    header::AUTHORIZATION,
    HeaderMap,
    StatusCode,
},
   
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
    Json, Router,
};

use axum_extra::extract::cookie::{
    Cookie,
    CookieJar,
    SameSite,
};

use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use uuid::Uuid;
use zanistarast_mira::{
    chat_orchestrator::ChatInteractionResult,
    chat_service::MiraChatService,
    chat_session::ChatMessage,
    MiraTask,
};
const MUDEBBIR_SESSION_COOKIE: &str =
    "mira_mudebbir_session";

const MUDEBBIR_SESSION_TTL_SECONDS: u64 =
    30 * 60;

/// Mira API tarafından paylaşılan uygulama durumu.
#[derive(Clone)]
struct AppState {
    chat_service: Arc<Mutex<MiraChatService>>,
    repository_root: PathBuf,
    auth: MudebbirAuth,
    session_store: MudebbirSessionStore,
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

/// Sohbet oturumu ve mesaj geçmişi cevabı.
#[derive(Debug, Serialize)]
struct SessionDetailResponse {
    session_id: Uuid,
    title: String,
    created_at: String,
    updated_at: String,
    message_count: usize,
    messages: Vec<ChatMessage>,
}

/// Mira görev listesinin salt okunur cevabı.
#[derive(Debug, Serialize)]
struct TaskListResponse {
    task_count: usize,
    tasks: Vec<MiraTask>,
}

/// Mira’ya gönderilecek yazılı mesaj.
#[derive(Debug, Deserialize)]
struct SendMessageRequest {
    message: String,
}

/// Standart API hata cevabı.
#[derive(Debug, Serialize)]
struct ApiError {
    error: String,
}

/// Başarılı Müdebbir giriş cevabı.
#[derive(Debug, Serialize)]
struct MudebbirLoginResponse {
    status: &'static str,
    expires_in_seconds: u64,
}

/// Başarılı Müdebbir çıkış cevabı.
#[derive(Debug, Serialize)]
struct MudebbirLogoutResponse {
    status: &'static str,
}

/// Korunan Mira API uç noktalarında Müdebbir anahtarını doğrular.
async fn require_mudebbir(
    State(state): State<AppState>,
    jar: CookieJar,
    request: Request,
    next: Next,
) -> Result<
    Response,
    (StatusCode, Json<ApiError>),
> {
    let authorization_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let bearer_is_valid = state
        .auth
        .verify_authorization_header(
            authorization_header,
        );

    let session_is_valid =
    match jar.get(MUDEBBIR_SESSION_COOKIE) {
        Some(cookie) => state
            .session_store
            .is_valid(cookie.value())
            .map_err(|error| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiError { error }),
                )
            })?,
        None => false,
    };

    if !bearer_is_valid && !session_is_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiError {
                error:
                    "Geçerli Müdebbir erişim anahtarı veya oturumu gerekli."
                        .to_string(),
            }),
        ));
    }

    Ok(next.run(request).await)
}

/// Geçerli Müdebbir anahtarıyla kısa ömürlü güvenli oturum oluşturur.
async fn login_mudebbir(
    State(state): State<AppState>,
    headers: HeaderMap,
    jar: CookieJar,
) -> Result<
    (CookieJar, Json<MudebbirLoginResponse>),
    (StatusCode, Json<ApiError>),
> {
    let authorization_header = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    if !state
        .auth
        .verify_authorization_header(authorization_header)
    {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiError {
                error: "Geçerli Müdebbir erişim anahtarı gerekli."
                    .to_string(),
            }),
        ));
    }
  let session_id = state
        .session_store
        .create_session()
        .map_err(|error| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiError { error }),
            )
        })?;

    let cookie = Cookie::build((
        MUDEBBIR_SESSION_COOKIE,
        session_id,
    ))
    .path("/")
    .http_only(true)
    .secure(true)
    .same_site(SameSite::Strict)
    .build();

    Ok((
        jar.add(cookie),
        Json(MudebbirLoginResponse {
            status: "authenticated",
            expires_in_seconds:
                MUDEBBIR_SESSION_TTL_SECONDS,
        }),
    ))
}

/// Mevcut Müdebbir oturumunu iptal eder ve çerezi temizler.
async fn logout_mudebbir(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<
    (CookieJar, Json<MudebbirLogoutResponse>),
    (StatusCode, Json<ApiError>),
> {
    if let Some(cookie) =
        jar.get(MUDEBBIR_SESSION_COOKIE)
    {
        state
            .session_store
            .revoke_session(cookie.value())
            .map_err(|error| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiError { error }),
                )
            })?;
    }

    let removal_cookie =
        Cookie::build(MUDEBBIR_SESSION_COOKIE)
            .path("/")
            .http_only(true)
            .secure(true)
            .same_site(SameSite::Strict)
            .build();

    Ok((
        jar.remove(removal_cookie),
        Json(MudebbirLogoutResponse {
            status: "logged_out",
        }),
    ))
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
                error: "Mira sohbet servisine erişilemedi."
                    .to_string(),
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

/// Belirtilen sohbet oturumunu ve mesaj geçmişini döndürür.
async fn get_session(
    Path(session_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<
    Json<SessionDetailResponse>,
    (StatusCode, Json<ApiError>),
> {
    let service = state.chat_service.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                error: "Mira sohbet servisine erişilemedi."
                    .to_string(),
            }),
        )
    })?;

    let session = service.session(session_id).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(ApiError {
                error: format!(
                    "Mira sohbet oturumu bulunamadı: {session_id}"
                ),
            }),
        )
    })?;

    Ok(Json(SessionDetailResponse {
        session_id: session.session_id,
        title: session.title.clone(),
        created_at: session.created_at.to_rfc3339(),
        updated_at: session.updated_at.to_rfc3339(),
        message_count: session.message_count(),
        messages: session.messages().to_vec(),
    }))
}

/// Mira'da kayıtlı bütün görevleri salt okunur döndürür.
async fn list_tasks(
    State(state): State<AppState>,
) -> Result<
    Json<TaskListResponse>,
    (StatusCode, Json<ApiError>),
> {
    let service = state.chat_service.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                error: "Mira sohbet servisine erişilemedi."
                    .to_string(),
            }),
        )
    })?;

    let tasks = service.tasks().to_vec();

    Ok(Json(TaskListResponse {
        task_count: tasks.len(),
        tasks,
    }))
}

/// Kimliğine göre tek bir Mira görevini salt okunur döndürür.
async fn get_task(
    Path(task_id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<
    Json<TaskDetailResponse>,
    (StatusCode, Json<ApiError>),
> {
    let service = state.chat_service.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                error: "Mira sohbet servisine erişilemedi."
                    .to_string(),
            }),
        )
    })?;

    let task = service
        .tasks()
        .iter()
        .find(|task| task.id == task_id)
        .cloned()
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(ApiError {
                    error: format!(
                        "Mira görevi bulunamadı: {task_id}"
                    ),
                }),
            )
        })?;

    Ok(Json(TaskDetailResponse { task }))
}

/// Tek bir Mira görevinin salt okunur cevabı.
#[derive(Debug, Serialize)]
struct TaskDetailResponse {
    task: MiraTask,
}

/// Mevcut Mira sohbet oturumuna Müdebbir mesajı gönderir.
async fn send_message(
    Path(session_id): Path<Uuid>,
    State(state): State<AppState>,
    Json(request): Json<SendMessageRequest>,
) -> Result<
    Json<ChatInteractionResult>,
    (StatusCode, Json<ApiError>),
> {
    let message = request.message.trim();

    if message.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ApiError {
                error: "Mesaj boş olamaz.".to_string(),
            }),
        ));
    }

    let mut service = state.chat_service.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiError {
                error: "Mira sohbet servisine erişilemedi."
                    .to_string(),
            }),
        )
    })?;

    let result = service
        .send_message(
            session_id,
            message,
            &state.repository_root,
        )
        .map_err(|error| {
            let status = if error.kind()
                == std::io::ErrorKind::NotFound
            {
                StatusCode::NOT_FOUND
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };

            (
                status,
                Json(ApiError {
                    error: error.to_string(),
                }),
            )
        })?;

    Ok(Json(result))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let auth = MudebbirAuth::from_environment().expect(
        "MIRA_MUDEBBIR_TOKEN ortam değişkeni tanımlanmalıdır",
    );

    let repository_root =
        std::env::var("MIRA_REPOSITORY_ROOT")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                std::env::current_dir()
                    .unwrap_or_else(|_| PathBuf::from("."))
            });

    let state = AppState {
        chat_service: Arc::new(Mutex::new(
            MiraChatService::new(),
        )),
        repository_root,
        auth: auth.clone(),
        session_store: MudebbirSessionStore::new(
            std::time::Duration::from_secs(
                MUDEBBIR_SESSION_TTL_SECONDS,
            ),
        ),
    };

    let protected_routes = Router::new()
        .route("/sessions", post(create_session))
        .route(
            "/sessions/{session_id}",
            get(get_session),
        )
        .route(
            "/sessions/{session_id}/messages",
            post(send_message),
        )
        .route("/tasks", get(list_tasks))
        .route(
            "/tasks/{task_id}",
            get(get_task),
        )
        .layer(
    middleware::from_fn_with_state(
        state.clone(),
        require_mudebbir,
    ),
);
    let app = Router::new()
        .route("/health", get(health))
        .route("/auth/login", post(login_mudebbir))
        .route("/auth/logout", post(logout_mudebbir))
        .merge(protected_routes)
        .with_state(state);

    let port = std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u16>().ok())
        .unwrap_or(3000);

    let address =
        SocketAddr::from(([0, 0, 0, 0], port));

    let listener =
        tokio::net::TcpListener::bind(address)
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
    use std::fs;
    use axum::body::Body;
    use tower::ServiceExt;
    
    fn create_test_repository() -> PathBuf {
        let test_root = std::env::temp_dir().join(format!(
            "zanistarast-mira-api-{}",
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
        .expect("Hebûn page should be written");

        test_root
    }

    fn test_state(repository_root: PathBuf) -> AppState {
    AppState {
    chat_service: Arc::new(Mutex::new(
        MiraChatService::new(),
    )),
    repository_root,
    auth: MudebbirAuth::new(
        "zanistarast-mudebbir-test-token-0001",
    )
    .expect("test token should be accepted"),
    session_store: MudebbirSessionStore::new(
  std::time::Duration::from_secs(
    MUDEBBIR_SESSION_TTL_SECONDS,
),

    ),
}
  }  
    #[tokio::test]
async fn protected_route_rejects_missing_token() {
    let test_root = create_test_repository();
    let state = test_state(test_root.clone());

    let app = Router::new()
        .route("/tasks", get(list_tasks))
        .layer(
           middleware::from_fn_with_state(
    state.clone(),
    require_mudebbir,
            ),
        )
        .with_state(state);

    let request = Request::builder()
        .uri("/tasks")
        .body(Body::empty())
        .expect("request should be created");

    let response = app
        .oneshot(request)
        .await
        .expect("router request should complete");

    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED
    );

    fs::remove_dir_all(test_root)
        .expect("test directory should be removed");
}

  #[tokio::test]
async fn login_rejects_missing_mudebbir_token() {
    let test_root = create_test_repository();
    let state = test_state(test_root.clone());

    let app = Router::new()
        .route("/auth/login", post(login_mudebbir))
        .with_state(state);

    let request = Request::builder()
        .method("POST")
        .uri("/auth/login")
        .body(Body::empty())
        .expect("request should be created");

    let response = app
        .oneshot(request)
        .await
        .expect("router request should complete");

    assert_eq!(
        response.status(),
        StatusCode::UNAUTHORIZED
    );

    fs::remove_dir_all(test_root)
        .expect("test directory should be removed");
}

#[tokio::test]
async fn login_sets_secure_session_cookie() {
    let test_root = create_test_repository();
    let state = test_state(test_root.clone());

    let app = Router::new()
        .route("/auth/login", post(login_mudebbir))
        .with_state(state);

    let request = Request::builder()
        .method("POST")
        .uri("/auth/login")
        .header(
            AUTHORIZATION,
            "Bearer zanistarast-mudebbir-test-token-0001",
        )
        .body(Body::empty())
        .expect("request should be created");

    let response = app
        .oneshot(request)
        .await
        .expect("router request should complete");

    assert_eq!(response.status(), StatusCode::OK);

    let set_cookie = response
        .headers()
        .get(axum::http::header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .expect("session cookie should be returned");

    assert!(set_cookie.contains(
        "mira_mudebbir_session="
    ));
    assert!(set_cookie.contains("HttpOnly"));
    assert!(set_cookie.contains("Secure"));
    assert!(set_cookie.contains("SameSite=Strict"));

    fs::remove_dir_all(test_root)
        .expect("test directory should be removed");
}

#[tokio::test]
async fn protected_route_accepts_valid_session_cookie() {
    let test_root = create_test_repository();
    let state = test_state(test_root.clone());

    let protected_routes = Router::new()
        .route("/tasks", get(list_tasks))
        .layer(
            middleware::from_fn_with_state(
                state.clone(),
                require_mudebbir,
            ),
        );

    let app = Router::new()
        .route("/auth/login", post(login_mudebbir))
        .merge(protected_routes)
        .with_state(state);

    let login_request = Request::builder()
        .method("POST")
        .uri("/auth/login")
        .header(
            AUTHORIZATION,
            "Bearer zanistarast-mudebbir-test-token-0001",
        )
        .body(Body::empty())
        .expect("login request should be created");

    let login_response = app
        .clone()
        .oneshot(login_request)
        .await
        .expect("login request should complete");

    assert_eq!(
        login_response.status(),
        StatusCode::OK
    );

    let session_cookie = login_response
        .headers()
        .get(axum::http::header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(';').next())
        .expect("session cookie should be returned")
        .to_string();

    let protected_request = Request::builder()
        .uri("/tasks")
        .header(
            axum::http::header::COOKIE,
            session_cookie,
        )
        .body(Body::empty())
        .expect("protected request should be created");

    let protected_response = app
        .oneshot(protected_request)
        .await
        .expect("protected request should complete");

    assert_eq!(
        protected_response.status(),
        StatusCode::OK
    );

    fs::remove_dir_all(test_root)
        .expect("test directory should be removed");
}
    
#[tokio::test]
async fn protected_route_accepts_valid_token() {
    let test_root = create_test_repository();
    let state = test_state(test_root.clone());

    let app = Router::new()
        .route("/tasks", get(list_tasks))
        .layer(
          middleware::from_fn_with_state(
    state.clone(),
    require_mudebbir,
), 
        )
        .with_state(state);

    let request = Request::builder()
        .uri("/tasks")
        .header(
            AUTHORIZATION,
            "Bearer zanistarast-mudebbir-test-token-0001",
        )
        .body(Body::empty())
        .expect("request should be created");

    let response = app
        .oneshot(request)
        .await
        .expect("router request should complete");

    assert_eq!(response.status(), StatusCode::OK);

    fs::remove_dir_all(test_root)
        .expect("test directory should be removed");
}

#[tokio::test]
async fn logout_revokes_session_and_clears_cookie() {
    let test_root = create_test_repository();
    let state = test_state(test_root.clone());

    let app = Router::new()
        .route("/auth/login", post(login_mudebbir))
        .route("/auth/logout", post(logout_mudebbir))
        .with_state(state.clone());

    let login_request = Request::builder()
        .method("POST")
        .uri("/auth/login")
        .header(
            AUTHORIZATION,
            "Bearer zanistarast-mudebbir-test-token-0001",
        )
        .body(Body::empty())
        .expect("login request should be created");

    let login_response = app
        .clone()
        .oneshot(login_request)
        .await
        .expect("login request should complete");

    let session_cookie = login_response
        .headers()
        .get(axum::http::header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(';').next())
        .expect("session cookie should be returned")
        .to_string();

    let session_id = session_cookie
        .split_once('=')
        .map(|(_, value)| value)
        .expect("session id should exist")
        .to_string();

    assert!(
        state
            .session_store
            .is_valid(&session_id)
            .expect("session should be checked")
    );

    let logout_request = Request::builder()
        .method("POST")
        .uri("/auth/logout")
        .header(
            axum::http::header::COOKIE,
            session_cookie,
        )
        .body(Body::empty())
        .expect("logout request should be created");

    let logout_response = app
        .oneshot(logout_request)
        .await
        .expect("logout request should complete");

    assert_eq!(
        logout_response.status(),
        StatusCode::OK
    );

    assert!(
        !state
            .session_store
            .is_valid(&session_id)
            .expect("revoked session should be checked")
    );

    let cleared_cookie = logout_response
        .headers()
        .get(axum::http::header::SET_COOKIE)
        .and_then(|value| value.to_str().ok())
        .expect("cleared cookie should be returned");

    assert!(cleared_cookie.contains(
        "mira_mudebbir_session="
    ));
    assert!(
        cleared_cookie.contains("Max-Age=0")
            || cleared_cookie.contains("Expires=")
    );

    fs::remove_dir_all(test_root)
        .expect("test directory should be removed");
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
        let test_root = create_test_repository();
        let state = test_state(test_root.clone());

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

        drop(service);

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }

    #[tokio::test]
    async fn message_endpoint_processes_repository_command() {
        let test_root = create_test_repository();
        let state = test_state(test_root.clone());

        let session_id = {
            let mut service = state
                .chat_service
                .lock()
                .expect("chat service lock should succeed");

            service.create_session(
                "Repository incelemesi",
            )
        };

        let result = send_message(
            Path(session_id),
            State(state.clone()),
            Json(SendMessageRequest {
                message: "depo tara".to_string(),
            }),
        )
        .await
        .expect("message should succeed");

        assert!(result.0.command_executed);
        assert!(result.0.created_task_id.is_some());
        assert_eq!(result.0.session_id, session_id);

        let service = state
            .chat_service
            .lock()
            .expect("chat service lock should succeed");

        assert_eq!(service.task_count(), 1);

        let session = service
            .session(session_id)
            .expect("session should exist");

        assert_eq!(session.message_count(), 2);

        drop(service);

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }

    #[tokio::test]
    async fn empty_message_is_rejected() {
        let test_root = create_test_repository();
        let state = test_state(test_root.clone());

        let session_id = {
            let mut service = state
                .chat_service
                .lock()
                .expect("chat service lock should succeed");

            service.create_session("Boş mesaj testi")
        };

        let error = send_message(
            Path(session_id),
            State(state),
            Json(SendMessageRequest {
                message: " ".to_string(),
            }),
        )
        .await
        .expect_err("empty message should fail");

        assert_eq!(error.0, StatusCode::BAD_REQUEST);
        assert_eq!(
            error.1.0.error,
            "Mesaj boş olamaz."
        );

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }

    #[tokio::test]
async fn session_detail_returns_message_history() {
    let test_root = create_test_repository();
    let state = test_state(test_root.clone());

    let session_id = {
        let mut service = state
            .chat_service
            .lock()
            .expect("chat service lock should succeed");

        service.create_session(
            "Hebûn mesaj geçmişi",
        )
    };

   let _ = send_message(
    Path(session_id),
    State(state.clone()),
    Json(SendMessageRequest {
        message: "durum".to_string(),
    }),
)
.await
.expect("message should succeed");

    let response = get_session(
        Path(session_id),
        State(state),
    )
    .await
    .expect("session detail should succeed");

    assert_eq!(response.0.session_id, session_id);
    assert_eq!(
        response.0.title,
        "Hebûn mesaj geçmişi"
    );
    assert_eq!(response.0.message_count, 2);
    assert_eq!(response.0.messages.len(), 2);

    fs::remove_dir_all(test_root)
        .expect("test directory should be removed");
}

#[tokio::test]
async fn unknown_session_detail_returns_not_found() {
    let test_root = create_test_repository();
    let state = test_state(test_root.clone());

    let error = get_session(
        Path(Uuid::new_v4()),
        State(state),
    )
    .await
    .expect_err("unknown session should fail");

    assert_eq!(error.0, StatusCode::NOT_FOUND);
    assert!(
        error
            .1
            .0
            .error
            .contains("Mira sohbet oturumu bulunamadı")
    );

    fs::remove_dir_all(test_root)
        .expect("test directory should be removed");
}
    
    #[tokio::test]
    async fn unknown_session_returns_not_found() {
        let test_root = create_test_repository();
        let state = test_state(test_root.clone());

        let error = send_message(
            Path(Uuid::new_v4()),
            State(state),
            Json(SendMessageRequest {
                message: "durum".to_string(),
            }),
        )
        .await
        .expect_err("unknown session should fail");

        assert_eq!(error.0, StatusCode::NOT_FOUND);

        fs::remove_dir_all(test_root)
            .expect("test directory should be removed");
    }
#[tokio::test]
async fn task_list_starts_empty() {
    let test_root = create_test_repository();
    let state = test_state(test_root.clone());

    let response = list_tasks(
        State(state),
    )
    .await
    .expect("task list should succeed");

    assert_eq!(response.0.task_count, 0);
    assert!(response.0.tasks.is_empty());

    fs::remove_dir_all(test_root)
        .expect("test directory should be removed");
}

#[tokio::test]
async fn task_list_returns_created_repository_task() {
    let test_root = create_test_repository();
    let state = test_state(test_root.clone());

    let session_id = {
        let mut service = state
            .chat_service
            .lock()
            .expect("chat service lock should succeed");

        service.create_session(
            "Görev listesi testi",
        )
    };

    let _ = send_message(
        Path(session_id),
        State(state.clone()),
        Json(SendMessageRequest {
            message: "depo tara".to_string(),
        }),
    )
    .await
    .expect("message should succeed");

    let response = list_tasks(
        State(state),
    )
    .await
    .expect("task list should succeed");

    assert_eq!(response.0.task_count, 1);
    assert_eq!(
        response.0.tasks[0].title,
        "Salt okunur repository taraması"
    );

    fs::remove_dir_all(test_root)
        .expect("test directory should be removed");
}

    #[tokio::test]
async fn task_detail_returns_created_task() {
    let test_root = create_test_repository();
    let state = test_state(test_root.clone());

    let session_id = {
        let mut service = state
            .chat_service
            .lock()
            .expect("chat service lock should succeed");

        service.create_session(
            "Tek görev ayrıntısı testi",
        )
    };

    let interaction = send_message(
        Path(session_id),
        State(state.clone()),
        Json(SendMessageRequest {
            message: "depo tara".to_string(),
        }),
    )
    .await
    .expect("message should succeed");

    let task_id = interaction
        .0
        .created_task_id
        .expect("task id should exist");

    let response = get_task(
        Path(task_id),
        State(state),
    )
    .await
    .expect("task detail should succeed");

    assert_eq!(response.0.task.id, task_id);
    assert_eq!(
        response.0.task.title,
        "Salt okunur repository taraması"
    );
    assert!(!response.0.task.requires_mudebbir_approval);

    fs::remove_dir_all(test_root)
        .expect("test directory should be removed");
}

#[tokio::test]
async fn unknown_task_detail_returns_not_found() {
    let test_root = create_test_repository();
    let state = test_state(test_root.clone());
    let task_id = Uuid::new_v4();

    let error = get_task(
        Path(task_id),
        State(state),
    )
    .await
    .expect_err("unknown task should fail");

    assert_eq!(error.0, StatusCode::NOT_FOUND);
    assert_eq!(
        error.1.0.error,
        format!("Mira görevi bulunamadı: {task_id}")
    );

    fs::remove_dir_all(test_root)
        .expect("test directory should be removed");
    }

 }

