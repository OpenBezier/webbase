#[allow(unused_imports)]
use redis::{
    aio::MultiplexedConnection, AsyncCommands, Client, ConnectionAddr, ConnectionInfo,
    RedisConnectionInfo,
};
use redlock::RedLock;
use std::{str::FromStr, sync::OnceLock};
use tracing::info;

#[derive(Clone)]
pub struct RedisPool {
    pub client: Client,
    pub connection: MultiplexedConnection,
    pub redlock: RedLock,
}

static REDISPOOL: OnceLock<RedisPool> = OnceLock::<RedisPool>::new();

/// redis url format
/// - `{redis|rediss}://[<username>][:<password>@]<hostname>[:port][/<db>]`
/// - redis_url = "redis://default:kZhOcgQmRUu7DZH@49.235.147.196:9015/0"
/// - redis-cli -h 49.235.147.196 -p 9015 -a kZhOcgQmRUu7DZH
///   - sudo apt-get install redis-tools
pub fn init_redis_pool(redis_url: String) -> &'static RedisPool {
    REDISPOOL.get_or_init(|| {
        // let client_info = ConnectionInfo {
        //     addr: ConnectionAddr::Tcp(redis_host.clone(), redis_port),
        //     redis: RedisConnectionInfo {
        //         db: 0,
        //         username: Some("default".into()),
        //         password: Some(redis_pass.clone()),
        //     },
        // };
        let client_info = ConnectionInfo::from_str(&redis_url).unwrap();
        let client = redis::Client::open(client_info).unwrap();
        let connection = futures::executor::block_on(async {
            client.get_multiplexed_async_connection().await.unwrap()
        });

        // let redis_url = format!(
        //     "redis://default:{}@{}:{}",
        //     redis_pass, redis_host, redis_port
        // );
        let redlock = redlock::RedLock::new(vec![redis_url]);
        info!("connect to redis successfully");
        RedisPool {
            client: client,
            connection: connection,
            redlock: redlock,
        }
    })
}

pub fn get_redis_pool() -> &'static RedisPool {
    REDISPOOL.get().unwrap()
}

// const REDIS_KEY_PREFIX: &str = "KvCache";

pub async fn get_kv_cache(key: &String) -> anyhow::Result<String> {
    let mut connection = get_redis_pool().connection.clone();
    // let key = format!("{}:{}", REDIS_KEY_PREFIX, key);
    let res: String = connection.get(&key).await?;
    Ok(res)
}

pub async fn set_kv_cache(key: &String, value: &String, ex: Option<u64>) -> anyhow::Result<()> {
    let mut connection = get_redis_pool().connection.clone();
    // let key = format!("{}:{}", REDIS_KEY_PREFIX, key);
    if ex.is_none() {
        let _: () = connection.set(key, value).await?;
    } else {
        let _: () = connection.set_ex(key, value, ex.unwrap() as u64).await?;
    }
    Ok(())
}

pub async fn delete_kv_cache(key: &String) -> anyhow::Result<()> {
    let mut connection = get_redis_pool().connection.clone();
    // let key = format!("{}:{}", REDIS_KEY_PREFIX, key);
    let _: () = connection.del(key).await?;
    Ok(())
}
