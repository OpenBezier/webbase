use super::influxclient::get_influxdb;
use crate::time_util;
use anyhow::anyhow;
#[allow(unused_imports)]
use chrono::{DateTime, FixedOffset, Utc};
use influxdb::InfluxDbWriteable;
use influxdb::{ReadQuery, WriteQuery};
use tracing::error;

static SYS_STATUS_ID: &str = "system_status";

#[derive(InfluxDbWriteable)]
pub struct TsSysStatus {
    pub time: DateTime<Utc>,
    pub data: String,
}

impl TsSysStatus {
    pub async fn write(data: String) {
        let client = get_influxdb();
        let tsdata = TsSysStatus {
            time: time_util::get_current_time(),
            data: data,
        }
        .into_query(SYS_STATUS_ID);
        let result = client.query(tsdata).await;
        if result.is_err() {
            error!("write ts data error: {:?}", result.err());
        }
    }

    pub async fn write_batch(data: Vec<String>) {
        let client = get_influxdb();
        let tsdata: Vec<WriteQuery> = data
            .into_iter()
            .map(|v| {
                TsSysStatus {
                    time: time_util::get_current_time(),
                    data: v,
                }
                .into_query(SYS_STATUS_ID)
            })
            .collect();
        let result = client.query(tsdata).await;
        if result.is_err() {
            error!("write ts data error: {:?}", result.err());
        }
    }

    pub async fn read() -> anyhow::Result<()> {
        let client = get_influxdb();
        let read_query = ReadQuery::new(format!("SELECT * FROM {}", SYS_STATUS_ID));
        let read_result = client.query(read_query).await;
        if read_result.is_err() {
            return Err(anyhow!("write ts data error: {:?}", read_result.err()));
        }
        let tsdata = read_result.unwrap();
        println!("{:?}", tsdata);
        anyhow::Ok(())
    }
}
