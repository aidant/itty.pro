use {
    chrono::{DateTime, Utc},
    std::num::TryFromIntError,
    thiserror::Error,
    uuid::Uuid,
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

pub fn uuid_to_ts(uuid: &Uuid) -> Result<DateTime<Utc>, DateTimeError> {
    let uuid_ts = uuid
        .get_timestamp()
        .ok_or(DateTimeError::IncorrectUuidVersion)?;
    let (secs, nsecs) = uuid_ts.to_unix();
    let ts = DateTime::from_timestamp(secs.try_into()?, nsecs).ok_or(DateTimeError::OutOfRange)?;
    Ok(ts)
}

pub fn uuid_and_ts() -> (Uuid, DateTime<Utc>) {
    let uuid = Uuid::now_v7();
    let ts = uuid_to_ts(&uuid).unwrap_or_else(|_| Utc::now());

    (uuid, ts)
}
