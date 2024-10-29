use fred::types::Expiration::EX;
use fred::{
    prelude::*,
    types::{PerformanceConfig, RedisConfig},
};
use redlock::RedLock;
use std::sync::OnceLock;
use tracing::{debug, info};

#[derive(Clone)]
pub struct RedisCachePool {
    pub client: RedisPool,
    pub redlock: RedLock,
}

static REDISPOOL: OnceLock<RedisCachePool> = OnceLock::<RedisCachePool>::new();

/// redis url format
/// - `{redis|rediss}://[<username>][:<password>@]<hostname>[:port][/<db>]`
pub fn init_redis_pool(redis_url: String) -> &'static RedisCachePool {
    REDISPOOL.get_or_init(|| {
        // let redis_url = format!(
        //     "redis://default:{}@{}:{}",
        //     redis_pass.clone(),
        //     redis_host.clone(),
        //     redis_port.clone()
        // );

        // let redis_config = RedisConfig {
        //     server: ServerConfig::new_centralized(&redis_host, redis_port),
        //     password: Some(redis_pass),
        //     ..Default::default()
        // };

        let redis_config = RedisConfig::from_url(&redis_url).unwrap();
        let performance = PerformanceConfig {
            default_command_timeout: std::time::Duration::from_millis(1000 * 3),
            ..Default::default()
        };
        let policy = ReconnectPolicy::new_linear(5, 1000 * 5, 100);
        let connection_config = ConnectionConfig::default();

        debug!("redis config: {:?}.", &redis_config);
        let redis_pool = RedisPool::new(
            redis_config,
            Some(performance),
            Some(connection_config),
            Some(policy),
            5,
        )
        .unwrap();
        let _join_handler = redis_pool.connect();
        futures::executor::block_on(async {
            redis_pool.wait_for_connect().await.unwrap();
        });
        info!("connect to redis successfully");

        let redlock = redlock::RedLock::new(vec![redis_url.clone()]);
        RedisCachePool {
            client: redis_pool,
            redlock: redlock,
        }
    })
}

pub fn get_redis_pool() -> &'static RedisCachePool {
    REDISPOOL.get().unwrap()
}

const REDIS_KEY_PREFIX: &str = "KvCache";

pub async fn get_kv_cache(key: &String) -> anyhow::Result<String> {
    let redis = get_redis_pool();
    let key = format!("{}:{}", REDIS_KEY_PREFIX, key);
    let res: String = redis.client.get(&key).await?;
    Ok(res)
}

pub async fn set_kv_cache(key: &String, value: &String, ex: Option<i64>) -> anyhow::Result<()> {
    let redis = get_redis_pool();
    let key = format!("{}:{}", REDIS_KEY_PREFIX, key);
    let _res = if ex.is_none() {
        let res: RedisValue = redis
            .client
            .set(&key, value.clone(), None, None, false)
            .await?;
        res
    } else {
        let res: RedisValue = redis
            .client
            .set(&key, value.clone(), Some(EX(ex.unwrap())), None, false)
            .await?;
        res
    };
    Ok(())
}

pub async fn delete_kv_cache(key: &String) -> anyhow::Result<()> {
    let redis = get_redis_pool();
    let key = format!("{}:{}", REDIS_KEY_PREFIX, key);
    let _res: String = redis.client.del(&key).await?;
    Ok(())
}
