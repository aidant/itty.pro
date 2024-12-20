use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tower_sessions::{
    cookie::time::OffsetDateTime,
    session::{Id, Record},
    session_store, ExpiredDeletion, SessionStore,
};
use tracing::debug;

use crate::AppState;

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
        .map_err(|err| session_store::Error::Backend(err.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl SessionStore for AppState {
    async fn create(&self, record: &mut Record) -> session_store::Result<()> {
        let id = record.id.to_string();
        let data = serde_json::to_string(&record.data)
            .map_err(|err| session_store::Error::Backend(err.to_string()))?;
        let now_ms = Utc::now().timestamp_millis();
        let exp_ms = (record.expiry_date.unix_timestamp_nanos() / 1000000) as i64;

        sqlx::query!(
            r#"
                insert or abort into session (id, data, created_at, updated_at, expires_at) values (?, JSONB(?), ?, ?, ?)
            "#,
            id,
            data,
            now_ms,
            now_ms,
            exp_ms
        )
        .execute(&self.conn)
        .await
        .map_err(|err| session_store::Error::Backend(err.to_string()))?;

        Ok(())
    }

    async fn save(&self, record: &Record) -> session_store::Result<()> {
        let id = record.id.to_string();
        let data = serde_json::to_string(&record.data)
            .map_err(|err| session_store::Error::Backend(err.to_string()))?;
        let now_ms = Utc::now().timestamp_millis();
        let exp_ms = (record.expiry_date.unix_timestamp_nanos() / 1000000) as i64;

        sqlx::query!(
            r#"
                insert into session (id, data, created_at, updated_at, expires_at) values (?, JSONB(?), ?, ?, ?)
                on conflict(id) do update set
                    data = excluded.data, updated_at = excluded.updated_at, expires_at = excluded.expires_at
            "#,
            id,
            data,
            now_ms,
            now_ms,
            exp_ms
        )
        .execute(&self.conn)
        .await
        .map_err(|err| session_store::Error::Backend(err.to_string()))?;

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
        .map_err(|err| session_store::Error::Backend(err.to_string()))?;

        if let Some(session) = session {
            let data = serde_json::from_str(
                &session
                    .data
                    .ok_or_else(|| {
                        session_store::Error::Backend(
                            "the data stored in sqlite was unable to be processed by json()"
                                .to_string(),
                        )
                    })?
                    .as_str(),
            )
            .map_err(|err| session_store::Error::Decode(err.to_string()))?;

            let expiry_date =
                OffsetDateTime::from_unix_timestamp_nanos((session.expires_at * 1000000) as i128)
                    .map_err(|err| session_store::Error::Decode(err.to_string()))?;

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
        .map_err(|err| session_store::Error::Backend(err.to_string()))?;

        Ok(())
    }
}
