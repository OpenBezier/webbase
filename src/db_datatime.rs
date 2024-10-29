use chrono::Utc;
use lazy_static::lazy_static;
use sea_orm::entity::prelude::*;
use time::macros::offset;

lazy_static! {
    static ref EAST_8: chrono::FixedOffset = chrono::FixedOffset::east_opt(8 * 3600).unwrap();
}

fn east_8_zone() -> &'static chrono::FixedOffset {
    &EAST_8
}

fn from_utc_to_east_8(
    utc: &chrono::DateTime<chrono::Utc>,
) -> chrono::DateTime<chrono::FixedOffset> {
    utc.with_timezone(east_8_zone())
}

pub fn get_current_time() -> chrono::DateTime<chrono::Utc> {
    chrono::Utc::now()
}

pub fn get_current_time_east_8() -> chrono::DateTime<chrono::FixedOffset> {
    from_utc_to_east_8(&get_current_time())
}

pub fn get_current_time_east_8_str() -> String {
    get_current_time_east_8()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
}

pub fn get_current_datetime() -> DateTime {
    let dt: String = get_current_time_east_8_str();
    DateTime::parse_from_str(&dt, "%Y-%m-%d %H:%M:%S").unwrap()
}

pub fn get_current_day_str() -> String {
    get_current_time_east_8().format("%Y-%m-%d").to_string()
}

pub fn get_current_day() -> Date {
    let dt: String = get_current_day_str();
    Date::parse_from_str(&dt, "%Y-%m-%d").unwrap()
}

pub fn get_db_utc_datetime() -> anyhow::Result<chrono::DateTime<Utc>> {
    let now: time::OffsetDateTime = time::OffsetDateTime::now_utc().to_offset(offset!(+8));
    let now_timestamp = now.unix_timestamp_nanos();
    if let Some(utc_time) = chrono::DateTime::<Utc>::from_timestamp(
        (now_timestamp / 1000000000) as i64,
        (now_timestamp % 1000000000) as u32,
    ) {
        Ok(utc_time)
    } else {
        Err(anyhow::anyhow!("get db utc timestamp with error"))
    }
}
