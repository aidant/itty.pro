use anyhow::Context;
use chrono::DateTime;
use uuid::{Timestamp, Uuid};

pub fn ts_to_ms(ts: Timestamp) -> Result<i64, anyhow::Error> {
    let (secs, nsecs) = ts.to_unix();
    let t = DateTime::from_timestamp(secs.try_into().context("secs as i64")?, nsecs)
        .context("DateTime::from_timestamp")?;
    Ok(t.timestamp_millis())
}

pub fn uuid_to_ms(uuid: &Uuid) -> Result<i64, anyhow::Error> {
    let ts = uuid.get_timestamp().context("uuid.get_timestamp()")?;
    ts_to_ms(ts)
}
