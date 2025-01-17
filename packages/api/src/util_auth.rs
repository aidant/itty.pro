use {
    crate::{
        store_user::{User, UserCredentials, UserError, UserStoreExt},
        AppState,
    },
    async_trait::async_trait,
    axum_login::{AuthUser, AuthnBackend, UserId},
    uuid::Uuid,
};

impl AuthUser for User {
    type Id = Uuid;

    fn id(&self) -> Self::Id {
        self.id
    }

    fn session_auth_hash(&self) -> &[u8] {
        self.password.as_bytes()
    }
}

#[async_trait]
impl AuthnBackend for AppState {
    type User = User;
    type Credentials = UserCredentials;
    type Error = UserError;

    async fn authenticate(
        &self,
        credentials: Self::Credentials,
    ) -> Result<Option<Self::User>, Self::Error> {
        let user = self.get_user_by_credentials(credentials).await?;

        Ok(user)
    }

    async fn get_user(&self, user_id: &UserId<Self>) -> Result<Option<Self::User>, Self::Error> {
        let user = self.get_user_by_id(user_id).await?;

        Ok(user)
    }
}
