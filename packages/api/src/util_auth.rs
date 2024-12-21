use async_trait::async_trait;
use axum_login::{AuthUser, AuthnBackend, UserId};
use chrono::{DateTime, Utc};
use password_auth::verify_password;
use serde::Deserialize;
use tokio::task;
use uuid::Uuid;

use crate::AppState;

#[derive(Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub display_name: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl std::fmt::Debug for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("User")
            .field("id", &self.id)
            .field("email", &self.email)
            .field("display_name", &self.display_name)
            .field("created_at", &self.created_at)
            .field("updated_at", &self.updated_at)
            .finish()
    }
}

impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

#[derive(Clone, Deserialize)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

#[async_trait]
impl AuthnBackend for AppState {
    type User = User;
    type Credentials = Credentials;
    type Error = std::convert::Infallible;

    async fn authenticate(
        &self,
        Credentials { email, password }: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
                select
                    id as "id: Uuid",
                    email,
                    display_name,
                    password,
                    created_at as "created_at: DateTime<Utc>",
                    updated_at as "updated_at: DateTime<Utc>"
                from user
                where email = ?
            "#,
            email
        )
        .fetch_optional(&self.conn)
        .await
        .unwrap();

        task::spawn_blocking(|| {
            Ok(user.filter(|user| verify_password(password, &user.password).is_ok()))
        })
        .await
        .unwrap()
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
                select
                    id as "id: Uuid",
                    email,
                    display_name,
                    password,
                    created_at as "created_at: DateTime<Utc>",
                    updated_at as "updated_at: DateTime<Utc>"
                from user
                where id = ?
            "#,
            user_id
        )
        .fetch_optional(&self.conn)
        .await
        .unwrap();

        Ok(user)
    }
}
