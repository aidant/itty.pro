use {resend_rs::Resend, sqlx::SqlitePool, std::env};

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

impl AppState {
    pub async fn new() -> Self {
        let conn = SqlitePool::connect(&env::var("DATABASE_URL").unwrap())
            .await
            .unwrap();

        sqlx::migrate!("./src/").run(&conn).await.unwrap();

        let app_state = Self {
            conn,
            email: Resend::default(),
        };

        app_state
    }
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
