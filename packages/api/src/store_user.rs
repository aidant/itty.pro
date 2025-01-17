use {
    crate::{util::uuid_and_datetime, util_token::Token, Database, Email},
    chrono::{DateTime, Duration, Utc},
    password_auth::{generate_hash, verify_password},
    resend_rs::types::CreateEmailBaseOptions,
    serde::Deserialize,
    thiserror::Error,
    tokio::task,
    uuid::Uuid,
    veil::Redact,
};

#[derive(Redact, Clone, Deserialize)]
pub struct NewUserCredentials {
    #[redact(partial)]
    pub display_name: String,
    #[redact(partial)]
    pub email: String,
    #[redact]
    pub password: String,
}

#[derive(Redact, Clone, Deserialize)]
pub struct UserCredentials {
    #[redact(partial)]
    pub email: String,
    #[redact]
    pub password: String,
}

#[derive(Redact, Clone)]
pub struct User {
    pub id: Uuid,

    #[redact(partial)]
    pub display_name: String,
    #[redact(partial)]
    pub email: String,
    pub email_verified: bool,
    #[redact]
    pub password: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<NewUserCredentials> for User {
    fn from(value: NewUserCredentials) -> Self {
        let (id, now_datetime) = uuid_and_datetime();
        let password_hash = generate_hash(value.password);

        Self {
            id,
            display_name: value.display_name,
            email: value.email,
            email_verified: false,
            password: password_hash,
            created_at: now_datetime,
            updated_at: now_datetime,
        }
    }
}

#[derive(Debug)]
pub struct UserEmailVerification {
    id: Uuid,
    user_id: Uuid,

    token: Token,

    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<&User> for UserEmailVerification {
    fn from(user: &User) -> Self {
        let (id, now_datetime) = uuid_and_datetime();
        let token = Token::new();

        Self {
            id,
            user_id: user.id,

            token,

            created_at: now_datetime,
            updated_at: now_datetime,
        }
    }
}

#[async_trait::async_trait]
pub trait UserStoreExt {
    async fn new_user(&self, credentials: NewUserCredentials) -> Result<User, UserError>;
    async fn set_user_email_verified(&self, token: &str) -> Result<Option<User>, UserError>;
    async fn get_user_by_id(&self, user_id: &Uuid) -> Result<Option<User>, UserError>;
    async fn get_user_by_credentials(
        &self,
        credentials: UserCredentials,
    ) -> Result<Option<User>, UserError>;
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error(transparent)]
    Sqlite(#[from] sqlx::Error),

    #[error(transparent)]
    Resend(#[from] resend_rs::Error),

    #[error(transparent)]
    TaskJoin(#[from] task::JoinError),
}

#[async_trait::async_trait]
impl<AppState: Database + Email> UserStoreExt for AppState {
    async fn new_user(&self, credentials: NewUserCredentials) -> Result<User, UserError> {
        let user: User = task::spawn_blocking(|| credentials.into()).await?;
        let user_email_verification: UserEmailVerification = (&user).into();

        let user_created_at = user.created_at.timestamp_millis();
        let user_updated_at = user.updated_at.timestamp_millis();
        let user_email_verification_created_at =
            user_email_verification.created_at.timestamp_millis();
        let user_email_verification_updated_at =
            user_email_verification.updated_at.timestamp_millis();

        sqlx::query!(
            r#"
                insert or rollback into user (id, display_name, email, email_verified, password, created_at, updated_at) values (?, ?, ?, ?, ?, ?, ?)
            "#,
            user.id,
            user.display_name,
            user.email,
            user.email_verified,
            user.password,
            user_created_at,
            user_updated_at,
        )
        .execute(self.conn())
        .await?;

        sqlx::query!(
            r#"
                insert or rollback into user_email_verification (id, user_id, token, created_at, updated_at) values (?, ?, ?, ?, ?)
            "#,
            user_email_verification.id,
            user_email_verification.user_id,
            user_email_verification.token,
            user_email_verification_created_at,
            user_email_verification_updated_at,
        )
        .execute(self.conn())
        .await?;

        self.email()
            .emails
            .send(
                CreateEmailBaseOptions::new(
                    "itty.pro <team@itty.pro>",
                    [&user.email],
                    "Activate your itty.pro account",
                )
                .with_text(
                    format!(
                        "https://itty.pro/app/sign-up/{}",
                        user_email_verification.token
                    )
                    .as_str(),
                ),
            )
            .await?;

        Ok(user)
    }

    async fn set_user_email_verified(&self, token: &str) -> Result<Option<User>, UserError> {
        let token: Token = match token.parse() {
            Ok(token) => token,
            Err(_) => return Ok(None),
        };

        let now_datetime = Utc::now();
        let ttl_datetime = now_datetime - Duration::hours(8);

        let now_ms = now_datetime.timestamp_millis();
        let ttl_ms = ttl_datetime.timestamp_millis();

        let mut tx = self.conn().begin().await?;

        sqlx::query!(
            r#"
                delete from user_email_verification where updated_at < ?
            "#,
            ttl_ms,
        )
        .execute(&mut *tx)
        .await?;

        let user = sqlx::query_as!(
            User,
            r#"
                update user set email_verified = true, updated_at = ?
                where id in (
                    select user_id from user_email_verification
                    where token = ?
                ) returning
                    id as "id: Uuid",
                    display_name,
                    email,
                    email_verified as "email_verified: bool",
                    password,
                    created_at as "created_at: DateTime<Utc>",
                    updated_at as "updated_at: DateTime<Utc>"
            "#,
            now_ms,
            token,
        )
        .fetch_optional(&mut *tx)
        .await?;

        sqlx::query_as!(
            User,
            r#"
                delete from user_email_verification where token = ?
            "#,
            token,
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await;

        Ok(user)
    }

    async fn get_user_by_id(&self, user_id: &Uuid) -> Result<Option<User>, UserError> {
        let user = sqlx::query_as!(
            User,
            r#"
                select
                    id as "id: Uuid",
                    display_name,
                    email,
                    email_verified as "email_verified: bool",
                    password,
                    created_at as "created_at: DateTime<Utc>",
                    updated_at as "updated_at: DateTime<Utc>"
                from user
                where id = ?
            "#,
            user_id
        )
        .fetch_optional(self.conn())
        .await?;

        Ok(user)
    }

    async fn get_user_by_credentials(
        &self,
        credentials: UserCredentials,
    ) -> Result<Option<User>, UserError> {
        let user = sqlx::query_as!(
            User,
            r#"
                select
                    id as "id: Uuid",
                    display_name,
                    email,
                    email_verified as "email_verified: bool",
                    password,
                    created_at as "created_at: DateTime<Utc>",
                    updated_at as "updated_at: DateTime<Utc>"
                from user
                where email = ?
            "#,
            credentials.email
        )
        .fetch_optional(self.conn())
        .await?;

        task::spawn_blocking(|| {
            Ok(user.filter(|user| verify_password(credentials.password, &user.password).is_ok()))
        })
        .await?
    }
}
