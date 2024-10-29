use anyhow::*;
use chrono::{DateTime, Datelike, Local, TimeZone, Utc};
use std::time::{Duration, SystemTime};

use lazy_static::lazy_static;

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

#[allow(dead_code)]
fn from_east_8_to_utc(
    east_8: &chrono::DateTime<chrono::FixedOffset>,
) -> chrono::DateTime<chrono::Utc> {
    east_8.with_timezone(&chrono::Utc)
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

pub fn get_current_time_east_8_rfc3339() -> String {
    get_current_time_east_8().to_rfc3339()
}

pub fn get_current_time_east_8_rfc2822() -> String {
    get_current_time_east_8().to_rfc2822()
}

pub fn get_current_ymd() -> String {
    get_current_time_east_8().format("%Y%m%d").to_string()
}

pub fn get_current_year() -> i32 {
    get_current_time_east_8().year()
}

pub fn get_current_month() -> u32 {
    get_current_time_east_8().month()
}

pub fn get_current_day() -> u32 {
    get_current_time_east_8().day()
}

#[derive(Debug, Default, Clone, Copy)]
pub struct TimeUtil {}

impl TimeUtil {
    pub fn get_now_duration() -> anyhow::Result<Duration> {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(now)
    }

    pub fn get_now_nanos_timestamp() -> anyhow::Result<u128> {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(now.as_nanos())
    }

    pub fn get_now_micros_timestamp() -> anyhow::Result<u128> {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(now.as_micros())
    }

    pub fn get_now_millis_timestamp() -> anyhow::Result<u128> {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(now.as_millis())
    }

    pub fn get_now_secs_timestamp() -> anyhow::Result<u64> {
        let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
        Ok(now.as_secs())
    }

    pub fn get_date_from_nano_timestamp(timestamp: u128) -> anyhow::Result<String> {
        let seconds = timestamp / 1000000000;
        let nano_se = timestamp % 1000000000;
        let d = DateTime::<Utc>::from_naive_utc_and_offset(
            DateTime::from_timestamp(seconds as i64, nano_se as u32)
                .unwrap()
                .naive_utc(),
            Utc,
        );
        let local_d = Local.from_utc_datetime(&d.naive_local());
        let d_str = local_d.format("%Y-%m-%d %H:%M:%S.%f").to_string();
        Ok(d_str)
    }

    pub fn get_rfc3339_from_micro_timestamp(timestamp: u128) -> String {
        let seconds = timestamp / 1000000;
        let nano_se = timestamp % 1000000;
        let d = DateTime::<Utc>::from_naive_utc_and_offset(
            DateTime::from_timestamp(seconds as i64, nano_se as u32)
                .unwrap()
                .naive_utc(),
            Utc,
        );
        let d = from_utc_to_east_8(&d);
        d.to_rfc3339()
    }

    pub fn get_rfc3339_from_nano_timestamp(timestamp: u128) -> String {
        let seconds = timestamp / 1000000000;
        let nano_se = timestamp % 1000000000;
        let d = DateTime::<Utc>::from_naive_utc_and_offset(
            DateTime::from_timestamp(seconds as i64, nano_se as u32)
                .unwrap()
                .naive_utc(),
            Utc,
        );
        let d = from_utc_to_east_8(&d);
        d.to_rfc3339()
    }
}

#[cfg(test)]
mod time_tests {
    use super::*;

    #[test]
    fn time_util_test() {
        let nano_time = TimeUtil::get_now_nanos_timestamp().unwrap();
        println!("nano time: {}", nano_time);

        let nano_str = TimeUtil::get_date_from_nano_timestamp(nano_time).unwrap();
        println!("nano str: {}", nano_str);
    }
}
