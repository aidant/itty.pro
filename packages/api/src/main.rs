use {
    axum::{
        extract::MatchedPath,
        http::{Request, StatusCode},
        response::IntoResponse,
        Json,
    },
    axum_login::AuthManagerLayerBuilder,
    resend_rs::Resend,
    serde_json::json,
    sqlx::SqlitePool,
    std::{env, net::SocketAddr},
    tokio::net::TcpListener,
    tower_http::trace::TraceLayer,
    tower_sessions::SessionManagerLayer,
    tracing::info_span,
    tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt},
    util_https::{serve_http, serve_https, InsecureCertificateResolver},
};

mod routes;
mod store_user;
mod util;
mod util_auth;
mod util_https;
mod util_session;
mod util_token;

pub(crate) trait Database: Send + Sync {
    fn conn(&self) -> &SqlitePool;
}

pub(crate) trait Email: Send + Sync {
    fn email(&self) -> &Resend;
}

#[derive(Clone, Debug)]
pub(crate) struct AppState {
    pub conn: SqlitePool,
    pub email: Resend,
}

impl Database for AppState {
    #[inline]
    fn conn(&self) -> &SqlitePool {
        &self.conn
    }
}

impl Email for AppState {
    #[inline]
    fn email(&self) -> &Resend {
        &self.email
    }
}

pub(crate) struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        tracing::debug!("{}", self.0);

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({ "error": "Internal server error" })),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        Self(error.into())
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let conn = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    sqlx::migrate!("./src/").run(&conn).await.unwrap();

    let app_state = AppState {
        conn,
        email: Resend::default(),
    };

    let session_layer = SessionManagerLayer::new(app_state.clone())
        .with_same_site(tower_sessions::cookie::SameSite::None);

    let auth_layer = AuthManagerLayerBuilder::new(app_state.clone(), session_layer).build();

    tokio::try_join!(
        serve_http(
            TcpListener::bind("127.0.0.1:8080").await.unwrap(),
            routes::AppRouter::http()
                .with_state(app_state.clone())
                .into_make_service_with_connect_info::<SocketAddr>(),
        ),
        serve_https(
            TcpListener::bind("127.0.0.1:3000").await.unwrap(),
            routes::AppRouter::https()
                .layer(auth_layer)
                .layer(
                    TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                        let matched_path = request
                            .extensions()
                            .get::<MatchedPath>()
                            .map(MatchedPath::as_str);

                        info_span!("http_request", method = ?request.method(), matched_path)
                    }),
                )
                .with_state(app_state)
                .into_make_service_with_connect_info::<SocketAddr>(),
            InsecureCertificateResolver::new(),
        )
    )
    .unwrap();
}
