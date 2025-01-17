use {
    crate::AppState,
    async_trait::async_trait,
    chrono::Utc,
    sqlx::SqliteConnection,
    thiserror::Error,
    tower_sessions::{
        cookie::time::OffsetDateTime,
        session::{Id, Record},
        session_store, ExpiredDeletion, SessionStore,
    },
};

#[derive(Error, Debug)]
pub enum SessionError {
    #[error(transparent)]
    Sqlite(#[from] sqlx::Error),
    #[error("the data persisted in sqlite was unable to be read by the json() function")]
    SqliteInvalidJson,
    #[error(transparent)]
    JsonEncode(serde_json::Error),
    #[error(transparent)]
    JsonDecode(serde_json::Error),
    #[error(transparent)]
    TimeDecode(#[from] tower_sessions::cookie::time::error::ComponentRange),
}

impl From<SessionError> for session_store::Error {
    fn from(session_error: SessionError) -> session_store::Error {
        match session_error {
            SessionError::Sqlite(_) => Self::Backend(session_error.to_string()),
            SessionError::SqliteInvalidJson => Self::Decode(session_error.to_string()),
            SessionError::JsonEncode(_) => Self::Encode(session_error.to_string()),
            SessionError::JsonDecode(_) => Self::Encode(session_error.to_string()),
            SessionError::TimeDecode(_) => Self::Encode(session_error.to_string()),
        }
    }
}

#[async_trait]
impl ExpiredDeletion for AppState {
    async fn delete_expired(&self) -> session_store::Result<()> {
        let now_ms = Utc::now().timestamp_millis();

        sqlx::query!(
            r#"
                delete from session
                where expires_at < ?
            "#,
            now_ms
        )
        .execute(&self.conn)
        .await
        .map_err(SessionError::Sqlite)?;

        Ok(())
    }
}

#[async_trait]
trait SessionStoreExt {
    async fn try_create(
        &self,
        record: &Record,
        conn: &mut SqliteConnection,
    ) -> session_store::Result<bool>;
}

#[async_trait]
impl SessionStoreExt for AppState {
    async fn try_create(
        &self,
        record: &Record,
        conn: &mut SqliteConnection,
    ) -> session_store::Result<bool> {
        let id = record.id.to_string();
        let data = serde_json::to_string(&record.data).map_err(SessionError::JsonEncode)?;
        let now_ms = Utc::now().timestamp_millis();
        let exp_ms = (record.expiry_date.unix_timestamp_nanos() / 1000000) as i64;

        match sqlx::query!(
            r#"
                insert or abort into session (id, data, created_at, updated_at, expires_at) values (?, JSONB(?), ?, ?, ?)
            "#,
            id,
            data,
            now_ms,
            now_ms,
            exp_ms
        )
        .execute(conn)
        .await {
            Ok(_) => Ok(true),
            Err(sqlx::Error::Database(e)) if e.is_unique_violation() => Ok(false),
            Err(err) => Err(SessionError::Sqlite(err).into()),
        }
    }
}

#[async_trait]
impl SessionStore for AppState {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        let mut tx = self.conn.begin().await.map_err(SessionError::Sqlite)?;

        while !self.try_create(record, &mut tx).await? {
            record.id = Id::default();
        }

        tx.commit().await.map_err(SessionError::Sqlite)?;

        Ok(())
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let id = record.id.to_string();
        let data = serde_json::to_string(&record.data).map_err(SessionError::JsonEncode)?;
        let now_ms = Utc::now().timestamp_millis();
        let exp_ms = (record.expiry_date.unix_timestamp_nanos() / 1000000) as i64;

        sqlx::query!(
            r#"
                insert into session (id, data, created_at, updated_at, expires_at) values (?, JSONB(?), ?, ?, ?)
                on conflict(id) do update set
                    data = excluded.data,
                    updated_at = excluded.updated_at,
                    expires_at = excluded.expires_at
            "#,
            id,
            data,
            now_ms,
            now_ms,
            exp_ms
        )
        .execute(&self.conn)
        .await
        .map_err(SessionError::Sqlite)?;

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> session_store::Result<Option<Record>> {
        let id = session_id.to_string();
        let now_ms = Utc::now().timestamp_millis();

        let session = sqlx::query!(
            r#"
                select json(data) as "data: String", expires_at from session
                where id = ? and expires_at > ?
            "#,
            id,
            now_ms
        )
        .fetch_optional(&self.conn)
        .await
        .map_err(SessionError::Sqlite)?;

        if let Some(session) = session {
            let data = serde_json::from_str(&session.data.ok_or(SessionError::SqliteInvalidJson)?)
                .map_err(SessionError::JsonDecode)?;

            let expiry_date =
                OffsetDateTime::from_unix_timestamp_nanos((session.expires_at * 1000000) as i128)
                    .map_err(SessionError::TimeDecode)?;

            Ok(Some(Record {
                id: *session_id,
                data,
                expiry_date,
            }))
        } else {
            Ok(None)
        }
    }

    async fn delete(&self, session_id: &Id) -> session_store::Result<()> {
        let id = session_id.to_string();

        sqlx::query!(
            r#"
                delete from session
                where id = ?
            "#,
            id
        )
        .execute(&self.conn)
        .await
        .map_err(SessionError::Sqlite)?;

        Ok(())
    }
}
