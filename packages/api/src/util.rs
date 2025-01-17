use {
    chrono::{DateTime, Utc},
    std::num::TryFromIntError,
    thiserror::Error,
    uuid::{Timestamp, Uuid},
};

#[derive(Error, Debug)]
pub enum DateTimeError {
    #[error("out-of-range number of seconds and/or invalid nanosecond")]
    OutOfRange,
    #[error(transparent)]
    TryFromIntError(#[from] TryFromIntError),
    #[error("incorrect uuid version expected v1, v6, or v7")]
    IncorrectUuidVersion,
}

pub fn ts_to_datetime(ts: Timestamp) -> Result<DateTime<Utc>, DateTimeError> {
    let (secs, nsecs) = ts.to_unix();
    let t = DateTime::from_timestamp(secs.try_into()?, nsecs).ok_or(DateTimeError::OutOfRange)?;
    Ok(t)
}

pub fn ts_to_ms(ts: Timestamp) -> Result<i64, DateTimeError> {
    let t = ts_to_datetime(ts)?;
    Ok(t.timestamp_millis())
}

pub fn uuid_to_datetime(uuid: &Uuid) -> Result<DateTime<Utc>, DateTimeError> {
    let ts = uuid
        .get_timestamp()
        .ok_or(DateTimeError::IncorrectUuidVersion)?;
    let t = ts_to_datetime(ts)?;
    Ok(t)
}

pub fn uuid_to_ms(uuid: &Uuid) -> Result<i64, DateTimeError> {
    let ts = uuid
        .get_timestamp()
        .ok_or(DateTimeError::IncorrectUuidVersion)?;
    ts_to_ms(ts)
}

pub fn uuid_and_datetime() -> (Uuid, DateTime<Utc>) {
    let uuid = Uuid::now_v7();
    let now = uuid_to_datetime(&uuid).unwrap_or_else(|_| Utc::now());

    (uuid, now)
}
