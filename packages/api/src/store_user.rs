use {
    crate::{
        util_app_error::InternalServerError,
        util_app_state::{Database, Email},
        util_token::Token,
        util_uuid::uuid_and_ts,
    },
    axum::{response::IntoResponse, Json},
    chrono::{DateTime, Duration, Utc},
    hyper::StatusCode,
    password_auth::{generate_hash, verify_password},
    resend_rs::types::CreateEmailBaseOptions,
    serde::{Deserialize, Serialize},
    thiserror::Error,
    tokio::task,
    utoipa::ToSchema,
    uuid::Uuid,
    veil::Redact,
};

#[derive(Redact, Clone, Deserialize, ToSchema)]
pub struct NewUserCredentials {
    #[redact(partial)]
    pub display_name: String,
    #[redact(partial)]
    pub email: String,
    #[redact]
    pub password: String,
}

#[derive(Redact, Clone, Deserialize, ToSchema)]
pub struct UserCredentials {
    #[redact(partial)]
    pub email: String,
    #[redact]
    pub password: String,
}

#[derive(Redact, Clone, Serialize, ToSchema)]
pub struct User {
    #[serde(skip_serializing)]
    pub id: Uuid,

    #[redact(partial)]
    pub display_name: String,
    #[redact(partial)]
    pub email: String,
    pub email_verified: bool,
    #[redact]
    #[serde(skip_serializing)]
    pub password: String,

    #[serde(skip_serializing)]
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing)]
    pub updated_at: DateTime<Utc>,
}

impl From<NewUserCredentials> for User {
    fn from(value: NewUserCredentials) -> Self {
        let (id, now_ts) = uuid_and_ts();
        let password_hash = generate_hash(value.password);

        Self {
            id,
            display_name: value.display_name,
            email: value.email,
            email_verified: false,
            password: password_hash,
            created_at: now_ts,
            updated_at: now_ts,
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
        let (id, now_ts) = uuid_and_ts();
        let token = Token::new();

        Self {
            id,
            user_id: user.id,

            token,

            created_at: now_ts,
            updated_at: now_ts,
        }
    }
}

#[derive(Error, Debug, Serialize, ToSchema)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum NewUserError {
    #[error("account exists")]
    AccountExists,
    #[error("invalid email")]
    InvalidEmail,
    #[error(transparent)]
    InternalServerError {
        #[serde(skip)]
        #[from]
        error: anyhow::Error,
    },
}

impl Into<StatusCode> for &NewUserError {
    fn into(self) -> StatusCode {
        match self {
            NewUserError::AccountExists => StatusCode::UNPROCESSABLE_ENTITY,
            NewUserError::InvalidEmail => StatusCode::UNPROCESSABLE_ENTITY,
            NewUserError::InternalServerError { error: _ } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for NewUserError {
    fn into_response(self) -> axum::response::Response {
        (Into::<StatusCode>::into(&self), Json(self)).into_response()
    }
}

#[async_trait::async_trait]
pub trait UserStoreExt {
    async fn new_user(&self, credentials: NewUserCredentials) -> Result<User, NewUserError>;
    async fn set_user_email_verified(
        &self,
        token: &str,
    ) -> Result<Option<User>, InternalServerError>;
    async fn get_user_by_id(&self, user_id: &Uuid) -> Result<Option<User>, InternalServerError>;
    async fn get_user_by_credentials(
        &self,
        credentials: UserCredentials,
    ) -> Result<Option<User>, InternalServerError>;
}

#[async_trait::async_trait]
impl<AppState: Database + Email> UserStoreExt for AppState {
    async fn new_user(&self, credentials: NewUserCredentials) -> Result<User, NewUserError> {
        let user: User = task::spawn_blocking(|| credentials.into())
            .await
            .map_err(anyhow::Error::new)?;
        let user_email_verification: UserEmailVerification = (&user).into();

        let user_created_at = user.created_at.timestamp_millis();
        let user_updated_at = user.updated_at.timestamp_millis();
        let user_email_verification_created_at =
            user_email_verification.created_at.timestamp_millis();
        let user_email_verification_updated_at =
            user_email_verification.updated_at.timestamp_millis();

        let mut tx = self.conn().begin().await.map_err(anyhow::Error::new)?;

        sqlx::query!(
            r#"
                insert into user (id, display_name, email, email_verified, password, created_at, updated_at) values (?, ?, ?, ?, ?, ?, ?)
            "#,
            user.id,
            user.display_name,
            user.email,
            user.email_verified,
            user.password,
            user_created_at,
            user_updated_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(|err| match err {
            sqlx::Error::Database(err) if err.is_unique_violation() => NewUserError::AccountExists,
            _ => NewUserError::InternalServerError { error: err.into() }
        })?;

        sqlx::query!(
            r#"
                insert into user_email_verification (id, user_id, token, created_at, updated_at) values (?, ?, ?, ?, ?)
            "#,
            user_email_verification.id,
            user_email_verification.user_id,
            user_email_verification.token,
            user_email_verification_created_at,
            user_email_verification_updated_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(anyhow::Error::new)?;

        let email_err = self
            .email()
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
            .await
            .map_err(|err| match err {
                resend_rs::Error::Resend(error_response)
                    if error_response.kind() == resend_rs::types::ErrorKind::InvalidToAddress =>
                {
                    NewUserError::InvalidEmail
                }
                _ => NewUserError::InternalServerError { error: err.into() },
            })
            .err();

        match email_err {
            Some(NewUserError::InvalidEmail) => {
                let _ = tx.rollback().await;
                return Err(NewUserError::InvalidEmail);
            }
            _ => tx.commit().await.map_err(anyhow::Error::new)?,
        }

        Ok(user)
    }

    async fn set_user_email_verified(
        &self,
        token: &str,
    ) -> Result<Option<User>, InternalServerError> {
        let token: Token = match token.parse() {
            Ok(token) => token,
            Err(_) => return Ok(None),
        };

        let now_ts = Utc::now();
        let ttl_datetime = now_ts - Duration::hours(8);

        let now_ms = now_ts.timestamp_millis();
        let ttl_ms = ttl_datetime.timestamp_millis();

        let mut tx = self.conn().begin().await.map_err(anyhow::Error::new)?;

        sqlx::query!(
            r#"
                delete from user_email_verification where updated_at < ?
            "#,
            ttl_ms,
        )
        .execute(&mut *tx)
        .await
        .map_err(anyhow::Error::new)?;

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
        .await
        .map_err(anyhow::Error::new)?;

        sqlx::query_as!(
            User,
            r#"
                delete from user_email_verification where token = ?
            "#,
            token,
        )
        .execute(&mut *tx)
        .await
        .map_err(anyhow::Error::new)?;

        tx.commit().await.map_err(anyhow::Error::new)?;

        Ok(user)
    }

    async fn get_user_by_id(&self, user_id: &Uuid) -> Result<Option<User>, InternalServerError> {
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
        .await
        .map_err(anyhow::Error::new)?;

        Ok(user)
    }

    async fn get_user_by_credentials(
        &self,
        credentials: UserCredentials,
    ) -> Result<Option<User>, InternalServerError> {
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
        .await
        .map_err(anyhow::Error::new)?;

        task::spawn_blocking(|| {
            Ok(user.filter(|user| verify_password(credentials.password, &user.password).is_ok()))
        })
        .await
        .map_err(anyhow::Error::new)?
    }
}
