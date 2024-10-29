use influxdb::Client;
use std::sync::OnceLock;

static TSDB: OnceLock<Client> = OnceLock::<Client>::new();

pub fn init_influxdb(dburl: String, db: String, token: String) -> &'static Client {
    TSDB.get_or_init(|| {
        let client = Client::new(dburl, db).with_token(token);
        client
    })
}

pub fn get_influxdb() -> &'static Client {
    TSDB.get().unwrap()
}
